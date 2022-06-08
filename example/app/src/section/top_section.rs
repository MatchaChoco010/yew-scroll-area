use yew::prelude::*;
use yew_style_in_rs::*;

#[function_component(TopSection)]
pub fn top_section() -> Html {
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
        "#};
    }
    html! {
        <section class={css}>
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
    }
}
