use super::*;

pub fn render_signal() -> impl Signal<Item = Option<Dom>> {
    APP.data
        .heatlar
        .signal_cloned()
        .map(|heatlar| Some(render(heatlar)))
}

const RENDER_TIMEOUT_MILLIS: u32 = 250;
static mut RENDER_TIMEOUT: Option<Timeout> = None;

pub fn render(heatlar: Arc<Vec<Heat>>) -> Dom {
    unsafe {
        if let Some(timeout) = RENDER_TIMEOUT.take() {
            timeout.cancel();
        }
        if RENDER_TIMEOUT.is_none() {
            RENDER_TIMEOUT = Some(Timeout::new(RENDER_TIMEOUT_MILLIS, move || {
                RENDER_TIMEOUT = None;
                {
                    scroll_to_cursor();
                }
            }));
        }
    }
    html!("div", {
        .class("content")
        .child(
            html!("table", {
                .class("schedule")
                .children([
                    caption(heatlar.clone()),
                    html!("tbody", {
                        .children({
                            heatlar.iter().map(tr)
                        })
                    }),
                ])
            })
        )
        .child_signal(map_ref! {
            let heat_details = APP.data.heat_details.entries_cloned().to_signal_cloned(),
            let cursor = APP.data.cursor.signal()
        => move {
            let cursor_start_at = cursor.and_then(|cursor| heatlar.iter().find(|Heat { number, ..}| number == &cursor).map(|Heat { time: Time { start_at, .. }, .. }| *start_at));
            let mut heat_details = heat_details
                .iter()
                .filter_map(|(key, value)|
                    heatlar.iter().find(|Heat { number, time: Time { start_at, .. }, ..}| number == &key.heat && if let Some(cursor_start_at) = cursor_start_at { start_at >= &cursor_start_at } else { true } ).map(|Heat { time: Time { start_at, .. }, ..}| (key, value, *start_at))
                ).collect::<Vec<_>>();
            heat_details.sort_by_key(|(_, _, start_at)| *start_at);
            Some(html!("div", {
                .class("heats")
                .children(
                    heat_details.into_iter().map(|(HeatDetailsKey { heat, category}, value, start_at)|
                        html!("div", { .class("item")
                            .class_signal("selected", APP.data.cursor
                                .signal()
                                .map(clone!(heat => move |cursor|
                                     cursor.map(|cursor| cursor == heat).unwrap_or(false)
                                ))
                            )
                            .attr("data-num", &heat.to_string())
                            .child(html!("label", { .class("number")
                                .text(&{
                                    use std::fmt::Write;
                                    let mut ret = heat.to_string();
                                    let _ = write!(ret, ", {}", start_at.format("%H:%M"));
                                    ret
                                })
                            }))
                            .child(html!("label", { .class("tour_category")
                                .text(&{
                                    use std::fmt::Write;
                                    let mut ret = String::new();
                                    if let Some(tour) = heatlar.iter().find(|Heat { number, ..}| number == heat).and_then(|Heat { category_tourlar, ..}| category_tourlar.iter().find(|CategoryTour{category: tst, ..}| category.starts_with(tst )).and_then(|CategoryTour{ tour, .. }| tour.as_ref())) {
                                        let _ = write!(ret, "{tour}");
                                    }
                                    if !category.is_empty() {
                                        if !ret.is_empty() {
                                            let _ = write!(ret, ", ");
                                        }
                                        let _ = write!(ret, "{category}");
                                    }
                                    ret
                                })
                            }))
                            .child({
                                match &**value {
                                    HeatDetailsValue::Simple { couples } => {
                                        html!("table", { .class("simple")
                                            .child(html!("tbody", {
                                                .children(couples.iter().map(|couple|
                                                    html!("tr", {
                                                        .child(
                                                            html!("td", {
                                                                .text(&couple.to_string())
                                                            })
                                                        )
                                                    })
                                                ))
                                            }))
                                        })
                                    }
                                    HeatDetailsValue::NonMixed { ords, rows } => {
                                        html!("table", { .class("non-mixed")
                                            .child(html!("thead", {
                                                .child(
                                                    html!("tr", {
                                                        .children(ords.iter().map(|ord|
                                                            html!("th", {
                                                                .text(&ord.to_string())
                                                            })
                                                        ))
                                                    })
                                                )
                                            }))
                                            .child(html!("tbody", {
                                                .children(rows.iter().map(|row|
                                                    html!("tr", {
                                                        .children(ords.iter().map(|ord|
                                                            html!("td", {
                                                                .text(&
                                                                    row.get(ord).map(|couple| couple.to_string()).unwrap_or_default()
                                                                )
                                                            })
                                                        ))
                                                    })
                                                ))
                                            }))
                                        })
                                    }
                                    HeatDetailsValue::Mixed { dances, couples } => {
                                        html!("table", { .class("mixed")
                                            .child(html!("thead", {
                                                .child(
                                                    html!("tr", {
                                                        .child(html!("th", {
                                                            .text("№")
                                                        }))
                                                        .children(dances.iter().map(|dance|
                                                            html!("th", {
                                                                .text(dance)
                                                            })
                                                        ))
                                                    })
                                                )
                                            }))
                                            .child(html!("tbody", {
                                                .children(couples.iter().map(|(couple, ords)|
                                                    html!("tr", {
                                                        .child(
                                                            html!("td", {
                                                                .text(&couple.to_string())
                                                            })
                                                        )
                                                        .children(dances.iter().map(|dance|
                                                            html!("td", {
                                                                .text(&
                                                                    ords.get(dance.as_str()).map(|ord| ord.to_string()).unwrap_or_default()
                                                                )
                                                            })
                                                        ))
                                                    })
                                                ))
                                            }))
                                        })
                                    }
                                }
                            })
                        })
                    )
                )
            }))
        }})
    })
}

