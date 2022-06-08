use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_scroll_area::*;
use yew_style_in_rs::*;

#[function_component(SmoothOptionSection)]
pub fn smooth_option_section() -> Html {
    let mouse_wheel_smooth_time = use_state(|| 0.15);
    let mouse_drag_thumb_smooth_time = use_state(|| 0.05);
    let mouse_wheel_speed_scale = use_state(|| 1.0);
    let touch_swipe_smooth_time = use_state(|| 0.15);
    let touch_drag_thumb_smooth_time = use_state(|| 0.05);
    let touch_swipe_speed_scale = use_state(|| 1.0);

    style! {
        let css = css! {r#"
            width: 100%;
            margin-top: 64px;

            & h1 {
                font-size: 2rem;
                margin: 1.2rem 1rem;
            }

            & h2 {
                font-size: 1.4rem;
                margin: 1.1rem 1rem;
            }

            & p {
                font-size: 1rem;
                margin: 0.8rem 1.2rem;

                & code {
                    font-family: Consolas,Monaco,monospace;
                    font-weight: bold;
                }
            }

            & pre {
                margin: 1rem 2rem;
            }

            & code {
                border-radius: 4px;
            }

            & .test-area-smooth-time-inputs {
                max-width: 720px;
                width: 70%;
                margin: auto;
                left: 0;
                right: 0;
                display: flex;
                flex-direction: column;
            }

            & .test-area-smooth-time {
                max-width: 600px;
                width: 70%;
                height: 400px;
                margin: auto;
                margin-bottom: 64px;
                left: 0;
                right: 0;
                background: white;
                border: 1px solid #123;
                border-radius: 4px;
            }
        "#};
    }
    html! {
        <section class={css}>
            <h1>{"Smooth damping of scrollbars"}</h1>
            <p>
                {"Add attributes "}
                <code>{"smooth_option"}</code>
                {" to change smooth damping of scrollbars."}
            </p>
            <p>
                {"The smooth time can be specified for mouse wheel operation, dragging a knob with the mouse, swiping with a touch device, and dragging a knob with a touch device, respectively."}
            </p>
            <pre>
                <code class="language-rust">
                    {
r#"#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea
                vertical=true
                smooth_option={SmoothOption {
                    mouse_wheel_smooth_time: *mouse_wheel_smooth_time,
                    mouse_drag_thumb_smooth_time: *mouse_drag_thumb_smooth_time,
                    touch_swipe_smooth_time: *touch_swipe_smooth_time,
                    touch_drag_thumb_smooth_time: *touch_drag_thumb_smooth_time,
                }}
            >
                { contents here ... }
            </ScrollArea>
        </div>
    }
}"#
                    }
                </code>
            </pre>
            <div class="test-area-smooth-time-inputs">
                <div>
                    <label for="mouse_wheel_smooth_time">{"mouse_wheel_smooth_time"}</label>
                    <input
                        id="mouse_wheel_smooth_time"
                        type="range" value={mouse_wheel_smooth_time.to_string()}
                        min="0.0" max="4.0" step="0.001"
                        oninput={
                            let mouse_wheel_smooth_time = mouse_wheel_smooth_time.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap();
                                let target: HtmlInputElement = target.dyn_into().unwrap();
                                let value = target.value().parse::<f64>().unwrap();
                                mouse_wheel_smooth_time.set(value);
                            })
                        }
                    />
                    {*mouse_wheel_smooth_time}
                </div>
                <div>
                    <label for="mouse_drag_thumb_smooth_time">{"mouse_drag_thumb_smooth_time"}</label>
                    <input
                        id="mouse_drag_thumb_smooth_time"
                        type="range" value={mouse_drag_thumb_smooth_time.to_string()}
                        min="0.0" max="4.0" step="0.001"
                        oninput={
                            let mouse_drag_thumb_smooth_time = mouse_drag_thumb_smooth_time.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap();
                                let target: HtmlInputElement = target.dyn_into().unwrap();
                                let value = target.value().parse::<f64>().unwrap();
                                mouse_drag_thumb_smooth_time.set(value);
                            })
                        }
                    />
                    {*mouse_drag_thumb_smooth_time}
                </div>
                <div>
                    <label for="mouse_wheel_speed_scale">{"mouse_wheel_speed_scale"}</label>
                    <input
                        id="mouse_wheel_speed_scale"
                        type="range" value={mouse_wheel_speed_scale.to_string()}
                        min="0.0" max="4.0" step="0.001"
                        oninput={
                            let mouse_wheel_speed_scale = mouse_wheel_speed_scale.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap();
                                let target: HtmlInputElement = target.dyn_into().unwrap();
                                let value = target.value().parse::<f64>().unwrap();
                                mouse_wheel_speed_scale.set(value);
                            })
                        }
                    />
                    {*mouse_wheel_speed_scale}
                </div>
                <div>
                    <label for="touch_swipe_smooth_time">{"touch_swipe_smooth_time"}</label>
                    <input
                        id="touch_swipe_smooth_time"
                        type="range" value={touch_swipe_smooth_time.to_string()}
                        min="0.0" max="4.0" step="0.001"
                        oninput={
                            let touch_swipe_smooth_time = touch_swipe_smooth_time.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap();
                                let target: HtmlInputElement = target.dyn_into().unwrap();
                                let value = target.value().parse::<f64>().unwrap();
                                touch_swipe_smooth_time.set(value);
                            })
                        }
                    />
                    {*touch_swipe_smooth_time}
                </div>
                <div>
                    <label for="touch_drag_thumb_smooth_time">{"touch_drag_thumb_smooth_time"}</label>
                    <input
                        id="touch_drag_thumb_smooth_time"
                        type="range" value={touch_drag_thumb_smooth_time.to_string()}
                        min="0.0" max="4.0" step="0.001"
                        oninput={
                            let touch_drag_thumb_smooth_time = touch_drag_thumb_smooth_time.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap();
                                let target: HtmlInputElement = target.dyn_into().unwrap();
                                let value = target.value().parse::<f64>().unwrap();
                                touch_drag_thumb_smooth_time.set(value);
                            })
                        }
                    />
                    {*touch_drag_thumb_smooth_time}
                </div>
                <div>
                    <label for="touch_swipe_speed_scale">{"touch_swipe_speed_scale"}</label>
                    <input
                        id="touch_swipe_speed_scale"
                        type="range" value={touch_swipe_speed_scale.to_string()}
                        min="0.0" max="4.0" step="0.001"
                        oninput={
                            let touch_swipe_speed_scale = touch_swipe_speed_scale.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap();
                                let target: HtmlInputElement = target.dyn_into().unwrap();
                                let value = target.value().parse::<f64>().unwrap();
                                touch_swipe_speed_scale.set(value);
                            })
                        }
                    />
                    {*touch_swipe_speed_scale}
                </div>
            </div>
            <section class="test-area-smooth-time">
                <ScrollArea
                    vertical=true
                    scroll_option={ScrollOption {
                        mouse_wheel_smooth_time: *mouse_wheel_smooth_time,
                        mouse_drag_thumb_smooth_time: *mouse_drag_thumb_smooth_time,
                        mouse_wheel_speed_scale: *mouse_wheel_speed_scale,
                        touch_swipe_smooth_time: *touch_swipe_smooth_time,
                        touch_drag_thumb_smooth_time: *touch_drag_thumb_smooth_time,
                        touch_swipe_speed_scale: *touch_swipe_speed_scale,
                    }}
                >
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                    <br />
                    <br />
                    <br />
                    <p>{"This is a scrollable area."}</p>
                    <br />
                    <br />
                    <br />
                    <h1>{"Test Area"}</h1>
                </ScrollArea>
            </section>
        </section>
    }
}
