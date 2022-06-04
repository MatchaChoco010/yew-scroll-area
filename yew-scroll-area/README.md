# yew-scroll-area

## Usage

If you want scrollbars, simply surround the item you want to scroll with a `<ScrollArea></ScrollArea>` component.

```rust
#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}
```
The `<ScrollArea></ScrollArea>` component will be 100% of the width and height of the parent element and will allow scrolling for any child elements that extend beyond it.

The `<ScrollArea></ScrollArea>` component is styled using yew-style-in-rs.In other words, you need to copy and place style.css from the build artifact.

### Display of vertical scrollbars
Add `vertical=true` if you want to use vertical scrollbars.

```rust
#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}
```

### Display of horizontal scrollbars
Add `horizontal=true` if you want to use horizontal scrollbars.

```rust
#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea horizontal=true>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}
```

### Display of scrollbars in both vertical and horizontal directions
Add `vertical=true` and `horizontal=true` if you want to use vertical and horizontal scrollbars.

```rust
#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true horizontal=true>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}
```

### Colorize of scrollbar
Add `color={Color::rgba(128, 255, 0, 0.8)}` if you want to colorize scrollbars.

```rust
#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true color={Color::rgba(128, 255, 0, 0.8)}>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}
```

### Other property of scrollbars
Add attributes width, round, hide_time and smooth_time for styling scrollbars.

```rust
#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true width={4.0} round=false hide_time={0.5} smooth_time={0.3}>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}
```

## Features

- `dry-run`: No write css file to disk. Without this feature, this crate depends on `yew-style-in-rs`, so create and write a CSS file to disk. This feature is useful for document build.

If you would like to publish some components uses `yew-scroll-area` to crates.io, you might need to write following contents to Cargo.toml because crates.io docs build environment can't write filesystem:

```toml
[features]
default = []
dry-run = ["yew-style-in-rs/dry-run"]

[package.metadata.docs.rs]
cargo-args = ["--features=dry-run"]
```

You might need to publish with `dry-run`.

```sh
$ cargo publish --features dry-run
```
