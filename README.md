# Civitai Companion

Lightweight Windows 10/11 x64 desktop companion for Civitai, built with Tauri 2, Rust and vanilla TypeScript.

## What it does

Civitai Companion keeps the parts of your Civitai account you check most often in a small desktop window, without requiring an open browser tab.

- Shows your account avatar, username, blue/yellow/green Buzz balances and new followers found today.
- Synchronizes Civitai notifications on a configurable interval or immediately through **Sync now**.
- Groups notifications into dedicated filters for comments and thread replies, followers, reactions, Buzz tips, model updates, collection/submission/bounty updates and miscellaneous events.
- Displays safe thumbnail previews for related images and videos when the Civitai API provides the required media data.
- Opens profiles, notifications, images, videos and other Civitai destinations on `civitai.red` in your default browser.
- Sends short-lived Windows notifications for new activity. Multiple simultaneous events are combined into a single notification count instead of producing misleading individual toasts.
- Supports configurable event sounds, sound previews, unread counters and mark-as-read actions.
- Provides an on-demand Buzz transaction ledger for blue, yellow and green Buzz, including received/spent entries and linked image previews where available.
- Runs from the Windows system tray: closing the window can keep synchronization active, while the tray menu offers **Show**, **Sync now** and **Quit**.
- Adds a **THANKS** button to Buzz tips received on images: one click posts a comment on the image that mentions the donor (`@username`) followed by your own message, at most once per user and image. Requires the optional *SocialWrite* permission.
- Optionally starts with Windows; autostart is disabled by default.
- Includes account connection testing, a permission inspector, preference backup/import, cache reset and complete local-account-data removal.

All polling, authenticated API access, notification decisions and persistent state are owned by the Rust backend. The interface never receives the API key.

## Getting started

1. Download either the installer or portable executable from [GitHub Releases](https://github.com/Tar-Tarj/Civitai-companion/releases).
2. Open **Settings → Account & Security**.
3. Create an API key at <https://civitai.red/user/account/security> (see [API key permissions](#api-key-permissions)), select **Configure API key** and paste it.
4. Run the connection test, then use **Sync now** or wait for the configured polling interval.

## API key permissions

Create the key at <https://civitai.red/user/account/security> and enable only the permissions below. **Settings → Account & Security** shows which of them your key currently has.

| Permission | Needed for |
|---|---|
| UserRead | Account identity, avatar and username |
| BuzzRead | Buzz balances and the transaction ledger |
| NotificationsRead | Notification history and unread count |
| NotificationsWrite | Marking notifications as read |
| MediaRead | Thumbnails for notifications |
| **SocialWrite** (optional) | The **THANKS** button, which posts a comment on your behalf |

Without *SocialWrite* everything else works and the THANKS button is shown disabled. If you add it later, generate a new key with the extra permission and replace the old one in the app. The thank-you text is set in **Settings → Notifications & Sounds**. Civitai limits comments to roughly 10 per hour, and the app never posts anything without a click on THANKS.

## Downloads

Version 1.0.26 is provided in two forms:

- NSIS setup executable for normal installation;
- portable executable for users who do not want to install the application.

Public builds are currently unsigned, so Windows SmartScreen may display a warning. Verify the SHA-256 values supplied with the release assets before running a download.

## Local development

Prerequisites:

- Rust stable with the `x86_64-pc-windows-msvc` target
- Microsoft Visual C++ Build Tools
- Node.js and npm
- Microsoft Edge WebView2 Runtime

Install and verify:

```powershell
npm ci
npm run check
npm test
cargo test --manifest-path src-tauri/Cargo.toml
npm run desktop:build
```

The release build creates Windows bundles under `src-tauri/target/release/bundle/`.

## Runtime model

- Rust owns all Civitai network access, synchronization, notification rules and persisted state.
- The WebView receives a sanitized `AppSnapshot`; credentials never cross back into the frontend.
- Civitai avatars and thumbnails are fetched by a credential-free Rust image path, restricted to the exact Civitai image/CDN hosts, validated, size-limited and delivered to the WebView as local Blob URLs.
- The account summary shows only positive blue, yellow and green Buzz balances, compacting all three into the existing card size.
- Clicking the Buzz balance card opens an on-demand, color-filtered transaction ledger for the previous and current month. Image-linked rows receive a safe local preview lookup and open only on `civitai.red`.
- Every user-facing Civitai destination is validated and canonicalized to `https://civitai.red` before opening the default browser.
- The default window is 430 pixels wide; both width and height remain user-resizable.
- Windows notification toasts are cleared from this app's notification history after four seconds.
- Closing the main window hides it. Use the tray menu to show, synchronize or quit.
- Start with Windows is opt-in and disabled by default.

## Persistence

The Civitai API key is stored as `Civitai Companion Desktop / civitai-api-key` through Windows Credential Manager.
Non-secret state is stored as an atomic, versioned `state-v1.json` file in the per-user application local-data directory.
Preference exports contain only the versioned `Settings` structure and are written atomically to the user's Downloads directory. Imports are limited to 1 MiB and preserve the current Start with Windows setting.

No automatic migration from the extension is performed. No automatic installer, telemetry, analytics, browsing data, cookies or browser bridge is included. The only non-Civitai request is an optional, unauthenticated lookup of the latest GitHub release once at startup (Settings > Behavior), which downloads nothing.

## Source availability

The complete application source and dependency lockfiles are included for inspection and reproducible local builds. Development credentials, local state, build caches and user data are never included.

No license has been selected yet. The source is public for transparency, but publication alone does not grant redistribution or modification rights until a license file is added.