pub fn scroll_to_cursor() {
    if let Some(cursor) = *APP.data.cursor.lock_ref() {
        let document = window().unwrap_throw().document().unwrap_throw();
        let delta = {
            if cursor == 1 {
                Some(0)
            } else if let Some(tr_elem) = document
                .query_selector(&format!(r#".schedule tr[data-num="{}"]"#, cursor - 1))
                .unwrap_throw()
            {
                let tr_elem: &HtmlElement = tr_elem.dyn_ref::<HtmlElement>().unwrap();
                let first_tr_elem = document
                    .query_selector(r#".schedule tr"#)
                    .unwrap_throw()
                    .unwrap();
                let first_tr_elem: &HtmlElement = first_tr_elem.dyn_ref::<HtmlElement>().unwrap();
                Some(tr_elem.offset_top() - first_tr_elem.offset_top())
            } else {
                None
            }
        };

        if let Some(delta) = delta {
            let mut scroll_to_options = web_sys::ScrollToOptions::new();
            scroll_to_options.top(delta as f64);
            scroll_to_options.behavior(web_sys::ScrollBehavior::Instant);
            if document.body().unwrap_throw().client_width() > 667 {
                if let Some(container) = document
                    .query_selector(r#".schedule > tbody"#)
                    .unwrap_throw()
                {
                    let container = container.dyn_ref::<HtmlElement>().unwrap();
                    container.scroll_with_scroll_to_options(&scroll_to_options);
                }
            } else {
                document
                    .body()
                    .unwrap_throw()
                    .scroll_with_scroll_to_options(&scroll_to_options);
            };
        }
    }
}

fn caption(heatlar: Arc<Vec<Heat>>) -> Dom {
    html!("caption", {
        .child(
            html!("div", {
                .class("inner")
                .child_signal(
                    APP.data.app_mode
                        .signal_cloned()
                        .map(clone!(heatlar => move |app_mode|
                            matches!(app_mode, Some(AppMode::User{..}))
                                .then_some(nav_left(heatlar.clone()))
                        ))
                )
                .child_signal(
                    map_ref! {
                        let heat_details = APP.data.heat_details.entries_cloned().to_signal_cloned(),
                        let cursor = APP.data.cursor.signal()
                    => move {
                        if let Some(cursor) = cursor {
                            let heat_details = heat_details
                                .iter()
                                .filter_map(|(key, value)|
                                    (&key.heat == cursor).then_some(value)
                                )
                                .collect::<Vec<_>>();

                            if heat_details.len() == 1 {
                                if let HeatDetailsValue::Mixed { dances, couples: _ } = &**heat_details[0] {
                                    Some(html!("div", {
                                        .class("dances")
                                        .children(dances.iter().map(|dance|
                                            html!("div", { .class("dance")
                                                .text(dance)
                                                .class_signal("selected", APP.data.dance.signal_cloned().map(clone!(dance => move |dance_signaled|
                                                     if let Some(dance_signaled) = dance_signaled {
                                                        dance_signaled == dance
                                                     } else {
                                                        false
                                                     }
                                                )))
                                                .event(clone!(dance => move |_: events::Click| {
                                                    *APP.data.dance.lock_mut() = Some(dance.clone());
                                                    let message = ClientMessage::MoveCursor(MoveCursor{
                                                        cursor: *APP.data.cursor.lock_ref(),
                                                        dance: Some(dance.clone()),
                                                    });
                                                    send_client_message(message);
                                                }))
                                            })
                                        ))
                                    }))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }}
                )
                .child_signal(
                    APP.data.app_mode
                        .signal_cloned()
                        .map(clone!(heatlar => move |app_mode|
                            matches!(app_mode, Some(AppMode::User{..}))
                                .then_some(nav_right(heatlar.clone()))
                        ))
                )
            })
        )
    })
}

fn nav_right(heatlar: Arc<Vec<Heat>>) -> Dom {
    html!("nav", {
        .class("right")
        .text_signal(APP.data.cursor
            .signal()
            .map(clone!(heatlar => move |cursor| {
                next_cursor(cursor, heatlar.clone()).map(|cursor| cursor.to_string()).unwrap_or("N".to_owned())
            }))
        )
        .event(move |_: events::Click| {
            let cursor = next_cursor(*APP.data.cursor.lock_ref(), heatlar.clone());
            let dance = if let Some(cursor) = cursor {
                let heat_details = APP.data.heat_details.lock_ref()
                    .iter()
                    .filter(|(key, _value)|
                        key.heat == cursor
                    ).map(|i| (*i.1).clone()).collect::<Vec<_>>();

                    if heat_details.len() == 1 {
                        if let HeatDetailsValue::Mixed { dances, couples: _ } = &*heat_details[0] {
                            dances.first().cloned()
                        } else { None }
                    } else { None }
            } else { None };
            *APP.data.cursor.lock_mut() = cursor;
            *APP.data.dance.lock_mut() = dance.clone();
            let message = ClientMessage::MoveCursor(MoveCursor{
                cursor,
                dance,
            });
            send_client_message(message);
            scroll_to_cursor();
        })
    })
}
//
fn prev_cursor(cursor: Option<i16>, heatlar: Arc<Vec<Heat>>) -> Option<i16> {
    if let Some(cursor) = cursor {
        if let Some(position) = heatlar.iter().position(|i| i.number == cursor) {
            if position == 0 {
                None
            } else {
                heatlar.get(position - 1)
            }
        } else {
            heatlar.last()
        }
    } else {
        heatlar.last()
    }
    .map(|i| i.number)
}

fn next_cursor(cursor: Option<i16>, heatlar: Arc<Vec<Heat>>) -> Option<i16> {
    let len = heatlar.len();
    if let Some(cursor) = cursor {
        if let Some(position) = heatlar.iter().position(|i| i.number == cursor) {
            if position >= len - 1 {
                None
            } else {
                heatlar.get(position + 1)
            }
        } else {
            heatlar.first()
        }
    } else {
        heatlar.first()
    }
    .map(|i| i.number)
}
//
fn nav_left(heatlar: Arc<Vec<Heat>>) -> Dom {
    html!("nav", {
        .class("left")
        .text_signal(APP.data.cursor
            .signal()
            .map(clone!(heatlar => move |cursor| {
                prev_cursor(cursor, heatlar.clone()).map(|cursor| cursor.to_string()).unwrap_or("N".to_owned())
            }))
        )
        .event(move |_: events::Click| {
            let cursor = prev_cursor(*APP.data.cursor.lock_ref(), heatlar.clone());
            let dance = if let Some(cursor) = cursor {
                let heat_details = APP.data.heat_details.lock_ref()
                    .iter()
                    .filter(|(key, _value)|
                        key.heat == cursor
                    ).map(|i| (*i.1).clone()).collect::<Vec<_>>();

                    if heat_details.len() == 1 {
                        if let HeatDetailsValue::Mixed { dances, couples: _ } = &*heat_details[0] {
                            dances.last().cloned()
                        } else { None }
                    } else { None }
            } else { None };
            *APP.data.cursor.lock_mut() = cursor;
            *APP.data.dance.lock_mut() = dance.clone();
            let message = ClientMessage::MoveCursor(MoveCursor{
                // event_hall,
                cursor,
                dance,
            });
            send_client_message(message);
            scroll_to_cursor();
        })
    })
}
//
fn tr(item: &Heat) -> Dom {
    html!("tr", {
        .class_signal("selected", APP.data.cursor
            .signal()
            .map(clone!(item => move |cursor|
                 cursor.map(|cursor| cursor == item.number).unwrap_or(false)
            ))
        )
        .attr("data-num", &item.number.to_string())
        .children([
            html!("td", {
                .class("num")
                .text(&item.number.to_string())
            }),
            html!("td", { .class("time")
                .text(&item.time.to_string())
            }),
            if item.category_tourlar.len() == 1 {
                if let Some(i) = item.category_tourlar.first() {
                    html!("td", {
                        .text(&i.to_string())
                    })
                } else {
                    html!("td", {
                        .text("TODO: unreachable")
                    })
                }
            } else {
                html!("td", {
                    .child(
                        html!("table", {
                            .child(
                                html!("tbody", {
                                    .children(
                                        item.category_tourlar.iter().map(|i|
                                            html!("tr", {
                                                .child(
                                                    html!("td", { .text(&i.to_string()) })
                                                )
                                            })
                                        )
                                    )
                                })
                            )
                        })
                    )
                })
            }
        ])
    })
}
