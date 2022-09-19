use crate::Manifest;

pub struct Linter {}

pub struct Results {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Results {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Linter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn lint(&self, manifest: &Manifest) -> Results {
        let mut errors = vec![];
        let mut warnings = vec![];

        if manifest.pipeline != manifest.pipeline.to_lowercase() {
            warnings.push("pipeline name should be lowercase".to_string());
        }

        if manifest.team != manifest.team.to_lowercase() {
            errors.push("team name must be lowercase".to_string());
        }

        Results { errors, warnings }
    }
}
