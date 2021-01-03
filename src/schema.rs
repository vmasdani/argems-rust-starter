table! {
    todos (id) {
        id -> Nullable<Integer>,
        name -> Text,
        completed -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
