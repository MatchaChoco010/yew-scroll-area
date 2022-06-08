use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_scroll_area::*;
use yew_style_in_rs::*;

#[function_component(HideTimeSection)]
pub fn hide_time_section() -> Html {
    let hide_time = use_state(|| 1.5);

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

            & .test-area-hide-time-inputs {
                max-width: 720px;
                width: 70%;
                margin: auto;
                left: 0;
                right: 0;
                display: flex;
                flex-direction: column;
            }

            & .test-area-hide-time {
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
            <h1>{"Time for hiding of scrollbars"}</h1>
            <p>
                {"Add attributes "}
                <code>{"hide"}</code>
                {" to change time to hide scrollbars."}
            <p>
            </p>
                {"If "}
                <code>{"hide_time"}</code>
                {" is set to "}
                <code>{"None"}</code>
                {", then scrollbars will be always visible."}
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
            <div class="test-area-hide-time-inputs">
                <div>
                    <label for="hide_time">{"hide_time"}</label>
                    <input
                        id="hide_time"
                        type="range" value={hide_time.to_string()}
                        min="0.0" max="10.0" step="0.01"
                        oninput={
                            let hide_time = hide_time.clone();
                            Callback::from(move |e: InputEvent| {
                                let target = e.target().unwrap();
                                let target: HtmlInputElement = target.dyn_into().unwrap();
                                let value = target.value().parse::<f64>().unwrap();
                                hide_time.set(value);
                            })
                        }
                    />
                    {*hide_time}
                </div>
            </div>
            <section class="test-area-hide-time">
                <ScrollArea
                    vertical=true
                    hide_time={*hide_time}
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
