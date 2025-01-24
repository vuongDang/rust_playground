// Complete the implementation of `Display` & `Debug` for `MonthError`.

use std::{
    error::Error,
    fmt::{Debug, Display},
};

enum Month {
    Jan,
    Feb,
    March,
    April,
    May,
    June,
    July,
    Aug,
    Sept,
    Oct,
    Nov,
    Dec,
}

impl TryFrom<u8> for Month {
    type Error = MonthError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Month::Jan),
            2 => Ok(Month::Feb),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::Aug),
            9 => Ok(Month::Sept),
            10 => Ok(Month::Oct),
            11 => Ok(Month::Nov),
            12 => Ok(Month::Dec),
            _ => Err(MonthError {
                source: None,
                msg: format!("{value} does not correspond to a month"),
            }),
        }
    }
}

impl ToString for Month {
    fn to_string(&self) -> String {
        match self {
            Self::Jan => "Jan",
            Self::Feb => "Feb",
            Self::March => "March",
            Self::April => "April",
            Self::May => "May",
            Self::June => "June",
            Self::July => "July",
            Self::Aug => "Aug",
            Self::Sept => "Sept",
            Self::Oct => "Oct",
            Self::Nov => "Nov",
            Self::Dec => "Dec",
        }
        .to_string()
    }
}

struct MonthError {
    source: Option<Box<dyn Error>>,
    msg: String,
}

impl Error for MonthError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

impl Display for MonthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error converting input to months, check your input")
    }
}

impl Debug for MonthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.source() {
            None => write!(f, "{}", self.msg),
            Some(month_err) => {
                write!(f, "{}\n Caused by: {:?}", self.msg, month_err)
            }
        }
        // if self.source is Some, write "{a}\nCaused by: {b:?}", a=self.msg, b=error stored inside Some
        // else, write self.msg to the Formatter
    }
}

// function to convert a string to corresponding vector of owned month strings
// e.g. "1 2 3 4" -> ["Jan", "Feb", "March", "April"]
fn get_months(months: &str) -> Result<Vec<String>, MonthError> {
    let nums = months
        .split(' ')
        .into_iter()
        .map(|num_str| {
            num_str.parse::<u8>().map_err(|e| MonthError {
                source: Some(Box::new(e)),
                msg: format!("Can not parse {num_str} to u8"),
            })
        })
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|e| MonthError {
            source: Some(Box::new(e)),
            msg: format!("Could not convert string to numbers"),
        })?;
    let month_strs = nums
        .into_iter()
        .map(|num| Month::try_from(num))
        .collect::<Result<Vec<Month>, _>>()
        .map_err(|e| MonthError {
            source: Some(Box::new(e)),
            msg: format!("Could not convert nums to months"),
        })?
        .into_iter()
        .map(|month| month.to_string())
        .collect::<Vec<String>>();
    Ok(month_strs)
}

fn convert_and_print(nums: &str) {
    match get_months(nums) {
        Ok(months) => println!("Months: {months:?}\n"),
        Err(e) => println!("{e:?}\n"),
    }
}

fn main() {
    let input1 = "1 2 3 4 9 10";
    let input2 = "xyz 10 12";
    let input3 = "1 3 20";
    convert_and_print(input1);
    convert_and_print(input2);
    convert_and_print(input3);
}
