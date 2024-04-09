use crate::{model::ModelController, Result};

use axum::{extract::State, routing::get, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/calculateCalories", get(calculate_calories))
        .route("/getmealplan", get(get_meal_plan))
        .with_state(mc)
}

#[derive(Debug, Deserialize)]
pub struct CalorieCalcParams {
    pub activity: String,
    pub gender: String,
    pub height: f32,
    pub weight: f32,
    pub age: u8,
}

pub async fn calculate_calories(State(mc): State<ModelController>, Json(payload): Json<CalorieCalcParams>) -> Result<Json<Value>> {
    println!("-->> {:12} - api_calculate_calories", "HANDLER");

    let calories = mc.calculate_calories(payload).await?;

    // Create the success body
    let body = Json(json!({
        "result": {
            "calories": calories
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct GetMealPlanParams {
    protein: f64, 
    carbs: f64, 
    sodium: f64, 
    calories: f64,
}

pub async fn get_meal_plan(State(mc): State<ModelController>, Json(payload): Json<GetMealPlanParams>) -> Result<Json<Value>> {
    println!("-->> {:12} - api_get_meal_plan", "HANDLER");

    let solution = mc.get_meal_plan(payload.protein, payload.carbs, payload.sodium, payload.calories).await?;

    // Create the success body
    let body = Json(json!({
        "result": {
            "solution": solution
        }
    }));

    Ok(body)
}
