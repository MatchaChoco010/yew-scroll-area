use gloo::events::EventListener;
use gloo::utils::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_scroll_area::*;
use yew_style_in_rs::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod highlight;

mod section;
use section::*;

#[function_component(App)]
fn app() -> Html {
    use_effect_with_deps(
        |_| {
            highlight::highlight();
            || ()
        },
        (),
    );

    // true vh for mobile with address bar area
    use_effect_with_deps(
        |_| {
            let callback = || {
                let vh = window().inner_height().unwrap().as_f64().unwrap() * 0.01;
                document()
                    .document_element()
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .unwrap()
                    .style()
                    .set_property("--vh", &format!("{vh}px"))
                    .unwrap();
            };
            callback();
            let listener = EventListener::new(&window(), "resize", move |_| callback());
            || drop(listener)
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
        "#};
    }
    html! {
        <div class={css}>
            <ScrollArea vertical=true
                width={10.0}
                draggable_width={40.0}
                scroll_option={ScrollOption {
                    touch_swipe_speed_scale: 2.8,
                    ..Default::default()
                }}>
                <TopSection />
                <VerticalScrollbarSection />
                <HorizontalScrollbarSection />
                <VerticalAndHorizontalSection />
                <ColorSection />
                <RoundSection />
                <WidthSection />
                <HideTimeSection />
                <CustomThumbSection />
                <SmoothOptionSection />
            </ScrollArea>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
