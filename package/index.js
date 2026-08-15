import { createRequire } from "node:module";
var __require = /* @__PURE__ */ createRequire(import.meta.url);

// index.ts
import { existsSync } from "node:fs";
import { join } from "node:path";
var nativeBinding = null;
function loadNativeBinding() {
  if (nativeBinding) {
    return nativeBinding;
  }
  const possiblePaths = [
    join(import.meta.dirname, "icon-toolkit-napi.node"),
    join(import.meta.dirname, "icon-toolkit-napi.darwin-arm64.node"),
    join(import.meta.dirname, "icon-toolkit-napi.darwin-x64.node"),
    join(import.meta.dirname, "icon-toolkit-napi.linux-x64-gnu.node"),
    join(import.meta.dirname, "icon-toolkit-napi.win32-x64-msvc.node"),
    join(import.meta.dirname, "index.node"),
    join(import.meta.dirname, "..", "icon-toolkit-napi.node"),
    join(import.meta.dirname, "..", "target", "release", "libicon_toolkit_napi.dylib"),
    join(import.meta.dirname, "..", "target", "release", "libicon_toolkit_napi.so"),
    join(import.meta.dirname, "..", "target", "release", "icon_toolkit_napi.dll"),
    join(import.meta.dirname, "..", "target", "debug", "libicon_toolkit_napi.dylib")
  ];
  for (const p of possiblePaths) {
    if (existsSync(p)) {
      try {
        nativeBinding = __require(p);
        if (nativeBinding) {
          return nativeBinding;
        }
      } catch {}
    }
  }
  try {
    nativeBinding = (()=>{throw new Error("Cannot require module "+"./index.node");})();
    return nativeBinding;
  } catch (e) {
    throw new Error("Failed to load icon-toolkit native binary. Please run 'bun run build' inside package/ directory.", { cause: e });
  }
}
function generateIcons(options) {
  const binding = loadNativeBinding();
  return binding.generateIcons({
    backgroundColor: options.backgroundColor,
    borderRadiusPercent: options.borderRadiusPercent,
    darkInputPath: options.darkInputPath,
    inputPath: options.inputPath,
    mode: options.mode,
    outputDir: options.outputDir,
    paddingPercent: options.paddingPercent,
    targets: options.targets
  });
}
function convertImage(options) {
  const binding = loadNativeBinding();
  return binding.convertImage({
    backgroundColor: options.backgroundColor,
    format: options.format,
    height: options.height,
    inputPath: options.inputPath,
    outputPath: options.outputPath,
    quality: options.quality,
    width: options.width
  });
}
async function generateAiIcon(options) {
  const binding = loadNativeBinding();
  return await binding.generateAiIcon({
    apiKey: options.apiKey,
    autoGenerateTargets: options.autoGenerateTargets,
    endpoint: options.endpoint,
    model: options.model,
    outputPath: options.outputPath,
    prompt: options.prompt,
    provider: options.provider,
    quality: options.quality,
    size: options.size
  });
}
export {
  generateIcons,
  generateAiIcon,
  convertImage
};
