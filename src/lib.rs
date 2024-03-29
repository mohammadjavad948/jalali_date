static DAY_SUM: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
static DAY_SUM_KABISE: [u16; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

#[derive(PartialEq, Eq, Debug)]
pub struct JalaliDate {
    pub day: u16,
    pub year: u16,
    pub month: u16,
}

pub fn to_jalali(day: u16, month: u16, year: u16) -> Result<JalaliDate, &'static str> {
    if month > 12 || month < 1 {
        return Err("cant do it");
    }

    let days_sum = if is_kabise(year) {
        DAY_SUM_KABISE[month as usize - 1] + day
    } else {
        DAY_SUM[month as usize - 1] + day
    };

    if days_sum < 79 {
        let days_sum = days_sum + dey_jan_diff(year);
        let jalai_year = year - 622;

        if days_sum % 30 == 0 {
            return Ok(JalaliDate {
                year: jalai_year,
                day: 30,
                month: (days_sum / 30) + 9,
            });
        } else {
            return Ok(JalaliDate {
                year: jalai_year,
                day: days_sum % 30,
                month: (days_sum / 30) + 10,
            });
        }
    } else {
        let days_sum = days_sum - 79;
        let jalali_year = year - 621;

        if days_sum <= 186 {
            if days_sum % 31 == 0 {
                return Ok(JalaliDate {
                    day: 31,
                    year: jalali_year,
                    month: days_sum / 31,
                });
            } else {
                return Ok(JalaliDate {
                    day: days_sum % 31,
                    year: jalali_year,
                    month: (days_sum / 31) + 1,
                });
            }
        } else {
            let days_sum = days_sum - 186;
            if days_sum % 30 == 0 {
                return Ok(JalaliDate {
                    day: 30,
                    year: jalali_year,
                    month: (days_sum / 30) + 6,
                });
            } else {
                return Ok(JalaliDate {
                    day: days_sum % 30,
                    year: jalali_year,
                    month: (days_sum / 30) + 7,
                });
            }
        }
    }
}

fn dey_jan_diff(year: u16) -> u16 {
    if is_kabise(year) {
        return 11;
    }

    return 10;
}

fn is_kabise(year: u16) -> bool {
    if year % 4 == 0 && year % 100 != 0 {
        return true;
    }

    if year % 400 == 0 && year % 100 == 0 {
        return true;
    }

    false
}

fn year_is_leap(gregorian_year: i32) -> bool {
    return ((gregorian_year % 100) != 0 && (gregorian_year % 4) == 0)
        || ((gregorian_year % 100) == 0 && (gregorian_year % 400) == 0);
}

static gregorian_months: [i32; 12] = [30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 28, 31];
static gregorian_month_leap: [i32; 12] = [30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29, 31];

#[derive(Debug, PartialEq, Eq)]
pub struct GregorianDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

/// month range is 1..12
/// day starts from 1
pub fn jalali_to_gregorian(year: i32, month: i32, day: i32) -> GregorianDate {
    let mut gregorian_year = year + 621;
    let mut gregorian_day_of_month = 0;
    let mut gregorian_month = 0;
    let march_day_diff = if year_is_leap(gregorian_year) { 12 } else { 11 };
    let mut day_count = 0;

    if (1..=6).contains(&month) {
        day_count = (month - 1) * 31 + day;
    } else {
        day_count = (6 * 31) + (month - 7) * 30 + day;
    }

    if day_count < march_day_diff {
        gregorian_day_of_month = day_count + (31 - march_day_diff);
        gregorian_month = 3;
    } else {
        let mut remain_days = day_count - march_day_diff;
        let mut i = 0;

        if year_is_leap(gregorian_year + 1) {
            while remain_days > gregorian_months[i] {
                remain_days -= gregorian_month_leap[i];
                i += 1;
            }
        } else {
            while remain_days > gregorian_months[i] {
                remain_days -= gregorian_months[i];
                i += 1;
            }
        }

        gregorian_day_of_month = remain_days;

        if i > 8 {
            gregorian_month = i - 8;
            gregorian_year += 1;
        } else {
            gregorian_month = i + 4;
        }
    }

    GregorianDate {
        year: gregorian_year,
        month: gregorian_month as i32,
        day: gregorian_day_of_month,
    }
}

#[cfg(test)]
mod tests {
    use crate::{jalali_to_gregorian, to_jalali, GregorianDate, JalaliDate};

    #[test]
    fn convert_date() {
        let result = to_jalali(6, 2, 2022).unwrap();
        assert_eq!(
            result,
            JalaliDate {
                day: 17,
                month: 11,
                year: 1400
            }
        );
    }

    #[test]
    fn convert_date_2() {
        let result = to_jalali(23, 11, 2015).unwrap();
        assert_eq!(
            result,
            JalaliDate {
                day: 2,
                month: 9,
                year: 1394
            }
        );
    }

    #[test]
    fn jalali_to_gregorian_date() {
        let result = jalali_to_gregorian(1402, 8, 24);
        assert_eq!(
            result,
            GregorianDate {
                day: 15,
                month: 11,
                year: 2023
            }
        );
    }

    #[test]
    fn jalali_to_gregorian_date_2() {
        let result = jalali_to_gregorian(1402, 3, 3);
        assert_eq!(
            result,
            GregorianDate {
                day: 24,
                month: 5,
                year: 2023
            }
        );
    }
}
