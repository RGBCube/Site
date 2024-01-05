use actix_web::get;
use maud::{
    html,
    Markup,
};

use crate::page::{
    text,
    Page,
};

#[get("/about")]
pub async fn handler() -> actix_web::Result<Markup> {
    Ok(text::create(
        Some("About"),
        Page::About,
        html! {
            h1 { "Lorem Ipsum" }
            p {
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed eget justo nec libero finibus facilisis. Curabitur fermentum quam et neque faucibus, nec pharetra nunc hendrerit."
            }
            h2 { "Section 1" }
            p {
                "Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Donec auctor velit id lectus vehicula molestie."
            }
            h3 { "Subsection 1.1" }
            p {
                "Nullam mollis nunc non nisl fermentum, a varius eros viverra. Fusce condimentum felis vitae nibh vehicula, a tincidunt ipsum eleifend."
            }
            h2 { "Section 2" }
            p {
                "Phasellus euismod eros a elit volutpat, sed volutpat eros placerat. Sed dictum est et metus consectetur, quis fringilla nunc venenatis."
            }
            h3 { "Subsection 2.1" }
            p {
                "Integer ac libero id nisi posuere bibendum. Vivamus ut enim auctor, scelerisque quam a, fermentum ligula."
            }
            h3 { "Subsection 2.2" }
            p {
                "Morbi ut ex vel odio congue lobortis sit amet vel lacus. Duis rhoncus risus eget justo tincidunt vehicula."
            }
            h2 { "Section 3" }
            p {
                "Etiam quis sapien quis lacus malesuada vestibulum. Nam bibendum risus sed dui maximus, sed posuere lorem ultricies."
            }
            h3 { "Subsection 3.1" }
            p {
                "Cras interdum arcu at dolor dictum, a posuere urna aliquam. Vestibulum nec tortor nec nunc cursus lobortis sit amet a arcu."
            }
            h3 { "Subsection 3.2" }
            p {
                "Nunc auctor mauris quis lacus molestie lobortis. Vivamus eu sapien vel ligula congue convallis."
            }
            p {
                a href="#" { "Link 1" }
                " | "
                a href="#" { "Link 2" }
                " | "
                a href="#" { "Link 3" }
            }
        },
    ))
}
