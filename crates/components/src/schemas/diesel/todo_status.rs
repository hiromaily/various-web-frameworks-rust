use crate::schemas::diesel::schema::sql_types::TodoStatus as TodoStatusType;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use std::io::Write;
use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(AsExpression, Debug, Display, EnumString, FromSqlRow, IntoStaticStr)]
#[diesel(sql_type = TodoStatusType)]
pub enum TodoStatus {
    #[strum(serialize = "pending")]
    Pending,
    #[strum(serialize = "doing")]
    Doing,
    #[strum(serialize = "canceled")]
    Canceled,
    #[strum(serialize = "done")]
    Done,
}

// impl<DB> serialize::ToSql<Text, DB> for TodoStatus
// where
//     DB: Backend,
//     str: serialize::ToSql<Text, DB>,
// {
//     fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, DB>) -> serialize::Result {
//         let name: &'static str = self.into();

//         name.to_sql(out)
//     }
// }
impl ToSql<TodoStatusType, Pg> for TodoStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TodoStatus::Pending => out.write_all(b"pending")?,
            TodoStatus::Doing => out.write_all(b"doing")?,
            TodoStatus::Canceled => out.write_all(b"canceled")?,
            TodoStatus::Done => out.write_all(b"done")?,
        }
        Ok(IsNull::No)
    }
}

// impl<DB> deserialize::FromSql<Text, DB> for TodoStatus
// where
//     DB: Backend,
//     *const str: deserialize::FromSql<diesel::sql_types::Text, DB>,
// {
//     fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
//         deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
//             string
//                 .parse::<TodoStatus>()
//                 .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
//         })
//     }
// }
impl FromSql<TodoStatusType, Pg> for TodoStatus {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"pending" => Ok(TodoStatus::Pending),
            b"doing" => Ok(TodoStatus::Doing),
            b"canceled" => Ok(TodoStatus::Canceled),
            b"done" => Ok(TodoStatus::Done),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
