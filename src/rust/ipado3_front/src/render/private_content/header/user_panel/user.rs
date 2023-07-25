use super::*;

pub fn render(user: Arc<User>) -> Dom {
    html!("div", {
        .class("user")
        .child(
            html!("div", {
                .class("picture")
                .style("background-image", &format!("url({:?})", user.auth.details.picture.as_deref().unwrap_or("")))
            })
        )
        .child(
            html!("div", {
                .class("email")
                .text(&user.auth.contact.to_string())
            })
        )
        .child(
            html!("div", {
                .class("logout")
                .event(move |_: events::Click| {
                    logout();
                })
            })
        )
    })
}
