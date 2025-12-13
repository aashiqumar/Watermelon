# Watermelon Packaging Guide

## 1. Flatpak (Universal Linux)

Flatpak is the recommended way to distribute Watermelon.

### Manifest: `build-aux/com.aashiqumar.watermelon.json`
We use the GNOME 45 runtime.

```json
{
    "app-id": "com.aashiqumar.watermelon",
    "runtime": "org.gnome.Platform",
    "runtime-version": "45",
    "sdk": "org.gnome.Sdk",
    "command": "watermelon",
    "finish-args": [
        "--share=ipc",
        "--socket=fallback-x11",
        "--socket=wayland",
        "--device=dri"
    ],
    "modules": [
        {
            "name": "watermelon",
            "buildsystem": "simple",
            "build-commands": [
                "cargo build --release",
                "install -D target/release/watermelon /app/bin/watermelon",
                "install -D data/com.aashiqumar.watermelon.desktop /app/share/applications/com.aashiqumar.watermelon.desktop",
                "install -D data/com.aashiqumar.watermelon.metainfo.xml /app/share/metainfo/com.aashiqumar.watermelon.metainfo.xml",
                "install -D assets/icon.svg /app/share/icons/hicolor/scalable/apps/com.aashiqumar.watermelon.svg"
            ],
            "sources": [
                {
                    "type": "dir",
                    "path": ".."
                }
            ]
        }
    ]
}
```

### Build Instructions
```bash
flatpak-builder --user --install --force-clean build-dir build-aux/com.aashiqumar.watermelon.json
flatpak run com.aashiqumar.watermelon
```

## 2. RPM (Fedora Native)

For native performance and integration.

### Spec File: `build-aux/watermelon.spec`

```spec
Name:           watermelon
Version:        0.1.0
Release:        1%{?dist}
Summary:        Fresh. Clean. Friendly. Notes App.

License:        MIT
URL:            https://github.com/example/watermelon
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust-packaging
BuildRequires:  gtk4-devel
BuildRequires:  libadwaita-devel

%description
Watermelon is a new Apple Notes–style productivity app built for Fedora Linux.

%prep
%autosetup

%build
%cargo_build

%install
%cargo_install
install -D -m 644 data/com.aashiqumar.watermelon.desktop %{buildroot}%{_datadir}/applications/com.aashiqumar.watermelon.desktop
install -D -m 644 data/com.aashiqumar.watermelon.metainfo.xml %{buildroot}%{_datadir}/metainfo/com.aashiqumar.watermelon.metainfo.xml
install -D -m 644 assets/icon.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/com.aashiqumar.watermelon.svg

%files
%{_bindir}/watermelon
%{_datadir}/applications/com.aashiqumar.watermelon.desktop
%{_datadir}/metainfo/com.aashiqumar.watermelon.metainfo.xml
%{_datadir}/icons/hicolor/scalable/apps/com.aashiqumar.watermelon.svg
```

## 3. Metadata

### Desktop File: `data/com.aashiqumar.watermelon.desktop`
```ini
[Desktop Entry]
Name=Watermelon
Comment=Fresh. Clean. Friendly. Notes App.
Exec=watermelon
Icon=com.aashiqumar.watermelon
Terminal=false
Type=Application
Categories=Office;Utility;
StartupNotify=true
```

### AppStream: `data/com.aashiqumar.watermelon.metainfo.xml`
Required for GNOME Software / Flathub.
```xml
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>com.aashiqumar.watermelon</id>
  <name>Watermelon</name>
  <summary>Fresh. Clean. Friendly. Notes App.</summary>
  <metadata_license>CC0-1.0</metadata_license>
  <project_license>MIT</project_license>
  <description>
    <p>Watermelon is a productivity app designed for Fedora Linux.</p>
  </description>
  <launchable type="desktop-id">com.aashiqumar.watermelon.desktop</launchable>
  <screenshots>
    <screenshot type="default">
      <image>https://raw.githubusercontent.com/example/watermelon/main/docs/assets/watermelon_ui_mockup.png</image>
    </screenshot>
  </screenshots>
</component>
```

## 4. CI/CD Automation (GitHub Actions)

### Workflow: `.github/workflows/nightly.yml`
Builds Flatpak and RPM on every push.

```yaml
name: Nightly Build

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

jobs:
  build-flatpak:
    runs-on: ubuntu-latest
    container:
      image: bilelmoussaoui/flatpak-github-actions:gnome-45
    steps:
    - uses: actions/checkout@v3
    - name: Build Flatpak
      uses: flatpak/flatpak-github-actions/flatpak-builder@v6
      with:
        bundle: watermelon.flatpak
        manifest-path: build-aux/com.aashiqumar.watermelon.json
    - name: Upload Flatpak
      uses: actions/upload-artifact@v3
      with:
        name: watermelon-flatpak
        path: watermelon.flatpak

  build-rpm:
    runs-on: ubuntu-latest
    container: fedora:latest
    steps:
    - uses: actions/checkout@v3
    - name: Install Deps
      run: dnf install -y cargo gtk4-devel libadwaita-devel rpm-build
    - name: Build RPM
      run: |
        # (Simplified for CI)
        cargo build --release
```
