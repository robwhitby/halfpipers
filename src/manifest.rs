use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Task {
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub command: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Manifest {
    pub pipeline: String,
    pub team: String,
    pub tasks: Vec<Task>,
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
        - type: task-type
          name: task-name
          command: task-command
        - type: task-type2
        ";

        let expected = Manifest {
            pipeline: "my-pipe".to_string(),
            team: "my-team".to_string(),
            tasks: vec![
                Task {
                    typ: "task-type".to_string(),
                    name: "task-name".to_string(),
                    command: "task-command".to_string(),
                },
                Task {
                    typ: "task-type2".to_string(),
                    name: "".to_string(),
                    command: "".to_string(),
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
