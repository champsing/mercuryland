# Development Instructions

## Vue.js Code Changes

After making changes to Vue-related code (files in `web/`, `public/`, `package.json`, etc.), always run the following commands to examine for any errors due to the change before committing:

1. `npm run format` - Formats the code according to the project's style guidelines.
2. `npx vue-tsc --noEmit` - Performs TypeScript type checking on Vue components without emitting files.
3. `npm run build` - Builds the Vue.js application to ensure there are no build errors.

These steps help maintain code quality and catch issues early.

## Rust Code Changes

For Rust-related changes (files in `src/`, `Cargo.toml`, etc.), always use `cargo dotenv` to load environment variables from a `.env` file for local testing.

Examples:
- `cargo dotenv test` - Runs tests with environment variables loaded.
- `cargo dotenv run` - Runs the application with environment variables loaded.

This ensures that local testing matches the production environment setup.