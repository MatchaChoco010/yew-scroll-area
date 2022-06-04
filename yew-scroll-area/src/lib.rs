#![doc = include_str!("../README.md")]

use gloo::render::request_animation_frame;
use gloo::timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;
use yew::prelude::*;
use yew_hooks::*;
use yew_style_in_rs::*;

fn smooth_damp(
    current: f64,
    target: f64,
    smooth_time: f64,
    current_velocity: f64,
    delta_time: f64,
) -> (f64, f64) {
    let smooth_time = smooth_time.max(0.0001);
    let omega = 2.0 / smooth_time;

    let x = omega * delta_time;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    let change = current - target;

    let temp = (current_velocity + omega * change) * delta_time;
    let mut current_velocity = (current_velocity - omega * temp) * exp;
    let mut output = target + (change + temp) * exp;

    if (target - current > 0.0) == (output > target) {
        output = target;
        current_velocity = (output - target) / delta_time;
    }

    (output, current_velocity)
}

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
    let horizontal = props.horizontal;
    let vertical = props.vertical;
    let color = format!(
        "rgba({}, {}, {}, {})",
        props.color.r, props.color.g, props.color.b, props.color.a
    );
    let hide_time = (props.hide_time * 1000.0) as u32;
    let smooth_time = props.smooth_time;

    // States
    let outer_ref = use_node_ref();
    let inner_ref = use_node_ref();
    let horizontal_thumb_ref = use_node_ref();
    let vertical_thumb_ref = use_node_ref();

    let drag_horizontal_animation_holder = use_mut_ref(|| None);
    let drag_vertical_animation_holder = use_mut_ref(|| None);

    let hide_class = use_state(|| String::from("hide"));
    let hide_timeout_holder = use_mut_ref(|| None);
    let mouse_cursor_pos = use_mut_ref(|| (0.0, 0.0));

    let drag_start_mouse_position = use_mut_ref(|| (0.0, 0.0));
    let drag_start_horizontal_thumb_position = use_mut_ref(|| 0.0);
    let drag_start_vertical_thumb_position = use_mut_ref(|| 0.0);
    let swipe_start_touch_position = use_mut_ref(|| (0.0, 0.0));
    let swipe_start_horizontal_position = use_mut_ref(|| 0.0);
    let swipe_start_vertical_position = use_mut_ref(|| 0.0);

    let horizontal_position = use_state(|| 0.0);
    let horizontal_thumb_is_dragging = use_state(|| false);
    let horizontal_is_swiping = use_state(|| false);

    let vertical_position = use_state(|| 0.0);
    let vertical_thumb_is_dragging = use_state(|| false);
    let vertical_is_swiping = use_state(|| false);

    let horizontal_velocity = use_mut_ref(|| 0.0);
    let horizontal_last_timestamp = use_mut_ref(|| None);
    let horizontal_target = use_mut_ref(|| 0.0);

    let vertical_velocity = use_mut_ref(|| 0.0);
    let vertical_last_timestamp = use_mut_ref(|| None);
    let vertical_target = use_mut_ref(|| 0.0);

    let horizontal_thumb_position = use_state(|| 0.0);
    let horizontal_thumb_width_scale = use_state(|| 0.0);

    let vertical_thumb_position = use_state(|| 0.0);
    let vertical_thumb_height_scale = use_state(|| 0.0);

    let horizontal_thumb_target = use_mut_ref(|| 0.0);
    let horizontal_thumb_velocity = use_mut_ref(|| 0.0);

    let vertical_thumb_target = use_mut_ref(|| 0.0);
    let vertical_thumb_velocity = use_mut_ref(|| 0.0);

    let animation_holder = use_mut_ref(|| None);

    // // Mouse Over Event
    use_event(outer_ref.clone(), "mouseover", {
        let outer_ref = outer_ref.clone();
        let inner_ref = inner_ref.clone();
        let hide_class = hide_class.clone();
        let hide_timeout_holder = hide_timeout_holder.clone();
        let horizontal_target = horizontal_target.clone();
        let horizontal_thumb_target = horizontal_thumb_target.clone();
        let horizontal_thumb_width_scale = horizontal_thumb_width_scale.clone();
        let vertical_target = vertical_target.clone();
        let vertical_thumb_target = vertical_thumb_target.clone();
        let vertical_thumb_height_scale = vertical_thumb_height_scale.clone();
        move |_: Event| {
            if let (Some(outer), Some(inner)) = (
                outer_ref.cast::<HtmlDivElement>(),
                inner_ref.cast::<HtmlDivElement>(),
            ) {
                if horizontal {
                    let outer_width = outer.offset_width() as f64;
                    let inner_width = inner.offset_width() as f64;
                    let scrollable_width = (inner_width - outer_width).max(0.0);
                    let translate_x =
                        (*horizontal_target.borrow() as f64).clamp(-scrollable_width, 0.0);

                    horizontal_thumb_width_scale.set(outer_width / inner_width);
                    *horizontal_thumb_target.borrow_mut() = -translate_x / scrollable_width
                        * (outer_width - *horizontal_thumb_width_scale * outer_width);
                }

                if vertical {
                    let outer_height = outer.offset_height() as f64;
                    let inner_height = inner.offset_height() as f64;
                    let scrollable_height = (inner_height - outer_height).max(0.0);
                    let translate_y =
                        (*vertical_target.borrow() as f64).clamp(-scrollable_height, 0.0);

                    vertical_thumb_height_scale.set(outer_height / inner_height);
                    *vertical_thumb_target.borrow_mut() = -translate_y / scrollable_height
                        * (outer_height - *vertical_thumb_height_scale * outer_height);
                }

                hide_class.set(String::from(""));
                let hide_class = hide_class.clone();
                *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                    hide_class.set(String::from("hide"));
                }));
            }
        }
    });
    use_event(outer_ref.clone(), "mousemove", {
        let hide_class = hide_class.clone();
        let hide_timeout_holder = hide_timeout_holder.clone();
        move |_: Event| {
            hide_class.set(String::from(""));
            let hide_class = hide_class.clone();
            *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                hide_class.set(String::from("hide"));
            }));
        }
    });
    // Mouse Over when scroll in
    let onscroll = {
        let outer_ref = outer_ref.clone();
        let hide_class = hide_class.clone();
        let hide_timeout_holder = hide_timeout_holder.clone();
        let mouse_cursor_pos = mouse_cursor_pos.clone();
        move || {
            let outer = outer_ref.cast::<HtmlDivElement>().unwrap();
            let (x, y) = *mouse_cursor_pos.borrow();
            let elements = gloo::utils::document().elements_from_point(x, y);
            if elements
                .iter()
                .find(|elem| {
                    let elem: Option<&HtmlDivElement> = elem.dyn_ref();
                    if let Some(elem) = elem {
                        elem.is_same_node(Some(&outer))
                    } else {
                        false
                    }
                })
                .is_some()
            {
                hide_class.set(String::from(""));
                let hide_class = hide_class.clone();
                *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                    hide_class.set(String::from("hide"));
                }));
            }
        }
    };

    // Thumb Dragging
    use_event(horizontal_thumb_ref.clone(), "mousedown", {
        let hide_class = hide_class.clone();
        let hide_timeout_holder = hide_timeout_holder.clone();
        let drag_start_mouse_position = drag_start_mouse_position.clone();
        let drag_start_horizontal_thumb_position = drag_start_horizontal_thumb_position.clone();
        let horizontal_thumb_is_dragging = horizontal_thumb_is_dragging.clone();
        let horizontal_thumb_target = horizontal_thumb_target.clone();
        move |evt: MouseEvent| {
            if horizontal {
                horizontal_thumb_is_dragging.set(true);
                *drag_start_mouse_position.borrow_mut() = (evt.x() as f64, evt.y() as f64);
                *drag_start_horizontal_thumb_position.borrow_mut() =
                    *horizontal_thumb_target.borrow();
            }

            hide_class.set(String::from(""));
            let hide_class = hide_class.clone();
            *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                hide_class.set(String::from("hide"));
            }));
        }
    });
    use_event(vertical_thumb_ref.clone(), "mousedown", {
        let hide_class = hide_class.clone();
        let hide_timeout_holder = hide_timeout_holder.clone();
        let drag_start_mouse_position = drag_start_mouse_position.clone();
        let drag_start_vertical_thumb_position = drag_start_vertical_thumb_position.clone();
        let vertical_thumb_is_dragging = vertical_thumb_is_dragging.clone();
        let vertical_thumb_target = vertical_thumb_target.clone();
        move |evt: MouseEvent| {
            if vertical {
                vertical_thumb_is_dragging.set(true);
                *drag_start_mouse_position.borrow_mut() = (evt.x() as f64, evt.y() as f64);
                *drag_start_vertical_thumb_position.borrow_mut() = *vertical_thumb_target.borrow();
            }

            hide_class.set(String::from(""));
            let hide_class = hide_class.clone();
            *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                hide_class.set(String::from("hide"));
            }));
        }
    });
    use_event_with_window("mousemove", {
        let outer_ref = outer_ref.clone();
        let inner_ref = inner_ref.clone();
        let mouse_cursor_pos = mouse_cursor_pos.clone();
        let drag_horizontal_animation_holder = drag_horizontal_animation_holder.clone();
        let drag_start_mouse_position = drag_start_mouse_position.clone();
        let drag_start_horizontal_thumb_position = drag_start_horizontal_thumb_position.clone();
        let horizontal_target = horizontal_target.clone();
        let horizontal_thumb_width_scale = horizontal_thumb_width_scale.clone();
        let horizontal_thumb_target = horizontal_thumb_target.clone();
        let horizontal_thumb_is_dragging = horizontal_thumb_is_dragging.clone();
        let drag_vertical_animation_holder = drag_vertical_animation_holder.clone();
        let drag_start_vertical_thumb_position = drag_start_vertical_thumb_position.clone();
        let vertical_target = vertical_target.clone();
        let vertical_thumb_height_scale = vertical_thumb_height_scale.clone();
        let vertical_thumb_target = vertical_thumb_target.clone();
        let vertical_thumb_is_dragging = vertical_thumb_is_dragging.clone();
        move |evt: MouseEvent| {
            *mouse_cursor_pos.borrow_mut() = (evt.client_x() as f32, evt.client_y() as f32);
            if *horizontal_thumb_is_dragging || *vertical_thumb_is_dragging {
                evt.prevent_default();
            }
            if *horizontal_thumb_is_dragging {
                let x = evt.x() as f64 - drag_start_mouse_position.borrow().0;
                let outer = outer_ref.cast::<HtmlDivElement>().unwrap();
                let inner = inner_ref.cast::<HtmlDivElement>().unwrap();
                let outer_width = outer.offset_width() as f64;
                let inner_width = inner.offset_width() as f64;
                let scrollable_width = (inner_width - outer_width).max(0.0);
                let thumb_translate_x = (*drag_start_horizontal_thumb_position.borrow() + x).clamp(
                    0.0,
                    outer_width - *horizontal_thumb_width_scale * outer_width,
                );

                let horizontal_target = horizontal_target.clone();
                let horizontal_thumb_width_scale = horizontal_thumb_width_scale.clone();
                let horizontal_thumb_target = horizontal_thumb_target.clone();
                let animation = request_animation_frame(move |_| {
                    *horizontal_target.borrow_mut() = -thumb_translate_x
                        / (outer_width - *horizontal_thumb_width_scale * outer_width)
                        * scrollable_width;
                    horizontal_thumb_width_scale.set(outer_width / inner_width);
                    *horizontal_thumb_target.borrow_mut() = thumb_translate_x;
                });
                *drag_horizontal_animation_holder.borrow_mut() = Some(animation);
            }
            if *vertical_thumb_is_dragging {
                let y = evt.y() as f64 - drag_start_mouse_position.borrow().1;
                let outer = outer_ref.cast::<HtmlDivElement>().unwrap();
                let inner = inner_ref.cast::<HtmlDivElement>().unwrap();
                let outer_height = outer.offset_height() as f64;
                let inner_height = inner.offset_height() as f64;
                let scrollable_height = (inner_height - outer_height).max(0.0);
                let thumb_translate_y = (*drag_start_vertical_thumb_position.borrow() + y).clamp(
                    0.0,
                    outer_height - *vertical_thumb_height_scale * outer_height,
                );

                let vertical_target = vertical_target.clone();
                let vertical_thumb_height_scale = vertical_thumb_height_scale.clone();
                let vertical_thumb_target = vertical_thumb_target.clone();
                let animation = request_animation_frame(move |_| {
                    *vertical_target.borrow_mut() = -thumb_translate_y
                        / (outer_height - *vertical_thumb_height_scale * outer_height)
                        * scrollable_height;
                    vertical_thumb_height_scale.set(outer_height / inner_height);
                    *vertical_thumb_target.borrow_mut() = thumb_translate_y;
                });
                *drag_vertical_animation_holder.borrow_mut() = Some(animation);
            }
        }
    });
    use_event_with_window("mouseup", {
        move |_: Event| {
            horizontal_thumb_is_dragging.set(false);
            vertical_thumb_is_dragging.set(false);
        }
    });

    // Swipe
    use_event(outer_ref.clone(), "touchstart", {
        let hide_class = hide_class.clone();
        let hide_timeout_holder = hide_timeout_holder.clone();
        let swipe_start_touch_position = swipe_start_touch_position.clone();
        let swipe_start_horizontal_position = swipe_start_horizontal_position.clone();
        let horizontal_is_dragging = horizontal_is_swiping.clone();
        let horizontal_target = horizontal_target.clone();
        let swipe_start_vertical_position = swipe_start_vertical_position.clone();
        let vertical_is_dragging = vertical_is_swiping.clone();
        let vertical_target = vertical_target.clone();
        move |evt: TouchEvent| {
            if let Some(touch) = evt.touches().item(0) {
                if horizontal {
                    horizontal_is_dragging.set(true);
                    *swipe_start_horizontal_position.borrow_mut() = *horizontal_target.borrow();
                }
                if vertical {
                    vertical_is_dragging.set(true);
                    *swipe_start_vertical_position.borrow_mut() = *vertical_target.borrow();
                }
                if horizontal || vertical {
                    *swipe_start_touch_position.borrow_mut() =
                        (touch.page_x() as f64, touch.page_y() as f64);
                }
            }

            hide_class.set(String::from(""));
            let hide_class = hide_class.clone();
            *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                hide_class.set(String::from("hide"));
            }));
        }
    });
    use_event(outer_ref.clone(), "touchmove", {
        let outer_ref = outer_ref.clone();
        let inner_ref = inner_ref.clone();
        let hide_class = hide_class.clone();
        let hide_timeout_holder = hide_timeout_holder.clone();
        let swipe_start_touch_position = swipe_start_touch_position.clone();
        let swipe_start_horizontal_position = swipe_start_horizontal_position.clone();
        let horizontal_target = horizontal_target.clone();
        let horizontal_thumb_width_scale = horizontal_thumb_width_scale.clone();
        let horizontal_thumb_target = horizontal_thumb_target.clone();
        let horizontal_is_dragging = horizontal_is_swiping.clone();
        let swipe_start_vertical_position = swipe_start_vertical_position.clone();
        let vertical_target = vertical_target.clone();
        let vertical_thumb_height_scale = vertical_thumb_height_scale.clone();
        let vertical_thumb_target = vertical_thumb_target.clone();
        let vertical_is_dragging = vertical_is_swiping.clone();
        move |evt: TouchEvent| {
            if let Some(touch) = evt.touches().item(0) {
                let (start_x, start_y) = *swipe_start_touch_position.borrow();
                if *horizontal_is_dragging {
                    let x = touch.page_x() as f64 - start_x;
                    let outer = outer_ref.cast::<HtmlDivElement>().unwrap();
                    let inner = inner_ref.cast::<HtmlDivElement>().unwrap();
                    let outer_width = outer.offset_width() as f64;
                    let inner_width = inner.offset_width() as f64;
                    let scrollable_width = (inner_width - outer_width).max(0.0);
                    let translate_x = (*swipe_start_horizontal_position.borrow() + x)
                        .clamp(-scrollable_width, 0.0);

                    *horizontal_target.borrow_mut() = translate_x;
                    horizontal_thumb_width_scale.set(outer_width / inner_width);
                    *horizontal_thumb_target.borrow_mut() = -translate_x / scrollable_width
                        * (outer_width - *horizontal_thumb_width_scale * outer_width);

                    hide_class.set(String::from(""));
                    let hide_class = hide_class.clone();
                    *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                        hide_class.set(String::from("hide"));
                    }));
                }
                if *vertical_is_dragging {
                    let y = touch.page_y() as f64 - start_y;
                    let outer = outer_ref.cast::<HtmlDivElement>().unwrap();
                    let inner = inner_ref.cast::<HtmlDivElement>().unwrap();
                    let outer_height = outer.offset_height() as f64;
                    let inner_height = inner.offset_height() as f64;
                    let scrollable_height = (inner_height - outer_height).max(0.0);
                    let translate_y = (*swipe_start_vertical_position.borrow() + y)
                        .clamp(-scrollable_height, 0.0);

                    *vertical_target.borrow_mut() = translate_y;
                    vertical_thumb_height_scale.set(outer_height / inner_height);
                    *vertical_thumb_target.borrow_mut() = -translate_y / scrollable_height
                        * (outer_height - *vertical_thumb_height_scale * outer_height);

                    hide_class.set(String::from(""));
                    let hide_class = hide_class.clone();
                    *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                        hide_class.set(String::from("hide"));
                    }));

                    if !(translate_y == 0.0 || translate_y == -scrollable_height) {
                        evt.stop_propagation();
                    }
                }
            }
        }
    });
    use_event_with_window("touchend", {
        let horizontal_is_dragging = horizontal_is_swiping.clone();
        let vertical_is_dragging = vertical_is_swiping.clone();
        move |_: Event| {
            horizontal_is_dragging.set(false);
            vertical_is_dragging.set(false);
        }
    });

    // Mouse Wheel Event
    use_event(outer_ref.clone(), "wheel", {
        let vertical = props.vertical;
        let horizontal = props.horizontal;
        let outer_ref = outer_ref.clone();
        let inner_ref = inner_ref.clone();
        let hide_class = hide_class.clone();
        let hide_timeout_holder = hide_timeout_holder.clone();
        let horizontal_target = horizontal_target.clone();
        let horizontal_thumb_width_scale = horizontal_thumb_width_scale.clone();
        let horizontal_thumb_target = horizontal_thumb_target.clone();
        let vertical_target = vertical_target.clone();
        let vertical_thumb_height_scale = vertical_thumb_height_scale.clone();
        let vertical_thumb_target = vertical_thumb_target.clone();
        move |evt: WheelEvent| {
            evt.prevent_default();
            let outer = outer_ref.cast::<HtmlDivElement>().unwrap();
            let inner = inner_ref.cast::<HtmlDivElement>().unwrap();

            if horizontal {
                let outer_width = outer.offset_width() as f64;
                let inner_width = inner.offset_width() as f64;
                let scrollable_width = (inner_width - outer_width).max(0.0);
                let delta_x = evt.delta_x();
                let translate_x =
                    (*horizontal_target.borrow() - delta_x).clamp(-scrollable_width, 0.0);

                *horizontal_target.borrow_mut() = translate_x;
                horizontal_thumb_width_scale.set(outer_width / inner_width);
                *horizontal_thumb_target.borrow_mut() = -translate_x / scrollable_width
                    * (outer_width - *horizontal_thumb_width_scale * outer_width);

                if !(translate_x == 0.0 || translate_x == scrollable_width || delta_x == 0.0) {
                    evt.stop_propagation();
                }
            }

            if vertical {
                let outer_height = outer.offset_height() as f64;
                let inner_height = inner.offset_height() as f64;
                let scrollable_height = (inner_height - outer_height).max(0.0);
                let delta_y = evt.delta_y();
                let translate_y =
                    (*vertical_target.borrow() - delta_y).clamp(-scrollable_height, 0.0);

                *vertical_target.borrow_mut() = translate_y;
                vertical_thumb_height_scale.set(outer_height / inner_height);
                *vertical_thumb_target.borrow_mut() = -translate_y / scrollable_height
                    * (outer_height - *vertical_thumb_height_scale * outer_height);

                if !(translate_y == 0.0 || translate_y == -scrollable_height || delta_y == 0.0) {
                    evt.stop_propagation();
                }
            }

            hide_class.set(String::from(""));
            let hide_class = hide_class.clone();
            *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                hide_class.set(String::from("hide"));
            }));

            onscroll();
        }
    });

    // smooth damping of scroll
    {
        let hide_class = hide_class.clone();
        let hide_timeout_holder = hide_timeout_holder.clone();
        let horizontal_target = horizontal_target.clone();
        let horizontal_position = horizontal_position.clone();
        let horizontal_thumb_position = horizontal_thumb_position.clone();
        let vertical_target = vertical_target.clone();
        let vertical_position = vertical_position.clone();
        let vertical_thumb_position = vertical_thumb_position.clone();

        let animation = request_animation_frame(move |timestamp| {
            if horizontal && *horizontal_target.borrow() != *horizontal_position {
                if (*horizontal_target.borrow() - *horizontal_position).abs() > 0.01 {
                    if let Some(last_timestamp) = *horizontal_last_timestamp.borrow() {
                        let delay = (timestamp - last_timestamp) / 1000.0;
                        let (next_x, next_velocity) = smooth_damp(
                            *horizontal_position,
                            *horizontal_target.borrow(),
                            0.15,
                            *horizontal_velocity.borrow(),
                            delay,
                        );
                        horizontal_position.set(next_x);
                        *horizontal_velocity.borrow_mut() = next_velocity;

                        let (next_thumb_x, next_thumb_velocity) = smooth_damp(
                            *horizontal_thumb_position,
                            *horizontal_thumb_target.borrow(),
                            0.15,
                            *horizontal_thumb_velocity.borrow(),
                            delay,
                        );
                        horizontal_thumb_position.set(next_thumb_x);
                        *horizontal_thumb_velocity.borrow_mut() = next_thumb_velocity;
                    }
                    *horizontal_last_timestamp.borrow_mut() = Some(timestamp);
                } else {
                    horizontal_position.set(*horizontal_target.borrow());
                    horizontal_thumb_position.set(*horizontal_thumb_target.borrow());
                    *horizontal_last_timestamp.borrow_mut() = None;
                }

                hide_class.set(String::from(""));
                let hide_class = hide_class.clone();
                *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                    hide_class.set(String::from("hide"));
                }));
            }

            if vertical && *vertical_target.borrow() != *vertical_position {
                if (*vertical_target.borrow() - *vertical_position).abs() > 0.01 {
                    if let Some(last_timestamp) = *vertical_last_timestamp.borrow() {
                        let delay = (timestamp - last_timestamp) / 1000.0;
                        let (next_x, next_velocity) = smooth_damp(
                            *vertical_position,
                            *vertical_target.borrow(),
                            smooth_time,
                            *vertical_velocity.borrow(),
                            delay,
                        );
                        vertical_position.set(next_x);
                        *vertical_velocity.borrow_mut() = next_velocity;

                        let (next_thumb_x, next_thumb_velocity) = smooth_damp(
                            *vertical_thumb_position,
                            *vertical_thumb_target.borrow(),
                            smooth_time,
                            *vertical_thumb_velocity.borrow(),
                            delay,
                        );
                        vertical_thumb_position.set(next_thumb_x);
                        *vertical_thumb_velocity.borrow_mut() = next_thumb_velocity;
                    }
                    *vertical_last_timestamp.borrow_mut() = Some(timestamp);
                } else {
                    vertical_position.set(*vertical_target.borrow());
                    vertical_thumb_position.set(*vertical_thumb_target.borrow());
                    *vertical_last_timestamp.borrow_mut() = None;
                }

                hide_class.set(String::from(""));
                let hide_class = hide_class.clone();
                *hide_timeout_holder.borrow_mut() = Some(Timeout::new(hide_time, move || {
                    hide_class.set(String::from("hide"));
                }));
            }
        });

        *animation_holder.borrow_mut() = Some(animation);
    };

    // values for css
    let horizontal_position = *horizontal_position;
    let vertical_position = *vertical_position;
    let horizontal_thumb_position = *horizontal_thumb_position;
    let vertical_thumb_position = *vertical_thumb_position;
    let horizontal_thumb_width_scale = *horizontal_thumb_width_scale;
    let vertical_thumb_height_scale = *vertical_thumb_height_scale;
    let thumb_width = props.width;
    let horizontal_round = if props.round {
        format!(
            "border-radius: {}px/{}px;",
            thumb_width / 2.0,
            thumb_width / 2.0 / &horizontal_thumb_width_scale
        )
    } else {
        String::from("")
    };
    let vertical_round = if props.round {
        format!(
            "border-radius: {}px/{}px;",
            thumb_width / 2.0,
            thumb_width / 2.0 / &vertical_thumb_height_scale
        )
    } else {
        String::from("")
    };

    // class for hide css
    let hide_class = &*hide_class;

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
                transform: translateX(${horizontal_thumb_position}px) ScaleX(${horizontal_thumb_width_scale});
                height: max(${thumb_width}px, min(24px, calc(20px + ${thumb_width}px)));
                &::before {
                    background-color: ${color};
                    height: ${thumb_width}px;
                    ${horizontal_round}
                }
            }
            & > .vertical-thumb {
                transform: translateY(${vertical_thumb_position}px) ScaleY(${vertical_thumb_height_scale});
                width: max(${thumb_width}px, min(24px, calc(20px + ${thumb_width}px)));
                &::before {
                    background-color: ${color};
                    width: ${thumb_width}px;
                    ${vertical_round}
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
