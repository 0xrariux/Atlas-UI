import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const manifestPath = path.join(root, "docs/atlas-ui-agent-manifest.json");
const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
const index = fs.readFileSync(path.join(root, manifest.documentation.component_index), "utf8");
const cargo = fs.readFileSync(path.join(root, "Cargo.toml"), "utf8");

if (manifest.schema_version !== 2 || !cargo.includes(`version = "${manifest.version}"`)) {
  throw new Error("Agent manifest version differs from the workspace");
}
if (!cargo.includes(`slint = "=${manifest.slint_version}"`)) {
  throw new Error("Agent manifest Slint version differs from the workspace pin");
}

const stable = new Set(manifest.stable_components);
const preview = new Set(manifest.preview_components);
if (stable.size !== manifest.counts.stable_components || preview.size !== manifest.counts.preview_components) {
  throw new Error("Agent manifest component counts are stale");
}
for (const component of stable) if (preview.has(component)) throw new Error(`Duplicate maturity: ${component}`);
if (stable.size + preview.size !== manifest.counts.components) throw new Error("Agent component total is stale");

const facadeText = Object.fromEntries(Object.entries(manifest.facades).map(([name, relative]) => {
  const absolute = path.join(root, relative);
  if (!fs.existsSync(absolute)) throw new Error(`Missing ${name} facade: ${relative}`);
  return [name, fs.readFileSync(absolute, "utf8")];
}));
const contains = (source, symbol) => new RegExp(`\\b${symbol}\\b`).test(source);
for (const component of stable) {
  if (!contains(facadeText.stable, component) || contains(facadeText.preview, component)) {
    throw new Error(`Stable agent classification mismatch: ${component}`);
  }
  if (!contains(index, component)) throw new Error(`Stable component missing from agent index: ${component}`);
}
for (const component of preview) {
  if (!contains(facadeText.preview, component) || contains(facadeText.stable, component)) {
    throw new Error(`Preview agent classification mismatch: ${component}`);
  }
  if (!contains(index, component)) throw new Error(`Preview component missing from agent index: ${component}`);
}
for (const component of [...stable, ...preview]) {
  if (!contains(facadeText.aggregate, component)) throw new Error(`Component missing from aggregate facade: ${component}`);
}

if (manifest.api.components.length !== manifest.counts.components) {
  throw new Error("Agent component signatures are incomplete");
}
for (const component of manifest.api.components) {
  if (!stable.has(component.name) && !preview.has(component.name)) {
    throw new Error(`Unclassified component signature: ${component.name}`);
  }
  if (!fs.existsSync(path.join(root, component.source))) {
    throw new Error(`Missing component signature source: ${component.source}`);
  }
  if (!Array.isArray(component.properties) || !Array.isArray(component.callbacks) || !component.minimal_example) {
    throw new Error(`Incomplete component signature: ${component.name}`);
  }
  if (new Set(component.properties.map((property) => property.name)).size !== component.properties.length) {
    throw new Error(`Duplicate component property signature: ${component.name}`);
  }
  if (new Set(component.callbacks.map((callback) => callback.name)).size !== component.callbacks.length) {
    throw new Error(`Duplicate component callback signature: ${component.name}`);
  }
}
const dataTable = manifest.api.components.find((component) => component.name === "AtlasDataTable");
if (!dataTable.properties.some((property) => property.name === "rows")
  || !dataTable.callbacks.some((callback) => callback.name === "selection-requested")) {
  throw new Error("Agent parser lost same-line AtlasDataTable declarations");
}
const button = manifest.api.components.find((component) => component.name === "AtlasButton");
if (!button.properties.some((property) => property.name === "density" && property.inherited)) {
  throw new Error("Agent parser lost inherited AtlasButton properties");
}

for (const relative of Object.values(manifest.documentation)) {
  if (!fs.existsSync(path.join(root, relative))) throw new Error(`Missing agent documentation target: ${relative}`);
}

console.log(`Public agent kit valid: ${stable.size} stable and ${preview.size} preview components.`);
