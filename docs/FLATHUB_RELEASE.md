# 🚀 Releasing Watermelon on Flathub

This guide walks you through submitting Watermelon to [Flathub](https://flathub.org), the primary app store for Linux.

## 1. Preparation

### ✅ Screenshot
You must provide a screenshot for the store page.
1.  Run the app: `cargo run`
2.  Take a screenshot of the window.
3.  Save it as `assets/screenshot.png`.
4.  Commit and push:
    ```bash
    git add assets/screenshot.png
    git commit -m "Add screenshot for Flathub"
    git push
    ```

### ✅ Verify Metadata
Ensure `data/com.aashiqumar.watermelon.metainfo.xml` has the correct version and date (I have already updated this for v0.1.0).

## 2. Test the Flatpak Locally

Before submitting, verify it builds:

1.  Install `flatpak-builder`:
    ```bash
    sudo dnf install flatpak-builder
    ```

2.  Build and install locally:
    ```bash
    flatpak-builder --user --install --force-clean build-dir build-aux/com.aashiqumar.watermelon.json
    ```

3.  Run it:
    ```bash
    flatpak run com.aashiqumar.watermelon
    ```

## 3. Submit to Flathub

1.  **Fork Flathub:**
    Go to [https://github.com/flathub/flathub](https://github.com/flathub/flathub) and click **Fork**.

2.  **Clone your fork:**
    ```bash
    git clone https://github.com/YOUR_USERNAME/flathub.git
    cd flathub
    git checkout -b new-app/com.aashiqumar.watermelon
    ```

3.  **Add Manifest:**
    Copy your manifest `build-aux/com.aashiqumar.watermelon.json` to the root of this repo.
    *Note: Flathub prefers the manifest to be named `com.aashiqumar.watermelon.json`.*

4.  **Commit and Push:**
    ```bash
    git add com.aashiqumar.watermelon.json
    git commit -m "Add com.aashiqumar.watermelon"
    git push origin new-app/com.aashiqumar.watermelon
    ```

5.  **Open Pull Request:**
    Go to your fork on GitHub and open a Pull Request against the official `flathub/flathub` repo.
    The Flathub bot will run checks. If everything passes, a maintainer will review and merge it.

6.  **Maintenance:**
    Once merged, a new repository `https://github.com/flathub/com.aashiqumar.watermelon` will be created. You will push future updates (manifest changes) there.

## 4. Fedora Copr (Optional)

If you also want an RPM repository:
1.  Create an account on [Fedora Copr](https://copr.fedorainfracloud.org/).
2.  Create a new project `watermelon`.
3.  Link it to your GitHub repo (it can build from `.spec` files).
    *(Note: You'll need to create a `watermelon.spec` file for this).*
