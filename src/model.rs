use crate::services::meal_calc::{Meal, MealCalculator};
use crate::{web::routes_main::CalorieCalcParams, Error, Result};

pub const FILE_NAME: &str = "./d2_database.csv";

// region:      --- Model Controller
#[derive(Clone)]
pub struct ModelController {

}

impl ModelController {
    pub fn new() -> Self {
        Self { }
    }
}

impl ModelController {
    pub async fn calculate_calories(&self, params: CalorieCalcParams) -> Result<f32> {
        let activity_multiplier = get_activity_multiplier(&params.gender, &params.activity)?;

        // calorie calculation
        let calories = 862.0 - (9.72 * params.age as f32) + ((activity_multiplier)*((14.2 * params.weight) + (503.0 * params.height as f32)));

        Ok(calories)
    }

    pub async fn get_meal_plan(&self, protein: f64, carbs: f64, sodium: f64, calories: f64) -> Result<Meal> {
        let Ok(meal_calc) = MealCalculator::new(FILE_NAME, protein, carbs, sodium, calories).await
        else {
            return Err(Error::InvalidFile)
        };

        meal_calc.calculate_meal()
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
