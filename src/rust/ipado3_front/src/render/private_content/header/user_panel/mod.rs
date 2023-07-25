use super::*;

mod user;

pub fn render(user: Arc<User>) -> Dom {
    html!("div", {
        .class("user_panel")
        .child(
            html!("div", {
                .class("refresh")
                .attr_signal("disabled", APP.data.is_refreshing.signal().map(|flag| if flag { "disabled" } else { "" } ))
                .event(move |_: events::Click| {
                    if !*APP.data.is_refreshing.lock_ref() && window().unwrap_throw().confirm_with_message("Do you really want to refresh?").unwrap_throw() {
                        let message = ClientMessage::NeedInitData{ key: InitDataKey {
                        }, refresh: true};
                        send_client_message(message);
                        *APP.data.is_refreshing.lock_mut() = true;
                    }

                })
            })
        )
        .child(user::render(user))
    })
}
