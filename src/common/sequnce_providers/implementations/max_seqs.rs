use crate::{
    error::Result, 
    parse::sequence_provide::{self}, 
    sequnce_providers::OperationSequence
};

pub struct Sequence {}
impl OperationSequence for Sequence {
    
    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo { 
            name: "max".to_owned(), 
            description: "Izračuna maksimum dveh zaporedij po členih. f(n) = max(a(n), b(n)), Zaporedja: [a, b]".to_owned(), 
            parameters: 0, 
            sequences: 2 
        }  
    }

    fn apply(&self, _parameters: &[f64], sequences: &[f64]) -> Result<f64> {
        Ok(sequences[0].max(sequences[1]))
    }

}

#[test]
fn test() {
    let fs = Sequence {};
    assert_eq!(
        fs.apply(&[], &[13., 83.]),
        Ok(83.)
    );
}