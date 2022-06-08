use std::cell::RefCell;
use std::rc::Rc;
use web_sys::HtmlDivElement;
use yew::prelude::*;
use yew_hooks::*;

use crate::hooks::*;
use crate::utils::*;

use crate::ScrollOption;

#[derive(Clone)]
pub struct UseVerticalStateHandle {
    scroll_position_state: UseSmoothDampHandle,
    thumb_position_state: UseSmoothDampHandle,
    thumb_size_state: UseVerticalThumbSizeHandle,
    is_thumb_dragging: Rc<RefCell<bool>>,
    drag_start_mouse_position: Rc<RefCell<f64>>,
    drag_start_thumb_position: Rc<RefCell<f64>>,
    is_touch_scrolling: Rc<RefCell<bool>>,
    touch_start_touch_position: Rc<RefCell<f64>>,
    touch_start_scroll_position: Rc<RefCell<f64>>,
    is_inner_scrolling: Rc<RefCell<bool>>,
    scroll_option: ScrollOption,
    outer_ref: NodeRef,
    inner_ref: NodeRef,
    thumb_ref: NodeRef,
}
impl UseVerticalStateHandle {
    fn new(
        scroll_position_state: UseSmoothDampHandle,
        thumb_position_state: UseSmoothDampHandle,
        thumb_size_state: UseVerticalThumbSizeHandle,
        is_thumb_dragging: Rc<RefCell<bool>>,
        drag_start_mouse_position: Rc<RefCell<f64>>,
        drag_start_thumb_position: Rc<RefCell<f64>>,
        is_touch_scrolling: Rc<RefCell<bool>>,
        touch_start_touch_position: Rc<RefCell<f64>>,
        touch_start_scroll_position: Rc<RefCell<f64>>,
        is_inner_scrolling: Rc<RefCell<bool>>,
        smooth_option: ScrollOption,
        outer_ref: NodeRef,
        inner_ref: NodeRef,
        thumb_ref: NodeRef,
    ) -> Self {
        Self {
            scroll_position_state,
            thumb_position_state,
            thumb_size_state,
            is_thumb_dragging,
            drag_start_mouse_position,
            drag_start_thumb_position,
            is_touch_scrolling,
            touch_start_touch_position,
            touch_start_scroll_position,
            is_inner_scrolling,
            scroll_option: smooth_option,
            outer_ref,
            inner_ref,
            thumb_ref,
        }
    }

    fn use_on_wheel(&self) {
        use_event(self.outer_ref.clone(), "wheel", {
            let this = self.clone();
            move |evt: WheelEvent| {
                evt.prevent_default();
                if let (Some(outer), Some(inner)) = (
                    this.outer_ref.cast::<HtmlDivElement>(),
                    this.inner_ref.cast::<HtmlDivElement>(),
                ) {
                    let already_processed =
                        get_event_custom_flag(&evt, "yew-scroll-area-vertical-wheel-processed");
                    if let Some(flag) = already_processed {
                        if flag {
                            return;
                        }
                    }

                    let delta = evt.delta_y();
                    let delta = delta * this.scroll_option.mouse_wheel_speed_scale;
                    let outer_height = outer.offset_height() as f64;
                    let inner_height = inner.offset_height() as f64;
                    let scrollable_height = (inner_height - outer_height).max(0.0);

                    let scroll_translation = (this.scroll_position_state.target() - delta)
                        .clamp(-scrollable_height, 0.0);
                    let thumb_translation = if scrollable_height == 0.0 {
                        0.0
                    } else {
                        -scroll_translation / scrollable_height
                            * (outer_height - *this.thumb_size_state)
                    };

                    this.scroll_position_state
                        .set_smooth_time(this.scroll_option.mouse_wheel_smooth_time);
                    this.thumb_position_state
                        .set_smooth_time(this.scroll_option.mouse_wheel_smooth_time);
                    this.scroll_position_state.set_target(scroll_translation);
                    this.thumb_position_state.set_target(thumb_translation);

                    let processed = !((delta < 0.0 && scroll_translation == 0.0)
                        || (delta > 0.0 && scroll_translation == -scrollable_height));
                    set_event_custom_flag(
                        &evt,
                        "yew-scroll-area-vertical-wheel-processed",
                        processed,
                    );
                }
            }
        });
    }

    fn use_on_start_drag_thumb(&self) {
        use_event(self.thumb_ref.clone(), "mousedown", {
            let this = self.clone();
            move |evt: MouseEvent| {
                if this.thumb_ref.get().is_some() {
                    let mouse_position = evt.client_y() as f64;
                    *this.is_thumb_dragging.borrow_mut() = true;
                    *this.drag_start_mouse_position.borrow_mut() = mouse_position;
                    *this.drag_start_thumb_position.borrow_mut() = *this.thumb_position_state;
                }
            }
        });
        use_event(self.thumb_ref.clone(), "touchstart", {
            let this = self.clone();
            move |evt: TouchEvent| {
                if this.thumb_ref.get().is_some() {
                    if let Some(touch) = evt.touches().item(0) {
                        let touch_position = touch.client_y() as f64;
                        *this.is_thumb_dragging.borrow_mut() = true;
                        *this.drag_start_mouse_position.borrow_mut() = touch_position;
                        *this.drag_start_thumb_position.borrow_mut() = *this.thumb_position_state;
                    }
                }
            }
        });
    }

