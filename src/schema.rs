table! {
    owners (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Text,
    }
}

table! {
    vehicle_brands (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

table! {
    vehicle_models (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    owners,
    vehicle_brands,
    vehicle_models,
);
