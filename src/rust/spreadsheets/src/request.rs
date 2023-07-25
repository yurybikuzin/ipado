use super::*;

// https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/request#pastetype
pub enum PasteType {
    Normal,
    Values,
    Format,
    NoBorders,
    Formula,
    DataValidation,
    ConditionalFormatting,
}

pub enum Dimension {
    Columns,
    Rows,
}

pub fn append_dimension_request(dimension: Dimension, length: u32, sheet_id: i32) -> Request {
    Request {
        append_dimension: Some(AppendDimensionRequest {
            dimension: Some(
                match dimension {
                    Dimension::Columns => "COLUMNS",
                    Dimension::Rows => "ROWS",
                }
                .to_owned(),
            ),
            length: Some(length as i32),
            sheet_id: Some(sheet_id),
        }),
        ..Request::default()
    }
}

pub fn copy_paste_request(
    paste_type: PasteType,
    source: GridRange,
    destination: GridRange,
) -> Request {
    Request {
        copy_paste: Some(CopyPasteRequest {
            paste_type: Some(
                match paste_type {
                    // https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/request#pastetype
                    PasteType::Normal => "PASTE_NORMAL",
                    PasteType::Values => "PASTE_VALUES",
                    PasteType::Format => "PASTE_FORMAT",
                    PasteType::NoBorders => "PASTE_NO_BORDERS",
                    PasteType::Formula => "PASTE_FORMULA",
                    PasteType::DataValidation => "PASTE_DATA_VALIDATION",
                    PasteType::ConditionalFormatting => "PASTE_CONDITIONAL_FORMATTING",
                }
                .to_owned(),
            ),
            source: Some(source),
            destination: Some(destination),
            ..CopyPasteRequest::default()
        }),
        ..Request::default()
    }
}

pub enum SortOrder {
    Asc,
    Desc,
}

pub fn sort_spec(dimension_index: u32, sort_order: SortOrder) -> SortSpec {
    SortSpec {
        dimension_index: Some(dimension_index as i32),
        sort_order: Some(
            match sort_order {
                SortOrder::Asc => "ASCENDING",
                SortOrder::Desc => "DESCENDING",
            }
            .to_owned(),
        ),
        ..SortSpec::default()
    }
}

pub fn sort_range_request(range: GridRange, sort_specs: Vec<SortSpec>) -> Request {
    Request {
        sort_range: Some(SortRangeRequest {
            range: Some(range),
            sort_specs: Some(sort_specs),
        }),
        ..Request::default()
    }
}

pub fn set_basic_filter_request(range: GridRange, sort_specs: Vec<SortSpec>) -> Request {
    Request {
        set_basic_filter: Some(SetBasicFilterRequest {
            filter: Some(BasicFilter {
                range: Some(range),
                sort_specs: if sort_specs.is_empty() {
                    None
                } else {
                    Some(sort_specs)
                },
                ..BasicFilter::default()
            }),
        }),
        ..Request::default()
    }
}

pub fn append_cells_request(rows: Vec<Vec<CellData>>, sheet_id: i32) -> Request {
    Request {
        append_cells: Some(AppendCellsRequest {
            sheet_id: Some(sheet_id),
            fields: Some("*".to_owned()),
            rows: Some(
                rows.into_iter()
                    .map(|values| RowData {
                        values: if values.is_empty() {
                            None
                        } else {
                            Some(values)
                        },
                    })
                    .collect(),
            ),
        }),
        ..Request::default()
    }
}

pub fn update_cells_request(range: GridRange, rows: Vec<RowData>) -> Request {
    Request {
        update_cells: Some(UpdateCellsRequest {
            rows: Some(rows),
            fields: Some("*".to_owned()),
            range: Some(range),
            ..UpdateCellsRequest::default()
        }),
        ..Request::default()
    }
}

pub fn clear_basic_filter_request(sheet_id: i32) -> Request {
    Request {
        clear_basic_filter: Some(ClearBasicFilterRequest {
            sheet_id: Some(sheet_id),
        }),
        ..Request::default()
    }
}

pub enum RangeDim {
    FromLen(u32, u32),
    From(u32),
}

impl RangeDim {
    fn start_index(&self) -> Option<i32> {
        Some(*match self {
            Self::FromLen(start_index, ..) => start_index,
            Self::From(start_index) => start_index,
        } as i32)
    }
    fn end_index(&self) -> Option<i32> {
        match self {
            Self::FromLen(start_index, len) => Some((*start_index + *len) as i32),
            Self::From(..) => None,
        }
    }
}

pub fn grid_range(row: RangeDim, column: RangeDim, sheet_id: i32) -> GridRange {
    GridRange {
        sheet_id: Some(sheet_id),
        start_row_index: row.start_index(),
        start_column_index: column.start_index(),
        end_row_index: row.end_index(),
        end_column_index: column.end_index(),
    }
}
