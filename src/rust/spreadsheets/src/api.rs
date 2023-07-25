use super::*;

pub async fn spreadsheets_get(
    ranges: Vec<String>,
    incluge_grid_data: bool,
    spreadsheet_id: &str,
    key: yup_oauth2::ServiceAccountKey,
) -> Result<Spreadsheet> {
    let auth = ServiceAccountAuthenticator::builder(key.clone())
        .build()
        .await
        .map_err(|err| anyhow!("ServiceAccountAuthenticator: {err}"))?;
    let hub = Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http2()
                .build(),
        ),
        auth,
    );
    let mut req = hub
        .spreadsheets()
        .get(spreadsheet_id)
        .include_grid_data(incluge_grid_data);
    for range in ranges {
        req = req.add_ranges(&range)
    }
    let (_resp, spreadsheet) = req.doit().await.map_err(|err| anyhow!("get: {err}"))?;
    Ok(spreadsheet)
}

pub async fn spreadsheets_batch_update(
    requests: Vec<Request>,
    spreadsheet_id: &str,
    key: yup_oauth2::ServiceAccountKey,
) -> Result<BatchUpdateSpreadsheetResponse> {
    let auth = ServiceAccountAuthenticator::builder(key.clone())
        .build()
        .await
        .map_err(|err| anyhow!("ServiceAccountAuthenticator: {err}"))?;
    let hub = Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http2()
                .build(),
        ),
        auth,
    );
    let (_resp, batch_update_response) = hub
        .spreadsheets()
        .batch_update(
            BatchUpdateSpreadsheetRequest {
                requests: Some(requests),
                ..BatchUpdateSpreadsheetRequest::default()
            },
            spreadsheet_id,
        )
        .doit()
        .await
        .map_err(|err| anyhow!("batch_update({spreadsheet_id}): {err}"))?;
    Ok(batch_update_response)
}

pub async fn get_range_data(
    range: String,
    spreadsheet_id: &str,
    key: yup_oauth2::ServiceAccountKey,
) -> Result<Option<Vec<RowData>>> {
    let spreadsheets = spreadsheets_get(vec![range.clone()], true, spreadsheet_id, key)
        .await
        .map_err(|err| anyhow!("get_range_data({range:?}): {err}"))?;
    let sheets = if let Some(sheets) = spreadsheets.sheets {
        sheets
    } else {
        warn!("{}:{}: spreadsheets.sheets is none", file!(), line!());
        return Ok(None);
    };
    let sheet = if let Some(sheet) = sheets.into_iter().next() {
        sheet
    } else {
        warn!("{}:{}: sheets.get(0) is none", file!(), line!());
        return Ok(None);
    };
    let data = if let Some(data) = sheet.data {
        data
    } else {
        warn!("{}:{}: sheet.data is none", file!(), line!());
        return Ok(None);
    };
    let grid_data = if let Some(grid_data) = data.into_iter().next() {
        grid_data
    } else {
        warn!("{}:{}: data.get(0) is none", file!(), line!());
        return Ok(None);
    };
    let row_data = if let Some(row_data) = grid_data.row_data {
        row_data
    } else {
        warn!("{}:{}: grid_data.row_data is none", file!(), line!());
        return Ok(None);
    };
    Ok(Some(row_data))
}

use std::collections::HashMap;
pub async fn get_rangelar_data(
    rangelar: Vec<String>,
    spreadsheet_id: &str,
    key: yup_oauth2::ServiceAccountKey,
) -> Result<HashMap<String, HashMap<DataRange, Vec<RowData>>>> {
    let spreadsheets = spreadsheets_get(rangelar.clone(), true, spreadsheet_id, key)
        .await
        .map_err(|err| anyhow!("get_rangelar_data({rangelar:?}): {err}"))?;
    let mut ret: HashMap<String, HashMap<DataRange, Vec<RowData>>> = HashMap::new();
    for sheet in spreadsheets.sheets.unwrap().into_iter() {
        let title = sheet.properties.unwrap().title.clone().unwrap();
        for grid_data in sheet.data.unwrap().into_iter() {
            if let Some(row_data) = grid_data.row_data {
                let data_range = DataRange {
                    start_col: grid_data.start_column.unwrap_or(0),
                    start_row: grid_data.start_row.unwrap_or(0),
                    row_len: row_data.len(),
                    col_len: itertools::max(row_data.iter().map(|row_data| {
                        row_data
                            .values
                            .as_ref()
                            .map(|values| values.len())
                            .unwrap_or(0)
                    })),
                };
                common_macros2::entry!(ret, title.clone() =>
                    and_modify |e| {
                        e.insert(data_range, row_data)
                    }
                    or_insert vec![(data_range, row_data)].into_iter().collect()
                )
            }
        }
    }
    Ok(ret)
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DataRange {
    pub start_col: i32,
    pub start_row: i32,
    pub row_len: usize,
    pub col_len: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct SheetProps {
    pub sheet_id: i32,
    pub row_count: u32,
    pub column_count: u32,
}

pub async fn get_sheet_props(
    sheet_name: &str,
    spreadsheet_id: &str,
    key: yup_oauth2::ServiceAccountKey,
) -> Result<Option<SheetProps>> {
    Ok(
        spreadsheets_get(vec![
                         sheet_name.to_owned()
        ], false, spreadsheet_id, key)
            .await
            .map_err(|err| {
                anyhow!("get_sheet_props(sheet_name: {sheet_name:?}, spreadsheet_id: {spreadsheet_id:?}, key): {err}")
            })?
            .sheets
            .and_then(|sheets| {
                sheets
                    .first()
                    .and_then(|sheet| sheet.properties.as_ref())
                    .map(|properties| {
                        let grid_properties = properties.grid_properties.as_ref().unwrap();
                        SheetProps {
                            sheet_id: properties.sheet_id.unwrap(),
                            row_count: grid_properties.row_count.unwrap() as u32,
                            column_count: grid_properties.column_count.unwrap() as u32,
                        }
                    })
            }),
    )
}

pub async fn get_sheets_props(
    sheets_name: Vec<String>,
    spreadsheet_id: &str,
    key: yup_oauth2::ServiceAccountKey,
) -> Result<HashMap<String, SheetProps>> {
    let mut ret = HashMap::new();
    if let Some(sheets) = spreadsheets_get(sheets_name.clone(), false, spreadsheet_id, key)
            .await
            .map_err(|err| {
                anyhow!("get_sheets_props(sheets_name: {sheets_name:?}, spreadsheet_id: {spreadsheet_id:?}, key): {err}")
            })?
            .sheets {
                for sheet in sheets {
                    if let Some(properties) = sheet.properties.as_ref() {
                        if let Some(sheet_name) = properties.title.clone() {
                            let grid_properties = properties.grid_properties.as_ref().unwrap();
                            ret.insert(sheet_name, SheetProps {
                                sheet_id: properties.sheet_id.unwrap(),
                                row_count: grid_properties.row_count.unwrap() as u32,
                                column_count: grid_properties.column_count.unwrap() as u32,
                            });
                        }
                    }
                }
            }
    Ok(ret)
}

pub async fn list_sheets(
    spreadsheet_id: &str,
    key: yup_oauth2::ServiceAccountKey,
) -> Result<Option<Vec<String>>> {
    Ok(spreadsheets_get(vec![], false, spreadsheet_id, key)
        .await?
        .sheets
        .map(|sheets| {
            sheets
                .iter()
                .filter_map(|sheet| sheet.properties.as_ref())
                .filter_map(|properties| properties.title.clone())
                .collect::<Vec<_>>()
        }))
}
