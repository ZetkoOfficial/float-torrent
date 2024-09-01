use nalgebra::{DMatrix, DVector};

use crate::common::{
    error::Result, 
    parse::sequence_provide::{self, SequenceInfo}, 
    sequnce_providers::SequenceProvider
};

/// [ Homogena linearna rekurzija poljubne stopnje ]
pub struct Provider { degree: usize }

impl Provider {
    pub fn new(degree: usize) -> Self { Self {degree} }
}

impl SequenceProvider for Provider {

    fn get_info(&self) -> SequenceInfo {
        let (degree, degree_m) = (self.degree, self.degree-1);
        let description = {
            if degree == 1 {
                "Homogeno linearno rekurzivno zaporedje, oblike: f(n) = a_1 f(n-1) \
                z robnimi pogoji f(0) = f0, Paramteri: [a1,f0]".to_owned()
            } else {
                format!("Homogeno linearno rekurzivno zaporedje, oblike: f(n) = a_1 f(n-1) + ... + a_{degree} f(n-{degree}) \
                z robnimi pogoji f(0) = f0, ..., f({degree_m}) = f_{degree_m}. Paramteri: [a1,...,a{degree},f0,...,f_{degree_m}]").to_owned()
            }
        };

        SequenceInfo {
            name: "linear_rec_h".to_owned(), 
            description,
            parameters: 2*self.degree,
            sequences: 0
        }
    }

    fn generate(&self,range:sequence_provide::Range, parameters: &[f64], _: &[Vec<f64>]) -> Result<Vec<f64> > {
        // ustvarimo matriko rekurzivne zveze, da M.(f(n),f(n+1),...f(n+k-1)) = (f(n+1),f(n+2),...f(n+k))
        let mat: DMatrix<f64> = DMatrix::from_fn(self.degree, self.degree, |i,j| {
            if i < self.degree-1 {
                if j == (i+1) { 1. } else { 0. } 
            } else { parameters[self.degree-j-1] }
        });
        
        // ustvarimo vektor z začetnimi pogoji in ga zamaknemo na range.from
        let inital = DVector::from_column_slice(&parameters[self.degree..]);
        let mut inital: DVector<f64> = mat.pow(range.from.try_into()?) * inital;

        // izračunamo matriko za step naenkrat
        let mat = mat.pow(range.step.try_into()?);

        // izračunamo vse člene, ki jih zahteva request
        let mut result = vec![];        
        let mut i = range.from; 
        while i < range.to {
            result.push(inital[0]);
            inital = (&mat) * inital;
            i += range.step;
        }
        Ok(result)
    }
}