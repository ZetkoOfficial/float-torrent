use crate::{
    error::{Error, Result}, 
    parse::sequence_provide::{self}, 
    sequnce_providers::SequenceProvider
};

/// Zaporedje, ki ga lahko predstavimo kot neka operacija nad istoležečimi členi drugih zaporedih
pub trait OperationSequence : Sync + Send {
    /// Ni potrebno skrbeti za napačno število parametrov ali zaporedij, garaniramo, da se ujema z get_info.
    fn apply(&self, parameters: &[f64], sequences: &[f64]) -> Result<f64>;
    fn get_info(&self) -> sequence_provide::SequenceInfo;
}

pub struct OperationSequenceProvider {
    base: Box<dyn OperationSequence>
}

impl OperationSequenceProvider {
    fn combine(&self, length: usize, parameters: &[f64], sequences: &[Vec<f64>]) -> Result<Vec<f64>> {

        let info = self.get_info();
        if info.sequences != sequences.len() || info.parameters != parameters.len() {
            return Err(Error::sequence_arithmetic_error(self.get_info(), "Število parametrov ali zaporedij je nepravilno."))
        } 

        if !sequences.iter().all(|s| s.len() == length) { 
            Err(Error::sequence_arithmetic_error(info, "Pridobljene dolžine zaporedij se ne ujemajo"))
        } else {
            let mut result = vec![];

            for i in 0..length {
                let collect: Vec<f64> = sequences.iter().map(|s| s[i]).collect();
                result.push(self.base.apply(parameters, &collect)?);
            }
            
            Ok(result)
        }

    }

    pub fn new(base: Box<dyn OperationSequence>) -> Self {
        OperationSequenceProvider {
            base
        }
    } 
}

impl SequenceProvider for OperationSequenceProvider {
    fn get_info(&self) -> sequence_provide::SequenceInfo { self.base.get_info().clone() }
    fn generate(&self, _range:sequence_provide::Range, parameters: &[f64],sequences: &[Vec<f64>]) -> Result<Vec<f64> > {
        Ok(self.combine(sequences[0].len(), parameters, sequences)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::sequnce_providers::{implementations::*, SequenceProvider};
    use super::OperationSequenceProvider;

    #[test]
    fn test() {
        let fs = OperationSequenceProvider::new(Box::new(max_seqs::Sequence {}));

        assert_eq!(
            fs.generate(crate::parse::sequence_provide::Range { from: 0, to: 4, step: 1 }, &[], &[vec![1.,4.,8.,13.], vec![3.,2.,13.,1.]]),
            Ok(vec![3.,4.,13.,13.])
        );
    }
}