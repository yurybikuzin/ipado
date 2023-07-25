use super::*;

#[derive(Debug, Clone)]
pub enum CompactExtendedValue {
    Bool(bool),
    Error(ErrorValue),
    Formula(String),
    Int(i64),
    Float(f64),
    String(String),
}

impl CompactExtendedValue {
    pub fn into_extended_value(self) -> ExtendedValue {
        match self {
            Self::Bool(value) => ExtendedValue {
                bool_value: Some(value),
                ..ExtendedValue::default()
            },
            Self::Error(value) => ExtendedValue {
                error_value: Some(value),
                ..ExtendedValue::default()
            },
            Self::Formula(value) => ExtendedValue {
                formula_value: Some(value),
                ..ExtendedValue::default()
            },
            Self::Int(value) => ExtendedValue {
                number_value: Some(value as f64),
                ..ExtendedValue::default()
            },
            Self::Float(value) => ExtendedValue {
                number_value: Some(value),
                ..ExtendedValue::default()
            },
            Self::String(value) => ExtendedValue {
                string_value: Some(value),
                ..ExtendedValue::default()
            },
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Int(i) => Some(*i as f64),
            Self::Float(i) => Some(*i),
            Self::String(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            Self::String(s) => s.parse::<i64>().ok(),
            _ => None,
        }
    }
    pub fn as_datetime(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Float(n) => Some(*n),
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
        .and_then(spreahsheet_number_value_to_datetime)
    }
    pub fn as_naive_date(&self) -> Option<NaiveDate> {
        match self {
            Self::Float(n) => Some(*n),
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
        .map(spreahsheet_number_value_to_naive_date)
    }
    pub fn as_naive_time(&self) -> Option<NaiveTime> {
        match self {
            Self::Float(n) => Some(*n),
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
        .map(spreahsheet_number_value_to_naive_time)
    }
    pub fn as_duration(&self) -> Option<std::time::Duration> {
        match self {
            Self::Float(n) => Some(*n),
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
        .map(spreahsheet_number_value_to_duration)
    }
    pub fn as_string(&self) -> Option<String> {
        match self {
            Self::Float(n) => Some(n.to_string()),
            Self::Int(i) => Some(i.to_string()),
            Self::String(s) => Some(s.clone()),
            _ => None,
        }
    }
    pub fn from_extended_value_opt(value: Option<&ExtendedValue>) -> Option<Self> {
        match value {
            None => None,
            Some(value) => {
                if let Some(string_value) = &value.string_value {
                    Some(Self::String(string_value.clone()))
                } else if let Some(number_value) = value.number_value {
                    if number_value.fract() == 0f64 {
                        Some(Self::Int(number_value as i64))
                    } else {
                        Some(Self::Float(number_value))
                    }
                } else if let Some(formula_value) = &value.formula_value {
                    Some(Self::Formula(formula_value.clone()))
                } else if let Some(error_value) = &value.error_value {
                    Some(Self::Error(error_value.clone()))
                } else {
                    value.bool_value.map(Self::Bool)
                }
            }
        }
    }
}
