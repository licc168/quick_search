use crate::parse_file::{ColumnValue, Data};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Header {
    field: String,
    caption: String,
    width: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementData {
    // 表头
    table_head: Vec<Header>,
    // 映射的表数据
    table_data: Vec<HashMap<String, String>>,
}
/**
* 解析 df_data数据为 element 表格的 data
 **/
pub fn parse_data_to_element_table_data(data: Data) -> Result<ElementData> {
    // 进行转换
    let mut table_head: Vec<Header> = Vec::new();
    let mut table_data: Vec<HashMap<String, String>> = Vec::new();
    let columns = data.columns.clone(); // 克隆一份，而不是移动
    for column in &columns {
        table_head.push(Header {
            field: column.name.clone(),
            caption: column.name.clone(),
            width: 100,
        });
    }
    let columns = data.columns;
    for i in 0..columns[0].values.len() {
        let mut entry = HashMap::new();
        for (j, column) in columns.iter().enumerate() {
            if let Some(value) = column.values.get(i) {
                let value_str = match value {
                    ColumnValue::Int64(num) => num.to_string(),
                    ColumnValue::Utf8(text) => text.clone(),
                    ColumnValue::Null => "".to_string(),
                };
                entry.insert(table_head[j].field.clone(), value_str);
            }
        }
        table_data.push(entry);
    }
    return Ok(ElementData {
        table_head,
        table_data,
    });
}
