use crate::{
    error::Result, 
    parse::sequence_provide::{self}, 
    sequnce_providers::OperationSequence
};

pub struct Sequence {}
impl OperationSequence for Sequence {

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "lin_com".to_owned(),
            description: "Linearna kombinacija zaporedij. Zaporedje f(n) = k1(n) * a(n) + k2(n) * b(n). Zaporedja: [a, b, k1, k2]".to_owned(),
            parameters: 0,
            sequences: 4
        }
    }

    fn apply(&self, _parameters: &[f64], sequences: &[f64]) -> Result<f64> {
        Ok(sequences[2] * sequences[0] + sequences[3] * sequences[1])
    }
}

#[test]
fn test() {
    let fs = Sequence {};
    assert_eq!(
        fs.apply(&[], &[1.,2.,3.,4.]),
        Ok(3.*1. + 4.*2.)
    );
}