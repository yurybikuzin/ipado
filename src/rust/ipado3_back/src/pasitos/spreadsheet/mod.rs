use super::*;

use spreadsheets::{
    from_range_data, get_rangelar_data, spreahsheet_number_value_to_naive_time,
    CompactExtendedValue,
};

// mod utils;
// use utils::*;

// #[derive(Serialize, Deserialize)]
// pub struct CoupleRow {
//     pub heat: i16,
//     pub couple_number: i16,
//     pub participant1: String,
//     pub participant2: Option<String>,
//     pub category: String,
//     pub tour: Option<TourEnum>,
//     pub hall: String,
//     pub start_at: NaiveTime,
// }
//
//
// #[derive(Debug)]
// pub struct ImportCoupleRow {
//     pub heat: i16,
//     pub couple_number: i16,
//     pub personlar: Vec<ImportPerson>,
//     // pub person2: Option<ImportPerson>,
//     pub category: String,
//     pub tour: Option<TourEnum>,
//     pub start_at: NaiveTime,
//     pub hall: String,
// }
//
// #[derive(Debug)]
// pub struct ImportScheduleRow {
//     pub heat: i16,
//     pub category: String,
//     pub tour: Option<TourEnum>,
//     pub start_at: NaiveTime,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleRow {
    pub heat: i16,
    pub category: String,
    pub tour: Option<String>,
    pub program: Option<String>,
    pub quantity: Option<i16>,
    pub dances: Option<String>,
    pub heats: Option<i16>,
    pub start_at: NaiveTime,
    pub duration: Duration,
    pub floor: Option<String>,
}

// =============================================================================

pub type GetInitDataResult = ImportResult;
pub async fn get_init_data(
    // use_halls: bool
    _key: InitDataKey,
) -> GetInitDataResult {
    import().await
}

pub fn get_init_data_sync(res: GetInitDataResult, tx: TxHandle) -> Result<()> {
    send_response_message(
        // ResponseMessage::InitData(res.map(|heatlar| InitData {
        //     heatlar,
        //     cursor: None, // TODO: read cursor from file
        //     heat_details: HashMap::new(),
        // })),
        ResponseMessage::InitData(res),
        tx,
    );
    Ok(())
}

// =============================================================================

