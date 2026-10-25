# egui-orrin-theme

The editor theme from the [Orrin](https://github.com/Orrin-Team/orrin) engine,
for any [egui](https://github.com/emilk/egui) app: dense, dark and quiet. Saturated
colour is kept for things that carry meaning.

```toml
[dependencies]
egui-orrin-theme = { git = "https://github.com/Orrin-Team/egui-orrin-theme" }
```

```rust
egui_orrin_theme::install_fonts(&ctx); // optional: JetBrains Mono + Phosphor icons
egui_orrin_theme::apply(&ctx, &egui_orrin_theme::Theme::default());
```

Try it with `cargo run --example demo`.

## A theme is nine greys and an accent

Every value in egui's `Visuals` is derived from ten colours. That makes a user
theme ten lines of TOML:

```toml
name = "Void"
text = [220, 224, 230]
panel = [16, 16, 18]
window = [22, 22, 25]
extreme = [10, 10, 12]
faint = [28, 28, 32]
widget = [38, 38, 44]
widget_hover = [50, 50, 58]
outline = [44, 44, 50]
accent = [140, 120, 255]
```

`ThemeSet::load(dir)` reads a folder of these in filename order and puts the
built-in theme first. It skips any file that fails to parse and returns the
error, so one bad file never costs you the rest.

## What it decides for you

- **Selection is a tint, not a flood.** A selected row is the panel colour
  moved a third of the way toward the accent.
- **Hover is grey.** The accent is kept for what is selected, focused or pressed.
- **Surfaces get lighter as they come forward.** Fields sit below the panel and
  windows sit above it.
- **Four type sizes:** 10.5, 11.5 (mono), 12.5 and 15. Emphasis comes from
  weight and colour, never size.
- **One control height** (22px), so a row of mixed widgets lines up.
- **Almost no motion.** Only disclosure triangles animate (120ms).
- **Floating scroll bars** stay hidden until the pointer comes near them.
- **Status colours** (`OK`, `PENDING`, `ERROR`) are fixed across themes, so
  green means the same thing in every palette.

Windows are opaque by default. Orrin draws its popups as frosted glass over a
blurred scene. If your renderer blurs behind windows too, use
`apply_with(ctx, theme, &Options { window_opacity: 0.66 })`.

## Features

| feature | default | what it adds |
|---|---|---|
| `fonts` | yes | JetBrains Mono as the first monospace face (+270 KB) |
| `icons` | yes | The Phosphor icon font and `icons::*` helpers (+0.5 MB) |
| `toml`  | yes | `ThemeSet::load` |

## License

The code is licensed under either of [Apache-2.0](LICENSE-APACHE) or
[MIT](LICENSE-MIT), at your option.

The bundled fonts keep their own licenses:
[JetBrains Mono](https://github.com/JetBrains/JetBrainsMono) is under the SIL Open
Font License 1.1 ([`assets/fonts/OFL.txt`](assets/fonts/OFL.txt)), and
[Phosphor Icons](https://phosphoricons.com) 2.1.1 is under MIT
([`assets/fonts/PHOSPHOR-LICENSE`](assets/fonts/PHOSPHOR-LICENSE)).
