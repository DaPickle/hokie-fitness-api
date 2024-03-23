use crate::{Error, Result};

// region:      enums
pub enum ActivityLevel {
    Sedentary,
    Light,
    Moderate,
    Vigorous,
}

#[derive(PartialEq)]
pub enum Gender {
    Male,
    Female,
}

pub struct CalorieCalc {
    activity: ActivityLevel,
    gender: Gender,
    height: f32,
    weight: f32,
    age: i8,
}

// region:      --- Model Controller
pub struct ModelController {
}

impl ModelController {
    pub async fn calculate_calories(params: CalorieCalc) -> f32 {
        let activityMultiplier = get_activity_multiplier(params.gender, params.activity);

        activityMultiplier
    }
}

// region:      Helper functions
fn get_activity_multiplier(gender: Gender, activity: ActivityLevel) -> f32 {
    match activity {
        ActivityLevel::Sedentary => 1.0,
        ActivityLevel::Light => {
            if Gender::Male == gender {
                1.12
            }
            else {
                1.14
            }
        },
        ActivityLevel::Moderate => 1.27,
        ActivityLevel::Vigorous => {
            if Gender::Male == gender {
                1.54
            }
            else {
                1.45
            }
        },
    }
}