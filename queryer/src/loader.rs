use std::io::Cursor;

use anyhow::Result;
use polars::prelude::*;

use crate::DataSet;

pub trait Load {
    type Error;
    fn load(self) -> Result<DataSet, Self::Error>;
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Loader {
    Csv(CsvLoader),
    Json(JsonLoader),
}

#[derive(Debug, Default)]
pub struct CsvLoader(pub(crate) String);
#[derive(Debug, Default)]
pub struct JsonLoader(pub(crate) String);

impl Loader {
    pub fn load(self) -> Result<DataSet> {
        match self {
            Loader::Csv(csv) => csv.load(),
            Loader::Json(json) => json.load(),
        }
    }
}

pub fn detect_content(data: String) -> Loader {
    // TODO 内容检测
    Loader::Csv(CsvLoader(data))
}

impl Load for CsvLoader {
    type Error = anyhow::Error;

    fn load(self) -> Result<DataSet, Self::Error> {
        let df = CsvReader::new(Cursor::new(self.0))
            .infer_schema(Some(16))
            .finish()?;
        Ok(DataSet(df))
    }
}

impl Load for JsonLoader {
    type Error = anyhow::Error;

    fn load(self) -> Result<DataSet, Self::Error> {
        let df = JsonReader::new(Cursor::new(self.0))
            .infer_schema(Some(16))
            .finish()?;
        Ok(DataSet(df))
    }
}
