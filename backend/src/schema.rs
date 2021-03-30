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
    orangeconnex_packages (tracking_number) {
        tracking_number -> Text,
        recieved -> Bool,
    }
}

table! {
    orangeconnex_traces (id) {
        id -> Text,
        tracking_number -> Text,
        description -> Text,
        city -> Nullable<Text>,
        country -> Text,
        time_recorded -> Text,
        time_zone -> Text,
        ts -> Integer,
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
    tokens (id) {
        id -> Text,
        sub -> Text,
        aud -> Text,
        iss -> Text,
        iat -> Text,
        exp -> Nullable<Integer>,
        valid -> Nullable<Integer>,
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

joinable!(orangeconnex_traces -> orangeconnex_packages (tracking_number));
joinable!(switches -> members (member_id));

allow_tables_to_appear_in_same_query!(
    blogposts,
    members,
    orangeconnex_packages,
    orangeconnex_traces,
    switches,
    tokens,
    weather,
    webmentions,
);
