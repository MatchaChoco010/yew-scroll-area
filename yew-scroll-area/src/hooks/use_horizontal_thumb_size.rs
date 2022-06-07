use std::ops::Deref;
use web_sys::HtmlDivElement;
use yew::prelude::*;

#[derive(Clone)]
pub struct UseHorizontalThumbSizeHandle {
    size: UseStateHandle<f64>,
    outer_ref: NodeRef,
    inner_ref: NodeRef,
}
impl UseHorizontalThumbSizeHandle {
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
            let outer_width = outer.offset_width() as f64;
            let inner_width = inner.offset_width() as f64;
            self.size.set(outer_width / inner_width * outer_width);
        }
    }
}
impl Deref for UseHorizontalThumbSizeHandle {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &*self.size
    }
}

pub fn use_horizontal_thumb_size(
    outer_ref: NodeRef,
    inner_ref: NodeRef,
) -> UseHorizontalThumbSizeHandle {
    let size = use_state_eq(|| 0.0);
    UseHorizontalThumbSizeHandle::new(size, outer_ref, inner_ref)
}
