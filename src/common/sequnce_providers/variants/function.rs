use std::vec;

use crate::{
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

#[cfg(test)]
mod tests {
    use crate::sequnce_providers::{implementations::*, SequenceProvider};
    use super::FunctionSequenceProvider;

    #[test]
    fn test() {
        let fs = FunctionSequenceProvider::new(Box::new(arithmetic::Sequence {}));

        assert_eq!(
            fs.generate(crate::parse::sequence_provide::Range { from: 4, to: 10, step: 3 }, &[0., 2.], &[]),
            Ok(vec![8., 14.])
        );
        assert_eq!(
            fs.generate(crate::parse::sequence_provide::Range { from: 0, to: 10, step: 1 }, &[0., 2.], &[]),
            Ok(vec![0., 2., 4., 6., 8., 10., 12., 14., 16., 18.])
        );
    }
}