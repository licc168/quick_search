#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::parse_element::ElementData;
use crate::parse_file::{CsvParser, Data, ExcelParser, JsonParser, Parser};
use crate::resp::Resp;
use polars::prelude::*;
use serde::{Deserialize, Serialize};

#[tauri::command]
fn file_data(sql: String) -> Resp<ElementData> {
    util::parse_sql_to_path_name(&sql)
        .map_err(|err| Resp::fail(1, "解析 SQL 错误"))
        .and_then(|file_path| {
            get_parser_for_file(&file_path)
                .ok_or_else(|| Resp::fail(1, "找不到对应的解析器"))
                .and_then(|parser| {
                    parser
                        .parse(&sql, &file_path)
                        .map_err(|err| Resp::fail(1, &err.to_string()))
                        .map(|data| parse_element::parse_data_to_element_table_data(data).into())
                })
        })
        .unwrap_or_else(|resp| resp)
}

// 根据文件类型获取相应的解析器
fn get_parser_for_file(file_path: &str) -> Option<Box<dyn Parser<Data>>> {
    if file_path.ends_with(".json") {
        Some(Box::new(JsonParser))
    } else if file_path.ends_with(".excel") {
        Some(Box::new(ExcelParser))
    } else if file_path.ends_with(".csv") {
        Some(Box::new(CsvParser))
    } else {
        None
    }
}
mod menu;
mod parse_element;
mod parse_file;
mod resp;
mod util;

fn main() {
    tauri::Builder::default()
        .setup(|_| Ok(()))
        .invoke_handler(tauri::generate_handler![file_data])
        .menu(menu::init()) // ✅ 将菜单添加到所有窗口
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
