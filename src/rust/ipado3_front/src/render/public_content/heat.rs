use super::*;

pub fn render(heatlar: Arc<Vec<Heat>>) -> Dom {
    html!("div", { .class("wrapper")
        .child(html!("div", { .class("heat")
            .text_signal(APP.data.cursor.signal().map(move |cursor| {
                cursor.and_then(clone!(heatlar => move |cursor| heatlar.iter().find_map(|Heat { number, ..}| (number == &cursor).then_some(number)).map(|number| format!("Heat {number}")))).unwrap_or_default()
            }))
        }))
        .child_signal(map_ref! {
            let heat_details = APP.data.heat_details.entries_cloned().to_signal_cloned(),
            let cursor = APP.data.cursor.signal(),
            let dance = APP.data.dance.signal_cloned()
        => move {
            if let Some(cursor) = cursor {
                let filtered = heat_details.iter().filter(|i| i.0.heat == *cursor).collect::<Vec<_>>();
                if filtered.is_empty() {
                    None
                } else {
                    let mut ret = vec![];
                    for (HeatDetailsKey { heat: _, category: _}, value) in filtered {
                        match &**value {
                            HeatDetailsValue::Simple { couples } => {
                                ret.push(html!("table", { .class("simple")
                                    .child(html!("tbody", {
                                        .child(
                                            html!("tr", {
                                                .children(couples.iter().map(|couple|
                                                    html!("td", {
                                                        .text(&couple.to_string())
                                                    })
                                                ))
                                            })
                                        )
                                    }))
                                }));
                            }
                            HeatDetailsValue::NonMixed { ords, rows } => {
                                ret.push(non_mixed(ords, rows));
                            }
                            HeatDetailsValue::Mixed { dances: _, couples } => {
                                if let Some(dance) = dance {
                                    let mut ords = std::collections::HashMap::<u8, Vec<u16>>::new();
                                    for (couple_number, couple_details) in couples.iter() {
                                        if let Some(ord) = couple_details.get(dance) {

                                            common_macros2::entry!(ords, *ord
                                            =>
                                                and_modify |e| { e.push(*couple_number) }
                                                or_insert vec![*couple_number]
                                            );
                                        }
                                    }
                                    let ords = ords.into_iter().collect::<std::collections::BTreeMap<_, _>>();

                                    ret.push(html!("div", { .class("dance") .text(
                                        DANCES.get(dance.as_str()).copied().unwrap_or(dance.as_str())
                                    ) }));
                                    ret.push(html!("table", { .class("non-mixed")
                                        .child(html!("tbody", {
                                            .children(ords.iter().map(|(ord, couple_numbers)|
                                                html!("tr", {
                                                    .child(
                                                        html!("th", {
                                                            .text(&ord.to_string())
                                                        })
                                                    )
                                                    .children(couple_numbers.iter().map(|couple_number|
                                                        html!("td", {
                                                            .text(&couple_number.to_string())
                                                        })
                                                    ))
                                                })
                                            ))
                                        }))
                                    }));
                                }
                            }
                        }
                    }
                    if ret.is_empty() {
                        None
                    } else {
                        Some(html!("div", { .class("details")
                            .children(ret)
                        }))
                    }
                }
            } else {
                None
            }
        }})
    })
}

fn non_mixed(ords: &BTreeSet<u8>, rows: &[HashMap<u8, u16>]) -> Dom {
    html!("table", { .class("non-mixed")
        .child(html!("tbody", {
            .children(ords.iter().map(|ord|
                html!("tr", {
                    .child(
                        html!("th", {
                            .text(&ord.to_string())
                        })
                    )
                    .children(rows.iter().map(|row|
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

pub static DANCES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("C", "Cha-Cha"),
        ("S", "Samba"),
        ("R", "Rumba"),
        ("P", "Paso"),
        ("J", "Jive"),
    ])
});
