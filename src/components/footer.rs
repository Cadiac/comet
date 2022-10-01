use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="container disclaimer">
            <small>
                {"Made by "}
                <a href="https://github.com/Cadiac">{"Jaakko Husso"}</a>
                {". The source code of this tool is "}
                <a href="https://github.com/Cadiac/goldfisher/blob/master/goldfisher-web/LICENSE">{"MIT"}</a>
                {" licensed, and can be found from "}
                <a href="https://github.com/Cadiac/goldfisher">{"here"}</a>
                {"."}
            </small>
            <br/>
            <small>
                {"The literal and graphical information presented on this site about Magic: The Gathering, 
                including the card images, the mana symbols, and Oracle text, is copyright Wizards of the Coast, 
                LLC, a subsidiary of Hasbro, Inc. This service is not affiliated with Wizards of the Coast."}
            </small>
        </footer>
    }
}