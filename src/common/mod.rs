use std::cmp::Ordering;

pub mod sequnce_providers;
pub mod error;
pub mod http;
pub mod parse;

/* ---------- Vsebuje splošne pomagalne funkcije ---------- */

#[derive(PartialOrd, Clone)]
pub struct OrderableF64(f64);

impl PartialEq for OrderableF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}
impl Eq for OrderableF64 {}
impl Ord for OrderableF64 { fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.total_cmp(&other.0) } }