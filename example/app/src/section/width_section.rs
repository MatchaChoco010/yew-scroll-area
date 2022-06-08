use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_scroll_area::*;
use yew_style_in_rs::*;

#[function_component(WidthSection)]
pub fn width_section() -> Html {
    let width = use_state(|| 16.0);
    let draggable_width = use_state(|| 24.0);

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

            & .test-area-width-inputs {
                max-width: 720px;
                width: 70%;
                margin: auto;
                left: 0;
                right: 0;
                display: flex;
                flex-direction: column;
            }

            & .test-area-width {
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
            <h1>{"Width of scrollbars"}</h1>
            <p>
                {"Add attributes "}
                <code>{"width"}</code>
                {" to set width of appearance, and "}
                <code>{"draggable_width"}</code>
                {" to set mouse-interactive width."}
            </p>
            <pre>
                <code class="language-rust">
                    {
r#"#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true width={4.0} draggable_width={16.0}>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}"#
                    }
                </code>
            </pre>
            <div class="test-area-width-inputs">
                <div>
                    <label for="width">{"width"}</label>
                    <input
                        id="width"
                        type="range" value={width.to_string()}
                        min="0.0" max="40.0" step="0.01"
                        oninput={
                            let width = width.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap();
                                let target: HtmlInputElement = target.dyn_into().unwrap();
                                let value = target.value().parse::<f64>().unwrap();
                                width.set(value);
                            })
                        }
                    />
                    {*width}
                </div>
                <div>
                    <label for="draggable_width">{"draggable_width"}</label>
                    <input
                        id="draggable_width"
                        type="range" value={draggable_width.to_string()}
                        min="0.0" max="40.0" step="0.01"
                        oninput={
                            let draggable_width = draggable_width.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap();
                                let target: HtmlInputElement = target.dyn_into().unwrap();
                                let value = target.value().parse::<f64>().unwrap();
                                draggable_width.set(value);
                            })
                        }
                    />
                    {*draggable_width}
                </div>
            </div>
            <section class="test-area-width">
                <ScrollArea
                    vertical=true
                    width={*width}
                    draggable_width={*draggable_width}
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
