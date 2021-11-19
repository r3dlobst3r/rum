# Rum
Libre game launcher for GNU/Linux hackers powered by [Tauri](https://tauri.studio).

Developed by the johncena141 hacker group from 1337x. For questions or contribution, talk with us on [Matrix](https://matrix.to/#/!SlYhhmreXjJylcsjfn:tedomum.net?via=matrix.org&via=tedomum.net).

#### Portable use

Download the compiled binary from [releases](https://notabug.org/johncena141/rum/releases). Required dependency is webkit2gtk.

#### Install from AUR on Arch and derivates

Two AUR packages are available:

- [rum-bin](https://aur.archlinux.org/packages/rum-bin/): Downloads latest release.
- [rum-git](https://aur.archlinux.org/packages/rum-git/): Builds the master branch from source.

#### Install on Gentoo with eselect-repository

Make sure eselect-repository is installed

```
yes | emerge --update app-eselect/eselect-repository
```

After that is completed you should be good to go and add the [chad-repo](https://notabug.org/agdfrhjlbzvf/chad-repo) repository to your system

```
eselect repository add chad-repo git https://notabug.org/agdfrhjlbzvf/chad-repo.git
emaint sync -r chad-repo
```

And finally install rum

```
emerge -av rum
```

#### Install from MPR (Debian/Ubuntu)

```
wget -qO - 'https://proget.hunterwittenborn.com/debian-feeds/makedeb.pub' | \
gpg --dearmor | \
sudo tee /usr/share/keyrings/makedeb-archive-keyring.gpg &> /dev/null
echo 'deb [signed-by=/usr/share/keyrings/makedeb-archive-keyring.gpg arch=all] https://proget.hunterwittenborn.com/ makedeb main' | \
sudo tee /etc/apt/sources.list.d/makedeb.list
sudo apt update && sudo apt install makedeb
git clone "https://mpr.hunterwittenborn.com/makedeb.git"
git clone "https://mpr.hunterwittenborn.com/makedeb-makepkg.git"
cd makedeb-makepkg/
makedeb -si
cd ../makedeb/
makedeb -si
```

Install tap (MPR helper)

```
git clone https://mpr.hunterwittenborn.com/tap.git && cd tap && makedeb -si
```

Install Rum

```
tap install rum-bin
```

#### Build from source

We recommend using `pnpm` to build this project ([AUR](https://aur.archlinux.org/packages/pnpm/)), but any other
package manager like `npm` or `yarn` should work too.

```
pnpm install
pnpm build
pnpm tauri build
```

#### Running development server

```
pnpm dev
pnpm tauri dev
```

#### More info on [Wiki](https://notabug.org/johncena141/rum/wiki)

<img src="https://i.postimg.cc/cHMfLtLy/3423423.png">

<img src="https://i.postimg.cc/cC2cG149/434.png">

Donations (Monero): 4ABGQLAeAgiauvay11VRrWXRRtraRCU6oaC6uG9RUnNCHN4eepzWjEB6sHF92sUrSED5b8GyY7Ayh57R1jUdcKZg7is2DW3