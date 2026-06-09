import { existsSync, mkdirSync, copyFileSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const dist = join(root, "dist");
const cart = join(root, "target/wasm32-unknown-unknown/release/falling_block.wasm");
const shellHtml = join(dist, "index.html");
const cartHtml = join(dist, "cart.html");
const appTitle = "Falling Block";
const appDescription = "A Tetris-like falling block puzzle for WASM-4 with offline PWA support, mobile controls, and handmade chiptune audio.";

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: root,
    stdio: "inherit",
    shell: false,
    ...options
  });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function resolveW4() {
  if (process.env.W4_CLI) {
    return { command: process.env.W4_CLI, args: [] };
  }

  const localCli = join(root, "node_modules/wasm4/cli.js");
  if (existsSync(localCli)) {
    return { command: process.execPath, args: [localCli] };
  }

  return { command: "w4", args: [] };
}

mkdirSync(dist, { recursive: true });
rmSync(join(dist, "og-candidates"), { recursive: true, force: true });

run("cargo", ["build", "--release"]);

const w4 = resolveW4();
run(w4.command, [
  ...w4.args,
  "bundle",
  cart,
  "--html",
  cartHtml,
  "--title",
  appTitle,
  "--description",
  appDescription,
  "--icon-file",
  join(root, "public/icon.svg"),
  "--html-disk-prefix",
  appTitle
]);

const cartPage = readFileSync(cartHtml, "utf8");
const shell = readFileSync(join(root, "web/template.html"), "utf8")
  .replaceAll("__APP_TITLE__", appTitle)
  .replaceAll("__APP_DESCRIPTION__", appDescription);
writeFileSync(shellHtml, shell);

const cacheVersion = createHash("sha256").update(shell).update(cartPage).digest("hex").slice(0, 12);
const serviceWorker = readFileSync(join(root, "public/sw.js"), "utf8").replace(
  "__CACHE_VERSION__",
  cacheVersion
);

copyFileSync(join(root, "public/manifest.webmanifest"), join(dist, "manifest.webmanifest"));
writeFileSync(join(dist, "sw.js"), serviceWorker);
copyFileSync(join(root, "public/favicon.ico"), join(dist, "favicon.ico"));
copyFileSync(join(root, "public/icon.svg"), join(dist, "icon.svg"));
copyFileSync(join(root, "public/icon-192.png"), join(dist, "icon-192.png"));
copyFileSync(join(root, "public/icon-512.png"), join(dist, "icon-512.png"));
copyFileSync(join(root, "public/main-visual.svg"), join(dist, "main-visual.svg"));
copyFileSync(join(root, "public/main-visual.png"), join(dist, "main-visual.png"));
copyFileSync(join(root, "public/og-image.png"), join(dist, "og-image.png"));
copyFileSync(join(root, "docs/prototype.png"), join(dist, "prototype.png"));

console.log("Built dist/index.html");
