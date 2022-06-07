#![doc = include_str!("../README.md")]

use gloo::render::request_animation_frame;
use yew::prelude::*;
use yew_style_in_rs::*;

mod hooks;
mod utils;
use hooks::*;

/// Color struct for ScrollArea's scrollbar.
#[derive(Debug, Clone, Copy, PartialEq, Properties)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}
impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn black() -> Self {
        Self::rgba(0, 0, 0, 1.0)
    }

    pub fn white() -> Self {
        Self::rgba(255, 255, 255, 1.0)
    }
}
impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

/// Props for ScrollArea.
#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub horizontal: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub color: Color,
    #[prop_or(4.0)]
    pub width: f64,
    #[prop_or(true)]
    pub round: bool,
    #[prop_or(1.5)]
    pub hide_time: f64,
    #[prop_or(0.15)]
    pub smooth_time: f64,
    pub children: Children,
}

/// ScrollArea component.
#[function_component(ScrollArea)]
pub fn scroll_area(props: &Props) -> Html {
    // props
    let horizontal = props.horizontal;
    let vertical = props.vertical;
    let color = format!(
        "rgba({}, {}, {}, {})",
        props.color.r, props.color.g, props.color.b, props.color.a
    );
    let thumb_width = props.width;
    let thumb_round = if props.round { thumb_width / 2.0 } else { 0.0 };
    let hide_time = props.hide_time;
    let smooth_time = props.smooth_time;

    // Node Refs
    let outer_ref = use_node_ref();
    let inner_ref = use_node_ref();
    let horizontal_thumb_ref = use_node_ref();
    let vertical_thumb_ref = use_node_ref();

    // Hide State
    let hide_state = use_hide_state(outer_ref.clone(), hide_time);

    // Scroll States
    let horizontal_state = use_horizontal_state(
        outer_ref.clone(),
        inner_ref.clone(),
        horizontal_thumb_ref.clone(),
        smooth_time,
    );
    let vertical_state = use_vertical_state(
        outer_ref.clone(),
        inner_ref.clone(),
        vertical_thumb_ref.clone(),
        smooth_time,
    );

    // Animation Frame
    let animation_holder = use_mut_ref(|| None);
    let animation = request_animation_frame({
        let horizontal_state = horizontal_state.clone();
        let vertical_state = vertical_state.clone();
        move |timestamp| {
            horizontal_state.update(timestamp);
            vertical_state.update(timestamp);
        }
    });
    *animation_holder.borrow_mut() = Some(animation);

    let hide_class = hide_state.hide_class();

    let horizontal_position = horizontal_state.scroll_position();
    let horizontal_thumb_position = horizontal_state.thumb_position();
    let horizontal_thumb_size = horizontal_state.thumb_size();

    let vertical_position = vertical_state.scroll_position();
    let vertical_thumb_position = vertical_state.thumb_position();
    let vertical_thumb_size = vertical_state.thumb_size();

    style! {
        let css = css! {r#"
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: relative;
            touch-action: none;

            & > div:first-of-type {
                min-width: 100%;
                min-height: 100%;
                position: absolute;
                top: 0;
                left: 0;
            }

            & > .horizontal-thumb {
                width: 100%;
                background: transparent;
                position: absolute;
                bottom: 4px;
                left: 0;
                display: flex;
                flex-direction: column-reverse;
                transform-origin: left;
                cursor: pointer;

                &::before {
                    content: " ";
                    display: block;
                    width: 100%;
                    opacity: 1;
                    transition: opacity 0.2s;
                }
            }

            & > .vertical-thumb {
                height: 100%;
                background: transparent;
                position: absolute;
                top: 0;
                right: 4px;
                display: flex;
                flex-direction: row-reverse;
                transform-origin: top;
                cursor: pointer;

                &::before {
                    content: " ";
                    display: block;
                    height: 100%;
                    opacity: 1;
                    transition: opacity 0.2s;
                }
            }

            &.hide > .horizontal-thumb::before {
                opacity: 0;
            }
            &.hide > .vertical-thumb::before {
                opacity: 0;
            }
        "#};
        let dyn_css = dyn css! {r#"
            & > div:first-of-type {
                transform: translateY(${vertical_position}px) translateX(${horizontal_position}px);
            }
            & > .horizontal-thumb {
                transform: translateX(${horizontal_thumb_position}px);
                width: ${horizontal_thumb_size}px;
                height: max(${thumb_width}px, min(24px, calc(20px + ${thumb_width}px)));
                &::before {
                    background-color: ${color};
                    height: ${thumb_width}px;
                    border-radius: ${thumb_round}px;
                }
            }
            & > .vertical-thumb {
                transform: translateY(${vertical_thumb_position}px);
                width: max(${thumb_width}px, min(24px, calc(20px + ${thumb_width}px)));
                height: ${vertical_thumb_size}px;
                &::before {
                    background-color: ${color};
                    width: ${thumb_width}px;
                    border-radius: ${thumb_round}px;
                }
            }
        "#};
    }
    html! {
        <div ref={outer_ref} class={classes!(css, dyn_css, hide_class)}>
            <div ref={inner_ref}>
                {props.children.clone()}
            </div>
            if horizontal {
                <div ref={horizontal_thumb_ref} class="horizontal-thumb" />
            }
            if vertical {
                <div ref={vertical_thumb_ref} class="vertical-thumb" />
            }
        </div>
    }
}
