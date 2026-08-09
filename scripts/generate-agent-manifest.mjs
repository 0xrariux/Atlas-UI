import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const outputPath = path.join(root, "docs/atlas-ui-agent-manifest.json");
const checkOnly = process.argv.includes("--check");

const relative = (absolute) => path.relative(root, absolute).split(path.sep).join("/");
const normalize = (value) => value
  .replace(/\/\*[\s\S]*?\*\//g, " ")
  .replace(/\/\/[^\n]*/g, " ")
  .replace(/\s+/g, " ")
  .trim();

function walk(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const absolute = path.join(directory, entry.name);
    return entry.isDirectory() ? walk(absolute) : [absolute];
  });
}

function findClosingBrace(source, opening) {
  let depth = 0;
  let quote = null;
  let lineComment = false;
  let blockComment = false;
  for (let index = opening; index < source.length; index += 1) {
    const character = source[index];
    const next = source[index + 1];
    if (lineComment) {
      if (character === "\n") lineComment = false;
      continue;
    }
    if (blockComment) {
      if (character === "*" && next === "/") {
        blockComment = false;
        index += 1;
      }
      continue;
    }
    if (quote) {
      if (character === "\\") {
        index += 1;
      } else if (character === quote) {
        quote = null;
      }
      continue;
    }
    if (character === "/" && next === "/") {
      lineComment = true;
      index += 1;
    } else if (character === "/" && next === "*") {
      blockComment = true;
      index += 1;
    } else if (character === '"' || character === "'") {
      quote = character;
    } else if (character === "{") {
      depth += 1;
    } else if (character === "}") {
      depth -= 1;
      if (depth === 0) return index;
    }
  }
  throw new Error(`Unclosed declaration at offset ${opening}`);
}

function declarationBlocks(source, expression) {
  const blocks = [];
  for (const match of source.matchAll(expression)) {
    const opening = source.indexOf("{", match.index);
    const closing = findClosingBrace(source, opening);
    blocks.push({ match, opening, closing, body: source.slice(opening + 1, closing) });
  }
  return blocks;
}

function parseProperties(body, declaredIn) {
  const properties = [];
  const expression = /\b(in-out|in|out)\s+property\s*<([^>]+)>\s+([A-Za-z][A-Za-z0-9-]*)\s*(?::\s*([\s\S]*?))?;/g;
  for (const match of body.matchAll(expression)) {
    properties.push({
      name: match[3],
      type: normalize(match[2]),
      direction: match[1],
      default: match[4] === undefined ? null : normalize(match[4]),
      declared_in: declaredIn,
    });
  }
  return properties;
}

function parseCallbacks(body, declaredIn) {
  const callbacks = [];
  const expression = /\bcallback\s+([A-Za-z][A-Za-z0-9-]*)\s*\(([\s\S]*?)\)\s*(?:->\s*([^;]+))?;/g;
  for (const match of body.matchAll(expression)) {
    const parameters = normalize(match[2]);
    callbacks.push({
      name: match[1],
      parameters: parameters === "" ? [] : parameters.split(",").map((type) => normalize(type)),
      return_type: match[3] === undefined ? null : normalize(match[3]),
      declared_in: declaredIn,
    });
  }
  return callbacks;
}

