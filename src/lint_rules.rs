use crate::{Issue, Manifest};

pub type Rule = fn(m: &Manifest) -> Option<Issue>;

pub fn team_must_not_contain_spaces(manifest: &Manifest) -> Option<Issue> {
    if manifest.team.contains(" ") {
        Some(Issue::error("team must not contain spaces"))
    } else {
        None
    }
}

pub fn pipeline_should_be_lowercase(manifest: &Manifest) -> Option<Issue> {
    if manifest.pipeline != manifest.pipeline.to_lowercase() {
        Some(Issue::warning("pipeline should be lowercase"))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_manifest() -> Manifest {
        Manifest {
            pipeline: "".to_string(),
            team: "".to_string(),
            tasks: vec![],
        }
    }

    #[test]
    fn team_name() {
        let bad = Manifest {
            team: "team name".to_string(),
            ..empty_manifest()
        };
        assert!(matches!(team_must_not_contain_spaces(&bad), Some(Issue::Error(..))));

        let good = Manifest {
            team: "team-name".to_string(),
            ..empty_manifest()
        };
        assert_eq!(team_must_not_contain_spaces(&good), None);
    }

    #[test]
    fn pipeline_name() {
        let bad = Manifest {
            pipeline: "Pipeline Name".to_string(),
            ..empty_manifest()
        };
        assert!(matches!(pipeline_should_be_lowercase(&bad), Some(Issue::Warning(..))));

        let good = Manifest {
            pipeline: "pipeline name".to_string(),
            ..empty_manifest()
        };
        assert_eq!(pipeline_should_be_lowercase(&good), None);
    }
}
