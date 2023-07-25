// ===================================================================

#[macro_export]
macro_rules! spreadsheet_rows_without_caption (
    ($source:expr => $struct:ident {
        $(
            $fieldlar:ident : $exprlar:expr
        ),+
        $(,)?
    }) => {
        spreadsheet_rows_without_caption!(@collect
            (),
            (),
            (),
        ;
            $source => $struct {
                $(
                    $fieldlar : $exprlar
                ),+
            }
        )
    };
    (@collect
         ( $($ret_columnlar:tt)* ),
         ( $($ret_fieldlar:tt)* ),
         ( $($ret_matchlar:tt)* ),
     ;
         $source:expr => $struct:ident {
             $field:ident : $expr:expr
             $(,
                 $(
                     $fieldlar:ident : $exprlar:expr
                 ),+
             )?
    }) => {
        spreadsheet_rows_without_caption!(@collect
            ( $($ret_columnlar)* stringify!($field) , ),
            ( $($ret_fieldlar)* $field , ),
            ( $($ret_matchlar)*
                stringify!($field) => $expr,
            ),
        ;
            $source => $struct {
                $(
                    $(
                        $fieldlar : $exprlar
                    ),+
                )?
            }
        )
    };
    (@collect
         ( $($ret_columnlar:tt)+ ),
         ( $($ret_fieldlar:tt)+ ),
         ( $($ret_matchlar:tt)+ ),
     ;
         $source:expr => $struct:ident {
    }) => {{
        let columns = [ $($ret_columnlar)+ ];
        $source.into_iter().map(
                |$struct { $($ret_fieldlar)+ .. }| {
                    columns
                        .iter()
                        .map(|col| match *col {
                            $($ret_matchlar)+
                            _ => unreachable!(),
                        })
                        .collect::<Vec<_>>()
                },
            ).collect::<Vec<_>>()
    }};
);

// ===================================================================

#[macro_export]
macro_rules! spreadsheet_rows (
    ($source:expr => $struct:ident {
        $(
            $fieldlar:ident : $exprlar:expr
        ),+
        $(,)?
    }) => {
        spreadsheet_rows!(@collect
            (),
            (),
            (),
        ;
            $source => $struct {
                $(
                    $fieldlar : $exprlar
                ),+
            }
        )
    };
    (@collect
         ( $($ret_columnlar:tt)* ),
         ( $($ret_fieldlar:tt)* ),
         ( $($ret_matchlar:tt)* ),
     ;
         $source:expr => $struct:ident {
             $field:ident : $expr:expr
             $(,
                 $(
                     $fieldlar:ident : $exprlar:expr
                 ),+
             )?
    }) => {
        spreadsheet_rows!(@collect
            ( $($ret_columnlar)* stringify!($field) , ),
            ( $($ret_fieldlar)* $field , ),
            ( $($ret_matchlar)*
                stringify!($field) => $expr,
            ),
        ;
            $source => $struct {
                $(
                    $(
                        $fieldlar : $exprlar
                    ),+
                )?
            }
        )
    };
    (@collect
         ( $($ret_columnlar:tt)+ ),
         ( $($ret_fieldlar:tt)+ ),
         ( $($ret_matchlar:tt)+ ),
     ;
         $source:expr => $struct:ident {
    }) => {{
        let columns = [ $($ret_columnlar)+ ];
        vec![None]
            .into_iter()
            .map(|_: Option<()>| {
                columns
                    .iter()
                    .map(|col| cell_data(Some(CompactExtendedValue::String(col.to_string())), None))
                    .collect::<Vec<_>>()
            })
            .chain($source.into_iter().map(
                |$struct { $($ret_fieldlar)+ .. }| {
                    columns
                        .iter()
                        .map(|col| match *col {
                            $($ret_matchlar)+
                            _ => unreachable!(),
                        })
                        .collect::<Vec<_>>()
                },
            ))
            .collect::<Vec<_>>()
    }};
);

// ===================================================================

