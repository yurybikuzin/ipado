#[allow(unused_imports)]
use anyhow::{anyhow, bail, Context, Error, Result};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

mod color;
pub use color::*;

extern crate google_sheets4 as sheets4;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
pub use sheets4::{
    api::{
        AppendCellsRequest, AppendDimensionRequest, BasicFilter, BatchUpdateSpreadsheetRequest,
        BatchUpdateSpreadsheetResponse, CellData, CellFormat, ClearBasicFilterRequest, Color,
        CopyPasteRequest, ErrorValue, ExtendedValue, GridRange, NumberFormat, Request, RowData,
        SetBasicFilterRequest, SortRangeRequest, SortSpec, Spreadsheet, TextFormat,
        UpdateCellsRequest,
    },
    Sheets,
};
use yup_oauth2::ServiceAccountAuthenticator;

pub mod api;
pub use api::*;

pub mod cell;
pub use cell::*;

pub mod compact_extended_value;
pub use compact_extended_value::*;

pub mod datetime;
pub use datetime::*;

pub mod request;
pub use request::*;

mod macros;
pub use macros::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
