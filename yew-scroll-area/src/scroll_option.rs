#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollOption {
    pub mouse_wheel_smooth_time: f64,
    pub mouse_drag_thumb_smooth_time: f64,
    pub mouse_wheel_speed_scale: f64,
    pub touch_swipe_smooth_time: f64,
    pub touch_drag_thumb_smooth_time: f64,
    pub touch_swipe_speed_scale: f64,
}
impl Default for ScrollOption {
    fn default() -> Self {
        ScrollOption {
            mouse_wheel_smooth_time: 0.15,
            mouse_drag_thumb_smooth_time: 0.05,
            mouse_wheel_speed_scale: 1.0,
            touch_swipe_smooth_time: 0.15,
            touch_drag_thumb_smooth_time: 0.05,
            touch_swipe_speed_scale: 1.0,
        }
    }
}
