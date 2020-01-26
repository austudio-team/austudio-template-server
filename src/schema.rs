table! {
    versions (id) {
        id -> Integer,
        major_version -> Integer,
        minor_version -> Integer,
        build_number -> Integer,
        description -> Text,
        branch_name -> Text,
        created_time -> Timestamp,
    }
}
