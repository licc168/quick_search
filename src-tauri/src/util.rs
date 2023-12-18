use anyhow::{anyhow, Result};
use polars::export::regex::Regex;

/**
 * 解析SQL 的文件路径  eg ：SELECT * FROM /work/file.csv  解析出来 /work/file.csv
 * @param  sql
 **/
pub fn parse_sql_to_path_name(sql: &str) -> Result<String> {
    let re = Regex::new(r#"(?i)from\s+([^\s;]+)"#)?;
    if let Some(captures) = re.captures(sql) {
        if let Some(table_name) = captures.get(1) {
            return Ok(table_name.as_str().to_string());
        }
    }
    return Err(anyhow!("您写的sql格式有问题"));
}
