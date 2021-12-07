table! {
    swap_config (conf) {
        conf -> Text,
        value -> Text,
    }
}

table! {
    swaps (swap_uuid) {
        swap_uuid -> Text,
        pair -> Text,
        side -> Text,
        book -> Text,
        quantity -> Text,
        price -> Text,
        time_satmp -> Text,
        fee -> Text,
        swapi_fee -> Text,
        fee_currency -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    swap_config,
    swaps,
);
