table! {
    papers (paper_id) {
        paper_id -> Integer,
        paper_title -> Varchar,
        paper_author -> Varchar,
        paper_year -> Integer,
        user_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users (user_id) {
        user_id -> Integer,
        user_name -> Varchar,
        user_email -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    papers,
    users,
);
