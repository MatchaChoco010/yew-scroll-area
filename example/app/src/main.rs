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
                        <h1>{"vertical scroll"}</h1>
                        <p>
                            {"Add"}
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
                        <h1>{"horizontal scroll"}</h1>
                        <p>
                            {"Add"}
                            <code>{"horizontal=true"}</code>
                            {" if you want to use vertical scrollbars."}
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
                        <h1>{"bidirectional scroll"}</h1>
                    </section>
                    <section>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                        <h1>{"colorize scroll thumb"}</h1>
                    </section>
                </ScrollArea>
            </div>
        }
}

fn main() {
    yew::start_app::<App>();
}
