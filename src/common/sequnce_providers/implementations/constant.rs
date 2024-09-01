use crate::common::{
    parse::sequence_provide::{self, SequenceInfo}, 
    error::Result,
    sequnce_providers::SequenceProvider
};

pub struct Provider {}
impl SequenceProvider for Provider {

    fn get_info(&self) -> SequenceInfo {
        SequenceInfo {
            name: "const".to_owned(),
            description: "Konstantno zaporedje s členi enakimi a. Parametri: [a]". to_owned(),
            parameters: 1,
            sequences: 0
        }
    }

    fn generate(&self, range: sequence_provide::Range, parameters: &[f64], _sequences: &[Vec<f64>]) -> Result<Vec<f64>> {
        let mut result = vec![];
        
        let mut i = range.from; 
        while i < range.to {
            result.push(parameters[0]); i += range.step;
        }
        Ok(result) 
    }
}