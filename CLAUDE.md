# CLAUDE.md

This file guides Claude Code when working in this repository.

## Project

**水星樂園 (Mercury Land)** — the official fan-community site for Malaysian streamer 惡靈 Oreki. It combines a VOD archive, a live-stream "penalty" tracker, a virtual economy (水星幣 / Mercury Coin) earned from YouTube live-chat, a Discord bot, a lucky-wheel, and a leaderboard. One Rust binary runs three concurrent tasks; a Vue 3 SPA is the frontend.

- Frontend: Vue 3 (`<script setup>` SFC) + TypeScript, Vite 7, vue-router 4, Vuestic UI 1.10 (dark theme), Tailwind CSS 3 + SCSS, axios, vue-echarts/echarts, @vicons.
- Backend: Rust (edition 2024), actix-web 4, rusqlite 0.32 (bundled SQLite) + r2d2 + sea-query, poise/serenity (Discord), google-youtube3 (YouTube Data API v3), JWT (HMAC-SHA256).
- Database: single SQLite file `data/sqlite.db`, WAL mode, migrations via `PRAGMA user_version` (currently v12).
- License: Apache 2.0.

## Commands

### Backend (Rust)
```sh
cargo run                # dev server on 0.0.0.0:8080
cargo build --release    # -> target/release/mercury_land
cargo fmt                # format
cargo test               # unit tests (#[cfg(test)] inline modules)
```

### Frontend (Node)
```sh
npm install
npm run dev              # Vite dev server on :5173
npm run build            # -> dist/
npm run preview
npm run format           # Prettier (tabWidth 4)
npm run format:check
npm test                 # vitest run
npm run test:watch
```

Run backend and frontend in two terminals during local dev; the frontend auto-targets `http://127.0.0.1:8080` in dev (see `web/composables/utils.ts`).

## Architecture

Single tokio binary (`src/main.rs`) runs three tasks; on error each logs and restarts after 60s; Ctrl+C triggers graceful shutdown:

1. **`webpage::run`** — actix-web API on :8080, ~22 routes under `/api`, CORS allowlist `localhost:5173` + `mercuryland.pp.ua`.
2. **`discord::run`** — poise/serenity bot, slash commands (all Traditional Chinese).
3. **`youtube::run`** — YouTube live-chat listener, polls every 60s, awards Mercury Coin.

Source layout:
- `src/webpage/` — API modules (`auth`, `wheel`, `video`, `penalty`, `leaderboard`, `setting`, `image`, `anonymous`); routes registered in `mod.rs`.
- `src/database/` — SQLite access layer (`user`, `video`, `penalty`, `image`, `config`, `anonymous`) + `migration/` (12 sequential SQL files).
- `src/coin/` — Mercury Coin rules and chat-command handling.
- `src/discord/`, `src/youtube/` — bot and live-chat listener.
- `src/config.rs`, `src/error.rs`, `src/lib.rs` — config, error type, module root.
- `web/` — SPA: `router.ts` (11 routes), `components/` (per-feature folders), `composables/`, `assets/data/` (static JSON).

## Conventions

- **UI text, Discord commands/responses, and (historically) docs are Traditional Chinese.** Match that language when writing user-facing strings.
- **Version sync is mandatory**: `Cargo.toml` and `package.json` must carry the same version; CI (`check.yml`) fails otherwise. Bump both together.
- Formatting: `cargo fmt` (Rust), Prettier with `tabWidth: 4` (Vue/TS/CSS). No prettier/lint config beyond that.
- Vue components use `<script setup>` + TypeScript; composables live in `web/composables/` as `*.ts` with a colocated `*.test.ts`.
- Frontend path aliases (defined in both `vite.config.ts` and `vitest.config.ts`): `@` → `./web`, `@assets` → `./web/assets`, `@styles` → `./styles`, `@components` → `./web/components`, `@composables` → `./web/composables`. Keep both configs in sync.
- Rust modules are declared in `src/lib.rs`; binary entry is `src/main.rs`.

## Testing

- **Frontend**: Vitest, `environment: "jsdom"`, setup file `web/test/setup.ts` mocks `vuestic-ui`'s `useToast` (keeps composable tests out of Vue's plugin context). Tests included via `web/**/*.test.ts`, colocated with the code under test.
- **Backend**: inline `#[cfg(test)]` unit tests in the modules themselves.
- New logic should include tests; `npm test` and `cargo test` are the gate (plus `vue-tsc --noEmit` for type-checking, run as part of CI).

## Gotchas

- **JWT secret is randomly generated on every boot** — server restart invalidates all sessions; admins re-login. This is intentional (no permanent admin session).
- **Admin auth** is a static allowlist: decode JWT → Discord user ID → compare against `discord.admin` in `data/config.json`. No OAuth.
- **No foreign-key constraints** — all cross-table relations are implicit/logical. Don't assume FK enforcement.
- **Migrations** are applied by comparing `PRAGMA user_version`; add a new sequential file under `src/database/migration/` and bump the version when changing schema.
- **`data/`** holds runtime state and secrets (`sqlite.db`, `config.json`, `youtube.secret`). Never commit secrets.
- **`.env`** (gitignored) supplies `DISCORD_TOKEN` and `YOUTUBE_TOKEN` (YouTube OAuth client-secret JSON for Device Flow).
- **Images** are stored in SQLite as BLOBs keyed by UUIDv5 (content-hash-derived) filenames.
- **CORS / API origin** is fixed in two places: the backend allowlist and `BASE_URL` in `web/composables/utils.ts` (switches between `127.0.0.1:8080` dev and `api.mercuryland.pp.ua` prod).

## Git / CI

- `main` is the protected branch; switch back to main first, pull the remote to local, and work on feature branches (e.g. `feat/…`) and open PRs to `main`.
- `.github/workflows/` has three pipelines:
  - `check.yml` (on PR): `cargo fmt --check`, `cargo test`, Docker build, `vue-tsc --noEmit`, Prettier check, `vite build`, and version-consistency check.
  - `publish.yml` (push to `main`): build frontend, deploy to Cloudflare Pages via `wrangler pages deploy`.
  - `build.yml` (on Cargo.toml version bump): multi-stage Docker build → deploy over cloudflared SSH tunnel → `docker load` + restart.
- Commit messages are concise; for fuller background and the complete API/DB/Discord reference, see `README.md`.
