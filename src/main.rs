#[macro_use]
extern crate diesel;

#[allow(unused_imports)]
use diesel::{ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl};

table! {
    contests(contest_id) {
        contest_id -> Integer,
        title -> Varchar,
    }
}

table! {
    contest_users(contest_id, user_id) {
        contest_id -> Integer,
        user_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(contests, contest_users);

/*
// This section compiles just fine, but I wonder if I can avoid `joinable!` and `.nullable()`, see
// below.

// I would prefer to explicitly provide `ON` clause and skip the need to use `joinable!`
joinable!(contest_users -> contests (contest_id));

pub fn join_implicitly_works_fine() -> diesel::dsl::InnerJoin<contest_users::table, contests::table>
{
    contest_users::dsl::contest_users.inner_join(contests::table)
}

pub fn join_explicitly_with_nullable_works_fine(
) -> diesel::dsl::InnerJoin<contest_users::table, contests::table> {
    contest_users::dsl::contest_users.inner_join(
        contests::table.on(contest_users::contest_id
            .nullable()
            .eq(contests::contest_id.nullable())),
    )
    // Why do I need `.nullable()`?
}
*/

// error[E0308]: mismatched types
//   --> src/main.rs:30:5
//    |
// 30 | /     contest_users::dsl::contest_users.inner_join(
// 31 | |         contests::table
// 32 | |             .on(contest_users::contest_id.eq(contests::contest_id)),
// 33 | |     )
//    | |_____^ expected struct `diesel::expression::nullable::Nullable`, found struct `contest_users::columns::contest_id`
//    |
//    = note: expected struct `diesel::query_builder::SelectStatement<JoinOn<_, diesel::expression::operators::Eq<diesel::expression::nullable::Nullable<contest_users::columns::contest_id>, diesel::expression::nullable::Nullable<contests::columns::contest_id>>>>`
//               found struct `diesel::query_builder::SelectStatement<JoinOn<_, diesel::expression::operators::Eq<contest_users::columns::contest_id, contests::columns::contest_id>>>`
pub fn join_explicitly_does_not_compile() -> diesel::dsl::InnerJoin<contest_users::table, contests::table> {
    contest_users::dsl::contest_users.inner_join(
        contests::table
            .on(contest_users::contest_id.eq(contests::contest_id)),
    )
}

/*
// This is my final goal (I can make it working by adding `joinable!` and using implicit join ON
// clause or with explicit join ON clause with extra `.nullable()`)
//
pub fn join_and_filter() -> diesel::dsl::Filter<
    diesel::dsl::Select<
        diesel::dsl::InnerJoin<contest_users::table, contests::table>,
        (contests::columns::contest_id,),
    >,
    diesel::expression::operators::Eq<
        contest_users::columns::user_id,
        diesel::expression::bound::Bound<diesel::sql_types::Integer, i32>,
    >,
> {
    contest_users::dsl::contest_users
        .inner_join(
            contests::table.on(contest_users::contest_id.eq(contests::contest_id)),
        )
        .filter(contest_users::user_id.eq(42))
        .select((contests::columns::contest_id,))
}
*/

fn main() {}
