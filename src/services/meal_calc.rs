use std::{fmt::{Display, Formatter}, str::FromStr};
use serde::Serialize;
use minilp::{LinearExpr, Problem, Solution, Variable};

use super::csv_reader::{CsvReader, Record};
use crate::{Error, Result};

pub struct MealCalculator {
    reader: CsvReader,
    protein: f64,
    carbs: f64,
    sodium: f64,
    calories: f64,
}

#[derive(Serialize, Debug)]
pub struct FoodItem {
    pub name: String,
    pub serving_size: u32,
    pub count: u8,
    pub calories: f64,
    pub protein: f64,
    pub carbs: f64,
    pub sodium: f64,
    pub allergens: Vec<Allergens>
}

#[derive(Serialize, Debug)]
pub enum Allergens {
    None,
    Milk,
    Eggs,
    Peanuts,
    Soybean,
    Wheat,
    TreeNut,
    Shellfish,
    Fish,
    Sesame,
    Vegan,
    Vegetarian,
    GlutenFree,
    LactoseIntolerance,
}

impl Display for Allergens {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl FromStr for Allergens {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "None" => Ok(Allergens::None),
            "Milk" => Ok(Allergens::Milk),
            "Eggs" => Ok(Allergens::Eggs),
            "Peanuts" => Ok(Allergens::Peanuts),
            "Soybean" => Ok(Allergens::Soybean),
            "Wheat" => Ok(Allergens::Wheat),
            "TreeNut" => Ok(Allergens::TreeNut),
            "Shellfish" => Ok(Allergens::Shellfish),
            "Fish" => Ok(Allergens::Fish),
            "Sesame" => Ok(Allergens::Sesame),
            "Vegan" => Ok(Allergens::Vegan),
            "Vegetarian" => Ok(Allergens::Vegetarian),
            "Gluten" => Ok(Allergens::GlutenFree),
            "LactoseIntolerance" => Ok(Allergens::LactoseIntolerance),
            _ => Err(Error::InvalidAllergen)
        }
    }

    type Err = Error;
}

#[derive(Serialize, Debug)]
pub struct Meal {
    items: Vec<FoodItem>,
    total_calories: f64,
    total_protein: f64,
    total_carbs: f64,
    total_sodium: f64,
    total_grams: u32,
}

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
    pub fn calculate_meal(&self) -> Result<Meal> {
        let solution = self.get_solution();

        match solution {
            Ok(sol) => {
                let valid_items = sol.iter().filter(| (_, value)| **value >= 0.5).collect::<Vec<(Variable, &f64)>>();

                let meal: Vec<FoodItem> = valid_items.iter().map(|(var, value)| {
                    let record: &Record = self.reader.get_record(var.idx()).unwrap();

                    let allergens: Vec<Allergens> = record.allergens
                        .split(',')
                        .map(|allergen| allergen.trim().parse::<Allergens>().unwrap())
                        .collect();

                    FoodItem {
                        name: record.item.clone(),
                        serving_size: record.serving_size,
                        count: (**value).round() as u8,
                        calories: record.calories,
                        protein: record.protein,
                        carbs: record.carbs,
                        sodium: record.sodium,
                        allergens: allergens,
                    }
                }).collect();

                Ok(Meal {
                    total_calories: meal.iter().map(|item| item.calories * item.count as f64).sum(),
                    total_protein: meal.iter().map(|item| item.protein * item.count as f64).sum(),
                    total_carbs: meal.iter().map(|item| item.carbs * item.count as f64).sum(),
                    total_sodium: meal.iter().map(|item| item.sodium * item.count as f64).sum(),
                    total_grams: sol.objective() as u32,
                    items: meal,
                })
            },
            Err(e) => {
                return Err(e)
            }
        }
    }

    fn get_solution(&self) -> Result<Solution> {
        let mut problem = Problem::new(minilp::OptimizationDirection::Maximize);
    
        // make a new var variable and flip the values of the serving sizes

        let vars: Vec<Variable> = self.reader.get_calories().iter().map(|coef| problem.add_var((*coef) as f64, (0.0, f64::INFINITY))).collect();

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
