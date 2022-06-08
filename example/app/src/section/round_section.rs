use yew::prelude::*;
use yew_scroll_area::*;
use yew_style_in_rs::*;

#[function_component(RoundSection)]
pub fn round_section() -> Html {
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

            & .test-area-round {
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

                & .test-area-enable-round{
                    background: #ddd;
                    color: black;
                }

                & .test-area-disable-round {
                    background: #ddd;
                    color: black;
                }
            }
        "#};
    }
    html! {
        <section class={css}>
            <h1>{"Corner round of scrollbar"}</h1>
            <p>
                {"Add "}
                <code>{"round=false"}</code>
                {" if you want to disable round of scrollbars."}
            </p>
            <pre>
                <code class="language-rust">
                    {
r#"#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true round=falses>
                { contents here ... }
            </ScrollArea>
        </div>
    }
}"#
                    }
                </code>
            </pre>
            <div class="test-area-round">
                <section class="test-area-enable-round">
                    <ScrollArea vertical=true round=true width={8.0} draggable_width={16.0}>
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
                <section class="test-area-disable-round">
                    <ScrollArea vertical=true round=false width={8.0} draggable_width={16.0}>
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
