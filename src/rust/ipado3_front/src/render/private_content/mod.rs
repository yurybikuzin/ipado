use super::*;

mod header;

pub fn render_signal() -> impl Signal<Item = Option<Dom>> {
    App::user_signal().map(|user| user.map(render))
}

pub fn render(user: Arc<User>) -> Dom {
    html!("div", {
        .class("private_content")
        .child(header::render(user))
        .child_signal(schedule::render_signal())
    })
}
