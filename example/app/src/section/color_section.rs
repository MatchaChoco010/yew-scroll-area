use yew::prelude::*;
use yew_scroll_area::*;
use yew_style_in_rs::*;

#[function_component(ColorSection)]
pub fn color_section() -> Html {
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

            & .test-area-color {
                max-width: 720px;
                width: 70%;
                height: 400px;
                display: flex;
                margin: auto;
                left: 0;
                right: 0;

                & section {
                    width: 100%;
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
        "#};
    }
    html! {
        <section class={css}>
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
    }
}
