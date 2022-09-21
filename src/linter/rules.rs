use crate::{Env, Issue, Manifest};

pub type Rule = fn(&Env, &Manifest) -> Option<Issue>;

pub fn team_must_not_contain_spaces(_env: &Env, manifest: &Manifest) -> Option<Issue> {
    if manifest.team.contains(' ') {
        Some(Issue::error("team must not contain spaces"))
    } else {
        None
    }
}

pub fn pipeline_should_be_lowercase(_env: &Env, manifest: &Manifest) -> Option<Issue> {
    if manifest.pipeline != manifest.pipeline.to_lowercase() {
        Some(Issue::warning("pipeline should be lowercase"))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn team_name() {
        let bad = Manifest {
            team: "team name".to_string(),
            ..Manifest::new()
        };
        assert!(matches!(team_must_not_contain_spaces(&Env::new(), &bad), Some(Issue::Error(..))));

        let good = Manifest {
            team: "team-name".to_string(),
            ..Manifest::new()
        };
        assert_eq!(team_must_not_contain_spaces(&Env::new(), &good), None);
    }

    #[test]
    fn pipeline_name() {
        let bad = Manifest {
            pipeline: "Pipeline Name".to_string(),
            ..Manifest::new()
        };
        assert!(matches!(pipeline_should_be_lowercase(&Env::new(), &bad), Some(Issue::Warning(..))));

        let good = Manifest {
            pipeline: "pipeline name".to_string(),
            ..Manifest::new()
        };
        assert_eq!(pipeline_should_be_lowercase(&Env::new(), &good), None);
    }
}
