# my-os

This repository contains a topic-organized Obsidian knowledge base, implementation projects, and a Quartz site under `site/`.

The Markdown vault is the source of truth. Existing HFT notes live under `topics/hft/`; future subjects such as DevOps or econometrics can be added as sibling directories when their first notes are ready.

```text
00 - Start/          Vault-wide entry point
topics/              Self-contained learning topics
  hft/               Existing HFT knowledge graph
projects/            Buildable code and project notes
site/                Quartz publishing application
```

## Useful commands

```bash
cd site
npm install
npm run build
npm run serve
```

Run the Telegram-to-PR capture backend:

```bash
cargo run -p knowledge-capture-backend
```

See `projects/knowledge-capture-backend/README.md` for Telegram, GitHub, transcription, hosted vision, and capture-root configuration.

Start with `00 - Start/README.md`, then enter a topic through `topics/README.md`. The HFT curriculum begins at:

- `topics/hft/00 - Start/README.md`
- `topics/hft/00 - Start/02 - Knowledge Tree.md`

Generated Quartz output is intentionally not committed.
