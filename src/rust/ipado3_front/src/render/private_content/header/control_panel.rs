use super::*;

pub fn control_panel() -> Dom {
    html!("h1",{
        .children([
            html!("nav", {
                .children([
                    html!("div", {
                        .class("prev")
                        .with_node!(_element => {
                            .event(|_: events::Click| {
                                  let next = if let Some(cursor) = &(*APP.data.cursor.lock_ref()) {
                                        (*APP.data.расписание.lock_ref()).iter().filter_map(|TimeSlot { номер_пункта, ..}|

                                            (*номер_пункта as i16 == cursor.id - 1).then(|| *номер_пункта as i16)

                                        ).next()
                                  } else {
                                        (*APP.data.расписание.lock_ref()).iter().rev().next().map(|TimeSlot { номер_пункта, ..}|
                                            *номер_пункта as i16

                                        )
                                  };


                                  *APP.data.cursor.lock_mut() = next.map(|id| Cursor { id });
                                   let message =
                                        ClientMessage::MoveCursor(MoveCursor{
                                            турнир: (*APP.data.турнир.lock_ref()).clone().unwrap(),
                                            cursor: (*APP.data.cursor.lock_ref()).clone()
                                        });
                                  send_client_message(message);

                            })
                        })
                        .child_signal(APP.data.cursor.signal_cloned().map(|cursor| {
                              if let Some(cursor) = cursor {
                                  Some(html!("span", {.text(
                                        &(*APP.data.расписание.lock_ref()).iter().filter_map(|TimeSlot { номер_пункта, ..}|
                                            (*номер_пункта as i16 == cursor.id - 1).then(|| format!("#{номер_пункта}"))
                                        ).next().unwrap_or_else(|| "N".to_owned())
                                   )}))
                              } else {
                                    (*APP.data.расписание.lock_ref()).iter().rev().next().map(|TimeSlot { номер_пункта, ..}| format!("#{номер_пункта}")).map(|s|
                                        html!("span", {
                                            .text(&s)
                                        })
                                    )
                              }
                        }))
                    }),
                    html!("div", {
                        .class("current")
                        .with_node!(_element => {
                            .event(|_: events::Click| {
                                App::cursor_to_visible();
                            })
                        })
                        .child_signal(APP.data.cursor.signal_cloned().map(|cursor| {
                              if let Some(cursor) = cursor {
                                  Some(html!("span", {
                                      .text(
                                            &if let Some(s) = (*APP.data.расписание.lock_ref()).iter().filter_map(|TimeSlot { номер_пункта, ..}|
                                                (*номер_пункта as i16 == cursor.id).then(|| format!("#{номер_пункта}"))
                                                ).next() {
                                                s
                                            } else {
                                                "N".to_owned()
                                            }
                                       )
                                  }))
                              } else {
                                  Some(html!("span", {
                                      .text("N")
                                  }))
                              }
                        }))
                    }),
                    html!("div", {
                        .class("next")
                        .with_node!(_element => {
                            .event(|_: events::Click| {
                                  let next = if let Some(cursor) = &(*APP.data.cursor.lock_ref()) {
                                            (*APP.data.расписание.lock_ref()).iter().filter_map(|TimeSlot { номер_пункта, ..}|
                                                (*номер_пункта as i16 == cursor.id + 1).then(|| *номер_пункта as i16)
                                            ).next()
                                  } else {
                                        (*APP.data.расписание.lock_ref()).iter().next().map(|TimeSlot { номер_пункта, ..}|

                                            *номер_пункта as i16
                                        )
                                  };
                                  *APP.data.cursor.lock_mut() = next.map(|id| Cursor { id });
                                   let message =
                                        ClientMessage::MoveCursor(MoveCursor{
                                            турнир: (*APP.data.турнир.lock_ref()).clone().unwrap(),
                                            cursor: (*APP.data.cursor.lock_ref()).clone()
                                        });
                                  send_client_message(message);

                            })
                        })
                        .child_signal(APP.data.cursor.signal_cloned().map(|cursor| {
                              if let Some(cursor) = cursor {
                                  Some(html!("span", {.text(
                                        &(*APP.data.расписание.lock_ref()).iter().filter_map(|TimeSlot { номер_пункта, ..}|
                                            (*номер_пункта as i16 == cursor.id + 1).then(|| format!("#{номер_пункта}"))
                                        ).next().unwrap_or_else(|| "N".to_owned())
                                   )}))
                              } else {
                                    (*APP.data.расписание.lock_ref()).iter().next().map(|TimeSlot { номер_пункта, ..}| format!("#{номер_пункта}")).map(|s|
                                            html!("span", {
                                                .text(&s)
                                            })
                                    )
                              }
                        }))
                    }),
                ])
            }),
        ])
    })
}
