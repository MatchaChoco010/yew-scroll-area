#![doc = include_str!("../README.md")]

use gloo::render::request_animation_frame;
use yew::prelude::*;
use yew_style_in_rs::*;

mod color;
mod default_thumb;
mod hooks;
mod scroll_option;
mod utils;

use default_thumb::*;
use hooks::*;

pub use color::Color;
pub use scroll_option::ScrollOption;

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
    #[prop_or(10.0)]
    pub draggable_width: f64,
    #[prop_or(true)]
    pub round: bool,
    #[prop_or(Some(1.5))]
    pub hide_time: Option<f64>,
    #[prop_or_default]
    pub scroll_option: ScrollOption,
    pub custom_vertical_thumb: Option<Html>,
    pub custom_horizontal_thumb: Option<Html>,
    pub children: Children,
}

/// ScrollArea component.
#[function_component(ScrollArea)]
pub fn scroll_area(props: &Props) -> Html {
    // props
    let horizontal = props.horizontal;
    let vertical = props.vertical;
    let color = props.color;
    let width = props.width;
    let draggable_width = props.draggable_width;
    let round = props.round;
    let hide_time = props.hide_time;
    let scroll_option = props.scroll_option;

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
        scroll_option,
    );
    let vertical_state = use_vertical_state(
        outer_ref.clone(),
        inner_ref.clone(),
        vertical_thumb_ref.clone(),
        scroll_option,
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
                bottom: 0;
                left: 0;
                opacity: 1;
                transition: opacity 0.2s;
                cursor: pointer;
            }

            & > .vertical-thumb {
                height: 100%;
                background: transparent;
                position: absolute;
                top: 0;
                right: 0;
                opacity: 1;
                transition: opacity 0.2s;
                cursor: pointer;
            }

            &.hide > .horizontal-thumb {
                opacity: 0;
            }
            &.hide > .vertical-thumb {
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
                height: ${draggable_width}px;
            }
            & > .vertical-thumb {
                transform: translateY(${vertical_thumb_position}px);
                width: ${draggable_width}px;
                height: ${vertical_thumb_size}px;
            }
        "#};
    }
    html! {
        <div ref={outer_ref} class={classes!(css, dyn_css, hide_class)}>
            <div ref={inner_ref}>
                {props.children.clone()}
            </div>
            if horizontal {
                <div ref={horizontal_thumb_ref} class="horizontal-thumb">
                    if let Some(custom_horizontal_thumb) = props.custom_horizontal_thumb.clone() {
                        {custom_horizontal_thumb}
                    } else {
                        <DefaultHorizontalThumb {color} {width} {round}/>
                    }
                </div>
            }
            if vertical {
                <div ref={vertical_thumb_ref} class="vertical-thumb">
                    if let Some(custom_vertical_thumb) = props.custom_vertical_thumb.clone() {
                        {custom_vertical_thumb}
                    } else {
                        <DefaultVerticalThumb {color} {width} {round}/>
                    }
                </div>
            }
        </div>
    }
}
