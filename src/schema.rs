// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        name -> Text,
        friend_ids -> Array<Uuid>,
    }
}
