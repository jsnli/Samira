# Samira

A Steam Achievement Manager for Linux

![example](./assets/screenshot.png)

## Description
Samira is a desktop application that allows you to unlock achievements and set statistics. The long term goal of this project is to recreate all features of the original [SAM](https://github.com/gibbed/SteamAchievementManager) on Linux, Windows, and MacOS. 

## Installation and Usage
Releases can be found [here](https://github.com/jsnli/Samira/releases).

Steam must be running and the user must be logged in.

Flatpak is not currently supported. Steam must be installed through your distribution's package manager or through the installer from the official [steam page](https://store.steampowered.com/about/).

## Building

To build this project you'll need the [prerequisites](https://tauri.app/start/prerequisites/) for Tauri as well as Rust, Node, and npm.

Clone the repository and install: 
```bash
cd Samira && npm install
```

Tauri does not automatically set library search paths. This is a known issue in Tauri and until an official fix is available we need to set it ourselves: 
```bash
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/lib/libsteam_api.so
```

The `libsteam_api.so` file is available in `/assets/`.

**Dev**
```bash
npm run tauri dev
```

**Build**
```bash
npm run tauri build
```

On Arch based distributions, a bundling error may occur due to _Tauri_ and _linuxdeploy_ using the wrong `strip` binary. A workaround is available:

```bash
NO_STRIP=true npm run tauri build
```

If that continues not to work, run:
```bash
sudo pacman -S rust
```

## Contributing
Like its predecessors, Samira is open source and open to contributions. Documentation will follow in the near future. Please feel free to ask questions, open pull requests, and raise issues.
