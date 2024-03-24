use serde::Deserialize;

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct CalorieCalcParams {
    activity: String,
    gender: String,
    height: f32,
    weight: f32,
    age: u8,
}

// region:      --- Model Controller
#[derive(Clone)]
pub struct ModelController {
}

impl ModelController {
    pub fn new() -> Self {
        Self {  }
    }
}

impl ModelController {
    pub async fn calculate_calories(&self, params: CalorieCalcParams) -> Result<f32> {
        let activity_multiplier = get_activity_multiplier(&params.gender, &params.activity)?;

        // calorie calculation
        let calories = 862.0 - (9.72 * params.age as f32) + ((activity_multiplier)*((14.2 * params.weight) + (503.0 * params.height as f32)));

        Ok(calories)
    }
}

// region:      Helper functions
fn get_activity_multiplier(gender: &str, activity: &str) -> Result<f32> {
    if gender != "male" && gender != "female" {
        return Err(Error::InvalidArgument)
    }


    match activity {
        "sedentary" => Ok(1.0),
        "light" => {
            if "male" == gender {
                Ok(1.12)
            }
            else {
                Ok(1.14)
            }
        },
        "moderate" => Ok(1.27),
        "vigorous" => {
            if "male" == gender {
                Ok(1.54)
            }
            else {
                Ok(1.45)
            }
        },
        _ => Err(Error::InvalidArgument)
    }
}