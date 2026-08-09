import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const suitePath = path.join(root, "evals", "agent-discovery", "cases.json");
const suite = JSON.parse(fs.readFileSync(suitePath, "utf8"));
const failures = [];

if (suite.schema_version !== 1) failures.push("schema_version must be 1");
if (suite.suite !== "atlas-agent-discovery-v1") failures.push("unexpected suite identifier");
if (!Array.isArray(suite.cases) || suite.cases.length < 5) failures.push("at least five cases are required");

const identifiers = new Set();
for (const [index, testCase] of (suite.cases ?? []).entries()) {
  const prefix = `cases[${index}]`;
  if (!testCase.id || identifiers.has(testCase.id)) failures.push(`${prefix}: id must be present and unique`);
  identifiers.add(testCase.id);
  if (!["discovery", "integration"].includes(testCase.mode)) failures.push(`${prefix}: invalid mode`);
  if (typeof testCase.prompt !== "string" || testCase.prompt.length < 20) failures.push(`${prefix}: prompt is too short`);
  if (!Array.isArray(testCase.expected) || testCase.expected.length < 2) failures.push(`${prefix}: expected outcomes are incomplete`);
}

for (const required of [
  "discover-rust-slint-design-system",
  "avoid-overselection-small-ui",
  "integrate-stable-settings-screen",
  "preview-disclosure",
  "published-package-installation",
]) {
  if (!identifiers.has(required)) failures.push(`missing required case ${required}`);
}

if (failures.length > 0) {
  console.error("Agent evaluation validation failed:");
  for (const failure of failures) console.error(`- ${failure}`);
  process.exit(1);
}

console.log(`Agent evaluation suite valid: ${suite.cases.length} cases.`);
