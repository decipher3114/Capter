<p align="center">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/decipher3114/Capter/master/assets/images/banner_dark.png">
      <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/decipher3114/Capter/master/assets/images/banner_light.png">
      <img src="https://raw.githubusercontent.com/decipher3114/Capter/master/assets/images/banner_dark.png">
    </picture>
</p>

A cross-platform screenshot tool made in Rust

## ✨ Features
- Captures Cropped, FullScreen and Window
- Region Selection assist
- Annotations support
- Beautiful minimalistic UI

## 📥 Installation
- Fulfill these [Requirements](#requirements).
- Prebuilts
    - Download from [Releases](https://github.com/decipher3114/Capter/releases/latest)
        - **.msi** for Windows
        - **.dmg** for Mac OS
        - **.deb** for Debian
- Compile
    - cargo
        ```
        cargo install --git https://github.com/decipher3114/Capter
        ```
    - AUR helper
        Package: `capter`

## 📋 Requirements
- **Linux**  
    - Install the following packages:
        - **Debian**
        ```
        sudo apt install libgtk-3-dev libxdo-dev libayatana-appindicator3-dev libxcb1 libxrandr2 libdbus-1-3
        ```
        - **Arch**
        ```
        sudo pacman -S gtk3 xdotool libayatana-appindicator libxcb libxrandr dbus
        ```
    - Add `$USER` to these groups: `input`, `evdev`(if present)

- **Mac OS**  
    - Grant Access to Accesiblity API: Add `Capter.exe` to **System Preferences > Security & Privacy > Privacy > Accessibility**

## 📖 Usage
- `Alt + Shift + S` to trigger Screenshot.
- `Alt + Shift + O` to open App window.
- Selection Mode
    - Hover mouse over a Window to select it, else Fullscreen will be selected
    - Click and Drag mouse to crop custom area
    - `Esc` to Cancel
    - `Enter` to Capture
    - Choose any Shape to enter **Annotation Mode**
    
- Annotation Mode
    - Click and Drag to Draw
    - `Ctrl + Z` to Undo
    - `Esc` to cancel annotations
    - `Enter` to switch back to **Selection mode**

### 🙌 Thanks to
- [iced](https://github.com/iced-rs) community for their help
- [XelXen](https://github.com/xelxen) for UI
- Other crate maintainers