use chrono::NaiveTime;
// use std::collections::{HashMap, HashSet};
use std::time::Duration;
pub type ImportResult = Result<Vec<Heat>>;
pub async fn import(// use_halls: bool
) -> ImportResult {
    // todo!();
    // #[derive(Serialize, Deserialize)]
    // struct Ret {
    //     schedule_rowlar: Vec<ScheduleRow>,
    //     // couple_rowlar: Vec<CoupleRow>,
    //     name: Option<String>,
    // }
    //
    // // let file_path = PathBuf::from("cache.json");
    // let cache_for_local_opt = if matches!(*OP_MODE.read().unwrap(), op_mode::OpMode::Local) {
    //     settings!(cache_for_local).clone()
    // } else {
    //     None
    // };
    // let ret_opt = if let Some(file_path) = &cache_for_local_opt {
    //     match std::fs::File::open(file_path) {
    //         Err(_err) => {
    //             // warn!("{}:{}: {err}", file!(), line!());
    //             None
    //         }
    //         Ok(mut file) => {
    //             let mut buf = vec![];
    //             use std::io::Read;
    //             if let Ok(_) = file.read_to_end(&mut buf) {
    //                 match std::str::from_utf8(&buf).map_err(|err| anyhow!("from_utf8: {err}")) {
    //                     Err(err) => {
    //                         warn!("{}:{}: {err}", file!(), line!());
    //                         None
    //                     }
    //                     Ok(s) => match serde_json::from_str(s)
    //                         .map_err(|err| anyhow!("from_str{file_path:?}: {err}"))
    //                     {
    //                         Err(err) => {
    //                             warn!("{}:{}: {err}", file!(), line!());
    //                             None
    //                         }
    //                         Ok(ret) => Some(ret),
    //                     },
    //                 }
    //             } else {
    //                 None
    //             }
    //         }
    //     }
    // } else {
    //     None
    // };
    // let Ret {
    //     schedule_rowlar,
    //     // couple_rowlar,
    //     name,
    // } = if let Some(ret) = ret_opt {
    //     ret
    // } else {
    let mut schedule_rowlar = vec![];
    // let mut couple_rowlar = vec![];
    // let mut name: Option<String> = None;

    let spreadsheet_id = settings!(spreadsheet.id).clone();
    let service_account_secret_file = settings!(spreadsheet.service_account_secret_file).clone();
    let key = yup_oauth2::read_service_account_key(&service_account_secret_file)
        .await
        .map_err(|err| anyhow!("read_service_account_key: {err}"))?;

    // let name_range = settings!(spreadsheet.name_range).clone();
    let schedule_range = settings!(spreadsheet.schedule_range).clone();
    // let couples_range = settings!(spreadsheet.couples_range).clone();

    // let name_sheet_name = (*name_range.split('!').next().unwrap()).to_owned();
    // let schedule_sheet_name = (*schedule_range.split('!').next().unwrap()).to_owned();
    // debug!("schedule_sheet_name: {schedule_sheet_name}");
    // let couples_sheet_name = (*couples_range.split('!').next().unwrap()).to_owned();

    let ret = will_did!(trace => "get_rangelar_data", get_rangelar_data(
            vec![
            // name_range.clone(), 
            schedule_range.clone()
            // , couples_range.clone()
            ],
            &spreadsheet_id,
            key.clone(),
        )
        .await)?;

    // debug!("spreadsheet_id: {spreadsheet_id}");

    for (_sheet_name, data_ranges) in ret {
        // debug!("sheet_name: {sheet_name}");
        for (_data_range, range_data) in data_ranges {
            // debug!("data_range: {data_range:?}");

            // if sheet_name == name_sheet_name
            //     && matches!((data_range.row_len, data_range.col_len), (1, Some(1)))
            // {
            //     struct NameRow {
            //         value: String,
            //     }
            //     let name_rowlar = spreadsheets::from_range_data!(range_data => NameRow { | cev |
            //         0 value: cev.as_string(),
            //     });
            //     name = name_rowlar.get(0).map(|i| i.value.clone());
            // } else if sheet_name == schedule_sheet_name {
            schedule_rowlar =
            //     if use_halls {
            //     spreadsheets::from_range_data!(range_data.clone() => ScheduleRow { | cev |
            //         0 heat: cev.as_int().map(|i| i as i16 ),
            //         0 category: cev.as_string(),
            //         0 tour Option: match TourEnum::try_from(cev.clone()) {
            //             Ok(tour) => Some(tour),
            //             Err(err) => panic!("{}: {err}", cev.as_string().unwrap()),
            //         },
            //         0 program Option: cev.as_string(),
            //         0 quantity: cev.as_int().map(|i| i as i16 ),
            //         0 dances Option: cev.as_string(),
            //         0 heats Option: cev.as_int().map(|i| i as i16),
            //         0 start_at: cev.as_f64().map(spreahsheet_number_value_to_naive_time),
            //         0 duration: cev.as_duration(),
            //         2 hall: cev.as_string(),
            //     })
            // } else {

                spreadsheets::from_range_data!(range_data.clone() => ScheduleRow { | cev |
                    0 heat: cev.as_int().map(|i| i as i16 ),
                    0 category: cev.as_string(),
                    // 0 tour Option: match TourEnum::try_from(cev.clone()) {
                    //     Ok(tour) => Some(tour),
                    //     Err(err) => panic!("{}: {err}", cev.as_string().unwrap()),
                    // },
                    0 tour Option: match cev {
                        spreadsheets::CompactExtendedValue::Int(n) => {
                            let naive_date = spreadsheets::spreahsheet_number_value_to_naive_date(n as f64);

                            use chrono::Datelike;
                            let ret = if naive_date.day() == 1 && naive_date.month() > 1 {
                                let log2 = fast_math::log2_raw(naive_date.month() as f32);
                                if log2.fract() == 0.0 {
                                    match log2 as u16 {
                                        1 => Some("1/2".to_owned()),
                                        2 =>Some("1/4".to_owned()),
                                        3 =>Some("1/8".to_owned()),
                                        _ => {
                                            error!( "{}:{}: {}/{}", file!(), line!(), naive_date.day(), naive_date.month());
                                            None
                                        },
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            if ret.is_none() {
                                error!("{}:{}: n: {n}", file!(), line!());
                            }
                            ret
                            // if let Some(ret) = ret {
                            //     Ok(ret)
                            // } else {
                            //     Err(anyhow!("n: {n}"))
                            // }
                        },
                        spreadsheets::CompactExtendedValue::String(s) => Some(s),
                        _ => {
                            error!("{}:{}: cev: {cev:?}", file!(), line!());
                            None
                        },
                    },
                    0 program Option: cev.as_string(),
                    0 quantity Option: cev.as_int().map(|i| i as i16 ),
                    0 dances Option: cev.as_string(),
                    0 heats Option: cev.as_int().map(|i| i as i16),
                    0 start_at: cev.as_f64().map(spreahsheet_number_value_to_naive_time),
                    0 duration: cev.as_duration(),
                    // 0 finish_at: cev.as_f64().map(spreahsheet_number_value_to_naive_time),
                    2 floor Option: cev.as_string(),
                    // 2 hall: Some({let _ = cev; "".to_owned()}),
                    // 2 hall: Some({let _ = cev; "".to_owned()}),
                })
            // }
        ;
            // } else if sheet_name == couples_sheet_name {
            //     couple_rowlar = if use_halls {
            //         spreadsheets::from_range_data!(range_data => CoupleRow { | cev |
            //             0 heat: cev.as_int().map(|i| i as i16),
            //             0 couple_number: cev.as_int().map(|i| i as i16),
            //             0 participant1: cev.as_string(),
            //             0 participant2 Option: cev.as_string(),
            //             0 category: cev.as_string(),
            //             // 0 tour Option: Some(TourEnum::from(cev)),
            //             // 0 tour Option: Some(TourEnum::try_from(cev.clone()).expect(&cev.as_string().unwrap())),
            //             0 tour Option: match TourEnum::try_from(cev.clone()) {
            //                 Ok(tour) => Some(tour),
            //                 Err(err) => panic!("{}: {err}", cev.as_string().unwrap()),
            //             },
            //             0 hall: cev.as_string(),
            //             0 start_at: cev.as_f64().map(spreahsheet_number_value_to_naive_time),
            //         })
            //     } else {
            //         spreadsheets::from_range_data!(range_data => CoupleRow { | cev |
            //             0 heat: cev.as_int().map(|i| i as i16),
            //             0 couple_number: cev.as_int().map(|i| i as i16),
            //             0 participant1: cev.as_string(),
            //             0 participant2 Option: cev.as_string(),
            //             0 category: cev.as_string(),
            //             // 0 tour Option: Some(TourEnum::from(cev)),
            //             // 0 tour Option: Some(TourEnum::try_from(cev.clone()).expect(&cev.as_string().unwrap())),
            //             0 tour Option: match TourEnum::try_from(cev.clone()) {
            //                 Ok(tour) => Some(tour),
            //                 Err(err) => panic!("{}: {err}", cev.as_string().unwrap()),
            //             },
            //             0 start_at: cev.as_f64().map(spreahsheet_number_value_to_naive_time),
            //             0 hall: Some({let _ = cev; "".to_owned()}),
            //         })
            //     };
            // } else {
            //     unreachable!();
            // }
        }
    }
    // schedule_rowlar.sort_by_key(|i| i.start_at);
    // debug!("{schedule_rowlar:#?}");
    let mut ret = vec![];
    for ScheduleRow {
        heat,
        category,
        tour,
        program,
        quantity,
        dances,
        heats,
        start_at,
        duration,
        // finish_at,
        floor,
    } in schedule_rowlar.into_iter()
    {
        let time = Time {
            start_at,
            duration,
            // finish_at,
        };
        let (time_of_new_heat, category_tour) = {
            let (time_of_new_heat, time_of_category_tour) = if let Some(Heat {
                time: time_prev,
                number: number_prev,
                ..
            }) = ret.last()
            {
                if number_prev != &heat {
                    (Some(time), None)
                } else if &time != time_prev {
                    (None, Some(time))
                } else {
                    (None, None)
                }
            } else {
                (Some(time), None)
            };
            (
                time_of_new_heat,
                CategoryTour {
                    time: time_of_category_tour,
                    quantity: quantity.and_then(|quantity| {
                        (!matches!(
                            category.as_str(),
                            "Break" | "Award Ceremony" | "Presentation"
                        ))
                        .then_some(quantity)
                    }),
                    category,
                    tour,
                    program,
                    dances,
                    heats: heats.and_then(|u| (u > 1).then_some(u)),
                },
            )
        };
        let is_new_heat = if let Some(Heat {
            ref mut category_tourlar,
            ..
        }) = &mut ret.last_mut()
        {
            if let Some(time) = time_of_new_heat {
                Some((time, category_tour))
            } else {
                category_tourlar.push(category_tour);
                None
            }
        } else {
            time_of_new_heat.map(|time| (time, category_tour))
        };
        if let Some((time, category_tour)) = is_new_heat {
            ret.push(Heat {
                number: heat,
                time,
                category_tourlar: vec![category_tour],
                floor,
            });
        };
    }
    Ok(ret)
    // let ret = Ret {
    //     schedule_rowlar,
    //     // couple_rowlar,
    //     name,
    // };
    // // if let Some(file_path) = &cache_for_local_opt {
    // //     let mut file = std::fs::File::create(file_path).context(format!("{:?}", file_path))?;
    // //     use std::io::Write;
    // //     file.write_all(serde_json::to_string(&ret)?.as_bytes())?;
    // //     debug!("did write file {file_path:?}");
    // // }
    // ret
    // };
    //
    // // =======================================================
    //
    // // let couple_rowlar = couple_rowlar
    // //     .into_iter()
    // //     .map(
    // //         |CoupleRow {
    // //              heat,
    // //              couple_number,
    // //              participant1,
    // //              participant2,
    // //              category,
    // //              tour,
    // //              start_at,
    // //              hall,
    // //          }| {
    // //             ImportCoupleRow {
    // //                 heat,
    // //                 couple_number,
    // //                 personlar: personlar(participant1, participant2),
    // //                 category,
    // //                 tour,
    // //                 start_at,
    // //                 hall,
    // //             }
    // //         },
    // //     )
    // //     .collect::<Vec<_>>();
    //
    // let (
    //     schedule_itemlar,
    //     // mut
    //     categorylar,
    // ) = import_schedule(schedule_rowlar)?;
    //
    // // todo!("schedule_itemlar: {schedule_itemlar:#?}");
    // // todo!("categorylar: {categorylar:#?}");
    // // todo!("schedule_itemlar: {schedule_itemlar:#?}\ncategorylar: {categorylar:#?}");
    //
    // // let _couple_categorylar = import_couplelar(couple_rowlar, &mut categorylar)?;
    // // todo!("categorylar: {categorylar:#?}");
    // // todo!("{couple_categorylar:#?}");
    // // todo!(
    // //     "{:#?}",
    // //     categorylar
    // //         .into_iter()
    // //         .map(|(category, _)| category)
    // //         .collect::<Vec<_>>()
    // // );
    // let categorylar = postprocess_categorylar(categorylar);
    // let schedule = postprocess_schedule_itemlar(schedule_itemlar, &categorylar);
    //
    // // =======================================================
    //
    // // if true {
    // //     // todo!("categorylar: {categorylar:#?}");
    // //     // show_categorylar(&categorylar);
    // //     // show_couple_categorylar(&couple_categorylar);
    // //     // categorylar.bar2();
    // //     show_schedule(&schedule, &categorylar);
    // //     // debug!("schedule_itemlar: {schedule_itemlar:#?}");
    // //     // todo!();
    // // }
    // // let couplelar = couple_categorylar
    // //     .into_iter()
    // //     .map(|(couple, _)| couple)
    // //     .collect();
    // //
    // // // {
    // // #[derive(Serialize)]
    // // pub struct CategoryDescription {
    // //     pub category_tourlar: Vec<CategoryTour>,
    // //     pub couplelar: Vec<ImportCouple>,
    // //     pub danceslar: Vec<Dances>,
    // // }
    // // let categorylar = categorylar
    // //     .into_iter()
    // //     .map(
    // //         |(
    // //             String,
    // //             CategoryDescription {
    // //                 category_tourlar,
    // //                 couplelar,
    // //                 danceslar,
    // //             },
    // //         )| CategoryDescription {
    // //             category_tourlar,
    // //             couplelar: couplelar.into_iter().collect(),
    // //             danceslar: danceslar.into_iter().collect(),
    // //         },
    // //     )
    // //     .collect::<Vec<_>>();
    // // debug!("{categorylar:#?}");
    // //
    // // let couple_categorylar = couple_categorylar.into_iter().collect::<Vec<_>>();
    // //     let schedule_itemlar = schedule_itemlar.into_iter().map(|(ScheduleItemNumber, ScheduleItem {
    // // start_at,
    // // duration,
    // // kind,
    // //     })|
    // //
    // //     ).collect::<Vec<_>>();
    // //
    // //     // let couple_categorylar = couple_categorylar.into_iter().collect::<Vec<_>>();
    // //     // let ret = MyRet {
    // //     //     categorylar,
    // //     //     // couple_categorylar,
    // //     //     // schedule_itemlar,
    // //     // };
    // //     // debug!("{}", serde_json::to_string_pretty(&ret)?);
    // // debug!("{}", serde_json::to_string_pretty(&categorylar)?);
    // // debug!("{}", serde_json::to_string_pretty(&couple_categorylar)?);
    // // debug!("{}", serde_json::to_string_pretty(&schedule_itemlar)?);
    // // }
    // // todo!();
    // // schedule_itemlar.foo();
    // // categorylar.bar3();
    //
    // Ok(ImportRet {
    //     name: name.unwrap(),
    //     categorylar,
    //     schedule,
    // })
}

// #[derive(Serialize)]
// pub struct MyRet {
//     pub categorylar: Vec<(String, CategoryDescription)>,
//     pub couple_categorylar: Vec<(ImportCouple, HashSet<String>)>,
//     pub schedule_itemlar: Vec<(i16, ScheduleItemImport)>,
// }

pub fn import_sync(res: ImportResult, // , op_mode: Option<OpMode>
) -> Result<()> {
    match res {
        Err(err) => {
            error!("import: {err}");
            // send_response_message(ResponseMessage::InitData(Err(err)), tx);
            // let response_message = ResponseMessage::InitData(Err(err));
            // let message_kind = ResponseMessageKind::from(&response_message);
            // if let Err(err) = tx.try_send(response_message) {
            //     error!("failed to send response {message_kind:?}: {err}");
            // }
        }
        Ok(ret) => {
            println!("{}", serde_yaml::to_string(&ret).unwrap());
            // pasitos!(db push_back Import { data, op_mode })
        }
    }
    Ok(())
}

// =============================================================================

// type ImportScheduleItemlar = HashMap<ScheduleItemNumber, ScheduleItemImport>;
//
// type ImportCategorylar = HashMap<
//     ImportCategory,
//     (
//         Tourlar,
//         // HashSet<ImportCouple>,
//         Danceslar,
//     ),
// >;
// fn import_schedule(
//     schedule_rowlar: Vec<pasitos::spreadsheet::ScheduleRow>,
// ) -> Result<(ImportScheduleItemlar, ImportCategorylar)> {
//     let mut categorylar: ImportCategorylar = HashMap::new();
//     let mut schedule_itemlar = HashMap::<ScheduleItemNumber, ScheduleItemImport>::new();
//
//     for schedule_row in schedule_rowlar.iter() {
//         let ScheduleRow {
//             heat,
//             category,
//             tour,
//             program,
//             quantity,
//             dances,
//             heats,
//             start_at,
//             duration,
//             hall,
//         } = schedule_row;
//
//         let (kind, category_opt) = {
//             let s = category.to_lowercase();
//             let s = s.as_str();
//             if s.contains("break") || s.contains("pause") {
//                 (
//                     ScheduleItemKind::Break {
//                         category: category.clone(),
//                         program: program.clone(),
//                     },
//                     None,
//                 )
//             } else if s.contains("award") || s.contains("награждение") {
//                 (
//                     ScheduleItemKind::AwardCeremony {
//                         category: category.clone(),
//                         program: program.clone(),
//                     },
//                     None,
//                 )
//             } else {
//                 let category = ImportCategory::from_category_program(category, program.as_deref());
//                 (
//                     match category.clone() {
//                         ImportCategory::OpeningCeremony {
//                             category,
//                             program, // , personlar: _
//                         } => ScheduleItemKind::OpeningCeremony { category, program },
//                         ImportCategory::Show {
//                             category,
//                             program,
//                             // personlar: _,
//                         } => ScheduleItemKind::Show { category, program },
//                         // ImportCategory::VoicePause { name, personlar: _ } => {
//                         //     ScheduleItemKind::VoicePause(name)
//                         // }
//                         ImportCategory::Just(category_just) => ScheduleItemKind::Dance(
//                             vec![CategoryDancesTour {
//                                 category: category.clone(),
//                                 dances: Dances::new(
//                                     &category_just,
//                                     program,
//                                     dances
//                                         .as_ref()
//                                         .ok_or_else(|| {
//                                             anyhow!("{}:{}: category: {category}", file!(), line!())
//                                         })?
//                                         .as_ref(),
//                                 ),
//                                 tour: *tour.as_ref().unwrap(),
//                             }]
//                             .into_iter()
//                             .enumerate()
//                             .map(|(i, item)| (item, i as i16))
//                             .collect(),
//                         ),
//                     },
//                     Some(category),
//                 )
//             }
//             // match category.to_lowercase().as_str() {
//             //     "break" | "dance pause" => (ScheduleItemKind::Break(category.clone()), None),
//             //     "award ceremony" | "награждение" => {
//             //         (ScheduleItemKind::AwardCeremony(category.clone()), None)
//             //     }
//             //     _ => {
//             //         let category =
//             //             ImportCategory::from_category_program(category, program.as_deref());
//             //         (
//             //             match category.clone() {
//             //                 ImportCategory::OpeningCeremony { name, personlar: _ } => {
//             //                     ScheduleItemKind::OpeningCeremony(name)
//             //                 }
//             //                 ImportCategory::Show {
//             //                     category,
//             //                     program,
//             //                     personlar: _,
//             //                 } => ScheduleItemKind::Show { category, program },
//             //                 ImportCategory::VoicePause { name, personlar: _ } => {
//             //                     ScheduleItemKind::VoicePause(name)
//             //                 }
//             //                 ImportCategory::Just(category_just) => ScheduleItemKind::Dance(
//             //                     vec![CategoryDancesTour {
//             //                         category: category.clone(),
//             //                         dances: Dances::new(
//             //                             &category_just,
//             //                             program,
//             //                             dances
//             //                                 .as_ref()
//             //                                 .ok_or_else(|| {
//             //                                     anyhow!(
//             //                                         "{}:{}: category: {category}",
//             //                                         file!(),
//             //                                         line!()
//             //                                     )
//             //                                 })?
//             //                                 .as_ref(),
//             //                         ),
//             //                         tour: *tour.as_ref().unwrap(),
//             //                     }]
//             //                     .into_iter()
//             //                     .enumerate()
//             //                     .map(|(i, item)| (item, i as i16))
//             //                     .collect(),
//             //                 ),
//             //             },
//             //             Some(category),
//             //         )
//             //     }
//             // }
//         };
//
//         common_macros2::entry!(schedule_itemlar, *heat
//         =>
//             and_modify |e| {
//                 if e.start_at != *start_at {
//                     warn!("e.start_at: {}, start_at: {}", e.start_at, start_at);
//                     if  e.start_at > *start_at {
//                         e.start_at = *start_at;
//                     }
//                 }
//                 if e.duration != *duration {
//                     warn!("e.duration: {:?}, duration: {:?}", e.duration, duration);
//                     if  e.duration < *duration {
//                         e.duration = *duration;
//                     }
//                 }
//                 if let ScheduleItemKind::Dance(category_dances_tourlar) = &mut e.kind {
//                     category_dances_tourlar
//                         .insert(CategoryDancesTour {
//                             category: category_opt.clone().unwrap(),
//                             dances: Dances::new(category, program, dances.as_ref().unwrap()),
//                             tour: *tour.as_ref().unwrap(),
//                         }, 0);
//                 } else {
//                     unreachable!("heat: {heat}, e.kind: {:?}\nERROR: multiple heat items must be Dance, non-Dance schedule items could not merged into one heat\n", e.kind);
//                 }
//             }
//             or_insert ScheduleItemImport {
//                 start_at: *start_at,
//                 duration: *duration,
//                 kind: kind.clone(),
//                 hall: hall.clone(),
//             }
//         );
//
//         // if let ScheduleItemKind::Dance(category_dances_tourlar) = &kind {
//         if let Some(category_import) = category_opt {
//             common_macros2::entry!(categorylar, category_import.clone()
//             =>
//                 and_modify |e| {
//                     // if !is_special_category(&category) {
//                     if let ScheduleItemKind::Dance(_category_dances_tourlar) = &kind {
//                         if let Some(tour) = tour {
//                             common_macros2::entry!(e.0, *tour
//                             =>
//                                 and_modify |e| {
//                                     assert_eq!(e.0, *quantity);
//                                     if let Some(heats_eta) = e.1 {
//                                         assert_eq!(heats_eta, heats.unwrap());
//                                     } else {
//                                         assert!(heats.is_none());
//                                     }
//                                 }
//                                 or_insert (*quantity, *heats)
//                             );
//                         }
//                         if let Some(dances) = dances {
//                             let dances = Dances::new(category, program, dances);
//                             // e.2.insert(dances.clone());
//                             e.1.insert(dances.clone());
//                         }
//                     }
//                 }
//                 or_insert (
//                     matches!(kind, ScheduleItemKind::Dance(_)).then_some(tour)
//                         .and_then(|tour| tour.as_ref())
//                         .map(|tour|
//                             vec![(*tour, (*quantity, *heats))].into_iter().collect()
//                         ).unwrap_or_else(HashMap::new),
//                     // HashSet::new(),
//                     {
//                         // debug!("{dances:?}");
//                     matches!(kind, ScheduleItemKind::Dance(_)).then_some(dances)
//                     // dances
//                     .and_then(|v| (*v).clone()).as_ref().map(|dances| {
//                         vec![Dances::new(category, program, dances)].into_iter().collect()
//                     }).unwrap_or_else(HashSet::new)
//                     },
//                 )
//             );
//         }
//     }
//
//     for (
//         _category,
//         (
//             tourlar,
//             // _couplelar,
//             _danceslar,
//         ),
//     ) in categorylar.iter()
//     {
//         let mut tourlar_vec = tourlar
//             .iter()
//             .map(|(tour, (quantity, heats))| CategoryTour {
//                 tour: *tour,
//                 heats: *heats,
//                 limit: (!tour.is_special()).then_some(*quantity),
//             })
//             .collect::<Vec<_>>();
//         tourlar_vec.sort_by_key(|i| i.tour);
//         if let Some(last) = tourlar_vec.last_mut() {
//             last.limit = None;
//         }
//     }
//
//     Ok((schedule_itemlar, categorylar))
// }
//
// // =============================================================================
//
// // type ImportCouplelar = HashMap<ImportCouple, HashSet<ImportCategory>>;
// // fn import_couplelar(
// //     couple_rowlar: Vec<pasitos::spreadsheet::ImportCoupleRow>,
// //     categorylar: &mut ImportCategorylar,
// // ) -> Result<ImportCouplelar> {
// //     let mut couple_categorylar: ImportCouplelar = HashMap::new();
// //     for ImportCoupleRow {
// //         category,
// //         couple_number,
// //         personlar,
// //         tour,
// //         ..
// //     } in couple_rowlar.iter()
// //     {
// //         let couple = ImportCouple {
// //             number: *couple_number,
// //             personlar: personlar.clone(),
// //         };
// //         let category: ImportCategory = ImportCategory::from_category_personlar(category, personlar);
// //         common_macros2::entry!(categorylar, category.clone()
// //         =>
// //             and_modify_entry |e| {
// //                 e.get_mut().1.insert(couple.clone());
// //                 if matches!(category, ImportCategory::Show{..}) {
// //                     e.replace_key();
// //                 }
// //                 // if let Some(tour) = tour {
// //                 //     if e.0.get(tour).is_none() {
// //                 //         unreachable!("category: {category}, tour: {tour:?}");
// //                 //     }
// //                 //     // assert!(e.0.get(tour).is_some());
// //                 // }
// //             }
// //             or_insert {
// //                 unreachable!("category: {category:?}, tour: {tour:?} found at Лист2, but not found at Лист1");
// //             }
// //         );
// //         // } else {
// //         //     warn!("category: {category:?}");
// //         // }
// //         common_macros2::entry!(couple_categorylar, couple
// //         =>
// //             and_modify |e| { e.insert(category.clone()); }
// //             or_insert vec![category.clone()].into_iter().collect()
// //         );
// //     }
// //     Ok(couple_categorylar)
// // }
//
// // =============================================================================
//
// fn postprocess_categorylar(categorylar: ImportCategorylar) -> Categorylar {
//     categorylar
//         .into_iter()
//         .map(
//             |(
//                 category,
//                 (
//                     tourlar,
//                     // couplelar,
//                     danceslar,
//                 ),
//             )| {
//                 let mut category_tourlar = tourlar
//                     .into_iter()
//                     .map(|(tour, (quantity, heats))| CategoryTour {
//                         tour,
//                         heats,
//                         limit: (!tour.is_special()).then_some(quantity),
//                     })
//                     .collect::<Vec<_>>();
//                 category_tourlar.sort_by_key(|i| i.tour);
//                 if let Some(last) = category_tourlar.last_mut() {
//                     last.limit = None;
//                 }
//                 (
//                     category,
//                     CategoryDescription {
//                         category_tourlar,
//                         // couplelar,
//                         danceslar,
//                     },
//                 )
//             },
//         )
//         .collect::<Categorylar>()
// }
//
// // =============================================================================
//
// // type Hall = String;
// // type ImportSchedule = HashMap<ImportHall, Vec<(i16, ScheduleItemImport)>>;
// fn postprocess_schedule_itemlar(
//     schedule_itemlar: HashMap<i16, ScheduleItemImport>,
//     _categorylar: &Categorylar,
// ) -> ImportSchedule {
//     let mut ret: ImportSchedule = HashMap::new();
//     for (
//         schedule_item_number,
//         ScheduleItemImport {
//             start_at,
//             duration,
//             kind,
//             hall,
//         },
//     ) in schedule_itemlar.into_iter()
//     {
//         let schedule_item = ImportScheduleItem {
//             start_at,
//             duration,
//             kind,
//         };
//         common_macros2::entry!(ret, hall
//         =>
//             and_modify |e| {
//                 e.push((schedule_item_number, schedule_item));
//             }
//             or_insert
//                 vec![(schedule_item_number, schedule_item)]
//         );
//     }
//
//     for (_hall, schedule_itemlar) in ret.iter_mut() {
//         schedule_itemlar.sort_by_key(|i| i.0);
//         // TODO:
//         // for (_schedule_item_number, schedule_item) in schedule_itemlar.iter_mut() {
//         //     if let ScheduleItemKind::Dance(ref mut category_dances_tourlar) = &mut schedule_item.kind {
//         //         *category_dances_tourlar = optimize(category_dances_tourlar, categorylar);
//         //     }
//         // }
//     }
//     ret
// }

// =============================================================================
