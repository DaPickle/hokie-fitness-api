use crate::{model::{CalorieCalcParams, ModelController}, Result};

use axum::{extract::State, routing::get, Json, Router};
use serde_json::{json, Value};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/calculateCalories", get(calculate_calories))
        .with_state(mc)
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