    fn use_on_drag_thumb(&self) {
        use_event_with_window("mousemove", {
            let this = self.clone();
            move |evt: MouseEvent| {
                if *this.is_thumb_dragging.borrow() {
                    evt.prevent_default();
                    if let (Some(outer), Some(inner)) = (
                        this.outer_ref.cast::<HtmlDivElement>(),
                        this.inner_ref.cast::<HtmlDivElement>(),
                    ) {
                        let mouse_position = evt.client_y() as f64;
                        let y = mouse_position - *this.drag_start_mouse_position.borrow();
                        let outer_height = outer.offset_height() as f64;
                        let inner_height = inner.offset_height() as f64;
                        let scrollable_height = (inner_height - outer_height).max(0.0);

                        let thumb_translation = (*this.drag_start_thumb_position.borrow() + y)
                            .clamp(0.0, (outer_height - *this.thumb_size_state).max(0.0));
                        let scroll_translation = -thumb_translation
                            / (outer_height - *this.thumb_size_state)
                            * scrollable_height;

                        this.scroll_position_state
                            .set_smooth_time(this.scroll_option.mouse_drag_thumb_smooth_time);
                        this.thumb_position_state
                            .set_smooth_time(this.scroll_option.mouse_drag_thumb_smooth_time);
                        this.scroll_position_state.set_target(scroll_translation);
                        this.thumb_position_state.set_target(thumb_translation);
                    }
                }
            }
        });
        use_event_with_window("touchmove", {
            let this = self.clone();
            move |evt: TouchEvent| {
                if *this.is_thumb_dragging.borrow() {
                    if let (Some(outer), Some(inner)) = (
                        this.outer_ref.cast::<HtmlDivElement>(),
                        this.inner_ref.cast::<HtmlDivElement>(),
                    ) {
                        if let Some(touch) = evt.touches().item(0) {
                            let touch_position = touch.client_y() as f64;
                            let y = touch_position - *this.drag_start_mouse_position.borrow();
                            let outer_height = outer.offset_height() as f64;
                            let inner_height = inner.offset_height() as f64;
                            let scrollable_height = (inner_height - outer_height).max(0.0);

                            let thumb_translation = (*this.drag_start_thumb_position.borrow() + y)
                                .clamp(0.0, (outer_height - *this.thumb_size_state).max(0.0));
                            let scroll_translation = -thumb_translation
                                / (outer_height - *this.thumb_size_state)
                                * scrollable_height;

                            this.scroll_position_state
                                .set_smooth_time(this.scroll_option.touch_drag_thumb_smooth_time);
                            this.thumb_position_state
                                .set_smooth_time(this.scroll_option.touch_drag_thumb_smooth_time);
                            this.scroll_position_state.set_target(scroll_translation);
                            this.thumb_position_state.set_target(thumb_translation);

                            set_event_custom_flag(
                                &evt,
                                "yew-scroll-area-vertical-touchmove-processed",
                                true,
                            );
                        }
                    }
                }
            }
        });
    }

    fn use_on_end_drag_thumb(&self) {
        use_event_with_window("mouseup", {
            let this = self.clone();
            move |_: Event| {
                *this.is_thumb_dragging.borrow_mut() = false;
            }
        });
        use_event_with_window("touchend", {
            let this = self.clone();
            move |_: Event| {
                *this.is_thumb_dragging.borrow_mut() = false;
            }
        });
    }

    fn use_on_touch_start(&self) {
        use_event(self.outer_ref.clone(), "touchstart", {
            let this = self.clone();
            move |evt: TouchEvent| {
                if let Some(touch) = evt.touches().item(0) {
                    let touch_position = touch.client_y() as f64;
                    *this.is_touch_scrolling.borrow_mut() = true;
                    *this.touch_start_touch_position.borrow_mut() = touch_position;
                    *this.touch_start_scroll_position.borrow_mut() = *this.scroll_position_state;
                }
            }
        });
    }

