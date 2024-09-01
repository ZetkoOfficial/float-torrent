use std::{cmp::Reverse, collections::BinaryHeap};

use crate::common::{
    error::{Error, Result}, 
    parse::sequence_provide::{self, SequenceInfo}, 
    sequnce_providers::SequenceProvider, OrderableF64
};

pub struct Provider { building_blocks: BinaryHeap<Reverse<OrderableF64>> }
impl Provider {
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
impl SequenceProvider for Provider {
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