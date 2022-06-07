use gloo::events::EventListener;
use gloo::timers::callback::Timeout;
use gloo::utils::document;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::*;

#[derive(Clone)]
pub struct UseHideStateHandle {
    hide: UseStateHandle<bool>,
    cursor_position: Rc<RefCell<(f32, f32)>>,
    timeout: Rc<RefCell<Option<Timeout>>>,
    outer_ref: NodeRef,
    hide_time: f64,
}
impl UseHideStateHandle {
    fn new(
        hide: UseStateHandle<bool>,
        cursor_position: Rc<RefCell<(f32, f32)>>,
        timeout: Rc<RefCell<Option<Timeout>>>,
        outer_ref: NodeRef,
        hide_time: f64,
    ) -> Self {
        Self {
            hide,
            cursor_position,
            timeout,
            outer_ref,
            hide_time,
        }
    }

    fn show(&self) {
        self.hide.set(false);
        let timeout = Timeout::new((self.hide_time * 1000.0) as u32, {
            let hide = self.hide.clone();
            move || hide.set(true)
        });
        self.timeout.replace(Some(timeout));
    }

    fn use_mouse_position(&self) {
        let this = self.clone();
        use_effect_with_deps(
            move |_| {
                let document = document();
                let listener = EventListener::new(&document, "mousemove", move |evt: &Event| {
                    let evt = evt.dyn_ref::<web_sys::MouseEvent>().unwrap();
                    *this.cursor_position.borrow_mut() =
                        (evt.client_x() as f32, evt.client_y() as f32);
                });
                move || drop(listener)
            },
            (),
        );
    }

    fn use_on_mouseover(&self) {
        use_event(self.outer_ref.clone(), "mouseover", {
            let this = self.clone();
            move |_evt: MouseEvent| this.show()
        });
    }

    fn use_on_mousemove(&self) {
        use_event(self.outer_ref.clone(), "mousemove", {
            let this = self.clone();
            move |_evt: MouseEvent| this.show()
        });
    }

    fn use_on_touchstart(&self) {
        use_event(self.outer_ref.clone(), "touchstart", {
            let this = self.clone();
            move |_evt: TouchEvent| this.show()
        });
    }

    fn use_on_touchmove(&self) {
        use_event(self.outer_ref.clone(), "touchmove", {
            let this = self.clone();
            move |_evt: TouchEvent| this.show()
        });
    }

    pub fn hide_class(&self) -> String {
        if *self.hide {
            "hide".to_string()
        } else {
            "".to_string()
        }
    }
}

pub fn use_hide_state(outer_ref: NodeRef, hide_time: f64) -> UseHideStateHandle {
    let hide = use_state(|| true);
    let timeout = use_mut_ref(|| None);
    let cursor_position = use_mut_ref(|| (0.0, 0.0));

    let handle = UseHideStateHandle::new(hide, cursor_position, timeout, outer_ref, hide_time);
    handle.use_mouse_position();
    handle.use_on_mouseover();
    handle.use_on_mousemove();
    handle.use_on_touchstart();
    handle.use_on_touchmove();
    handle
}
