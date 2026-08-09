import childProcess from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const screenshots = path.join(root, "screenshots");
const manifestPath = path.join(screenshots, "scenarios.json");
const promptPath = path.join(screenshots, "visual-review.prompt.md");
const schemaPath = path.join(screenshots, "visual-review.schema.json");
const contextsPath = path.join(screenshots, "review-contexts.json");
const reviewsDirectory = path.join(screenshots, "reviews");
const statePath = path.join(reviewsDirectory, "state.json");
const argumentsList = process.argv.slice(2);
const requestedScenario = valueAfter("--scenario");
const reset = argumentsList.includes("--reset");
const rerunLastBatch = argumentsList.includes("--rerun-last-batch");
const dryRun = argumentsList.includes("--dry-run");
const batchSize = parseBatchSize(valueAfter("--batch-size") ?? "4");

function valueAfter(flag) {
  const index = argumentsList.indexOf(flag);
  return index >= 0 ? argumentsList[index + 1] : undefined;
}

function parseBatchSize(rawValue) {
  const value = Number.parseInt(rawValue, 10);
  if (!Number.isInteger(value) || value < 1 || value > 4) {
    throw new Error("--batch-size must be an integer between 1 and 4");
  }
  return value;
}

function run(command, args, options = {}) {
  const result = childProcess.spawnSync(command, args, {
    cwd: root,
    encoding: "utf8",
    input: options.input,
    stdio: ["pipe", "pipe", "pipe"],
    maxBuffer: 16 * 1024 * 1024,
    env: process.env,
  });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    if (result.stdout) process.stderr.write(result.stdout);
    if (result.stderr) process.stderr.write(result.stderr);
    throw new Error(`${command} failed with status ${result.status}`);
  }
  return result;
}

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, "utf8"));
}

