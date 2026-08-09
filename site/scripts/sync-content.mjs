import { cpSync, existsSync, mkdirSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs"
import path from "node:path"
import { fileURLToPath } from "node:url"

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const siteDir = path.resolve(scriptDir, "..")
const vaultDir = path.resolve(siteDir, "..")
const contentDir = path.join(siteDir, "content")

const vaultFolders = [
  "00 - Start",
  "10 - Foundations",
  "20 - Deep Dives",
  "30 - Data and Research",
  "40 - Ops and Frontend",
  "50 - Interview Prep",
]

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
      if (entry === ".git" || entry === ".obsidian" || entry === "node_modules" || entry === "target") {
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

copySelectedFiles(path.join(vaultDir, "projects"), path.join(contentDir, "projects"))

writeFileSync(
  path.join(contentDir, "index.md"),
  `---
title: Learn HFT
---

# Learn HFT

A public knowledge site for high-frequency trading systems, Rust implementation work, market microstructure, data infrastructure, operations, and interview preparation.

## Start Here

- [[00 - Start/00 - Roadmap|Roadmap]]
- [[00 - Start/01 - HFT Map|HFT Map]]
- [[00 - Start/02 - Knowledge Tree|Knowledge Tree]]
- [[00 - Start/08 - Build Projects|Build Projects]]
- [[00 - Start/13 - System Design Map|System Design Map]]

## Main Areas

- [[10 - Foundations/99 - Glossary|Foundations]]
- [[20 - Deep Dives/20 - Detailed Guides|Deep Dives]]
- [[30 - Data and Research/40 - Data Systems Hub|Data and Research]]
- [[40 - Ops and Frontend/50 - Frontend and Operator Systems Hub|Ops and Frontend]]
- [[50 - Interview Prep/61 - HFT Interview Drills and Portfolio Packaging|Interview Prep]]
`,
)

console.log(`Synced Quartz content from ${vaultDir} to ${contentDir}`)
