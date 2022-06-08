use yew::prelude::*;
use yew_scroll_area::*;
use yew_style_in_rs::*;

#[function_component(FishThumb0)]
fn fish_thumb0() -> Html {
    style! {
        let css = css! {r#"
            width: 8px;
            height: 100%;
            margin: auto;
            left: 0;
            right: 0;
            background: rgba(128, 196, 255, 0.2);
            border: 2px solid rgba(196, 196, 196, 0.3);
            border-radius: 4px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            transition: border-color 0.5s;
            transition: background 0.5s;

            & img {
                max-width: unset;
                width: 50px;
                height: 80px;
            }
        "#};
    }
    html! {
        <div class={classes!(css)}>
            <img src="./fish-0.png" alt=""/>
        </div>
    }
}

#[function_component(FishThumb1)]
fn fish_thumb1() -> Html {
    style! {
        let css = css! {r#"
            width: 8px;
            height: 100%;
            margin: auto;
            left: 0;
            right: 0;
            background: rgba(255, 96, 128, 0.2);
            border: 2px solid rgba(196, 196, 196, 0.3);
            border-radius: 4px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            transition: border-color 0.5s;
            transition: background 0.5s;

            & img {
                max-width: unset;
                width: 44px;
                height: 100px;
            }
        "#};
    }
    html! {
        <div class={classes!(css)}>
            <img src="./fish-1.png" alt=""/>
        </div>
    }
}

#[function_component(FishThumb2)]
fn fish_thumb2() -> Html {
    style! {
        let css = css! {r#"
            width: 8px;
            height: 100%;
            margin: auto;
            left: 0;
            right: 0;
            background: rgba(196, 256, 128, 0.2);
            border: 2px solid rgba(196, 196, 196, 0.3);
            border-radius: 4px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;

            & img {
                max-width: unset;
                width: 56px;
                height: 100px;
            }
        "#};
    }
    html! {
        <div class={classes!(css)}>
            <img src="./fish-2.png" alt=""/>
        </div>
    }
}

#[function_component(CustomThumbSection)]
pub fn custom_thumb_section() -> Html {
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

            & .test-area-custom-thumb {
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

                & .test-area-custom-thumb-0 {
                    background: #123;
                    color: white;
                }

                & .test-area-custom-thumb-1 {
                    background: #ddd;
                    color: black;
                }

                & .test-area-custom-thumb-2 {
                    background: #011;
                    color: white;
                }
            }
        "#};
    }
    html! {
        <section class={css}>
            <h1>{"Customize humb"}</h1>
            <p>
                {"Add "}
                <code>{"custom_horizontal_thumb={html!(<YourComponent />)}"}</code>
                {" or "}
                <code>{"custom_vertical_thumb={html!(<YourComponent />)}"}</code>
                {" to customize scrollbars elements."}
            </p>
            <p>
                {"If you add "}
                <code>{"custom_vertical_thumb"}</code>
                {" or "}
                <code>{"custom_horizontal_thumb"}</code>
                {", then "}
                <code>{"round"}</code>
                {" nor "}
                <code>{"width"}</code>
                {" are ignored."}
            </p>
            <pre>
                <code class="language-rust">
                    {
r#"#[function_component(App)]
fn app() -> Html {
    html!{
        <div style="width: 100%; height: 100%;">
            <ScrollArea vertical=true
                hide_time={None}
                draggable_width={16.0}
                custom_vertical_thumb={html!(<CustomVerticalThumb />)}
                custom_horizontal_thumb={html!(<CustomHorizontalThumb />)}
            >
                { contents here ... }
            </ScrollArea>
        </div>
    }
}"#
                    }
                </code>
            </pre>
            <div class="test-area-custom-thumb">
                <section class="test-area-custom-thumb-0">
                    <ScrollArea vertical=true
                        draggable_width={16.0}
                        hide_time={None}
                        custom_vertical_thumb={html!(<FishThumb0 />)}
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
                    </ScrollArea>
                </section>
                <section class="test-area-custom-thumb-1">
                    <ScrollArea vertical=true
                        draggable_width={16.0}
                        hide_time={None}
                        custom_vertical_thumb={html!(<FishThumb1 />)}
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
                    </ScrollArea>
                </section>
                <section class="test-area-custom-thumb-2">
                    <ScrollArea vertical=true
                        draggable_width={16.0}
                        hide_time={None}
                        custom_vertical_thumb={html!(<FishThumb2 />)}
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
                    </ScrollArea>
                </section>
            </div>
        </section>
    }
}
