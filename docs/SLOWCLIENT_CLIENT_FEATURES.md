# SlowClient Client Features

This document tracks the implementation contract for the next SlowClient feature milestone.

## 1. Custom skin library

- Store user-imported PNG skins in the launcher data directory, separate from game instances.
- Accept standard 64x64 and legacy 64x32 Minecraft skin PNGs only.
- Allow listing, selecting, and deleting local skin entries.
- Keep the selected skin metadata separate from Mojang account state.
- Later UI work: skin preview, import picker, active-skin selector, and profile association.
- Applying a skin to an online Java account requires an authenticated account skin-upload flow; offline accounts can only use a local/custom client-side solution.

Backend commands added in this branch: `skins_list`, `skins_import`, `skins_set_active`, and `skins_delete`.

## 2. Version-aware HUD

- Resolve the exact Minecraft game version, mod loader, and loader version from the selected profile/instance at every launch.
- Resolve a SlowClient HUD build only when its declared game-version and loader compatibility match.
- Never install a guessed or incompatible JAR. Cache builds by game version + loader + HUD build version and verify download hashes before installation.
- Run a pre-launch compatibility check and emit a clear log entry when a compatible HUD build is unavailable.
- The HUD itself must be shipped as a separately built Minecraft client mod (Fabric/Forge/NeoForge/Quilt as supported); the launcher cannot create HUD rendering merely by copying a generic plugin.

## 3. SlowClient player badge

- The client mod will render the supplied SlowClient logo before the names of other SlowClient-enabled players.
- A client-only badge is visible only to clients that have the compatible mod and receive presence data.
- Cross-player visibility on ordinary multiplayer servers requires a compatible server plugin/mod or a supported server networking bridge to broadcast SlowClient presence. Do not claim that an unmodified vanilla server can broadcast this metadata.
- Use the final user-supplied logo asset when it becomes available; until then use a placeholder asset and keep the badge artwork configurable.

## Implementation order

1. Skin library backend and UI.
2. Version/loader compatibility resolver and HUD artifact manifest.
3. Build the first client HUD mod and integrate automatic install/update.
4. Add badge rendering and multiplayer presence bridge.
5. Test multiple Minecraft versions and loaders before enabling automatic installation by default.
