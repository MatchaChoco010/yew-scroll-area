use yew::prelude::*;
use yew_scroll_area::*;
use yew_style_in_rs::*;

#[function_component(HorizontalScrollbarSection)]
pub fn horizontal_scrollbar_section() -> Html {
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
        "#};
    }
    html! {
        <section class={css}>
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
    }
}
