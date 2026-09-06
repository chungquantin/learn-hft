# my-os Quartz Site

This directory contains the Quartz site used to publish the Obsidian vault in the repository root.

## Local Preview

```sh
npm install
npm run serve
```

The `serve` script syncs Markdown from the vault folders into `site/content`, builds Quartz, and starts a local server.

## Production Build

```sh
npm ci
npm run build
```

GitHub Pages builds this directory with `.github/workflows/deploy-quartz.yml`.