    fn use_on_touch_move(&self) {
        use_event(self.outer_ref.clone(), "touchmove", {
            let this = self.clone();
            move |evt: TouchEvent| {
                if let Some(touch) = evt.touches().item(0) {
                    let touch_position = touch.client_y() as f64;
                    if *this.is_touch_scrolling.borrow() {
                        if let (Some(outer), Some(inner)) = (
                            this.outer_ref.cast::<HtmlDivElement>(),
                            this.inner_ref.cast::<HtmlDivElement>(),
                        ) {
                            // if thumb is dragged, it should not be process
                            if *this.is_thumb_dragging.borrow() {
                                set_event_custom_flag(
                                    &evt,
                                    "yew-scroll-area-vertical-touchmove-processed",
                                    true,
                                );
                                return;
                            }

                            // if inner scroll area is scrolled, it should not be scrolling
                            let already_processed = get_event_custom_flag(
                                &evt,
                                "yew-scroll-area-vertical-touchmove-processed",
                            );
                            if let Some(flag) = already_processed {
                                if flag {
                                    *this.is_inner_scrolling.borrow_mut() = true;
                                    return;
                                } else {
                                    if *this.is_inner_scrolling.borrow() {
                                        *this.touch_start_touch_position.borrow_mut() =
                                            touch_position;
                                        *this.touch_start_scroll_position.borrow_mut() =
                                            *this.scroll_position_state;
                                        *this.is_inner_scrolling.borrow_mut() = false;
                                    }
                                }
                            }

                            let y =
                                touch_position as f64 - *this.touch_start_touch_position.borrow();
                            let y = y * this.scroll_option.touch_swipe_speed_scale;
                            let outer_height = outer.offset_height() as f64;
                            let inner_height = inner.offset_height() as f64;
                            let scrollable_height = (inner_height - outer_height).max(0.0);

                            let scroll_translation = (*this.touch_start_scroll_position.borrow()
                                + y)
                                .clamp(-scrollable_height, 0.0);
                            let thumb_translation = if scrollable_height != 0.0 {
                                -scroll_translation / scrollable_height
                                    * (outer_height - *this.thumb_size_state)
                            } else {
                                0.0
                            };

                            this.scroll_position_state
                                .set_smooth_time(this.scroll_option.touch_swipe_smooth_time);
                            this.thumb_position_state
                                .set_smooth_time(this.scroll_option.touch_swipe_smooth_time);
                            this.scroll_position_state.set_target(scroll_translation);
                            this.thumb_position_state.set_target(thumb_translation);

                            // if scrolling is not ended, stop scrolling parent scroll area
                            let processed = !((y > 0.0 && scroll_translation == 0.0)
                                || (y < 0.0 && scroll_translation == -scrollable_height));
                            set_event_custom_flag(
                                &evt,
                                "yew-scroll-area-vertical-touchmove-processed",
                                processed,
                            );
                        }
                    }
                }
            }
        });
    }

    fn use_on_touch_end(&self) {
        use_event_with_window("touchend", {
            let this = self.clone();
            move |_: Event| {
                *this.is_touch_scrolling.borrow_mut() = false;
            }
        });
    }

    pub fn update(&self, timestamp: f64) {
        self.scroll_position_state.update(timestamp);
        self.thumb_position_state.update(timestamp);
        self.thumb_size_state.update();
    }

    pub fn scroll_position(&self) -> f64 {
        *self.scroll_position_state
    }

    pub fn thumb_position(&self) -> f64 {
        *self.thumb_position_state
    }

    pub fn thumb_size(&self) -> f64 {
        *self.thumb_size_state
    }
}

pub fn use_vertical_state(
    outer_ref: NodeRef,
    inner_ref: NodeRef,
    thumb_ref: NodeRef,
    scroll_option: ScrollOption,
) -> UseVerticalStateHandle {
    let scroll_position_state = use_smooth_damp();
    let thumb_position_state = use_smooth_damp();
    let thumb_size_state = use_vertical_thumb_size(outer_ref.clone(), inner_ref.clone());

    let is_thumb_dragging = use_mut_ref(|| false);
    let drag_start_mouse_position = use_mut_ref(|| 0.0);
    let drag_start_thumb_position = use_mut_ref(|| 0.0);

    let is_touch_scrolling = use_mut_ref(|| false);
    let touch_start_touch_position = use_mut_ref(|| 0.0);
    let touch_start_scroll_position = use_mut_ref(|| 0.0);

    let is_inner_scrolling = use_mut_ref(|| false);

    let handle = UseVerticalStateHandle::new(
        scroll_position_state,
        thumb_position_state,
        thumb_size_state,
        is_thumb_dragging,
        drag_start_mouse_position,
        drag_start_thumb_position,
        is_touch_scrolling,
        touch_start_touch_position,
        touch_start_scroll_position,
        is_inner_scrolling,
        scroll_option,
        outer_ref,
        inner_ref,
        thumb_ref,
    );

    handle.use_on_wheel();

    handle.use_on_start_drag_thumb();
    handle.use_on_drag_thumb();
    handle.use_on_end_drag_thumb();

    handle.use_on_touch_start();
    handle.use_on_touch_move();
    handle.use_on_touch_end();

    handle
}
