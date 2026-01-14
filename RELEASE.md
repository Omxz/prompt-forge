# Release Process

This document explains how to create releases with automatic updates for Prompt Forge.

## Auto-Update System

Prompt Forge uses Tauri's built-in updater plugin to provide automatic updates. Users can check for updates and install them directly from the Settings page without manually downloading installers.

## Prerequisites

### Signing Key Setup

⚠️ **CRITICAL: The signing key should ONLY be generated ONCE and NEVER regenerated unless lost or compromised.**

If you regenerate the key:
- All existing users will be unable to auto-update (signature mismatch)
- Users must manually download the new version
- This breaks the update chain

**Check if key already exists:**
```bash
ls ~/.tauri/myapp.key
```

**Only if the key does NOT exist**, generate one:
```bash
npx tauri signer generate -w ~/.tauri/myapp.key
```

This creates two files:
- Private key: `~/.tauri/myapp.key` (keep this SECRET, back it up securely!)
- Public key: `~/.tauri/myapp.key.pub`

The public key must be added to `tauri.conf.json` under `plugins.updater.pubkey`. This only needs to be done once when first setting up the project.

### Update tauri.conf.json (first-time only)

Replace `UPDATER_PUBLIC_KEY_PLACEHOLDER` in `src-tauri/tauri.conf.json` with your actual public key:

```json
{
  "plugins": {
    "updater": {
      "endpoints": [
        "https://github.com/Omxz/prompt-forge/releases/latest/download/latest.json"
      ],
      "pubkey": "YOUR_PUBLIC_KEY_HERE"
    }
  }
}
```

## Creating a Release

### 1. Update Version Number

Update the version in:
- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`

Example: `"version": "0.3.0"`

### 2. Build the Release

Build on each platform you want to release for:

**macOS:**
```bash
npm run tauri build
```

Creates installers in `src-tauri/target/release/bundle/`:
- DMG: `dmg/Prompt Forge_X.X.X_aarch64.dmg` (Apple Silicon)
- App: `macos/Prompt Forge.app`

**Windows:**
```powershell
npm run tauri build
```

Creates installers in `src-tauri/target/release/bundle/`:
- MSI: `msi/Prompt Forge_X.X.X_x64_en-US.msi`
- NSIS: `nsis/Prompt Forge_X.X.X_x64-setup.exe`

### 3. Sign the Installers

Sign each installer with your private key (keep private key secure, never commit to git).

**Important:** Use `npx` and the correct flags:
- `-f` for the key file path
- `-p` for the password (if your key is password-protected)

**macOS:**
```bash
# Without password:
npx tauri signer sign -f ~/.tauri/myapp.key "src-tauri/target/release/bundle/dmg/Prompt Forge_X.X.X_aarch64.dmg"

# With password:
npx tauri signer sign -f ~/.tauri/myapp.key -p "YOUR_PASSWORD" "src-tauri/target/release/bundle/dmg/Prompt Forge_X.X.X_aarch64.dmg"
```

**Windows:**
```powershell
# Sign the NSIS installer
npx tauri signer sign -f ~/.tauri/myapp.key -p "YOUR_PASSWORD" "src-tauri/target/release/bundle/nsis/Prompt Forge_X.X.X_x64-setup.exe"

