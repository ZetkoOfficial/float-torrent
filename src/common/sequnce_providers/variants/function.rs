use std::vec;

use crate::common::{
    error::Result, 
    parse::sequence_provide::{self}, 
    sequnce_providers::SequenceProvider
};

/// Zaporedje, ki ga lahko predstavimo kot funkcija nekih parametrov in indeksa
pub trait FunctionSequence : Sync + Send {
    fn evaluate(&self, parameters: &[f64], n: u64) -> Result<f64>;
    fn get_info(&self) -> sequence_provide::SequenceInfo;
}

pub struct FunctionSequenceProvider {
    base: Box<dyn FunctionSequence>
}

impl FunctionSequenceProvider {
    pub fn new(base: Box<dyn FunctionSequence>) -> Self {
        FunctionSequenceProvider { base }
    }
}

impl SequenceProvider for FunctionSequenceProvider {
    fn get_info(&self) -> sequence_provide::SequenceInfo {
        self.base.get_info()
    }

    fn generate(&self,range:sequence_provide::Range,parameters: &[f64], _: &[Vec<f64>]) -> Result<Vec<f64> > {
        let mut result = vec![];        

        let mut i = range.from; 
        while i < range.to {
            result.push(self.base.evaluate(parameters, i)?); i += range.step;
        }

        Ok(result)
    }
}