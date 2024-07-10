use rusqlite::{types::ToSqlOutput, ToSql};


#[derive(serde::Serialize,serde::Deserialize)]
pub(crate) enum Cycle{
    WEEKLY,
    MONTHLY,
    BIWEEKLY,
}

impl Cycle {
    fn as_str(&self) -> &'static str {
        match self {
            Cycle::WEEKLY => "WEEKLY",
            Cycle::MONTHLY => "MONTHLY",
            Cycle::BIWEEKLY => "BIWEEKLY"
        }
    }
}

impl ToSql for Cycle{
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}