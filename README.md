<p align="center">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/decipher3114/Capter/master/assets/Banner_dark.png">
      <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/decipher3114/Capter/master/assets/Banner_light.png">
      <img src="https://raw.githubusercontent.com/decipher3114/Capter/master/assets/Banner_dark.png">
    </picture>
</p>

A simple cross-platform screenshot tool made in rust

> Well, this app is not made to replace the native screenshot tools. They are too good.
> The app is only made for learning and experience purposes
> However it is more intuitive than the native ones.

## ✨ Features
- Captures FreeForm, FullScreen and Window
- Keybindings support
- Better usablity (Fullscreen and Window (focused) screenshots are immediate)
- Decent looking UI

> **Shortcomings:**
> - No taskbar support (iced doesn't support taskbar yet)
> - UI is not native

## 📥 Installation
- ### Cargo
    Install Rust and Run the following command
    ```
    cargo install --git https://github.com/decipher3114/Capter
    ```
- ### Prebuilts
    Download from [Releases](https://github.com/decipher3114/Capter/releases/latest)

- Fulfill these [Requirements](#requirements).
- Run `Capter`(Case sensitive) from terminal.

> You can add capter as symlink to startup directory depending on your OS

## 📋 Requirements
- **Linux**  
    - Install the following packages: `libxcb`, `libxrandr`, `dbus`.
        > Package name can be different depending on the distro
    - Add `$USER` to these groups: `input`, `evdev`(if present)

- **Mac OS**  
    - Grant Access to Accesiblity API: Add `Capter.exe` to **System Preferences > Security & Privacy > Privacy > Accessibility**

## 🖥️ Images

![Dark](assets/config_dark.png) | ![Light](assets/config_light.png) |
--- | --- |
Dark Mode | Light Mode |

### 🙌 Thanks to
- [iced](https://github.com/iced-rs) community for their help
- [XelXen](https://github.com/xelxen) for UI
- Other crate maintainers