use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;

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

#[derive(Clone)]
pub struct UseSmoothDampHandle {
    current: UseStateHandle<f64>,
    target: UseStateHandle<f64>,
    current_velocity: Rc<RefCell<f64>>,
    last_timestamp: Rc<RefCell<f64>>,
    smooth_time: Rc<RefCell<f64>>,
}
impl UseSmoothDampHandle {
    fn new(
        current: UseStateHandle<f64>,
        target: UseStateHandle<f64>,
        current_velocity: Rc<RefCell<f64>>,
        last_timestamp: Rc<RefCell<f64>>,
        smooth_time: Rc<RefCell<f64>>,
    ) -> Self {
        Self {
            current,
            target,
            current_velocity,
            last_timestamp,
            smooth_time,
        }
    }

    pub fn set_target(&self, target: f64) {
        self.target.set(target);
    }

    pub fn target(&self) -> f64 {
        *self.target
    }

    pub fn set_smooth_time(&self, smooth_time: f64) {
        *self.smooth_time.borrow_mut() = smooth_time;
    }

    // Return is updated or not
    pub fn update(&self, timestamp: f64) {
        if *self.target == *self.current {
            return;
        }

        if (*self.target - *self.current).abs() > 0.01 {
            // max delta_time is 30fps
            let delta_time = ((timestamp - *self.last_timestamp.borrow()) / 1000.0).min(1.0 / 30.0);
            let (next_position, next_velocity) = smooth_damp(
                *self.current,
                *self.target,
                *self.smooth_time.borrow(),
                *self.current_velocity.borrow(),
                delta_time,
            );
            self.current.set(next_position);
            *self.current_velocity.borrow_mut() = next_velocity;
            *self.last_timestamp.borrow_mut() = timestamp;
        } else {
            self.current.set(*self.target);
        }
    }
}
impl Deref for UseSmoothDampHandle {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &*self.current
    }
}

pub fn use_smooth_damp() -> UseSmoothDampHandle {
    let current = use_state_eq(|| 0.0);
    let target = use_state_eq(|| 0.0);
    let current_velocity = use_mut_ref(|| 0.0);
    let last_timestamp = use_mut_ref(|| 0.0);
    let smooth_time = use_mut_ref(|| 0.15);

    UseSmoothDampHandle::new(
        current,
        target,
        current_velocity,
        last_timestamp,
        smooth_time,
    )
}
