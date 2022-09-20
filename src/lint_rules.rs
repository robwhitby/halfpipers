use crate::Manifest;

pub type Rule = fn(m: &Manifest) -> Option<Issue>;

#[derive(Debug, PartialEq)]
pub enum Issue {
    Warning(String),
    Error(String),
}

pub fn team_must_not_contain_spaces(manifest: &Manifest) -> Option<Issue> {
    if manifest.team.contains(" ") {
        Some(Issue::Error("team must not contain spaces".to_string()))
    } else {
        None
    }
}

pub fn pipeline_should_be_lowercase(manifest: &Manifest) -> Option<Issue> {
    if manifest.pipeline == manifest.pipeline.to_lowercase() {
        Some(Issue::Warning("pipeline should be lowercase".to_string()))
    } else {
        None
    }
}
