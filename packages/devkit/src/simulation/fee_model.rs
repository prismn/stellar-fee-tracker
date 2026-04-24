/// Models for simulating Stellar transaction fee behaviour.
pub struct FeeModel;

impl FeeModel {
    /// Generates `count` baseline fee values (in stroops) at the Stellar minimum (100).
    pub fn baseline(count: usize) -> Vec<f64> {
        vec![100.0; count]
    }
}
