use crate::parse_element::ElementData;
use anyhow::{anyhow, Result};
use polars::frame::DataFrame;
use polars::prelude::{IntoLazy, JsonReader, LazyCsvReader, LazyFileListReader, SerReader};
use polars::sql::SQLContext;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
// 解析 polars  df 的数据结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub(crate) columns: Vec<Column>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    pub(crate) name: String,
    datatype: String,
    bit_settings: Option<String>,
    pub(crate) values: Vec<ColumnValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ColumnValue {
    Int64(i64),
    Utf8(String),
    Null,
}

// 解析器
pub trait Parser<T> {
    fn parse(&self, sql: &str, file_path: &str) -> Result<Data>;

    /**
     * 将 dataFrame 解析出 Object
     * @param  data_frame  polars 解析出来的数据
     **/
    fn parse_df_object(&self, data_frame: DataFrame) -> Result<Data> {
        let json = serde_json::to_string(&data_frame)?;
        let data: Data = serde_json::from_str(&json)?;
        Ok(data)
    }
}
// Excel 解析器
pub struct ExcelParser;

impl Parser<Data> for ExcelParser {
    fn parse(&self, sql: &str, file_path: &str) -> Result<Data> {
        println!("excel:{:?}", file_path);
        let mut ctx = SQLContext::new();
        let read_csv = LazyCsvReader::new(file_path)
            .has_header(true)
            .with_ignore_errors(true)
            .with_infer_schema_length(Some(1000))
            .finish()?;
        let sql = sql.replace(file_path, "df");
        ctx.register("df", read_csv);
        let df = ctx.execute(&sql)?.collect()?;
        self.parse_df_object(df)
    }
}

// CSV 解析器
pub struct CsvParser;

impl Parser<Data> for CsvParser {
    fn parse(&self, sql: &str, file_path: &str) -> Result<Data> {
        println!("csv_path:{:?}", file_path);
        let mut ctx = SQLContext::new();
        let read_csv = LazyCsvReader::new(file_path)
            .has_header(true)
            .with_ignore_errors(true)
            .with_infer_schema_length(Some(1000))
            .finish()?;
        let sql = sql.replace(file_path, "df");
        ctx.register("df", read_csv);
        let df = ctx.execute(&sql)?.collect()?;
        self.parse_df_object(df)
    }
}

// JSON 解析器
pub struct JsonParser;

impl Parser<Data> for JsonParser {
    fn parse(&self, sql: &str, file_path: &str) -> Result<Data> {
        let mut ctx = SQLContext::new();
        println!("json_path:{:?}", file_path);
        // 读取文件内容并存储为 String
        let file_content = fs::read_to_string(file_path)?;
        let res = JsonReader::new(Cursor::new(file_content)).finish()?;
        let sql = sql.replace(file_path, "df");
        ctx.register("df", res.lazy());
        let df = ctx.execute(&sql)?.collect()?;
        self.parse_df_object(df)
    }
}
