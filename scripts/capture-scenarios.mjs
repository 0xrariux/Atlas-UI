import childProcess from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const screenshots = path.join(root, "screenshots");
const manifest = JSON.parse(fs.readFileSync(path.join(screenshots, "scenarios.json"), "utf8"));
const argumentsList = process.argv.slice(2);
const updateBaselines = argumentsList.includes("--update-baselines");
const validateOnly = argumentsList.includes("--validate-only");
const scenarioArgument = valueAfter("--scenario");
const approvalArgument = valueAfter("--approve-baseline");
const reviewer = valueAfter("--reviewer");
const approvalNote = valueAfter("--note") ?? "Baseline reviewed against the declared fixture.";
const cargo = process.env.CARGO_BIN ?? "cargo";

function valueAfter(flag) {
  const index = argumentsList.indexOf(flag);
  return index >= 0 ? argumentsList[index + 1] : undefined;
}

function run(command, args, options = {}) {
  const result = childProcess.spawnSync(command, args, {
    cwd: root,
    encoding: "utf8",
    stdio: options.capture ? "pipe" : "inherit",
    env: options.env ?? process.env,
  });
  if (result.status !== 0 && !options.allowFailure) {
    if (options.capture) process.stderr.write(result.stderr ?? "");
    throw new Error(`${command} failed with status ${result.status}`);
  }
  return result;
}

function identity(scenario) {
  const result = {
    schema_version: manifest.schema_version,
    id: scenario.id,
    page: scenario.page,
    fixture: scenario.fixture,
    theme: scenario.theme,
    density: scenario.density,
    motion: scenario.motion,
    grid: scenario.grid,
    viewport: scenario.viewport,
    renderer: scenario.renderer,
    scale_factor: scenario.scale_factor,
  };
  if (scenario.theme_mode) result.theme_mode = scenario.theme_mode;
  if (typeof scenario.system_dark === "boolean") result.system_dark = scenario.system_dark;
  if (scenario.typography_scale) result.typography_scale = scenario.typography_scale;
  return result;
}

function pngSize(file) {
  const data = fs.readFileSync(file);
  if (data.length < 24 || data.toString("ascii", 1, 4) !== "PNG") throw new Error(`Not a PNG: ${file}`);
  return { width: data.readUInt32BE(16), height: data.readUInt32BE(20) };
}

function stableJson(value) {
  return JSON.stringify(value);
}

const byId = new Map();
for (const scenario of manifest.scenarios) {
  if (!scenario.id || byId.has(scenario.id)) throw new Error(`Invalid or duplicate scenario: ${scenario.id}`);
  byId.set(scenario.id, scenario);
}

if (approvalArgument) {
  if (!reviewer) throw new Error("--approve-baseline requires --reviewer");
  if (!byId.has(approvalArgument)) throw new Error(`Unknown scenario: ${approvalArgument}`);
  const metadataPath = path.join(screenshots, "metadata", `${approvalArgument}.baseline.json`);
  if (!fs.existsSync(metadataPath)) throw new Error(`Missing baseline metadata: ${approvalArgument}`);
  const metadata = JSON.parse(fs.readFileSync(metadataPath, "utf8"));
  if (stableJson(metadata.identity) !== stableJson(identity(byId.get(approvalArgument)))) {
    throw new Error(`Refusing approval: scenario identity changed for ${approvalArgument}`);
  }
  metadata.approval = {
    status: "approved",
    reviewer,
    note: approvalNote,
    approved_at: new Date().toISOString(),
  };
  fs.writeFileSync(metadataPath, `${JSON.stringify(metadata, null, 2)}\n`);
  console.log(`Approved baseline ${approvalArgument} as ${reviewer}.`);
  process.exit(0);
}

if (validateOnly) {
  console.log(`Capture manifest valid: ${manifest.scenarios.length} scenarios.`);
  process.exit(0);
}

const selected = scenarioArgument ? [byId.get(scenarioArgument)] : manifest.scenarios;
if (selected.some((scenario) => !scenario)) throw new Error(`Unknown scenario: ${scenarioArgument}`);

for (const directory of ["baselines", "results", "diffs", "metadata"]) {
  fs.mkdirSync(path.join(screenshots, directory), { recursive: true });
}

run(cargo, ["build", "-p", "atlas-ui-gallery", "-p", "atlas-ui-testing", "--bins"]);
const extension = process.platform === "win32" ? ".exe" : "";
const galleryBinary = path.join(root, "target", "debug", `atlas-ui-gallery${extension}`);
const compareBinary = path.join(root, "target", "debug", `visual_compare${extension}`);

