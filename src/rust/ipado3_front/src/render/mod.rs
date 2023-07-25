use super::*;

pub mod private_content;
mod public_content;
pub mod schedule;

pub fn app() -> Dom {
    html!("div", {
        .future(routing::url()
            .signal_ref(|url| Route::from_url(url))
            .dedupe_cloned()
            .for_each(move |route| {
                APP.data.route.set_neq(route);
                async {}
            })
        )
        .class_signal("is_alive", App::is_alive_signal())
        .child_signal(private_content::render_signal())
        .child_signal(public_content::render_signal())
    })
}
