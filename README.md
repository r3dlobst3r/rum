# chad launcher
Libre/Free game launcher/GUI for running bash scripts.

This project was previously about developing a pirate game store. But due to issues with maintaining the database and relying on online services, we decided to move our focus on creating an offline GUI where you build your library of games manually, as well as still developing some features that we still want such as automated backups of game saves.

We are making this as an alternative to Lutris that doesn't do more than it needs to. We believe that patching games needs to be done in bash instead of an ambiguous collection of settings trough a GUI. So the GUI should only serve as a frontend and not as a debugger or compatiblity tool.

Developed by the [GNU/Linux P2P Pirates](https://matrix.to/#/!SlYhhmreXjJylcsjfn:tedomum.net?via=matrix.org&via=tedomum.net) matrix community and johncena141 release group from 1337x.

## Important: looking for new developers/maintainers.

## Installation or portable use

### Portable use

Download the compiled binary from [releases](https://gitlab.com/Gnurur/chad_launcher/-/releases). Required dependency is webkit2gtk.

Alternatively, compiled CI artifacts for each commit can be downloaded [here](https://gitlab.com/Gnurur/chad_launcher/-/pipelines).

### Install from AUR

Two AUR packages are available:

- [chad_launcher-bin](https://aur.archlinux.org/packages/chad_launcher-bin/): Downloads GitLab CI artifact of latest release.
- [chad_launcher-git](https://aur.archlinux.org/packages/chad_launcher-git/): Builds the master branch from source.

### Install from MPR (Debian)

- Work in progress.

### Install from Fedora Projects

- Work in progress.

### Build from source

We recommend using `pnpm` to build this project ([AUR](https://aur.archlinux.org/packages/pnpm/)), but any other 
package manager like `npm` or `yarn` should work too.

```
pnpm install
pnpm build
pnpm tauri build
```

This will create a `chad_launcher` executable and a debian package.

## Development

See Developer Guide in the [wiki](https://gitlab.com/Gnurur/chad_launcher/-/wikis/home).

### Running development server

```
pnpm dev
pnpm tauri dev
```

## Wiki

Read the [wiki](https://gitlab.com/Gnurur/chad_launcher/-/wikis/home).

# Donations
Monero: 4ABGQLAeAgiauvay11VRrWXRRtraRCU6oaC6uG9RUnNCHN4eepzWjEB6sHF92sUrSED5b8GyY7Ayh57R1jUdcKZg7is2DW3

Powered by [Tauri](https://tauri.studio)
