pub(crate) struct Config {
    pub(crate) author_name: String,
    pub(crate) author_email: String,
}

impl Config {
    pub(crate) fn load() -> Config {
        Self {
            author_name: String::from("georgesmyr"),
            author_email: String::from("70004539+georgesmyr@users.noreply.github.com"),
        }
    }
}
