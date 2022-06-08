use yew::prelude::*;

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
