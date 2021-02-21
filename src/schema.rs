table! {
    papers (paper_id) {
        paper_id -> Integer,
        paper_title -> Varchar,
        paper_author -> Nullable<Varchar>,
        paper_year -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
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

joinable!(papers -> users (user_id));

allow_tables_to_appear_in_same_query!(papers, users,);
