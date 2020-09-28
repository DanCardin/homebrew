table! {
    batch (id) {
        id -> Integer,
        beer_id -> Integer,
        date -> Date,
    }
}

table! {
    batch_ingredient (id) {
        id -> Integer,
        batch_id -> Integer,
        ingredient -> Text,
        amount -> Double,
        unit -> Text,
        time -> Nullable<Text>,
    }
}

table! {
    batch_measurement (batch_id, name) {
        batch_id -> Integer,
        name -> Text,
        value -> Double,
        time -> Nullable<Timestamp>,
    }
}

table! {
    batch_note (id) {
        id -> Integer,
        beer_id -> Integer,
        value -> Text,
    }
}

table! {
    beer (id) {
        id -> Integer,
        name -> Text,
        style -> Text,
    }
}

allow_tables_to_appear_in_same_query!(batch, batch_ingredient, batch_measurement, batch_note, beer,);
