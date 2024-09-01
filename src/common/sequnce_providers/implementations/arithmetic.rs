use crate::{
    error::{Error, Result}, 
    parse::sequence_provide::{self}, 
    sequnce_providers::FunctionSequence
};

pub struct Sequence {}
impl FunctionSequence for Sequence {

    fn get_info(&self) -> sequence_provide::SequenceInfo {
        sequence_provide::SequenceInfo {
            name: "arithmetic".to_owned(),
            description: "Aritmetično zaporedje a(n) = a0 + d * n. Parametri: [a0, d]".to_owned(),
            parameters: 2,
            sequences: 0
        }
    }

    fn evaluate(&self, parameters: &[f64], n: u64) -> Result<f64> {
        if parameters.len() != 2 {
            Err(Error::sequence_arithmetic_error(self.get_info(),"Potrebna sta parametra `a0` in `d`."))
        } else {
            Ok(parameters[0] + parameters[1] * (n as f64))
        }
    }
}

#[test]
fn test() {
    let fs = Sequence {};
    assert_eq!(fs.evaluate(&[-4.,8.], 3), Ok(20.));
    assert_eq!(fs.evaluate(&[4.,8.], 3), Ok(28.));
    assert_eq!(fs.evaluate(&[1.,-2.], 13), Ok(-25.));
}