function writeJson(file, value) {
  fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`);
}

function interpolate(template, values) {
  return template.replace(/\{\{([a-z_]+)\}\}/g, (_, key) => {
    if (!(key in values)) throw new Error(`Missing prompt variable: ${key}`);
    return String(values[key]);
  });
}

function familyFor(scenario) {
  if (scenario.page.startsWith("documentation") || scenario.page === "document-tools") return "documentation";
  if (["editorial", "rich-content", "markdown-presentation", "footnotes", "reference-system", "selection-copy"].includes(scenario.page)) return "editorial content";
  if (scenario.page.includes("template") || scenario.page === "web-proof") return "templates";
  if (["navigation", "interactions"].includes(scenario.page)) return "navigation and interactions";
  if (scenario.page === "data") return "data";
  return "foundations and components";
}

function bulletList(values) {
  return values.map((value) => `- ${value}`).join("\n");
}

fs.mkdirSync(reviewsDirectory, { recursive: true });
if (reset && fs.existsSync(statePath)) fs.rmSync(statePath);
run("codex", ["login", "status"]);

const manifest = readJson(manifestPath);
const reviewContexts = readJson(contextsPath);
const promptTemplate = fs.readFileSync(promptPath, "utf8");
const scenarioById = new Map(manifest.scenarios.map((scenario) => [scenario.id, scenario]));
const previousState = fs.existsSync(statePath)
  ? readJson(statePath)
  : { schema_version: 1, prompt_version: "atlas-visual-review-v3", completed: [] };
const completed = new Set(previousState.completed);

if (requestedScenario && !scenarioById.has(requestedScenario)) throw new Error(`Unknown scenario: ${requestedScenario}`);
const completedInOrder = manifest.scenarios.filter((scenario) => completed.has(scenario.id));
const candidates = requestedScenario
  ? [scenarioById.get(requestedScenario)]
  : rerunLastBatch
    ? completedInOrder.slice(-batchSize)
    : manifest.scenarios.filter((scenario) => !completed.has(scenario.id));
const selected = candidates.slice(0, batchSize);

if (selected.length === 0) {
  console.log("All visual scenarios have a Codex pre-review. Use --reset to start over.");
  process.exit(0);
}

console.log(`Visual review batch (${selected.length}/4 maximum):`);
for (const scenario of selected) console.log(`- ${scenario.id}`);
if (dryRun) process.exit(0);

const cargoManifest = fs.readFileSync(path.join(root, "Cargo.toml"), "utf8");
const atlasVersion = cargoManifest.match(/^version\s*=\s*"([^"]+)"/m)?.[1] ?? "unknown";
const slintVersion = cargoManifest.match(/^slint\s*=\s*[^\n]*"([^"]+)"/m)?.[1] ?? "1.17.1";
const batchReports = [];

for (const scenario of selected) {
  const imagePath = path.join(screenshots, "baselines", `${scenario.id}.png`);
  const metadataPath = path.join(screenshots, "metadata", `${scenario.id}.baseline.json`);
  const reportPath = path.join(reviewsDirectory, `${scenario.id}.json`);
  if (!fs.existsSync(imagePath)) throw new Error(`Missing screenshot: ${imagePath}`);
  if (!fs.existsSync(metadataPath)) throw new Error(`Missing metadata: ${metadataPath}`);
  const metadata = readJson(metadataPath);
  const reviewContext = reviewContexts.pages[scenario.page] ?? reviewContexts.default;
  const prompt = interpolate(promptTemplate, {
    scenario_id: scenario.id,
    family: familyFor(scenario),
    theme: scenario.theme,
    density: scenario.density,
    viewport_width: scenario.viewport.width,
    viewport_height: scenario.viewport.height,
    reduced_motion: scenario.motion === "reduced",
    state: scenario.fixture,
    atlas_version: atlasVersion,
    slint_version: slintVersion,
    metadata_json: JSON.stringify(metadata.identity),
    fixture_intent: reviewContext.intent,
    intentional_traits: bulletList(reviewContext.intentional_traits),
    invariants: bulletList(reviewContext.invariants),
    exclusions: bulletList(reviewContext.exclusions),
  });

  console.log(`Reviewing ${scenario.id}...`);
  run("codex", [
    "exec", "--ephemeral", "--sandbox", "read-only", "--skip-git-repo-check",
    "--image", imagePath, "--output-schema", schemaPath,
    "--output-last-message", reportPath, "-",
  ], { input: prompt });

  const report = readJson(reportPath);
  if (report.scenario_id !== scenario.id) throw new Error(`Report scenario mismatch: expected ${scenario.id}, got ${report.scenario_id}`);
  batchReports.push(report);
  completed.add(scenario.id);
  writeJson(statePath, {
    schema_version: 1,
    prompt_version: "atlas-visual-review-v3",
    completed: manifest.scenarios.map((item) => item.id).filter((id) => completed.has(id)),
    updated_at: new Date().toISOString(),
  });
}

const batchPath = path.join(reviewsDirectory, `batch-${new Date().toISOString().replace(/[:.]/g, "-")}.json`);
const defectCandidates = batchReports.flatMap((report) => report.observations
  .filter((observation) => observation.classification === "defect"
    && observation.auto_correction_eligible
    && observation.confidence >= 0.9
    && observation.violated_contract.trim() !== "")
  .map((observation) => ({ scenario_id: report.scenario_id, ...observation })));
function consensusKeyFor(observation) {
  if (observation.scope.kind === "probably-shared") {
    return `contract:${observation.violated_contract.trim().toLocaleLowerCase("fr")}`;
  }
  return `local:${observation.scenario_id}:${observation.consensus_key}`;
}
const consensusScenarios = new Map();
for (const observation of defectCandidates) {
  const consensusKey = consensusKeyFor(observation);
  const scenarios = consensusScenarios.get(consensusKey) ?? new Set();
  scenarios.add(observation.scenario_id);
  consensusScenarios.set(consensusKey, scenarios);
}
const actionableObservations = defectCandidates
  .filter((observation) => observation.scope.kind === "local"
    || (observation.scope.kind === "probably-shared" && consensusScenarios.get(consensusKeyFor(observation)).size >= 2))
  .map((observation) => ({
    scenario_id: observation.scenario_id,
    observation_id: observation.id,
    consensus_key: observation.consensus_key,
    consensus_count: consensusScenarios.get(consensusKeyFor(observation)).size,
    probable_source_layer: observation.probable_source_layer,
    target: observation.scope.target,
  }));
writeJson(batchPath, {
  schema_version: 1,
  prompt_version: "atlas-visual-review-v3",
  generated_at: new Date().toISOString(),
  scenarios: batchReports.map((report) => ({ scenario_id: report.scenario_id, verdict: report.verdict, score: report.score, priority_issues: report.priority_issues })),
  correction_gate: {
    candidate_count: defectCandidates.length,
    actionable_count: actionableObservations.length,
    actionable_observations: actionableObservations,
  },
});
console.log(`Batch complete: ${batchPath}`);
console.log("Stopped after this batch. Run the same command manually for the next four scenarios.");
