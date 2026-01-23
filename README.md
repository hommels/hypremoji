📘 También disponible en [Español 🇪🇸](./README.es.md)

# 😀 HyprEmoji

HyprEmoji is a lightweight and fast emoji picker for the **Hyprland** window manager, built with GTK4 and Rust.  
A sleek way to copy emojis into any window on your system!

![preview](./banner.png)
![preview](./screenshot.png)

## ✨ Features

- 🔍 **Instant search:** find emojis by typing their name (with debounce included).
- 📂 **Category navigation:** Smileys, Animals, Food, Objects, and more!
- 📋 **Clipboard copy with auto Ctrl+V** into the focused window.
- 🧠 **Recent history:** frequently used emojis are saved automatically.
- 🎨 **Modern and minimal UI**, customizable through CSS (including on launch with `hypremoji -s <path>`).
- 💾 **Remembers window size and position** across sessions.

## 📥 Installation

### 📦 From the AUR *(recommended)*

```bash
paru -S hypremoji
```
Or...
```bash
yay -S hypremoji
```

> ✅ Once installed, it will try to auto-configure your `hyprland.conf` with the necessary rules.

🎉 Launch with `Super` + `.` and enjoy!

## ⚙️ Manual Installation


### 📦 Requirements

- **Rust + Cargo**
- **GTK 4**
- **Hyprland** (with `hyprctl`)
- **wl-clipboard** (`wl-copy`, `wl-paste`)
- **Noto Color Emoji (Default)** or similar font 

### 🚀 Steps

1. 🎯 Make sure you have **Rust** and **GTK 4** installed.
2. 📥 Clone the repo:

```bash
git clone https://github.com/Musagy/HyprEmoji.git
cd HyprEmoji
```
> Using Arch? Just run `makepkg -si` and you’re done.
>
> Otherwise, continue with the steps below.

3. ⚙️ Add this to your `hyprland.conf`:

```conf
# SUPER + PERIOD to open Hypremoji
bind = SUPER, period, exec, hypremoji

# Window rules for HyprEmoji
windowrule = float true, match:title ^(HyprEmoji)$ 
windowrule = move (cursor_x-(window_w*0.5)) (cursor_y-(window_h*0.05)), match:title ^(HyprEmoji)$
```

4. 🛠️ Build it:

```bash
cargo build --release
```

5. 🎉 Launch with `Super` + `.` and enjoy!

### Quick dependency install (Arch Linux):

```bash
sudo pacman -S gtk4 wl-clipboard noto-fonts-emoji
```

>⚠️ You also need a running Hyprland setup for this to work!

## 🖱️ CLI Commands

HyprEmoji includes a command-line interface for configuration:
```bash
# Show help
hypremoji --help

# Launch with a custom CSS file for this session
hypremoji -s ~/.config/hypremoji/dark.css

# Reset configuration to defaults (window follows cursor below)
hypremoji reset

# Configure window position behavior
  # Window appears above cursor
hypremoji init-in-mouse up

  # Window appears below cursor (default)
hypremoji init-in-mouse down

  # Disables cursor follow uses last pinned position
  # (or screen center if no pin set)
hypremoji init-in-mouse none
```

> 💡 **Tip:** By default, the window follows your cursor and appears below it. You can change this behavior anytime with the commands above, or use the 📌 pin button inside the app to set a fixed position.

## 🎨 Customization

You can tweak the theme via:

```bash
~/.config/hypremoji/style.css
```

You can also keep multiple variants (for example, `dark.css`, `light.css`) and launch Hypremoji with any of them on demand using `hypremoji -s /path/to/theme.css`. If you want to bind a specific theme to a Hyprland shortcut, update the corresponding `bind = … hypremoji` line in your Hyprland config to append the `-s` flag with the desired file.

#### Example:

```css
:root {
  --primary-col: #4b60a5;
  --primary-col-glow: #4b60a5aa;
  --gray: #444;
  --bg-col: #0F0F0F;
  --input-text-col: #FFFFFF;
  --btn-list-col: #181818;
  --entry-unfocus: #c41313;
  --btn-list-col-hover: #272727;
  --btn-list-col-hover-glow: #27272777;
  --btn-nav-col: #3E3E3E;
  --btn-nav-col-hover: #0F0F0F;
  --emoji-font: "Noto Color Emoji";
}
```

> 💬 Want that classic Discord emoji look? Check out [Twemoji](https://github.com/twitter/twemoji), it's the same font they use.

#### How change the 📌 icon color:

Edit the `fill="#xxxxxx"` values in:

```bash
/usr/share/hypremoji/assets/icons/AiFillPushpin.svg
```

## 🤝 Contributions

Ideas, bug reports, and pull requests are very welcome!  
Open an [issue](https://github.com/Musagy/HyprEmoji/issues) or collaborate directly.

## 📄 License

This project is licensed under **ISC**. See [`LICENSE`](./LICENSE) for more details.

## 💸 Support me 

<p align="center"> 
  <a href="https://www.buymeacoffee.com/musagy" target="_blank" >
    <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;">
  </a>
</p>

![tengo-hambre](https://i.imgur.com/dT2gV43.png)  

<p align="center"> I'm hungry 🥵 </p>
