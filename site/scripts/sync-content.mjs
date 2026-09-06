import {
  cpSync,
  existsSync,
  mkdirSync,
  readdirSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs"
import path from "node:path"
import { fileURLToPath } from "node:url"

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const siteDir = path.resolve(scriptDir, "..")
const vaultDir = path.resolve(siteDir, "..")
const contentDir = path.join(siteDir, "content")

const vaultFolders = ["00 - Start", "topics"]

const assetExtensions = new Set([
  ".avif",
  ".base",
  ".canvas",
  ".gif",
  ".jpeg",
  ".jpg",
  ".md",
  ".mp4",
  ".pdf",
  ".png",
  ".svg",
  ".webp",
])

function copySelectedFiles(from, to) {
  if (!existsSync(from)) return

  const stats = statSync(from)
  if (stats.isDirectory()) {
    for (const entry of readdirSync(from)) {
      if (
        entry === ".git" ||
        entry === ".obsidian" ||
        entry === "node_modules" ||
        entry === "target"
      ) {
        continue
      }

      copySelectedFiles(path.join(from, entry), path.join(to, entry))
    }
    return
  }

  if (!stats.isFile() || !assetExtensions.has(path.extname(from).toLowerCase())) {
    return
  }

  mkdirSync(path.dirname(to), { recursive: true })
  cpSync(from, to)
}

rmSync(contentDir, { recursive: true, force: true })
mkdirSync(contentDir, { recursive: true })

for (const folder of vaultFolders) {
  copySelectedFiles(path.join(vaultDir, folder), path.join(contentDir, folder))
}

copySelectedFiles(path.join(vaultDir, "CHANGELOG.md"), path.join(contentDir, "CHANGELOG.md"))
copySelectedFiles(path.join(vaultDir, "projects"), path.join(contentDir, "projects"))

writeFileSync(
  path.join(contentDir, "index.md"),
  `---
title: my-os
tags: [index, my-os]
---

# my-os

A topic-organized knowledge base for technical learning and implementation projects.

## Start Here

- [[00 - Start/README|Vault Home]]
- [[topics/README|Topics]]
- [[projects/README|Projects]]

## Current Topics

- [[topics/hft/00 - Start/README|High-Frequency Trading]]

## HFT Paths

- [[topics/hft/00 - Start/02 - Knowledge Tree|Knowledge Tree]]
- [[topics/hft/20 - Deep Dives/81 - Expert HFT Technology Curriculum|Expert HFT Technology Curriculum]]
- [[topics/hft/20 - Deep Dives/72 - Production Low-Latency Trading System Construction|Production Low-Latency System Construction]]
`,
)

console.log(`Synced Quartz content from ${vaultDir} to ${contentDir}`)
