use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="logo-box">
                <img src="images/logo.svg" alt="ロゴ画像" />
                <p class="small-text">{ "Copyright © FaBomb All Rights Reserved." }</p>
            </div>
            <div class="about">
                <a href="https://twitter.com/FaBombLab" target="_blank">{"Twitter"}</a>
                <a href="https://www.instagram.com/fabomb_lab" target="_blank">{"Instagram"}</a>
                <a href="https://github.com/FaBomb" target="_blank">{"GitHub"}</a>
            </div>
        </footer>
    }
}
