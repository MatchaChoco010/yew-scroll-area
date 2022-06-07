use std::ops::Deref;
use web_sys::HtmlDivElement;
use yew::prelude::*;

#[derive(Clone)]
pub struct UseVerticalThumbSizeHandle {
    size: UseStateHandle<f64>,
    outer_ref: NodeRef,
    inner_ref: NodeRef,
}
impl UseVerticalThumbSizeHandle {
    fn new(size: UseStateHandle<f64>, outer_ref: NodeRef, inner_ref: NodeRef) -> Self {
        Self {
            size,
            outer_ref,
            inner_ref,
        }
    }

    pub fn update(&self) {
        if let (Some(outer), Some(inner)) = (
            self.outer_ref.cast::<HtmlDivElement>(),
            self.inner_ref.cast::<HtmlDivElement>(),
        ) {
            let outer_height = outer.offset_height() as f64;
            let inner_height = inner.offset_height() as f64;
            self.size.set(outer_height / inner_height * outer_height);
        }
    }
}
impl Deref for UseVerticalThumbSizeHandle {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &*self.size
    }
}

pub fn use_vertical_thumb_size(
    outer_ref: NodeRef,
    inner_ref: NodeRef,
) -> UseVerticalThumbSizeHandle {
    let size = use_state_eq(|| 0.0);
    UseVerticalThumbSizeHandle::new(size, outer_ref, inner_ref)
}
