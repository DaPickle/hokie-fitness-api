// use std::fmt::{Display, Formatter};

use minilp::{LinearExpr, Problem, Solution, Variable};

use super::csv_reader::CsvReader;
use crate::{Error, Result};

pub struct MealCalculator {
    reader: CsvReader,
    protein: f64,
    carbs: f64,
    sodium: f64,
    calories: f64,
}

// pub struct FoodItem {
//     serving_size: u32,
//     calories: u32,
//     protein: u32,
//     carbs: u32,
//     sodium: u32,
//     allergens: Vec<Allergens>
// }

// #[derive(Debug)]
// pub enum Allergens {
//     None,
//     Milk,
//     Eggs,
//     Peanuts,
//     Soybean,
//     Wheat,
//     TreeNut,
//     Shellfish,
//     Fish,
//     Sesame,
//     Vegan,
//     Vegetarian,
//     GlutenFree,
//     LactoseIntolerance,
// }

// impl Display for Allergens {
//     fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(fmt, "{self:?}")
//     }
// }

// region:      --- Meal Calculation
impl MealCalculator {
    pub async fn new(file_name: &str, protein: f64, carbs: f64, sodium: f64, calories: f64) -> Result<Self> {
        Ok(Self {
            reader: CsvReader::new(String::from(file_name)).await?,
            protein: protein,
            carbs: carbs,
            sodium: sodium,
            calories: calories
        })
    }
}

impl MealCalculator {
    pub fn calculate_meal(&self) -> Result<f64> {
        let solution = self.get_solution();

        match solution {
            Ok(sol) => {
                sol.iter().for_each(|sol| {
                    println!("{:?}: {}", sol.0, sol.1.round() as u32);
                });

                Ok(sol.objective())
            },
            Err(e) => {
                return Err(e)
            }
        }
    }

    fn get_solution(&self) -> Result<Solution> {
        let mut problem = Problem::new(minilp::OptimizationDirection::Maximize);
    
        // make a new var variable and flip the values of the serving sizes

        let vars: Vec<Variable> = self.reader.get_serving_sizes().iter().map(|coef| problem.add_var((*coef) as f64, (0.0, f64::INFINITY))).collect();

        // calories constraint
        // takes the var and makes a tuple of (Variable, f64)
        // It is collected into a LinearExpr
        let calorie_constraint: LinearExpr = vars.iter().zip(self.reader.get_calories()).map(|(var, coef)| (*var, coef)).collect::<LinearExpr>();

        // protein constraint
        let protein_constraint: LinearExpr = vars.iter().zip(self.reader.get_protein()).map(|(var, coef)| (*var, coef)).collect::<LinearExpr>();

        // carbs constraint
        let carbs_constraint: LinearExpr = vars.iter().zip(self.reader.get_carbs()).map(|(var, coef)| (*var, coef)).collect::<LinearExpr>();

        // sodium constraint
        let sodium_constraint: LinearExpr = vars.iter().zip(self.reader.get_sodium()).map(|(var, coef)| (*var, coef)).collect::<LinearExpr>();

        problem.add_constraint(calorie_constraint, minilp::ComparisonOp::Le, self.calories);
        problem.add_constraint(protein_constraint, minilp::ComparisonOp::Ge, self.protein);
        problem.add_constraint(carbs_constraint, minilp::ComparisonOp::Le, self.carbs);
        problem.add_constraint(sodium_constraint, minilp::ComparisonOp::Le, self.sodium);

        let sol = problem.solve();

        match sol {
            Ok(s) => {
                Ok(s)
            },
            Err(_) => {
                Err(Error::ImpossibleSolution)
            }
        }
    }
}
