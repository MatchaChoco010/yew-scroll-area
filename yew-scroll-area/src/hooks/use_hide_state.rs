use gloo::timers::callback::Timeout;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::*;

#[derive(Clone)]
pub struct UseHideStateHandle {
    hide: UseStateHandle<bool>,
    timeout: Rc<RefCell<Option<Timeout>>>,
    outer_ref: NodeRef,
    hide_time: Option<f64>,
}
impl UseHideStateHandle {
    fn new(
        hide: UseStateHandle<bool>,
        timeout: Rc<RefCell<Option<Timeout>>>,
        outer_ref: NodeRef,
        hide_time: Option<f64>,
    ) -> Self {
        Self {
            hide,
            timeout,
            outer_ref,
            hide_time,
        }
    }

    fn show(&self) {
        if let Some(hide_time) = self.hide_time {
            self.hide.set(false);
            let timeout = Timeout::new((hide_time * 1000.0) as u32, {
                let hide = self.hide.clone();
                move || hide.set(true)
            });
            self.timeout.replace(Some(timeout));
        }
    }
    fn use_on_wheel(&self) {
        use_event(self.outer_ref.clone(), "wheel", {
            let this = self.clone();
            move |_evt: MouseEvent| this.show()
        });
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

pub fn use_hide_state(outer_ref: NodeRef, hide_time: Option<f64>) -> UseHideStateHandle {
    let hide = use_state_eq(|| hide_time.is_some());
    let timeout = use_mut_ref(|| None);

    if hide_time.is_none() {
        hide.set(false);
    }

    let handle = UseHideStateHandle::new(hide, timeout, outer_ref, hide_time);
    handle.use_on_wheel();
    handle.use_on_mouseover();
    handle.use_on_mousemove();
    handle.use_on_touchstart();
    handle.use_on_touchmove();
    handle
}
