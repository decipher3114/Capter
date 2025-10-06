use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrgranizeMode {
    #[default]
    Flat,
    ByYear,
    ByYearAndMonth,
}

impl OrgranizeMode {
    pub const ALL: [OrgranizeMode; 3] = [
        OrgranizeMode::Flat,
        OrgranizeMode::ByYear,
        OrgranizeMode::ByYearAndMonth,
    ];
}

impl std::fmt::Display for OrgranizeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrgranizeMode::Flat => write!(f, "Flat"),
            OrgranizeMode::ByYear => write!(f, "By Year"),
            OrgranizeMode::ByYearAndMonth => write!(f, "By Year And Month"),
        }
    }
}
