pub mod sequence_provide {
    use serde::{Deserialize, Serialize};
    use crate::error::error::Result;

    use super::parse_helper::Sendable;

    #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
    pub struct Range {
        pub from:   u64,
        pub to:     u64,
        pub step:   u64
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SequenceParameter {
        pub name: String, 
        pub parameters: Vec<f64>,
        pub sequences: Vec<SequenceParameter>
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SequenceInfo {
        pub name: String,
        pub description: String,
        pub parameters: usize,
        pub sequences: usize
    }
    impl Sendable for SequenceInfo {}

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request {
        pub range: Range,
        pub parameters: Vec<f64>,
        pub sequences: Vec<SequenceParameter>
    }
    impl Sendable for Request {}

    impl Request {
        pub fn get_info(&self, name: &str) -> SequenceInfo {
            SequenceInfo {
                name: name.to_owned(),
                parameters: self.parameters.len(),
                sequences: self.sequences.len(),
                description: "".to_owned()
            }
        } 
    }

    impl SequenceParameter {
        pub fn get_info(&self) -> SequenceInfo {
            SequenceInfo {
                name: self.name.to_owned(),
                parameters: self.parameters.len(),
                sequences: self.sequences.len(),
                description: "".to_owned()
            }
        } 
    }

    pub fn parse_request(data: &[u8]) -> Result<Request> { Ok(serde_json::from_slice(&data)?) }
}

pub mod parse_helper {
    use crate::error::error::Result;

    pub trait Sendable : serde::Serialize {
        fn as_sendable(&self) -> Result<Vec<u8>> {
            Ok(serde_json::to_vec_pretty(&self)?)
        }
    }
}