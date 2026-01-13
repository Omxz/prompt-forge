# Release Process

This document explains how to create releases with automatic updates for Prompt Forge.

## Auto-Update System

Prompt Forge uses Tauri's built-in updater plugin to provide automatic updates. Users can check for updates and install them directly from the Settings page without manually downloading installers.

## Prerequisites

1. **Generate signing keys** (one-time setup):

```powershell
# Install Tauri CLI globally
npm install -g @tauri-apps/cli

# Generate update signature keys
tauri signer generate -w ~/.tauri/myapp.key
```

This creates two files:
- Private key: `~/.tauri/myapp.key` (keep this SECRET!)
- Public key: printed to console (add this to `tauri.conf.json`)

2. **Update tauri.conf.json**:

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

Update the version in both:
- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`

Example: `"version": "0.2.0"`

### 2. Build the Release

```powershell
npm run tauri build
```

This creates installers in `src-tauri/target/release/bundle/`:
- MSI: `msi/Prompt Forge_X.X.X_x64_en-US.msi`
- NSIS: `nsis/Prompt Forge_X.X.X_x64-setup.exe`

### 3. Sign the Installers

Sign each installer with your private key:

```powershell
# Sign the MSI
tauri signer sign "src-tauri/target/release/bundle/msi/Prompt Forge_0.2.0_x64_en-US.msi" -k ~/.tauri/myapp.key

# Sign the NSIS installer
tauri signer sign "src-tauri/target/release/bundle/nsis/Prompt Forge_0.2.0_x64-setup.exe" -k ~/.tauri/myapp.key
```

This creates `.sig` files next to each installer.

### 4. Create GitHub Release

1. Go to https://github.com/Omxz/prompt-forge/releases/new
2. Create a new tag: `v0.2.0`
3. Set release title: `v0.2.0`
4. Add release notes describing changes
5. Upload ALL files:
   - `Prompt Forge_0.2.0_x64_en-US.msi`
   - `Prompt Forge_0.2.0_x64_en-US.msi.sig`
   - `Prompt Forge_0.2.0_x64-setup.exe`
   - `Prompt Forge_0.2.0_x64-setup.exe.sig`
6. **Important**: Mark as "Latest release"

### 5. Create Update Manifest

Create a `latest.json` file with this structure:

```json
{
  "version": "0.2.0",
  "notes": "Release notes here",
  "pub_date": "2024-01-13T00:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "SIGNATURE_FROM_MSI_SIG_FILE",
      "url": "https://github.com/Omxz/prompt-forge/releases/download/v0.2.0/Prompt Forge_0.2.0_x64_en-US.msi"
    }
  }
}
```

To get the signature:
```powershell
cat "src-tauri/target/release/bundle/msi/Prompt Forge_0.2.0_x64_en-US.msi.sig"
```

Upload `latest.json` to the same GitHub release.

### 6. Publish the Release

Click "Publish release" on GitHub. The updater will now detect this version!

## How Users Get Updates

1. Users open Prompt Forge
2. Go to Settings → Updates section
3. Click "Check for Updates"
4. If an update is available, click "Install Update"
5. The app downloads, verifies (using signature), and installs automatically
6. The app restarts with the new version

## Security Notes

- **Never commit your private key** (`.key` file) to git
- Add `*.key` to `.gitignore`
- Keep your private key secure - anyone with it can sign fake updates
- The public key in `tauri.conf.json` is safe to commit
- Signatures ensure users only install authentic updates from you

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
# Generate keys (once)
tauri signer generate -w ~/.tauri/myapp.key

# Build release
npm run tauri build

# Sign files
tauri signer sign "path/to/installer.msi" -k ~/.tauri/myapp.key

# Get signature content
cat "path/to/installer.msi.sig"
```