function parseDefinitions(files) {
  const definitions = new Map();
  for (const absolute of files) {
    const source = fs.readFileSync(absolute, "utf8");
    const sourcePath = relative(absolute);

    for (const block of declarationBlocks(source, /export\s+component\s+([A-Za-z][A-Za-z0-9]*)\s+inherits\s+([A-Za-z][A-Za-z0-9]*)\s*\{/g)) {
      const name = block.match[1];
      definitions.set(name, {
        kind: "component",
        name,
        inherits: block.match[2],
        source: sourcePath,
        source_line: source.slice(0, block.match.index).split("\n").length,
        declared_properties: parseProperties(block.body, name),
        declared_callbacks: parseCallbacks(block.body, name),
      });
    }

    for (const block of declarationBlocks(source, /export\s+global\s+([A-Za-z][A-Za-z0-9]*)\s*\{/g)) {
      const name = block.match[1];
      definitions.set(name, {
        kind: "global",
        name,
        source: sourcePath,
        source_line: source.slice(0, block.match.index).split("\n").length,
        properties: parseProperties(block.body, name),
        callbacks: parseCallbacks(block.body, name),
      });
    }

    for (const block of declarationBlocks(source, /export\s+enum\s+([A-Za-z][A-Za-z0-9]*)\s*\{/g)) {
      const name = block.match[1];
      definitions.set(name, {
        kind: "enum",
        name,
        source: sourcePath,
        source_line: source.slice(0, block.match.index).split("\n").length,
        values: normalize(block.body).split(",").map((value) => value.trim()).filter(Boolean),
      });
    }

    for (const block of declarationBlocks(source, /export\s+struct\s+([A-Za-z][A-Za-z0-9]*)\s*\{/g)) {
      const name = block.match[1];
      const fields = [];
      for (const field of block.body.matchAll(/([A-Za-z][A-Za-z0-9-]*)\s*:\s*([^,}\n]+)/g)) {
        fields.push({ name: field[1], type: normalize(field[2]) });
      }
      definitions.set(name, {
        kind: "struct",
        name,
        source: sourcePath,
        source_line: source.slice(0, block.match.index).split("\n").length,
        fields,
      });
    }
  }
  return definitions;
}

function facadeSymbols(relativePath) {
  const source = fs.readFileSync(path.join(root, relativePath), "utf8");
  const symbols = [];
  for (const match of source.matchAll(/export\s*\{([\s\S]*?)\}\s*from\s*"[^"]+"\s*;/g)) {
    symbols.push(...match[1].split(",").map((symbol) => normalize(symbol)).filter(Boolean));
  }
  return symbols;
}

function effectiveMembers(component, definitions, member) {
  const inherited = definitions.get(component.inherits);
  const inheritedMembers = inherited?.kind === "component"
    ? effectiveMembers(inherited, definitions, member)
    : [];
  const declared = component[member];
  const merged = new Map(inheritedMembers.map((item) => [item.name, item]));
  for (const item of declared) merged.set(item.name, item);
  return [...merged.values()].map((item) => ({
    ...item,
    inherited: item.declared_in !== component.name,
  }));
}

const facades = {
  stable: "crates/atlas-ui-components/ui/stable.slint",
  preview: "crates/atlas-ui-components/ui/preview.slint",
  aggregate: "crates/atlas-ui-components/ui/components.slint",
};
const slintFiles = [
  "crates/atlas-ui-tokens/ui",
  "crates/atlas-ui-core/ui",
  "crates/atlas-ui-icons/ui",
  "crates/atlas-ui-components/ui",
].flatMap((directory) => walk(path.join(root, directory))).filter((file) => file.endsWith(".slint"));
const definitions = parseDefinitions(slintFiles);
const stableSymbols = facadeSymbols(facades.stable);
const previewSymbols = facadeSymbols(facades.preview);
const maturity = new Map([
  ...stableSymbols.map((name) => [name, "stable"]),
  ...previewSymbols.map((name) => [name, "preview"]),
]);

const exported = [...new Set([...stableSymbols, ...previewSymbols])]
  .map((name) => definitions.get(name))
  .filter(Boolean);
const components = exported.filter((definition) => definition.kind === "component")
  .map((component) => {
    const level = maturity.get(component.name);
    const facade = level === "stable" ? "stable.slint" : "preview.slint";
    return {
      name: component.name,
      maturity: level,
      source: component.source,
      source_line: component.source_line,
      inherits: component.inherits,
      properties: effectiveMembers(component, definitions, "declared_properties"),
      callbacks: effectiveMembers(component, definitions, "declared_callbacks"),
      inputs_without_explicit_default: effectiveMembers(component, definitions, "declared_properties")
        .filter((property) => property.direction !== "out" && property.default === null)
        .map((property) => property.name),
      minimal_example: `import { ${component.name} } from "${facade}";\n\n${component.name} { }`,
    };
  }).sort((left, right) => left.name.localeCompare(right.name));
const types = exported.filter((definition) => definition.kind === "enum" || definition.kind === "struct")
  .map((definition) => ({ ...definition, maturity: maturity.get(definition.name) }))
  .sort((left, right) => left.name.localeCompare(right.name));
const globals = exported.filter((definition) => definition.kind === "global")
  .map((definition) => ({ ...definition, maturity: maturity.get(definition.name) }))
  .sort((left, right) => left.name.localeCompare(right.name));

const cargo = fs.readFileSync(path.join(root, "Cargo.toml"), "utf8");
const version = cargo.match(/\[workspace\.package\][\s\S]*?version\s*=\s*"([^"]+)"/)?.[1];
const slintVersion = cargo.match(/^slint\s*=\s*"=([^"]+)"/m)?.[1];
const scenarios = JSON.parse(fs.readFileSync(path.join(root, "screenshots/scenarios.json"), "utf8"));
if (!version || !slintVersion) throw new Error("Unable to read workspace or Slint version");

const stableComponents = components.filter((component) => component.maturity === "stable").map((component) => component.name);
const previewComponents = components.filter((component) => component.maturity === "preview").map((component) => component.name);
const stableTypes = types.filter((type) => type.maturity === "stable").map((type) => type.name);
const stableGlobals = globals.filter((global) => global.maturity === "stable").map((global) => global.name);
const manifest = {
  schema_version: 2,
  generated_by: "scripts/generate-agent-manifest.mjs",
  library: "Atlas UI",
  version,
  license: "MIT",
  language: ["Rust", "Slint"],
  slint_version: slintVersion,
  facades,
  documentation: {
    manifest_schema: "docs/AGENT_MANIFEST.md",
    component_index: "docs/AGENT_COMPONENT_INDEX.md",
    integration_guide: "docs/AI_INTEGRATION_GUIDE.md",
    architecture: "docs/ARCHITECTURE.md",
    compiled_example: "examples/getting-started",
    gallery: "apps/gallery",
  },
  ownership: {
    slint: ["presentation", "local-interaction-state", "rendering", "animation"],
    rust_host: ["domain-data", "navigation", "network", "filesystem", "clipboard", "persistence"],
    callback_semantics: "controlled-intent",
  },
  agent_workflow: [
    "Select a component by need using docs/AGENT_COMPONENT_INDEX.md.",
    "Prefer stable symbols and make preview dependencies explicit.",
    "Read the component signature in this manifest.",
    "Confirm final details in the referenced Slint source declaration.",
    "Compile the consumer and run scripts/quality-gate.sh.",
  ],
  stable_components: stableComponents,
  preview_components: previewComponents,
  stable_types: stableTypes,
  stable_globals: stableGlobals,
  counts: {
    components: components.length,
    stable_components: stableComponents.length,
    preview_components: previewComponents.length,
    stable_symbols: stableSymbols.length,
    preview_symbols: previewSymbols.length,
    visual_scenarios: scenarios.scenarios.length,
  },
  api: { components, types, globals },
};
const output = `${JSON.stringify(manifest, null, 2)}\n`;

if (checkOnly) {
  const current = fs.readFileSync(outputPath, "utf8");
  if (current !== output) {
    throw new Error("Agent manifest is stale; run node scripts/generate-agent-manifest.mjs");
  }
  console.log(`Agent manifest is current: ${components.length} component signatures, ${types.length} types, ${globals.length} globals.`);
} else {
  fs.writeFileSync(outputPath, output);
  console.log(`Wrote ${relative(outputPath)} with ${components.length} component signatures, ${types.length} types, and ${globals.length} globals.`);
}