for (const scenario of selected) {
  const currentIdentity = identity(scenario);
  const resultPath = path.join(screenshots, "results", `${scenario.id}.png`);
  const resultMetadataPath = path.join(screenshots, "results", `${scenario.id}.json`);
  const baselinePath = path.join(screenshots, "baselines", `${scenario.id}.png`);
  const baselineMetadataPath = path.join(screenshots, "metadata", `${scenario.id}.baseline.json`);
  const diffPath = path.join(screenshots, "diffs", `${scenario.id}.png`);

  if (!updateBaselines) {
    if (!fs.existsSync(baselinePath) || !fs.existsSync(baselineMetadataPath)) {
      throw new Error(`Missing baseline for ${scenario.id}; run with --update-baselines`);
    }
    const baselineMetadata = JSON.parse(fs.readFileSync(baselineMetadataPath, "utf8"));
    if (stableJson(baselineMetadata.identity) !== stableJson(currentIdentity)) {
      throw new Error(`Refusing invalid comparison: metadata identity differs for ${scenario.id}`);
    }
  }

  const captureEnvironment = {
    ...process.env,
    ATLAS_UI_GALLERY_CAPTURE: resultPath,
    ATLAS_UI_GALLERY_PAGE: scenario.page,
    ATLAS_UI_GALLERY_DENSITY: scenario.density,
    ATLAS_UI_GALLERY_MOTION: scenario.motion,
    ATLAS_UI_GALLERY_TYPOGRAPHY_SCALE: scenario.typography_scale ?? "normal",
    ATLAS_UI_GALLERY_WIDTH: String(scenario.viewport.width),
    ATLAS_UI_GALLERY_HEIGHT: String(scenario.viewport.height),
    ATLAS_UI_GALLERY_DELAY_MS: "300",
    SLINT_BACKEND: scenario.renderer,
    SLINT_SCALE_FACTOR: String(scenario.scale_factor),
  };
  if (scenario.theme === "light") captureEnvironment.ATLAS_UI_GALLERY_LIGHT = "1";
  else delete captureEnvironment.ATLAS_UI_GALLERY_LIGHT;
  if (scenario.theme_mode === "system") captureEnvironment.ATLAS_UI_GALLERY_SYSTEM_THEME = "1";
  else delete captureEnvironment.ATLAS_UI_GALLERY_SYSTEM_THEME;
  if (scenario.system_dark) captureEnvironment.ATLAS_UI_GALLERY_SYSTEM_DARK = "1";
  else delete captureEnvironment.ATLAS_UI_GALLERY_SYSTEM_DARK;
  if (scenario.grid) captureEnvironment.ATLAS_UI_GALLERY_GRID = "1";
  else delete captureEnvironment.ATLAS_UI_GALLERY_GRID;

  run(galleryBinary, [], { env: captureEnvironment });
  const dimensions = pngSize(resultPath);
  const expectedDimensions = scenario.viewport;
  if (dimensions.width !== expectedDimensions.width || dimensions.height !== expectedDimensions.height) {
    throw new Error(
      `Capture scale mismatch for ${scenario.id}: expected ${expectedDimensions.width}x${expectedDimensions.height}, got ${dimensions.width}x${dimensions.height}`,
    );
  }
  const resultMetadata = {
    identity: currentIdentity,
    image: dimensions,
    platform: process.platform,
    architecture: process.arch,
  };
  fs.writeFileSync(resultMetadataPath, `${JSON.stringify(resultMetadata, null, 2)}\n`);

  if (updateBaselines) {
    fs.copyFileSync(resultPath, baselinePath);
    fs.writeFileSync(
      baselineMetadataPath,
      `${JSON.stringify({
        identity: currentIdentity,
        image: dimensions,
        platform: process.platform,
        architecture: process.arch,
        approval: { status: "pending-human", reviewer: null, note: null, approved_at: null },
      }, null, 2)}\n`,
    );
    console.log(`Updated baseline ${scenario.id} (pending human approval).`);
    continue;
  }

  const comparison = run(
    compareBinary,
    [baselinePath, resultPath, diffPath, String(scenario.threshold)],
    { capture: true, allowFailure: true },
  );
  if (comparison.stdout) process.stdout.write(`${scenario.id}: ${comparison.stdout}`);
  if (comparison.status === 2) throw new Error(`Invalid visual comparison for ${scenario.id}: ${comparison.stderr}`);
  if (comparison.status !== 0) throw new Error(`Visual regression detected for ${scenario.id}`);
}