# Sign the MSI
npx tauri signer sign -f ~/.tauri/myapp.key -p "YOUR_PASSWORD" "src-tauri/target/release/bundle/msi/Prompt Forge_X.X.X_x64_en-US.msi"
```

This creates `.sig` files next to each installer and prints the signature to use in `latest.json`.

### 4. Create GitHub Release

1. Go to https://github.com/Omxz/prompt-forge/releases/new
2. Create a new tag: `v0.3.0`
3. Set release title: `v0.3.0`
4. Add release notes describing changes
5. Upload ALL installer files and their signatures:

   **macOS:**
   - `Prompt Forge_0.3.0_aarch64.dmg`
   - `Prompt Forge_0.3.0_aarch64.dmg.sig`

   **Windows:**
   - `Prompt Forge_0.3.0_x64-setup.exe`
   - `Prompt Forge_0.3.0_x64-setup.exe.sig`
   - `Prompt Forge_0.3.0_x64_en-US.msi`
   - `Prompt Forge_0.3.0_x64_en-US.msi.sig`

6. **Important**: Mark as "Latest release"

### 5. Create Update Manifest

Create a `latest.json` file with all platforms:

```json
{
  "version": "0.3.0",
  "notes": "Release notes here",
  "pub_date": "2026-01-14T00:00:00Z",
  "platforms": {
    "darwin-aarch64": {
      "signature": "SIGNATURE_FROM_DMG_SIG_FILE",
      "url": "https://github.com/Omxz/prompt-forge/releases/download/v0.3.0/Prompt.Forge_0.3.0_aarch64.dmg"
    },
    "windows-x86_64": {
      "signature": "SIGNATURE_FROM_NSIS_SIG_FILE",
      "url": "https://github.com/Omxz/prompt-forge/releases/download/v0.3.0/Prompt.Forge_0.3.0_x64-setup.exe"
    }
  }
}
```

To get the signatures:
```bash
# macOS
cat "src-tauri/target/release/bundle/dmg/Prompt Forge_0.3.0_aarch64.dmg.sig"

# Windows
cat "src-tauri/target/release/bundle/nsis/Prompt Forge_0.3.0_x64-setup.exe.sig"
```

Upload `latest.json` to the same GitHub release.

### 6. Publish the Release

Click "Publish release" on GitHub. The updater will now detect this version!

## Platform Identifiers

For `latest.json`, use these platform keys:
- `darwin-aarch64` - macOS Apple Silicon (M1/M2/M3)
- `darwin-x86_64` - macOS Intel
- `windows-x86_64` - Windows 64-bit
- `linux-x86_64` - Linux 64-bit

## How Users Get Updates

1. Users open Prompt Forge
2. Go to Settings → Updates section
3. Click "Check for Updates"
4. If an update is available, click "Install Update"
5. The app downloads, verifies (using signature), and installs automatically
6. The app restarts with the new version

## Security Notes

- **Never commit your private key** (`.key` file) to git
- The `.gitignore` already excludes `*.key` files
- Keep your private key secure - anyone with it can sign fake updates
- The public key in `tauri.conf.json` is safe to commit
- Signatures ensure users only install authentic updates from you
- Never commit `.sig` files or signature strings to the repo

## Troubleshooting

**"Failed to check for updates"**
- Ensure `latest.json` exists in the GitHub release
- Check that the endpoint URL in `tauri.conf.json` is correct
- Verify the release is marked as "Latest"

**"Signature verification failed"**
- Ensure you signed the installer with the correct private key
- Check that the public key in `tauri.conf.json` matches your private key
- Verify the signature string in `latest.json` matches the `.sig` file content

**"Update not detected"**
- Version in `latest.json` must be higher than current version
- Ensure GitHub release is published (not draft)
- Check browser network tab for 404s when checking for updates

## Quick Reference

```bash
# Check if signing key exists (DO NOT regenerate if it does!)
ls ~/.tauri/myapp.key

# Build release
npm run tauri build

# Sign files (use -f for file path, -p for password)
npx tauri signer sign -f ~/.tauri/myapp.key -p "YOUR_PASSWORD" "src-tauri/target/release/bundle/dmg/Prompt Forge_X.X.X_aarch64.dmg"

# The signature is printed to console - copy it to latest.json
# It's also saved to a .sig file next to the installer
```

## Common Errors

**"Signing without password" + base64 error:**
Your key is password-protected. Use the `-p` flag:
```bash
npx tauri signer sign -f ~/.tauri/myapp.key -p "YOUR_PASSWORD" "path/to/installer"
```

**"command not found: tauri":**
Use `npx tauri` instead of just `tauri`:
```bash
npx tauri signer sign ...
```

**"Unable to find the private key":**
Use `-f` flag for file path (not `-k`):
```bash
npx tauri signer sign -f ~/.tauri/myapp.key ...
```
