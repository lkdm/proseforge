use std::fs;

pub(crate) struct Migration {
    pub up: String,
    pub down: Option<String>,
}

impl Migration {
    fn new(up: &str, down: Option<&str>) -> Self {
        Migration {
            up: up.to_owned(),
            down: down.map(|s| s.to_owned()),
        }
    }
}

pub fn migration_file(up: &str, down: Option<&str>) -> Migration {
    let up_content = fs::read_to_string(up).expect("Failed to read the up migration file");
    let down_content =
        down.map(|d| fs::read_to_string(d).expect("Failed to read the down migration file"));
    Migration::new(&up_content, down_content.as_deref())
}

pub(crate) fn get_migrations() -> [Migration; 1] {
    [migration_file("./migrations/migration_001.sql", None)]
}
