import childProcess from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const manifest = JSON.parse(fs.readFileSync(path.join(root, "screenshots/scenarios.json"), "utf8"));
const budgets = JSON.parse(fs.readFileSync(path.join(root, "scripts/render-performance-budgets.json"), "utf8"));
const cargo = process.env.CARGO_BIN ?? "cargo";
const extension = process.platform === "win32" ? ".exe" : "";
const galleryBinary = path.join(root, "target", "debug", `atlas-ui-gallery${extension}`);
const temporaryDirectory = fs.mkdtempSync(path.join(os.tmpdir(), "atlas-render-performance-"));

function run(command, args, options = {}) {
  const result = childProcess.spawnSync(command, args, {
    cwd: root,
    encoding: "utf8",
    stdio: options.quiet ? "pipe" : "inherit",
    env: options.env ?? process.env,
  });
  if (result.status !== 0) throw new Error(`${command} failed with status ${result.status}: ${result.stderr ?? ""}`);
}

function percentile(sorted, percentage) {
  return sorted[Math.max(0, Math.ceil(sorted.length * percentage) - 1)];
}

function pngSize(file) {
  const data = fs.readFileSync(file);
  if (data.length < 24 || data.toString("ascii", 1, 4) !== "PNG") throw new Error(`Invalid performance capture: ${file}`);
  return { width: data.readUInt32BE(16), height: data.readUInt32BE(20) };
}

function environmentFor(scenario, output) {
  const environment = {
    ...process.env,
    ATLAS_UI_GALLERY_CAPTURE: output,
    ATLAS_UI_GALLERY_PAGE: scenario.page,
    ATLAS_UI_GALLERY_DENSITY: scenario.density,
    ATLAS_UI_GALLERY_MOTION: "reduced",
    ATLAS_UI_GALLERY_TYPOGRAPHY_SCALE: scenario.typography_scale ?? "normal",
    ATLAS_UI_GALLERY_WIDTH: String(scenario.viewport.width),
    ATLAS_UI_GALLERY_HEIGHT: String(scenario.viewport.height),
    ATLAS_UI_GALLERY_DELAY_MS: "100",
    SLINT_BACKEND: "software",
    SLINT_SCALE_FACTOR: "1",
  };
  if (scenario.theme === "light") environment.ATLAS_UI_GALLERY_LIGHT = "1";
  if (scenario.theme_mode === "system") environment.ATLAS_UI_GALLERY_SYSTEM_THEME = "1";
  if (scenario.system_dark) environment.ATLAS_UI_GALLERY_SYSTEM_DARK = "1";
  return environment;
}

try {
  // Compilation is deliberately completed before the first clock starts.
  run(cargo, ["build", "-p", "atlas-ui-gallery"], { quiet: false });
  const scenarioById = new Map(manifest.scenarios.map((scenario) => [scenario.id, scenario]));
  const results = [];

  for (const budget of budgets.render_budgets) {
    const scenario = scenarioById.get(budget.scenario);
    if (!scenario) throw new Error(`Unknown render budget scenario: ${budget.scenario}`);
    const output = path.join(temporaryDirectory, `${budget.id}.png`);
    for (let iteration = 0; iteration < budget.warmup_iterations; iteration += 1) {
      run(galleryBinary, [], { quiet: true, env: environmentFor(scenario, output) });
    }
    const samples = [];
    for (let iteration = 0; iteration < budget.sample_count; iteration += 1) {
      const started = process.hrtime.bigint();
      run(galleryBinary, [], { quiet: true, env: environmentFor(scenario, output) });
      samples.push(Number(process.hrtime.bigint() - started) / 1_000_000);
      const dimensions = pngSize(output);
      if (dimensions.width !== scenario.viewport.width || dimensions.height !== scenario.viewport.height) {
        throw new Error(`Render budget produced invalid dimensions: ${budget.id}`);
      }
    }
    samples.sort((left, right) => left - right);
    const result = {
      id: budget.id,
      scenario: budget.scenario,
      warmup_iterations: budget.warmup_iterations,
      sample_count: samples.length,
      median_limit_ms: budget.median_limit_ms,
      minimum_ms: samples[0],
      median_ms: samples[Math.floor(samples.length / 2)],
      p95_ms: percentile(samples, 0.95),
      maximum_ms: samples[samples.length - 1],
      samples_ms: samples,
    };
    if (result.median_ms >= result.median_limit_ms) throw new Error(`Render median exceeded budget: ${budget.id} ${result.median_ms.toFixed(2)}ms`);
    results.push(result);
  }

  const report = {
    schema_version: 1,
    platform_profile: budgets.platform_profile,
    renderer: "software",
    scale_factor: 1,
    compilation_included: false,
    generated_at: new Date().toISOString(),
    results,
  };
  const reportPath = path.join(root, "screenshots/performance/local-render.json");
  fs.mkdirSync(path.dirname(reportPath), { recursive: true });
  fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
  console.log(`Render performance measured: ${results.length} scenarios, report ${path.relative(root, reportPath)}.`);
} finally {
  for (const entry of fs.readdirSync(temporaryDirectory)) fs.unlinkSync(path.join(temporaryDirectory, entry));
  fs.rmdirSync(temporaryDirectory);
}
