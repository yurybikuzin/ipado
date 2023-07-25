use super::*;

pub fn render() -> Dom {
    html!("div", {
        .class("menu_panel")
        .child(html!("select", {
            .attr("id", "select")
            .children(UserRoute::iter().map(|option|
                html!("option", {
                    .attr("value", &(option as u8).to_string())
                    .attr_signal("selected",
                        APP.data.route.signal_cloned().map(clone!(option => move |route|
                            if let Route::User(option_tst) = route {
                                (option_tst == option).then_some("")
                            } else {
                                None
                            }
                        ))
                    )
                    .text(&option.to_string())
                })
            ))
            .with_node!(element => {
                .event(move |_event: events::Change| {
                    if let Some(element) = element.dyn_ref::<web_sys::HtmlSelectElement>() {
                        let value = element.value().parse::<u8>().unwrap();
                        if let Some(user_route) = UserRoute::from_repr(value) {
                            let url = Route::User(user_route).to_url();
                            debug!("value: {value}, url: {url}");
                            // cancel_delayed_go_to_url();
                            go_to_url(&url);
                        }
                    }
                })
            })
        }))
    })
}
