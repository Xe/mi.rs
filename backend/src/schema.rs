table! {
    blogposts (url) {
        url -> Text,
        title -> Text,
    }
}

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
        member_id -> Integer,
        started_at -> Timestamp,
        ended_at -> Nullable<Timestamp>,
    }
}

table! {
    weather (ts) {
        ts -> Timestamp,
        region -> Text,
        body -> Binary,
    }
}

table! {
    webmentions (id) {
        id -> Text,
        source_url -> Text,
        target_url -> Text,
        title -> Nullable<Text>,
    }
}

joinable!(switches -> members (member_id));

allow_tables_to_appear_in_same_query!(
    blogposts,
    members,
    switches,
    weather,
    webmentions,
);
