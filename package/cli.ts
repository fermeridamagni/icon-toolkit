#!/usr/bin/env bun
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";

function findCliBinary(): string | null {
  const candidates = [
    join(import.meta.dirname, "..", "..", "target", "release", "icon-toolkit"),
    join(import.meta.dirname, "..", "..", "target", "debug", "icon-toolkit"),
    join(import.meta.dirname, "..", "target", "release", "icon-toolkit"),
    join(import.meta.dirname, "..", "target", "debug", "icon-toolkit"),
    join(import.meta.dirname, "..", "bin", "icon-toolkit"),
    join(import.meta.dirname, "bin", "icon-toolkit"),
  ];

  for (const c of candidates) {
    if (existsSync(c)) {
      return c;
    }
  }

  return null;
}

const binaryPath = findCliBinary();

if (binaryPath) {
  const child = spawn(binaryPath, process.argv.slice(2), {
    stdio: "inherit",
  });

  child.on("exit", (code) => {
    process.exit(code ?? 0);
  });
} else {
  console.error(
    "icon-toolkit binary not found. Building project with cargo build --release..."
  );
  const repoRoot = existsSync(join(import.meta.dirname, "..", "Cargo.toml"))
    ? join(import.meta.dirname, "..")
    : join(import.meta.dirname, "..", "..");

  const child = spawn(
    "cargo",
    ["build", "--release", "--bin", "icon-toolkit"],
    {
      cwd: repoRoot,
      stdio: "inherit",
    }
  );

  child.on("exit", (code) => {
    if (code === 0) {
      const builtBin = join(repoRoot, "target", "release", "icon-toolkit");
      spawn(builtBin, process.argv.slice(2), { stdio: "inherit" }).on(
        "exit",
        (c) => process.exit(c ?? 0)
      );
    } else {
      console.error("Failed to build icon-toolkit binary.");
      process.exit(code ?? 1);
    }
  });
}
