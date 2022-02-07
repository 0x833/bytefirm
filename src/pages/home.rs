use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Welcome..." }</h1>
                        <h2 class="subtitle">{ "...to the best yew content" }</h2>
                    </div>
                </div>

                <div class="tile is-child">
                    <figure class="image is-3by1">
                        <img alt="software solutions image" src="public/landing-image.jpg" />
                    </figure>
                </div>

                <div class="tile is-parent container">
                    { self.view_info_tiles() }
                </div>
            </div>
        }
    }
}
impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <>
                <div class="tile is-parent">
                    <div class="tile is-parent">
                        <div class="tile is-child box">
                            <p class="title">{ "Who are we?" }</p>

                            <div class="content">
                                { "We're a small team working tirelessly to bring you the software solutions you need most." }
                                <br />
                                {r#"
                                    Please check out our blog posts for what we are up to!
                                "#}
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
