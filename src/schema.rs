table! {
    members (id) {
        id -> Integer,
        cmene -> Text,
        picurl -> Text,
    }
}

table! {
    switches (id) {
        id -> Text,
        who -> Text,
        started_at -> Text,
        ended_at -> Nullable<Text>,
        duration -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    members,
    switches,
);
