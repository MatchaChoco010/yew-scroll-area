# yew-scroll-area
[![crates.io](https://img.shields.io/crates/v/yew-scroll-area.svg)](https://crates.io/crates/yew-style-in-rs)
[![docs.rs](https://docs.rs/yew-scroll-area/badge.svg)](https://docs.rs/yew-style-in-rs)
[![yew version: 0.19.3](https://img.shields.io/badge/yew%20version-0.19.3-yellow)](https://docs.rs/egui/0.14.2/egui/index.html)

Custom scroll area for yew.

## example

```sh
$ cd example/app
$ trunk serve --release
```

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

## License
MIT OR Apache-2.0
