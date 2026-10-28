# Horizon on Xbox One (Dev Mode)

`.github/workflows/xbox.yml` builds `crates/horizon-xbox` for `x86_64-pc-windows-msvc`, packs it with this
folder's `AppxManifest.xml` and `Assets/` (makeappx), signs it with a throwaway `CN=Horizon` certificate
(signtool) and uploads `Horizon-xbox.msix` with `Horizon-xbox.cer`.

## Installing

1. Put the console in Developer Mode (the Dev Mode Activation app, a Partner Center developer account).
2. Open Dev Home's Device Portal from a PC browser, Apps → Add, and choose `Horizon-xbox.msix`.
3. Launch Horizon. It writes `horizon-xbox.log` to its LocalState folder (Device Portal → File explorer).

## What it is, and is not

A UWP shell (`CoreApplication`, from the `windows` crate) that opens a window and runs a check of the
simulation core on the console. It is **not the game yet**. An Xbox runs UWP packages only, and the game's
window and input come from winit, which has no UWP backend. Still to do, in order:

| Gap | Plan |
|---|---|
| Window and input | Drive `Game` from `CoreWindow` events instead of winit's `ApplicationHandler`; the pad through `Windows.Gaming.Input` |
| Graphics | wgpu's DirectX 12 backend on the `CoreWindow` (raw-window-handle's `WinRt` handle) |
| Files | The package's LocalState folder as the game home; the game files through the existing import screen |
| Sound | cpal's WASAPI backend, to be tried under UWP |
| Memory | Dev Mode gives a UWP game about 5 GB on a One S; lower render distance by default |

The executable is a desktop-target Rust build in a UWP package: Dev Mode sideloading runs it, but the Store's
certification would not.
