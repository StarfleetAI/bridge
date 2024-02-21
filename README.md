# StarfleetAI Bridge

Bridge is a self-contained fully-featured IDE for building and running autonomous AI agents.

## Development Setup

0. Ensure you have Rust, `tauri-cli` and pnpm installed locally
1. Clone the repository
2. Prepare config file

   ```shell
   cp src-tauri/.env{.example,}
   ```

3. Run the application

   ```shell
   SQLX_OFFLINE=true cargo tauri dev
   ```

   The `SQLX_OFFLINE=true` is only required for the cold start, since we have the `DATABASE_URL` set in `.env`, which forces SQLx to build against the databse it points to.

## Vue DevTools

Check the [Vue DevTools Standalone installation guide](https://devtools.vuejs.org/guide/installation.html#standalone) for a complete set of instructions.

Set **NUXT_PUBLIC_DEVTOOLS_PORT** in your `.env` file to the port number you wish to use.

Run `PORT=YOUR_PORT vue-devtools` and enjoy!
