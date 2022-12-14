use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Manifest {
    pub pipeline: String,
    pub team: String,
    pub tasks: Vec<Task>,
}

impl Manifest {
    #[cfg(test)]
    pub fn new() -> Manifest {
        Manifest {
            pipeline: "".to_string(),
            team: "".to_string(),
            tasks: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Task {
    Run(Run),
    DockerCompose(DockerCompose),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Run {
    script: String,
    #[serde(flatten)]
    common: CommonTask,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DockerCompose {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    compose_file: Option<String>,
    #[serde(flatten)]
    common: CommonTask,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct CommonTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    retries: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timeout: Option<String>,
}

impl Manifest {
    pub fn from_yaml(raw_manifest: &str) -> Result<Manifest, serde_yaml::Error> {
        let manifest: Manifest = serde_yaml::from_str(raw_manifest)?;
        Ok(manifest)
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        let yaml = serde_yaml::to_string(self)?;
        Ok(yaml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_level() {
        let input = "
        pipeline: p
        team: t
        tasks: []
        ";

        let manifest = Manifest::from_yaml(&input.to_string()).unwrap();
        assert_eq!(manifest.pipeline, "p");
        assert_eq!(manifest.team, "t");
    }

    #[test]
    fn missing_field() {
        let input = "
        pipeline: p
        ";

        let err = Manifest::from_yaml(&input.to_string()).unwrap_err();
        assert!(err.to_string().contains("missing field `team`"));
    }

    #[test]
    fn all_task_types() {
        let input = "
        pipeline: p
        team: t
        tasks: 
        - type: run
          script: s
        - type: docker-compose
        ";

        let tasks = Manifest::from_yaml(&input.to_string()).unwrap().tasks;
        assert!(matches!(tasks.get(0).unwrap(), Task::Run { .. }));
        assert!(matches!(tasks.get(1).unwrap(), Task::DockerCompose { .. }));
    }

    fn get_task(task_input: &str) -> Task {
        let input = format!(
            "
        pipeline: p
        team: t
        tasks:
        {}",
            task_input
        );

        Manifest::from_yaml(&input.to_string()).unwrap().tasks.first().unwrap().clone()
    }

    #[test]
    fn task_run() {
        let input = "
        - type: run
          script: s
        ";

        let expected = Run {
            script: "s".to_string(),
            common: Default::default(),
        };

        assert_eq!(get_task(&input), Task::Run(expected))
    }

    #[test]
    fn task_docker_compose() {
        let input = "
        - type: docker-compose
          compose_file: cf
        ";

        let expected = DockerCompose {
            compose_file: Some("cf".to_string()),
            common: Default::default(),
        };

        assert_eq!(get_task(&input), Task::DockerCompose(expected))
    }

    #[test]
    fn task_common() {
        let input = "
        - type: run
          name: n
          script: s
          retries: 1
          timeout: t
        ";

        let expected = Run {
            script: "s".to_string(),
            common: CommonTask {
                name: Some("n".to_string()),
                retries: Some(1),
                timeout: Some("t".to_string()),
            },
        };

        assert_eq!(get_task(&input), Task::Run(expected))
    }

    #[test]
    fn invalid_yaml() {
        let input = String::from("some rubbish");

        let err = Manifest::from_yaml(&input).unwrap_err();
        assert!(err.to_string().contains("invalid type"));
    }
}
