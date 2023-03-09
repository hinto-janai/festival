//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use chrono::{
	NaiveDate,
	Datelike,
};

//---------------------------------------------------------------------------------------------------- Date format constants & macros.
const YEAR_MONTH_DAY: [&str; 5] = ["%Y-%m-%d", "%Y%m%d", "%Y/%m/%d", "%Y.%m.%d", "%Y_%m_%d"];
const MONTH_DAY_YEAR: [&str; 5] = ["%m-%d-%Y", "%m%d%Y", "%m/%d/%Y", "%m.%d.%Y", "%m_%d_%Y"];
const DAY_MONTH_YEAR: [&str; 5] = ["%d-%m-%Y", "%d%m%Y", "%d/%m/%Y", "%d.%m.%Y", "%d_%m_%Y"];

macro_rules! parse_year_month_day {
    ($string:ident) => {
		for format in YEAR_MONTH_DAY {
			if let Ok(d) = chrono::NaiveDate::parse_from_str($string, format) {
				return (Some(d.year()), Some(d.month()), Some(d.day()));
			}
		}
    }
}

macro_rules! parse_month_day_year {
    ($string:ident) => {
		for format in MONTH_DAY_YEAR {
			if let Ok(d) = chrono::NaiveDate::parse_from_str($string, format) {
				return (Some(d.year()), Some(d.month()), Some(d.day()));
			}
		}
    }
}

macro_rules! parse_day_month_year {
    ($string:ident) => {
		for format in DAY_MONTH_YEAR {
			if let Ok(d) = chrono::NaiveDate::parse_from_str($string, format) {
				return (Some(d.year()), Some(d.month()), Some(d.day()));
			}
		}
    }
}

//---------------------------------------------------------------------------------------------------- Main date format function.
impl super::Ccd {
	#[inline]
	// Parse arbitrary strings for a date.
	// The inner format is (YEAR, MONTH, DAY).
	pub(super) fn parse_str_date(string: &str) -> (Option<i32>, Option<u32>, Option<u32>) {
		// 4 length must mean it's only the
		// year, so only attempt to parse that.
		let length = string.len();
		if length == 4 {
			if let Ok(d) = string.parse::<i32>() {
				return (Some(d), None, None)
			} else {
				return (None, None, None)
			}
		}

		// Attempt various formats.
		parse_year_month_day!(string);
		parse_month_day_year!(string);
		parse_day_month_year!(string);

		// I have some albums with some strange prefixed date tags.
		// This will attempt to cover those by using the last 8-10 characters.
		if length >= 10 {
			// Attempt last 10.
			let last_ten = &string[(length - 10)..];
			parse_year_month_day!(last_ten);
			parse_month_day_year!(last_ten);
			parse_day_month_year!(last_ten);
		}
		if length >= 8 {
			// Attempt last 8.
			let last_eight = &string[(length - 8)..];
			parse_year_month_day!(last_eight);
			parse_month_day_year!(last_eight);
			parse_day_month_year!(last_eight);
		}

		// Give up.
		(None, None, None)
	}

	#[inline]
	// Takes in the output from above and formats it into a `String`.
	pub(super) fn date_to_string(date: (Option<i32>, Option<u32>, Option<u32>)) -> String {
		match date {
			(Some(year), None, None)             => format!("{}", year),
			(Some(year), Some(month), None)      => format!("{}-{:0>2}", year, month),
			(Some(year), Some(month), Some(day)) => format!("{}-{:0>2}-{:0>2}", year, month, day),
			_                                    => String::new(),
		}
	}

