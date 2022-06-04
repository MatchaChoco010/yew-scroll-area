use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_scroll_area::*;
use yew_style_in_rs::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod highlight;

#[function_component(App)]
fn app() -> Html {
    use_effect_with_deps(
        |_| {
            highlight::highlight();
            || ()
        },
        (),
    );

    let width = use_state(|| 16.0);
    let round = use_state(|| false);
    let hide_time = use_state(|| 1.5);
    let smooth_time = use_state(|| 0.15);

    style! {
        let css = css! {r#"
            max-width: 960px;
            width: 100%;
            height: 100%;
            position: absolute;
            left: 0;
            right: 0;
            margin: auto;

            & section {
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

                & .test-area-vertical {
                    max-width: 600px;
                    width: 70%;
                    height: 400px;
                    margin: auto;
                    left: 0;
                    right: 0;
                    background: white;
                    border: 1px solid #123;
                    border-radius: 4px;
                }

                & .test-area-horizontal {
                    max-width: 600px;
                    width: 70%;
                    height: 200px;
                    margin: auto;
                    left: 0;
                    right: 0;
                    background: white;
                    border: 1px solid #123;
                    border-radius: 4px;

                    & .test-area-horizontal-inner {
                        display: flex;
                        flex-wrap: nowrap;

                        & .test-area-horizontal-item {
                            width: 220px;
                            background: rgba(255, 196, 0, 0.1);
                            margin: 1rem;
                        }
                    }
                }

                & .test-area-vertical-and-horizontal {
                    max-width: 600px;
                    width: 70%;
                    height: 400px;
                    margin: auto;
                    left: 0;
                    right: 0;
                    background: white;
                    border: 1px solid #123;
                    border-radius: 4px;

                    & .test-area-vertical-and-horizontal-inner {
                        display: flex;
                        flex-direction: column;
                        flex-wrap: nowrap;
                    }

                    & .test-area-vertical-and-horizontal-row {
                        width: 100%;
                        display: flex;
                        flex-wrap: nowrap;
                    }

                    & .test-area-vertical-and-horizontal-item {
                        width: 220px;
                        background: rgba(255, 196, 0, 0.1);
                        margin: 1rem;
                    }
                }

                & .test-area-color {
                    max-width: 720px;
                    width: 70%;
                    height: 400px;
                    display: flex;
                    margin: auto;
                    left: 0;
                    right: 0;

                    & section {
                        margin: 20px;
                    }

                    & .test-area-color-0 {
                        background: #123;
                        color: white;
                    }

                    & .test-area-color-1 {
                        background: #ddd;
                        color: black;
                    }

                    & .test-area-color-2 {
                        background: #011;
                        color: white;
                    }
                }

                & .test-area-style-inputs {
                    max-width: 720px;
                    width: 70%;
                    margin: auto;
                    left: 0;
                    right: 0;
                    display: flex;
                    flex-direction: column;
                }

                & .test-area-style {
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
            }
        "#};
    }
    html! {
            <div class={css}>
                <ScrollArea vertical=true width={10.0}>
                    <section>
                        <h1>{"yew-scroll-area"}</h1>
                        <p>{"A scroll area component for yew."}</p>
                        <h2>{"Usage"}</h2>
                        <p>
                            {"If you want scrollbars, simply surround the item you want to scroll with a "}
                            <code>{"<ScrollArea></ScrollArea>"}</code>
                            {" component."}
                        </p>
                        <pre>
                            <code class="language-rust">
                                {
r#"#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}"#
                                }
                            </code>
                        </pre>
                        <p>
                            {"The "}
                            <code>{"<ScrollArea></ScrollArea>"}</code>
                            {" component will be 100% of the width and height of the parent element and will allow scrolling for any child elements that extend beyond it."}
                        </p>
                        <p>
                            {"The "}
                            <code>{"<ScrollArea></ScrollArea>"}</code>
                            {" component is styled using yew-style-in-rs."}
                            {"In other words, you need to copy and place style.css from the build artifact."}
                        </p>
                    </section>
                    <section>
                        <h1>{"Display of vertical scrollbars"}</h1>
                        <p>
                            {"Add "}
                            <code>{"vertical=true"}</code>
                            {" if you want to use vertical scrollbars."}
                        </p>
                        <pre>
                            <code class="language-rust">
                                {
r#"#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}"#
                                }
                            </code>
                        </pre>
                        <section class="test-area-vertical">
                            <ScrollArea vertical=true>
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
                    <section>
                        <h1>{"Display of horizontal scrollbars"}</h1>
                        <p>
                            {"Add "}
                            <code>{"horizontal=true"}</code>
                            {" if you want to use horizontal scrollbars."}
                        </p>
                        <pre>
                            <code class="language-rust">
                                {
    r#"#[function_component(App)]
    fn app() -> Html {
        html!{
            <div style="width: 100%; height: 100%;">
                <ScrollArea horizontal=true>
                    { contents here ... }
                </ScrollArea>
            </div>
        }
    }"#
                                }
                            </code>
                        </pre>
                        <section class="test-area-horizontal">
                            <ScrollArea horizontal=true>
                                <div class="test-area-horizontal-inner">
                                    <div class="test-area-horizontal-item">
                                        <h1>{"Test Area"}</h1>
                                        <p>{"This is a scrollable area."}</p>
                                    </div>
                                    <div class="test-area-horizontal-item">
                                        <h1>{"Test Area"}</h1>
                                        <p>{"This is a scrollable area."}</p>
                                    </div>
                                    <div class="test-area-horizontal-item">
                                        <h1>{"Test Area"}</h1>
                                        <p>{"This is a scrollable area."}</p>
                                    </div>
                                    <div class="test-area-horizontal-item">
                                        <h1>{"Test Area"}</h1>
                                        <p>{"This is a scrollable area."}</p>
                                    </div>
                                </div>
                            </ScrollArea>
                        </section>
                    </section>
                    <section>
                        <h1>{"Display of scrollbars in both vertical and horizontal directions"}</h1>
                        <p>
                            {"Add "}
                            <code>{"vertical=true"}</code>
                            {" and "}
                            <code>{"horizontal=true"}</code>
                            {" if you want to use vertical and horizontal scrollbars."}
                        </p>
                        <pre>
                            <code class="language-rust">
                                {
    r#"#[function_component(App)]
    fn app() -> Html {
        html!{
            <div style="width: 100%; height: 100%;">
                <ScrollArea vertical=true horizontal=true>
                    { contents here ... }
                </ScrollArea>
            </div>
        }
    }"#
                                }
                            </code>
                        </pre>
                        <section class="test-area-vertical-and-horizontal">
                            <ScrollArea vertical=true horizontal=true>
                                <div class="test-area-vertical-and-horizontal-inner">
                                    <div class="test-area-vertical-and-horizontal-row">
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                    </div>
                                    <div class="test-area-vertical-and-horizontal-row">
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                    </div>
                                    <div class="test-area-vertical-and-horizontal-row">
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                    </div>
                                    <div class="test-area-vertical-and-horizontal-row">
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                        <div class="test-area-vertical-and-horizontal-item">
                                            <h1>{"Test Area"}</h1>
                                            <p>{"This is a scrollable area."}</p>
                                        </div>
                                    </div>
                                </div>
                            </ScrollArea>
                        </section>
                    </section>
                    <section>
                        <h1>{"Colorize of scrollbar"}</h1>
                        <p>
                            {"Add "}
                            <code>{"color={Color::rgba(128, 255, 0, 0.8)}"}</code>
                            {" if you want to colorize scrollbars."}
                        </p>
                        <pre>
                            <code class="language-rust">
                                {
r#"#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true color={Color::rgba(128, 255, 0, 0.8)}>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}"#
                                }
                            </code>
                        </pre>
                        <div class="test-area-color">
                            <section class="test-area-color-0">
                                <ScrollArea vertical=true  color={Color::rgba(128, 255, 0, 0.8)}>
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
                            <section class="test-area-color-1">
                                <ScrollArea vertical=true color={Color::black()}>
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
                            <section class="test-area-color-2">
                                <ScrollArea vertical=true color={Color::white()}>
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
                        </div>
                    </section>
                    <section>
                        <h1>{"Other property of scrollbars"}</h1>
                        <p>
                            {"Add attributes "}
                            <code>{"width"}</code>
                            {", "}
                            <code>{"round"}</code>
                            {", "}
                            <code>{"hide_time"}</code>
                            {" and "}
                            <code>{"smooth_time"}</code>
                            {" for styling scrollbars."}
                        </p>
                        <pre>
                            <code class="language-rust">
                                {
r#"#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true width={4.0} round=false hide_time={0.5} smooth_time={0.3}>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}"#
                                }
                            </code>
                        </pre>
                        <div class="test-area-style-inputs">
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
                                <label for="round">{"round"}</label>
                                <input
                                    id="round"
                                    type="checkbox"
                                    checked={*round}
                                    oninput={
                                        let round = round.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let target = e.target().unwrap();
                                            let target: HtmlInputElement = target.dyn_into().unwrap();
                                            let value = target.checked();
                                            round.set(value);
                                        })
                                    }
                                />
                            </div>
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
                            <div>
                                <label for="smooth_time">{"smooth_time"}</label>
                                <input
                                    id="smooth_time"
                                    type="range" value={smooth_time.to_string()}
                                    min="0.0" max="1.0" step="0.01"
                                    oninput={
                                        let smooth_time = smooth_time.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let target = e.target().unwrap();
                                            let target: HtmlInputElement = target.dyn_into().unwrap();
                                            let value = target.value().parse::<f64>().unwrap();
                                            smooth_time.set(value);
                                        })
                                    }
                                />
                                {*smooth_time}
                            </div>
                        </div>
                        <section class="test-area-style">
                            <ScrollArea
                                vertical=true
                                width={*width}
                                round={*round}
                                hide_time={*hide_time}
                                smooth_time={*smooth_time}
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
                </ScrollArea>
            </div>
        }
}

fn main() {
    yew::start_app::<App>();
}
