use crate::pb::Spec;

mod photon;
pub use photon::Photon;

pub trait Engine {
    fn apply(&mut self, specs: &[Spec]);
    // currently only supports JEPG
    fn generate(self, quality: u8) -> Vec<u8>;
}

pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
