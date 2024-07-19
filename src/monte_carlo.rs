use crate::bond_pricer::BondPricer;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

pub struct MonteCarlo<'a> {
    pricer: &'a BondPricer<'a>,
    num_simulations: u32,
}

impl<'a> MonteCarlo<'a> {
    pub fn new(pricer: &'a BondPricer, num_simulations: u32) -> Self {
        MonteCarlo {
            pricer,
            num_simulations,
        }
    }

    pub fn simulate_price(
        &self,
        face_value: f64,
        coupon_rate: f64,
        maturity: f64,
        payments_per_year: u32,
    ) -> f64 {
        let normal = Normal::new(0.0, 0.01).unwrap();

        (0..self.num_simulations)
            .into_par_iter()
            .map(|_| {
                let random_shock = normal.sample(&mut thread_rng());
                let adjusted_coupon_rate = coupon_rate + random_shock;
                self.pricer.price(
                    face_value,
                    adjusted_coupon_rate,
                    maturity,
                    payments_per_year,
                )
            })
            .sum::<f64>()
            / self.num_simulations as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::yield_curve::YieldCurve;
    use approx::assert_relative_eq;

    #[test]
    fn test_monte_carlo() {
        let maturities = vec![1.0, 2.0, 3.0, 5.0, 10.0];
        let rates = vec![0.01, 0.02, 0.025, 0.03, 0.035];
        let curve = YieldCurve::new(maturities, rates).unwrap();
        let pricer = BondPricer::new(&curve);
        let mc = MonteCarlo::new(&pricer, 10000);

        let theoretical_price = pricer.price(100.0, 0.05, 5.0, 2);
        let simulated_price = mc.simulate_price(100.0, 0.05, 5.0, 2);

        assert_relative_eq!(simulated_price, theoretical_price, epsilon = 0.5);
    }
}
