pub struct Repo {
    pub name: String,
    pub path: String,
}

impl Repo {
    pub fn new(path: &mut String) -> (bool, Repo) {
        (
            !path.trim().is_empty(),
            Repo {
                name: "".to_string(),
                path: path.trim().to_string(),
            },
        )
    }
}
