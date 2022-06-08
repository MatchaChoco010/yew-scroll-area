use yew::prelude::*;
use yew_style_in_rs::*;

use crate::Color;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub color: Color,
    #[prop_or(4.0)]
    pub width: f64,
    #[prop_or(true)]
    pub round: bool,
}

#[function_component(DefaultVerticalThumb)]
pub fn default_vertical_thumb(props: &Props) -> Html {
    let color = format!(
        "rgba({}, {}, {}, {})",
        props.color.r, props.color.g, props.color.b, props.color.a
    );
    let thumb_width = props.width;
    let thumb_round = if props.round { thumb_width / 2.0 } else { 0.0 };

    style! {
        let css = css! {r#"
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: row;
            justify-content: center;
        "#};
        let dyn_css = dyn css! {r#"
            & > div {
                width: ${thumb_width}px;
                height: 100%;
                background: ${color};
                border-radius: ${thumb_round}px;
            }
        "#};
    }
    html! {
        <div class={classes!(css, dyn_css)}>
            <div></div>
        </div>
    }
}

#[function_component(DefaultHorizontalThumb)]
pub fn default_horizontal_thumb(props: &Props) -> Html {
    let color = format!(
        "rgba({}, {}, {}, {})",
        props.color.r, props.color.g, props.color.b, props.color.a
    );
    let thumb_width = props.width;
    let thumb_round = if props.round { thumb_width / 2.0 } else { 0.0 };

    style! {
        let css = css! {r#"
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: column;
            justify-content: center;
        "#};
        let dyn_css = dyn css! {r#"
            & > div{
                width: 100%;
                height: ${thumb_width}px;
                background: ${color};
                border-radius: ${thumb_round}px;
            }
        "#};
    }
    html! {
        <div class={classes!(css, dyn_css)}>
            <div></div>
        </div>
    }
}
