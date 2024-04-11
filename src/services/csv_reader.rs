use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio_stream::StreamExt;

use crate::{Result, Error};

pub struct CsvReader {
    pub records: Vec<Record>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Record {
    pub item: String,
    pub allergens: String,
    pub serving_size: u32,
    pub calories: f64,	
    pub protein: f64,
    pub carbs: f64,
    pub sodium: f64,

}

impl CsvReader {
    pub async fn new(file_name: String) -> Result<Self> {
        // Create a CSV reader
        let Ok(file) = File::open(file_name).await
        else {
            return Err(Error::InvalidFile)
        };

        let mut rdr = csv_async::AsyncDeserializer::from_reader(file);

        let mut record_vec: Vec<Record> = Vec::new();
        let mut records = rdr.deserialize::<Record>();
        while let Some(record) = records.next().await {
            let Ok(record) = record
            else {
                return Err(Error::FileParseError)
            };

            record_vec.push(record);
        }
        
        Ok(Self {
            records: record_vec
        })
    }
}

impl CsvReader {
    // pub fn get_serving_sizes(&self) -> Vec<u32> {
    //     self.records.iter().map(|record| record.serving_size).collect::<Vec<u32>>()
    // }

    pub fn get_calories(&self) -> Vec<f64> {
        self.records.iter().map(|record| record.calories).collect::<Vec<f64>>()
    }

    pub fn get_protein(&self) -> Vec<f64> {
        self.records.iter().map(|record| record.protein).collect::<Vec<f64>>()
    }

    pub fn get_carbs(&self) -> Vec<f64> {
        self.records.iter().map(|record| record.carbs).collect::<Vec<f64>>()
    }

    pub fn get_sodium(&self) -> Vec<f64> {
        self.records.iter().map(|record| record.sodium).collect::<Vec<f64>>()
    }

    pub fn get_record(&self, index: usize) -> Option<&Record> {
        self.records.get(index)
    }
}
