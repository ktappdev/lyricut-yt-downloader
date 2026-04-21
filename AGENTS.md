# AGENTS.md

## Project

Lyricut YT Downloader — Tauri 2 desktop app that downloads YouTube audio as MP3s with metadata tagging. Frontend: React 19 + TypeScript + Tailwind v4 + Radix. Backend: Rust (src-tauri/).

## Build / Dev / Test

```bash
# Frontend only (Vite dev server)
npm run dev

# Full Tauri app (dev mode)
npm run tauri dev

# Production build
npm run build          # tsc + vite build
npm run tauri build    # full Tauri bundle

# Tests (Vitest + jsdom + Testing Library)
npm run test                    # watch mode
npx vitest run                  # single run all tests
npx vitest run src/test/Button.test.tsx   # single file
npx vitest run -t "test name"             # by name filter
```

## MANDATORY: Use td for Task Management

You must run td usage --new-session at conversation start (or after /clear) to see current work.
Use td usage -q for subsequent reads.

## Conventions

- Path alias `@/` maps to `./src/` (configured in vitest.config.ts and vite)
- Test files live in `src/test/` and follow `*.test.tsx` naming
- Vitest globals enabled (`describe`, `it`, `expect` — no imports needed)
- `@testing-library/jest-dom` matchers available via setup in `src/test/setup.ts`
- No eslint, prettier, or rustfmt configs — follow existing code style
- No pre-commit hooks

## Rust (src-tauri/)

- Modules: `main.rs`, `lib.rs`, `youtube_client.rs`, `ytdlp_setup.rs`, `ffmpeg_setup.rs`, `csv_parser.rs`, `file_processor.rs`, `metadata.rs`
- Uses `reqwest` with rustls (no openssl)
- Tauri plugins: `tauri-plugin-dialog`, `tauri-plugin-opener`

## CI

- GitHub Actions in `.github/workflows/build.yml`
- Triggers on version tags (`v*`)
- Builds macOS (universal) and Windows

## Gotchas

- Never auto-run `npm run tauri dev` or `npm run tauri build` — user runs the app
- yt-dlp is downloaded at runtime, not bundled at build time
- Tailwind v4 — uses `@tailwindcss/postcss`, not the v3 plugin syntax
