# StarfleetAI Bridge

Bridge is a self-contained, fully-featured IDE for building and running autonomous AI agents.

## Download

You can download the latest version of Bridge from the [releases page](https://github.com/StarfleetAI/bridge/releases).

### Fixing "App is damaged and can't be opened" error on macOS

This error occurs because the app is not yet signed. To fix it, run the following command:

```shell
xattr -cr /Applications/Bridge.app
```

## Development Setup

1. Ensure you have Rust, Docker, `tauri-cli` and pnpm installed locally
2. Clone the repository
3. Prepare config file

   ```shell
   cp src-tauri/.env{.example,}
   ```

4. Run the application

   ```shell
   SQLX_OFFLINE=true cargo tauri dev
   ```

   The `SQLX_OFFLINE=true` is only required for the cold start, since we have the `DATABASE_URL` set in `.env`, which forces SQLx to build against the database it points to.

### Vue DevTools

Just run `pnpm devtools` and enjoy!
