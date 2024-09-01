use std::{cmp::Reverse, collections::BinaryHeap};

use async_trait::async_trait;
use nalgebra::{DMatrix, DVector};
use tokio::sync::RwLock;

use crate::common::{
    error::{Error, Result}, 
    parse::sequence_provide::{self, SequenceInfo}, 
    sequnce_providers::{ProviderManager, SequenceProvider},
    OrderableF64
};

// ---------- implementacije nekaj posebnih primerov ----------

/// [ Homogena linearna rekurzija poljubne stopnje ]
pub struct LinearRecursionHSequenceProvider { degree: usize }
impl LinearRecursionHSequenceProvider {
    pub fn new(degree: usize) -> Self { Self {degree} }
}
impl SequenceProvider for LinearRecursionHSequenceProvider {

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

pub struct ConstantSequenceProvider {}
impl SequenceProvider for ConstantSequenceProvider {

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

pub struct DropSequenceProvider {}
#[async_trait]
impl SequenceProvider for DropSequenceProvider {

    fn get_info(&self) -> SequenceInfo {
        SequenceInfo {
            name: "drop".to_owned(),
            description: "Zaporedje enako s(n+a). Zaporedja: [s], Parametri: [a]".to_owned(),
            parameters: 1,
            sequences: 1
        }
    }
    
    fn generate(&self,_:sequence_provide::Range,_: &[f64],_: &[Vec<f64>]) -> Result<Vec<f64> > {
        panic!("Unreachable code!")
    }

    // modificiramo Range v requestu in prepošljemo naprej
    async fn provide(&self, request: sequence_provide::Request, manager: &RwLock<ProviderManager>) -> Result<Vec<f64>> {
        let drop_count = request.parameters[0].trunc();
        if drop_count < 0. { return Err(Error::sequence_arithmetic_error("Parameter mora biti pozitiven")); }
        let drop_count = drop_count as u64;

        let sequence = request.sequences[0].clone();

        let ammended = sequence_provide::Request {
            range: sequence_provide::Range { from: request.range.from+drop_count, to: request.range.to+drop_count, step: request.range.step },
            parameters: sequence.parameters.clone(),
            sequences: sequence.sequences.clone()
        };

        manager.read().await.find(&sequence.get_info())?.provide(ammended,&manager).await
    }
}

pub struct PowerModSequenceProvider {}
impl SequenceProvider for PowerModSequenceProvider {

    fn get_info(&self) -> SequenceInfo {
        SequenceInfo {
            name: "power_mod".to_owned(),
            description: "f(n) = (a^(p^n) mod M), Parametri [a >= 0, p >= 0, M > 0] so truncirani u32.". to_owned(),
            parameters: 3,
            sequences: 0
        }
    }

    fn generate(&self, range: sequence_provide::Range, parameters: &[f64], _sequences: &[Vec<f64>]) -> Result<Vec<f64>> {
        let mut result = vec![];
        if parameters[0] < 0. || parameters[1] < 0. || parameters[2] <= 0.  { return Err(Error::sequence_arithmetic_error("Neveljavni parametri")); }        
        let (mut a, p, m): (u32, u32, u32) = (parameters[0].trunc() as u32, parameters[1].trunc() as u32, parameters[2].trunc() as u32);

        let mut i = 0; 
        while i < range.from {
            a = a.wrapping_pow(p) % m;
            i += 1;
        }

        let p = p.pow(range.step.try_into()?);
        while i < range.to {
            result.push(a);
            a = a.wrapping_pow(p) % m;
            i += range.step;
        }

        Ok(result.iter().map(|v| *v as f64).collect())
    }
}

pub struct PEulerSequenceProvider { building_blocks: BinaryHeap<Reverse<OrderableF64>> }
impl PEulerSequenceProvider {
    fn get_bit(num: u8, i: u8) -> bool {
        (num >> i) % 2 == 1
    }

    pub fn new() -> Self {
        let fermat= [3.,5.,17.,257.,65537.];
        let mut building_blocks = BinaryHeap::new();

        // dodamo produkte fermata
        for i in 0..(2 as u8).pow(fermat.len() as u32) {
            let mut val = 1.;
            for j in 0..fermat.len() {
                if Self::get_bit(i, j as u8) {
                    val *= fermat[j];
                }
            }
            building_blocks.push(Reverse(OrderableF64(val)));
        }

        Self { building_blocks }
    } 
}
impl SequenceProvider for PEulerSequenceProvider {
    fn get_info(&self) -> sequence_provide::SequenceInfo {
        SequenceInfo {
            name: "p_euler".to_owned(),
            description: "(Najverjetneje) V dokaj naključnem vrstnem redu števila M za katere je phi(M) potenca praštevila. phi(n) je Eulerjeva funkcija fi.".to_owned(),
            parameters: 0,
            sequences: 0
        }
    }

    // Uporabimo min-heap da v O(n log m) časa izračunamo člene od 1..n, kjer je m število gradnikov
    fn generate(&self,range:sequence_provide::Range, _: &[f64], _: &[Vec<f64>]) -> Result<Vec<f64> > {
        let mut heap = self.building_blocks.clone(); // dokaj majheno
        let mut result = vec![];
        
        let mut i = 0;  // rabimo iti čez vse, in sproti pobiramo iskane
        while i < range.to {
            let top = heap.pop()
                .ok_or(Error::sequence_arithmetic_error("Prazen min-heap, nepričakovana napaka."))?.0.0;
            
            // če je v našem range-u, ga dodamo, potem pa v heap dodamo dvakratnik
            if range.from <= i && (i-range.from) % range.step == 0 { result.push(top); } 
            heap.push(Reverse(OrderableF64(2. * top)));

            i += 1;
        }

        Ok(result) 
    }
}