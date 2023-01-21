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

#[cfg(test)]
mod tests {
    use crate::{to_jalali, JalaliDate};

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
}
