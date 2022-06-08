use yew::prelude::*;
use yew_scroll_area::*;
use yew_style_in_rs::*;

#[function_component(VerticalScrollbarSection)]
pub fn vertical_scrollbar_section() -> Html {
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
        "#};
    }
    html! {
        <section class={css}>
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
    }
}
