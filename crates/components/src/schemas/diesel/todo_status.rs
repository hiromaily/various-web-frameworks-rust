use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
use std::error::Error as StdError;
use strum_macros::{Display, EnumString, IntoStaticStr};

use crate::schemas::diesel::schema::sql_types::TodoStatus as TodoStatusType;

// #[derive(
//     AsExpression,
//     Clone,
//     Copy,
//     Debug,
//     Display,
//     EnumString,
//     Eq,
//     FromSqlRow,
//     Hash,
//     IntoStaticStr,
//     PartialEq,
// )]
#[derive(AsExpression, Debug, Display, EnumString, FromSqlRow, IntoStaticStr)]
//#[diesel(sql_type = Text)]
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

impl<DB> serialize::ToSql<Text, DB> for TodoStatus
where
    DB: Backend,
    str: serialize::ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, DB>) -> serialize::Result {
        let name: &'static str = self.into();

        name.to_sql(out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for TodoStatus
where
    DB: Backend,
    *const str: deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<TodoStatus>()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
        })
    }
}