	#[inline]
	// Compares two tuple dates.
	pub(super) fn cmp_tuple_dates(
		a: (Option<i32>, Option<u32>, Option<u32>),
		b: (Option<i32>, Option<u32>, Option<u32>),
	) -> std::cmp::Ordering {
		use std::cmp::Ordering;
		match (a, b) {
			// None.
			((None, _, _), (None, _, _)) => Ordering::Equal,
			// Years.
			((Some(a), _, _), (None,    _, _))    => Ordering::Greater,
			((None,    _, _), (Some(b), _, _))    => Ordering::Less,
			((Some(a), _, _), (Some(b),    _, _)) => if a > b { Ordering::Greater } else if a < b { Ordering::Less } else { Ordering::Equal },
			// Years + Months.
			((Some(a1), Some(a2), _), (Some(b1), Some(b2), _)) => {
				// Year.
				if a1 > b1 {
					return Ordering::Greater
				} else if a1 < b1 {
					return Ordering::Less
				} else if a1 == b1 {
					// Month.
					if a2 > b2 {
						return Ordering::Greater
					} else if a2 < b2 {
						return Ordering::Less
					}
				}

				Ordering::Equal
			},
			// Years + Months + Days.
			((Some(a1), Some(a2), Some(a3)), (Some(b1), Some(b2), Some(b3))) => {
				// Year.
				if a1 > b1 {
					return Ordering::Greater
				} else if a1 < b1 {
					return Ordering::Less
				} else if a1 == b1 {
					// Month.
					if a2 > b2 {
						return Ordering::Greater
					} else if a2 < b2 {
						return Ordering::Less
					} else if a2 == b2 {
						// Days.
						if a3 > b3 {
							return Ordering::Greater
						} else if a3 < b3 {
							return Ordering::Less
						}
					}
				}

				Ordering::Equal
			},
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use crate::ccd::Ccd;

	const EXPECTED: (Option<i32>, Option<u32>, Option<u32>) = (Some(2020), Some(12), Some(25));

	#[test]
	fn year() {
		for i in 1000..=9999 {
			eprintln!("{}", i);
			assert!(Ccd::parse_str_date(&i.to_string()) == (Some(i), None, None));
		}
	}

	#[test]
	fn year_month_day() {
		assert!(Ccd::parse_str_date("2020-12-25") == EXPECTED);
		assert!(Ccd::parse_str_date("2020 12 25") == EXPECTED);
		assert!(Ccd::parse_str_date("20201225")   == EXPECTED);
		assert!(Ccd::parse_str_date("2020/12/25") == EXPECTED);
		assert!(Ccd::parse_str_date("2020.12.25") == EXPECTED);
		assert!(Ccd::parse_str_date("2020_12_25") == EXPECTED);
	}

	#[test]
	fn month_day_year() {
		assert!(Ccd::parse_str_date("12-25-2020") == EXPECTED);
		assert!(Ccd::parse_str_date("12 25 2020") == EXPECTED);
		assert!(Ccd::parse_str_date("12252020")   == EXPECTED);
		assert!(Ccd::parse_str_date("12/25/2020") == EXPECTED);
		assert!(Ccd::parse_str_date("12.25.2020") == EXPECTED);
		assert!(Ccd::parse_str_date("12_25_2020") == EXPECTED);
	}

	#[test]
	fn day_month_year() {
		assert!(Ccd::parse_str_date("25-12-2020") == EXPECTED);
		assert!(Ccd::parse_str_date("25 12 2020") == EXPECTED);
		assert!(Ccd::parse_str_date("25122020")   == EXPECTED);
		assert!(Ccd::parse_str_date("25/12/2020") == EXPECTED);
		assert!(Ccd::parse_str_date("25.12.2020") == EXPECTED);
		assert!(Ccd::parse_str_date("25_12_2020") == EXPECTED);
	}

	#[test]
	fn prefixed() {
		assert!(Ccd::parse_str_date("sejfioswe-joifewijfio_25-12-2020") == EXPECTED);
		assert!(Ccd::parse_str_date("aaaaaaaaaaaaaaaaaaaaaa25 12 2020") == EXPECTED);
		assert!(Ccd::parse_str_date("1234233432890371289d437125122020") == EXPECTED);
		assert!(Ccd::parse_str_date("jbogfjh'>>><44*&&&&'''25/12/2020") == EXPECTED);
		assert!(Ccd::parse_str_date("vcx/b.****.>D<FD>?GF>D25.12.2020") == EXPECTED);
		assert!(Ccd::parse_str_date("796632984y%#HTHSRDU(Gh25_12_2020") == EXPECTED);
	}

	#[test]
	fn to_string() {
		assert!(Ccd::date_to_string((Some(2020), Some(12), Some(25))) == "2020-12-25");
		assert!(Ccd::date_to_string((Some(2020), Some(12), None))     == "2020-12");
		assert!(Ccd::date_to_string((Some(2020), None,     None))     == "2020");
		assert!(Ccd::date_to_string((None,       Some(12), Some(25))) == "");
		assert!(Ccd::date_to_string((None,       None,     Some(25))) == "");
		assert!(Ccd::date_to_string((None,       None,     None))     == "");
	}
}
