use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::ChatValidationError;

const CURRENCY_CODE_LENGTH: usize = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatDatasetCoverage {
    pub source_rows: u64,
    pub unlinked_rows: u64,
    pub outside_scope_rows: u64,
}

impl ChatDatasetCoverage {
    pub fn validate(&self, linked_rows: u64) -> Result<(), ChatValidationError> {
        let total = linked_rows
            .checked_add(self.unlinked_rows)
            .and_then(|rows| rows.checked_add(self.outside_scope_rows));
        if total != Some(self.source_rows) {
            return Err(ChatValidationError::InvalidTable);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(
    feature = "sqlx",
    sqlx(type_name = "chat_profit_measure", rename_all = "snake_case")
)]
pub enum ChatProfitMeasure {
    GopbdProxy,
    GopbdOnly,
}

impl ChatProfitMeasure {
    pub fn label(self) -> &'static str {
        match self {
            Self::GopbdProxy => "EBITDA estimate — using GOPBD",
            Self::GopbdOnly => "GOPBD",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatReportingContext {
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub currency: Option<String>,
    pub profit_measure: ChatProfitMeasure,
}

impl ChatReportingContext {
    pub fn validate(&self) -> Result<(), ChatValidationError> {
        let valid_period = match (self.period_start, self.period_end) {
            (Some(start), Some(end)) => start <= end,
            (None, None) => true,
            _ => false,
        };
        let valid_currency = self.currency.as_ref().is_none_or(|currency| {
            currency.len() == CURRENCY_CODE_LENGTH
                && currency.bytes().all(|byte| byte.is_ascii_uppercase())
        });
        if !valid_period || !valid_currency {
            return Err(ChatValidationError::InvalidTable);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reporting_context_keeps_unknown_periods_explicit_and_rejects_partial_or_reversed_periods() {
        let mut context = ChatReportingContext {
            period_start: None,
            period_end: None,
            currency: None,
            profit_measure: ChatProfitMeasure::GopbdProxy,
        };
        assert!(context.validate().is_ok());
        context.period_start = Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        assert!(context.validate().is_err());
        context.period_end = Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        assert!(context.validate().is_err());
        context.period_end = Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());
        context.currency = Some("EUR".into());
        assert!(context.validate().is_ok());
        context.currency = Some("€".into());
        assert!(context.validate().is_err());
    }
}
