pub mod sequence_provide {
    use serde::Deserialize;
    use crate::error::error::Result;

    #[derive(Deserialize, Debug)]
    struct Range {
        from:   u64,
        to:     u64,
        step:   u64
    }

    #[derive(Deserialize, Debug)]
    struct SequenceParameter {
        name: String, 
        parameters: Vec<f64>,
        sequences: Vec<SequenceParameter>
    }

    #[derive(Deserialize, Debug)]
    pub struct Request {
        range: Range,
        parameters: Vec<f64>,
        sequences: Vec<SequenceParameter>
    }

    pub fn parse_request(data: &[u8]) -> Result<Request> { Ok(serde_json::from_slice(&data)?) }
}
