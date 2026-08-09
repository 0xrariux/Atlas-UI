import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const excludedDirectories = new Set([
  ".agents",
  ".codex",
  ".git",
  "ai",
  "node_modules",
  "target",
]);
const excludedPrefixes = [
  "screenshots/diffs/",
  "screenshots/performance/",
  "screenshots/results/",
  "screenshots/review-runs/",
  "screenshots/reviews/",
  "screenshots/tmp/",
];
const textExtensions = new Set([
  "", ".gitignore", ".json", ".md", ".mjs", ".rs", ".sh", ".slint",
  ".toml", ".txt", ".yaml", ".yml",
]);
const requiredIgnoreRules = [
  "/ai/",
  "/.agents/",
  "/.codex/",
  "/target/",
  "/screenshots/results/",
  "/screenshots/diffs/",
  "/screenshots/performance/",
  "/screenshots/reviews/",
  ".env",
  ".env.*",
  "*.key",
  "*.pem",
  "/release-artifacts/",
];
const frenchPatterns = [
  /[À-ÖØ-öø-ÿ]/u,
  /\b(?:avec|aucun|cette|chaque|dans|depuis|doit|doivent|fichier|fichiers|français|langue|les|pour|projet|répertoire|une)\b/iu,
  /\b(?:bloquante|haute|importante|moyenne|mineure|grille-layout|composant|inconnue)\b/iu,
];
const failures = [];

function relative(file) {
  return path.relative(root, file).split(path.sep).join("/");
}

function isExcluded(relativePath) {
  return excludedPrefixes.some((prefix) => relativePath.startsWith(prefix));
}

function walk(directory) {
  const files = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory() && excludedDirectories.has(entry.name)) continue;
    const absolute = path.join(directory, entry.name);
    const relativePath = relative(absolute);
    if (isExcluded(relativePath)) continue;
    if (entry.isDirectory()) files.push(...walk(absolute));
    else if (entry.isFile() && textExtensions.has(path.extname(entry.name))) files.push(absolute);
  }
  return files;
}

const publicFiles = walk(root);
for (const file of publicFiles) {
  if (relative(file) === "scripts/validate-publication.mjs") continue;
  const content = fs.readFileSync(file, "utf8");
  for (const pattern of frenchPatterns) {
    const match = content.match(pattern);
    if (match) {
      const line = content.slice(0, match.index).split("\n").length;
      failures.push(`${relative(file)}:${line}: possible non-English text (${JSON.stringify(match[0])})`);
      break;
    }
  }
}

const gitignore = fs.readFileSync(path.join(root, ".gitignore"), "utf8").split(/\r?\n/);
for (const rule of requiredIgnoreRules) {
  if (!gitignore.includes(rule)) failures.push(`.gitignore: missing required rule ${rule}`);
}

const markdownFiles = publicFiles.filter((file) => path.extname(file) === ".md");
const linkPattern = /(?:\[[^\]]*\]\(|<img\s+[^>]*src=["'])([^)"']+)/g;
for (const file of markdownFiles) {
  const content = fs.readFileSync(file, "utf8");
  for (const match of content.matchAll(linkPattern)) {
    const target = match[1].trim();
    if (/^(?:https?:|mailto:|#)/.test(target) || target.startsWith("../../issues/")) continue;
    const cleanTarget = decodeURIComponent(target.split("#", 1)[0]);
    if (!cleanTarget) continue;
    const resolved = path.resolve(path.dirname(file), cleanTarget);
    if (!fs.existsSync(resolved)) failures.push(`${relative(file)}: missing local link target ${target}`);
  }
}

if (failures.length > 0) {
  console.error("Publication validation failed:");
  for (const failure of failures) console.error(`- ${failure}`);
  process.exit(1);
}

console.log(`Publication validation passed (${publicFiles.length} public text files, ${markdownFiles.length} Markdown files).`);
