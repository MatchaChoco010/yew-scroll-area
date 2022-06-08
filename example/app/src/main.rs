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
            <ScrollArea vertical=true width={10.0} draggable_width={20.0}>
                <TopSection />
                <VerticalScrollbarSection />
                <HorizontalScrollbarSection />
                <VerticalAndHorizontalSection />
                <ColorSection />
                <RoundSection />
                <WidthSection />
                <HideTimeSection />
                <CustomThumbSection />
                <SmoothTimeSection />
            </ScrollArea>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
