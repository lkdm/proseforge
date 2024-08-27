use derive_more::derive::From;

#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
pub enum Theme {
    System,
    Light,
    Dark,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
pub struct Config {
    pub theme: Theme,
}
