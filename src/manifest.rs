use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Manifest {
    pub pipeline: String,
    pub team: String,
    pub tasks: Vec<Task>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum Task {
    Run {
        #[serde(default)]
        name: String,
        command: String,
    },
    DockerCompose {
        #[serde(default)]
        name: String,
        #[serde(default)]
        compose_file: String,
    },
}

impl Manifest {
    pub fn from_string(raw_manifest: &String) -> Result<Manifest, serde_yaml::Error> {
        let manifest: Manifest = serde_yaml::from_str(&raw_manifest)?;
        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy() {
        let input = "
        pipeline: my-pipe
        team: my-team
        tasks: 
        - type: run
          name: build
          command: ./build
        - type: run
          command: ./test
        - type: docker-compose
        ";

        let expected = Manifest {
            pipeline: "my-pipe".to_string(),
            team: "my-team".to_string(),
            tasks: vec![
                Task::Run {
                    name: "build".to_string(),
                    command: "./build".to_string(),
                },
                Task::Run {
                    name: "".to_string(),
                    command: "./test".to_string(),
                },
                Task::DockerCompose {
                    name: "".to_string(),
                    compose_file: "".to_string(),
                },
            ],
        };

        assert_eq!(expected, Manifest::from_string(&input.to_string()).unwrap());
    }

    #[test]
    fn sad_yaml() {
        let input = String::from("some rubbish");

        let err = Manifest::from_string(&input).unwrap_err();
        assert!(err.to_string().contains("invalid type"));
    }

    #[test]
    fn missing_field() {
        let input = "
        pipeline: my-pipe
        ";

        let err = Manifest::from_string(&input.to_string()).unwrap_err();
        assert!(err.to_string().contains("missing field `team`"));
    }
}
