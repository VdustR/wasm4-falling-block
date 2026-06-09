import { existsSync, readFileSync, statSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const required = [
  "dist/index.html",
  "dist/manifest.webmanifest",
  "dist/sw.js",
  "dist/favicon.ico",
  "dist/icon.svg",
  "dist/icon-192.png",
  "dist/icon-512.png",
  "dist/main-visual.svg",
  "dist/main-visual.png",
  "dist/og-image.png",
  "target/wasm32-unknown-unknown/release/falling_block.wasm"
];

for (const file of required) {
  const path = join(root, file);
  if (!existsSync(path) || statSync(path).size === 0) {
    throw new Error(`Missing build artifact: ${file}`);
  }
}

const html = readFileSync(join(root, "dist/index.html"), "utf8");
const sw = readFileSync(join(root, "dist/sw.js"), "utf8");
const manifest = readFileSync(join(root, "dist/manifest.webmanifest"), "utf8");

for (const needle of [
  "<wasm4-app",
  "Falling Block",
  "A Tetris-like falling block puzzle for WASM-4 with offline PWA support, mobile controls, and handmade chiptune audio.",
  "id=\"startGame\"",
  "id=\"updatePrompt\"",
  "navigator.serviceWorker.register",
  "favicon.ico",
  "icon.svg",
  "class=\"brand-icon\"",
  "src=\"icon-192.png\"",
  "apple-touch-icon",
  "https://vdustr.dev/wasm4-falling-block/",
  "https://vdustr.dev/wasm4-falling-block/og-image.png",
  "og:image:width",
  "og:image:height"
]) {
  if (!html.includes(needle)) {
    throw new Error(`dist/index.html missing ${needle}`);
  }
}

if (!sw.includes("CACHE_ASSETS") || !sw.includes("SKIP_WAITING")) {
  throw new Error("dist/sw.js is missing offline/update behavior");
}

if (!sw.includes('event.request.mode === "navigate"') || !sw.includes('cache.put("./index.html", copy)')) {
  throw new Error("dist/sw.js must use network-first navigation with offline index fallback");
}

if (sw.includes('  "./",')) {
  throw new Error("dist/sw.js must not cache root navigation as a cache-first asset");
}

const parsedManifest = JSON.parse(manifest);
if (
  parsedManifest.display !== "fullscreen" ||
  parsedManifest.start_url !== "./" ||
  !parsedManifest.display_override?.includes("standalone")
) {
  throw new Error("manifest is not configured for fullscreen PWA launch");
}

for (const icon of ["icon-192.png", "icon-512.png", "icon.svg"]) {
  if (!parsedManifest.icons?.some((entry) => entry.src === icon)) {
    throw new Error(`manifest is missing ${icon}`);
  }
}

for (const cachedAsset of ["./favicon.ico", "./icon-192.png", "./icon-512.png", "./icon.svg"]) {
  if (!sw.includes(cachedAsset)) {
    throw new Error(`dist/sw.js is missing cached asset ${cachedAsset}`);
  }
}

for (const cachedAsset of ["./main-visual.svg", "./main-visual.png", "./og-image.png"]) {
  if (!sw.includes(cachedAsset)) {
    throw new Error(`dist/sw.js is missing brand asset ${cachedAsset}`);
  }
}

for (const needle of [
  "apple-mobile-web-app-capable",
  "mobile-web-app-capable",
  "data-play-view",
  "requestFullscreen",
  "wasm4-virtual-gamepad",
  "falling-block-mobile-vpad"
]) {
  if (!html.includes(needle)) {
    throw new Error(`dist/index.html missing ${needle}`);
  }
}

for (const removedNeedle of ["class=\"touch-controls\"", "data-key-code=\"ArrowLeft\""]) {
  if (html.includes(removedNeedle)) {
    throw new Error(`dist/index.html still includes shell-owned touch controls: ${removedNeedle}`);
  }
}

console.log("Smoke test passed");
