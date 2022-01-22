use sqlparser::dialect::Dialect;

#[derive(Debug, Default)]
pub struct JoDialect;

// 创建自己的 sql 方言。JoDialect 支持 identifier 可以是简单的url
impl Dialect for JoDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}

/// 测试辅助函数
pub fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
        url
    );

    sql
}

#[cfg(test)]
mod tests {
    use sqlparser::parser::Parser;
    use tracing::Level;

    use super::*;

    #[test]
    fn it_works() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        assert!(Parser::parse_sql(&JoDialect::default(), &example_sql()).is_ok());
    }
}