#[macro_export]
macro_rules! from_range_data (
    ($source:expr => $struct:ident { | $cev:ident |
        $(
            $inclar:literal $fieldlar:ident
            $( Option $( { .$($unwrap:tt)+ } )?  )?
            : $exprlar:expr
        ,)+
    }) => {
        from_range_data!(@collect
            (),
            (),
            (),
            0
        ;
            $source => $struct { | values, $cev |
                $(
                    $inclar $fieldlar
                    $( Option $( { .$($unwrap)+ } )?  )?
                    : $exprlar
                ,)+
            }
        )
    };
    (@collect
         ( $($ret_columnlar:tt)* ),
         ( $($ret_fieldlar:tt)* ),
         ( $($ret_extractlar:tt)* ),
         $i:expr
    ;
         $source:expr => $struct:ident { | $values:ident, $cev:ident |
             $inc:literal $field:ident : $expr:expr,
             $(
                $inclar:literal $fieldlar:ident
                $( Option $( { .$($unwrap:tt)+ } )?  )?
                : $exprlar:expr
             ,)*
        }
    ) => {
        from_range_data!(@collect
            ( $($ret_columnlar)* $field , ),
            ( $($ret_fieldlar)* $field , ),
            ( $($ret_extractlar)*
                .and_then(|($($ret_columnlar)*)| {
                    $values
                        .get($i + $inc)
                        .and_then(|cell_data| {
                            CompactExtendedValue::from_extended_value_opt(
                                cell_data.effective_value.as_ref(),
                            )
                        })
                        .and_then(|$cev| {
                            $expr
                        })
                        .map(|value| ( $($ret_columnlar)* value, ))
                })
            ),
            $i + $inc + 1
        ;
            $source => $struct { | $values, $cev |
                $(
                    $inclar $fieldlar
                    $( Option $( { .$($unwrap)+ } )?  )?
                    : $exprlar
                ,)*
            }
        )
    };
    (@collect
         ( $($ret_columnlar:tt)* ),
         ( $($ret_fieldlar:tt)* ),
         ( $($ret_extractlar:tt)* ),
         $i:expr
    ;
         $source:expr => $struct:ident { | $values:ident, $cev:ident |
             $inc:literal $field:ident Option : $expr:expr,
             $(
                $inclar:literal $fieldlar:ident
                $( Option $( { .$($unwrap:tt)+ } )?  )?
                : $exprlar:expr
             ,)*
        }
    ) => {
        from_range_data!(@collect
            ( $($ret_columnlar)* $field , ),
            ( $($ret_fieldlar)* $field , ),
            ( $($ret_extractlar)*
                .map(|($($ret_columnlar)*)| {
                    let value = $values
                        .get($i + $inc)
                        .and_then(|cell_data| {
                            CompactExtendedValue::from_extended_value_opt(
                                cell_data.effective_value.as_ref(),
                            )
                        })
                        .and_then(|$cev| $expr );
                    ( $($ret_columnlar)* value, )
                })
            ),
            $i + $inc + 1
        ;
            $source => $struct { | $values, $cev |
                $(
                    $inclar $fieldlar
                    $( Option $( { .$($unwrap)+ } )?  )?
                    : $exprlar
                ,)*
            }
        )
    };
    (@collect
         ( $($ret_columnlar:tt)* ),
         ( $($ret_fieldlar:tt)* ),
         ( $($ret_extractlar:tt)* ),
         $i:expr
    ;
         $source:expr => $struct:ident { | $values:ident, $cev:ident |
             $inc:literal $field:ident Option { . $($unwrap:tt)+ } : $expr:expr,
             $(
                $inclar:literal $fieldlar:ident
                $( Option $( { .$($unwraplar:tt)+ } )?  )?
                : $exprlar:expr
             ,)*
    }) => {
        from_range_data!(@collect
            ( $($ret_columnlar)* $field , ),
            ( $($ret_fieldlar)* $field , ),
            ( $($ret_extractlar)*
                .map(|($($ret_columnlar)*)| {
                    let value = $values
                        .get($i + $inc)
                        .and_then(|cell_data| {
                            CompactExtendedValue::from_extended_value_opt(
                                cell_data.effective_value.as_ref(),
                            )
                        })
                        .and_then(|$cev| $expr )
                        .$($unwrap)+;
                    ( $($ret_columnlar)* value, )
                })
            ),
            $i + $inc + 1
        ;
            $source => $struct { | $values, $cev |
                $(
                    $inclar $fieldlar
                    $( Option $( { .$($unwraplar)+ } )?  )?
                    : $exprlar
                ,)*
            }
        )
    };
    (@collect
         ( $($ret_columnlar:tt)+ ),
         ( $($ret_fieldlar:tt)+ ),
         ( $($ret_extractlar:tt)+ ),
         $i:expr
    ;
         $source:expr => $struct:ident { | $values:ident, $cev:ident | }
    ) => {
        $source
            .into_iter()
            .filter_map(|row_data| row_data.values)
            .filter_map(|$values| {
                Some(())
                    $($ret_extractlar)+
                    .map(|( $($ret_fieldlar)+ )|
                        $struct {
                            $($ret_fieldlar)+
                        }
                    )
            })
            .collect::<Vec<_>>()
    };
);

// ===================================================================
