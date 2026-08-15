import { describe, expect, it } from "bun:test";
import { existsSync, mkdirSync, rmSync } from "node:fs";
import { join } from "node:path";
import { convertImage, generateIcons } from "../package/index";

const rootDir = join(import.meta.dirname, "..");
const demoIconPath = join(rootDir, "demo-icon.png");
const testOutDir = join(rootDir, "output", "test-ts-run");

describe("icon-toolkit TypeScript Package", () => {
  it("should generate icon sets for Web, Mobile, and Desktop targets", () => {
    if (existsSync(testOutDir)) {
      rmSync(testOutDir, { force: true, recursive: true });
    }
    mkdirSync(testOutDir, { recursive: true });

    const result = generateIcons({
      backgroundColor: "#ffffff",
      borderRadiusPercent: 10,
      inputPath: demoIconPath,
      mode: "light",
      outputDir: testOutDir,
      paddingPercent: 5,
      targets: ["web", "mobile", "desktop"],
    });

    expect(result.createdFiles.length).toBeGreaterThan(0);
    expect(result.summary).toContain("generated");

    // Web checks
    const webDir = join(testOutDir, "web");
    expect(existsSync(join(webDir, "favicon.ico"))).toBe(true);
    expect(existsSync(join(webDir, "apple-touch-icon.png"))).toBe(true);
    expect(existsSync(join(webDir, "site.webmanifest"))).toBe(true);

    // Mobile checks
    const iosDir = join(testOutDir, "mobile", "ios", "AppIcon.appiconset");
    expect(existsSync(join(iosDir, "Contents.json"))).toBe(true);
    expect(existsSync(join(iosDir, "Icon-1024.png"))).toBe(true);

    // Desktop checks
    const winIco = join(testOutDir, "desktop", "windows", "icon.ico");
    expect(existsSync(winIco)).toBe(true);
  }, 30_000);

  it("should convert image between formats (WebP, SVG, ICO)", () => {
    const convertOutDir = join(testOutDir, "converted");
    mkdirSync(convertOutDir, { recursive: true });

    // WebP conversion
    const webpPath = join(convertOutDir, "test.webp");
    const webpRes = convertImage({
      format: "webp",
      height: 256,
      inputPath: demoIconPath,
      outputPath: webpPath,
      quality: 85,
      width: 256,
    });
    expect(existsSync(webpPath)).toBe(true);
    expect(webpRes.fileSize).toBeGreaterThan(0);

    // SVG canvas wrapper conversion
    const svgPath = join(convertOutDir, "test.svg");
    const svgRes = convertImage({
      format: "svg",
      inputPath: demoIconPath,
      outputPath: svgPath,
    });
    expect(existsSync(svgPath)).toBe(true);
    expect(svgRes.fileSize).toBeGreaterThan(0);

    // Multi-size ICO conversion
    const icoPath = join(convertOutDir, "test.ico");
    const icoRes = convertImage({
      format: "ico",
      inputPath: demoIconPath,
      outputPath: icoPath,
    });
    expect(existsSync(icoPath)).toBe(true);
    expect(icoRes.fileSize).toBeGreaterThan(0);
  }, 30_000);
});
