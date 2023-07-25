use super::*;

pub fn cell_data(
    user_entered_value: Option<CompactExtendedValue>,
    user_entered_format: Option<CellFormat>,
) -> CellData {
    CellData {
        user_entered_value: user_entered_value.map(|value| value.into_extended_value()),
        user_entered_format,
        ..CellData::default()
    }
}

pub fn cell_data_from_naive_date(naive_date: NaiveDate) -> CellData {
    let duration = naive_date - NaiveDate::from_ymd_opt(1899, 12, 30).unwrap(); // https://social.msdn.microsoft.com/Forums/office/en-US/f1eef5fe-ef5e-4ab6-9d92-0998d3fa6e14/what-is-story-behind-december-30-1899-as-base-date?forum=accessdev
    cell_data(
        Some(CompactExtendedValue::Int(duration.num_days())),
        Some(cell_format(Some(CellNumberFormat::Date), None)),
    )
}

pub fn cell_data_from_datetime(datetime: DateTime<Utc>) -> CellData {
    cell_data(
        Some(CompactExtendedValue::Float(
            datetime_to_spreadsheet_number_value(datetime),
        )),
        Some(cell_format(Some(CellNumberFormat::DateTime), None)),
    )
}

pub enum CellNumberFormat {
    DateTime,
    Date,
}

pub fn cell_format(
    number_format: Option<CellNumberFormat>,
    background_color: Option<Color>,
) -> CellFormat {
    CellFormat {
        number_format: number_format.map(|number_format| NumberFormat {
            type_: Some(
                match number_format {
                    CellNumberFormat::DateTime => "DATE_TIME",
                    CellNumberFormat::Date => "DATE",
                }
                .to_owned(),
            ),
            ..NumberFormat::default()
        }),
        background_color,
        ..CellFormat::default()
    }
}
