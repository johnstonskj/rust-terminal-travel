error_chain! {
    errors {
        InvalidVersionNumber(v: String) {
            description("Invalid version number")
            display("Invalid version number: '{}'", v)
        }
    }

    foreign_links {
        Api(crate::amadeus::error::Error);
        Env(::std::env::VarError);
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Yaml(::serde_yaml::Error);
        Json(::serde_json::Error);
    }
}
