use super::*;

pub mod heat;

pub fn render_signal() -> impl Signal<Item = Option<Dom>> {
    App::user_signal().map(|user| if user.is_none() { Some(render()) } else { None })
}

pub fn render() -> Dom {
    html!("div", {
        .class("public_content")
        .children([
            html!("header",{
                .child(h1())
                .child(h2())
            }),
            html!("div", {
                .class("background")
            }),
        ])
        .child_signal(map_ref!{
            let route = APP.data.route.signal_cloned(),
            let heatlar = APP.data.heatlar.signal_cloned()
        => {
            match route {
                Route::User(_) => None,
                Route::Guest(GuestRoute::Tablo) => Some(schedule::render(heatlar.clone())),
                Route::Guest(GuestRoute::Heat) => Some(heat::render(heatlar.clone())),
            }
        }})
    })
}

fn h1() -> Dom {
    html!("h1",{
        .children([
            html!("div", {
                .class("ear")
                .child(html!("div", { .child(html!("div", {})) }))
            }),
            html!("div", {
                .class("title")
                .child(html!("div", { .child(html!("div", {})) }))
            }),
            html!("div", {
                .class("ear")
                .child(html!("div", {
                    .child(html!("div", {
                        .child(html!("div", {
                            .class("clock")
                            .text_signal(APP.clock.signal().map(|clock| {
                                let (timestamp, fmt) = match clock {
                                    Clock::Odd(timestamp) => {
                                        (timestamp, "%H:%M")
                                    },
                                    Clock::Even(timestamp) => {
                                        (timestamp, "%H %M")
                                    },
                                };
                                use chrono::{Utc, DateTime, NaiveDateTime};
                                let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
                                let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                                use chrono_tz::Europe::Moscow;
                                let datetime_local = datetime.with_timezone(&Moscow);
                                datetime_local.time().format(fmt).to_string()
                            }))
                        }))
                    }))
                }))
            }),
        ])
    })
}

pub fn h2() -> Dom {
    html!("h2", {})
}
