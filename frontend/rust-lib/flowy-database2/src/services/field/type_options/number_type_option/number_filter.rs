use std::str::FromStr;

use rust_decimal::Decimal;

use crate::entities::{NumberFilterConditionPB, NumberFilterPB};
use crate::services::field::NumberCellFormat;

impl NumberFilterPB {
  pub fn is_visible(&self, cell_data: &NumberCellFormat) -> Option<bool> {
    let expected_decimal = || Decimal::from_str(&self.content).ok();

    let strategy = match self.condition {
      NumberFilterConditionPB::Equal => NumberFilterStrategy::Equal(expected_decimal()?),
      NumberFilterConditionPB::NotEqual => NumberFilterStrategy::NotEqual(expected_decimal()?),
      NumberFilterConditionPB::GreaterThan => {
        NumberFilterStrategy::GreaterThan(expected_decimal()?)
      },
      NumberFilterConditionPB::LessThan => NumberFilterStrategy::LessThan(expected_decimal()?),
      NumberFilterConditionPB::GreaterThanOrEqualTo => {
        NumberFilterStrategy::GreaterThanOrEqualTo(expected_decimal()?)
      },
      NumberFilterConditionPB::LessThanOrEqualTo => {
        NumberFilterStrategy::LessThanOrEqualTo(expected_decimal()?)
      },
      NumberFilterConditionPB::NumberIsEmpty => NumberFilterStrategy::Empty,
      NumberFilterConditionPB::NumberIsNotEmpty => NumberFilterStrategy::NotEmpty,
    };

    Some(strategy.filter(cell_data))
  }
}

enum NumberFilterStrategy {
  Equal(Decimal),
  NotEqual(Decimal),
  GreaterThan(Decimal),
  LessThan(Decimal),
  GreaterThanOrEqualTo(Decimal),
  LessThanOrEqualTo(Decimal),
  Empty,
  NotEmpty,
}

impl NumberFilterStrategy {
  fn filter(self, cell_data: &NumberCellFormat) -> bool {
    match self {
      NumberFilterStrategy::Equal(expected_value) => cell_data
        .decimal()
        .is_some_and(|decimal| decimal == expected_value),
      NumberFilterStrategy::NotEqual(expected_value) => cell_data
        .decimal()
        .is_some_and(|decimal| decimal != expected_value),
      NumberFilterStrategy::GreaterThan(expected_value) => cell_data
        .decimal()
        .is_some_and(|decimal| decimal > expected_value),
      NumberFilterStrategy::LessThan(expected_value) => cell_data
        .decimal()
        .is_some_and(|decimal| decimal < expected_value),
      NumberFilterStrategy::GreaterThanOrEqualTo(expected_value) => cell_data
        .decimal()
        .is_some_and(|decimal| decimal >= expected_value),
      NumberFilterStrategy::LessThanOrEqualTo(expected_value) => cell_data
        .decimal()
        .is_some_and(|decimal| decimal <= expected_value),
      NumberFilterStrategy::Empty => cell_data.is_empty(),
      NumberFilterStrategy::NotEmpty => !cell_data.is_empty(),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::entities::{NumberFilterConditionPB, NumberFilterPB};
  use crate::services::field::{NumberCellFormat, NumberFormat};

  #[test]
  fn number_filter_equal_test() {
    let number_filter = NumberFilterPB {
      condition: NumberFilterConditionPB::Equal,
      content: "123".to_owned(),
    };

    for (num_str, visible) in [("123", true), ("1234", false), ("", false)] {
      let data = NumberCellFormat::from_format_str(num_str, &NumberFormat::Num).unwrap_or_default();
      assert_eq!(number_filter.is_visible(&data), Some(visible));
    }

    let format = NumberFormat::USD;
    for (num_str, visible) in [("$123", true), ("1234", false), ("", false)] {
      let data = NumberCellFormat::from_format_str(num_str, &format).unwrap();
      assert_eq!(number_filter.is_visible(&data), Some(visible));
    }
  }

  #[test]
  fn number_filter_greater_than_test() {
    let number_filter = NumberFilterPB {
      condition: NumberFilterConditionPB::GreaterThan,
      content: "12".to_owned(),
    };
    for (num_str, visible) in [("123", true), ("10", false), ("30", true), ("", false)] {
      let data = NumberCellFormat::from_format_str(num_str, &NumberFormat::Num).unwrap_or_default();
      assert_eq!(number_filter.is_visible(&data), Some(visible));
    }
  }

  #[test]
  fn number_filter_less_than_test() {
    let number_filter = NumberFilterPB {
      condition: NumberFilterConditionPB::LessThan,
      content: "100".to_owned(),
    };
    for (num_str, visible) in [("12", true), ("1234", false), ("30", true), ("", false)] {
      let data = NumberCellFormat::from_format_str(num_str, &NumberFormat::Num).unwrap_or_default();
      assert_eq!(number_filter.is_visible(&data), Some(visible));
    }
  }
}
