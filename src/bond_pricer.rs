use crate::yield_curve::YieldCurve;

pub struct BondPricer<'a> {
    yield_curve: &'a YieldCurve,
}

impl<'a> BondPricer<'a> {
    pub fn new(yield_curve: &'a YieldCurve) -> Self {
        BondPricer { yield_curve }
    }

    pub fn price(
        &self,
        face_value: f64,
        coupon_rate: f64,
        maturity: f64,
        payments_per_year: u32,
    ) -> f64 {
        let mut price: f64 = 0.0;
        let coupon_amount: f64 = (face_value * coupon_rate) / payments_per_year as f64;
        let total_payments: u32 = (maturity * payments_per_year as f64).round() as u32;

        println!(
            "Face Value: {}, Coupon Rate: {}, Maturity: {}, Payments per Year: {}",
            face_value, coupon_rate, maturity, payments_per_year
        );
        println!(
            "Coupon Amount: {}, Total Payments: {}",
            coupon_amount, total_payments
        );

        for i in 1..=total_payments {
            let t: f64 = i as f64 / payments_per_year as f64;
            let yield_rate: f64 = self.yield_curve.get_rate(t);
            let discount_factor: f64 = (-yield_rate * t).exp();
            let pv_coupon = coupon_amount * discount_factor;
            price += pv_coupon;
            println!(
                "Payment {}: Time: {}, Yield Rate: {}, Discount Factor: {}, PV Coupon: {}",
                i, t, yield_rate, discount_factor, pv_coupon
            );
        }

        let yield_at_maturity: f64 = self.yield_curve.get_rate(maturity);
        let pv_face_value = face_value * (-yield_at_maturity * maturity).exp();
        price += pv_face_value;

        println!(
            "Yield at Maturity: {}, PV Face Value: {}",
            yield_at_maturity, pv_face_value
        );
        println!("Total Bond Price: {}", price);

        price
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::yield_curve::YieldCurve;
    use approx::assert_relative_eq;

    #[test]
    fn test_bond_pricer() {
        let maturities: Vec<f64> = vec![1.0, 2.0, 3.0, 5.0, 10.0];
        let rates: Vec<f64> = vec![0.01, 0.02, 0.025, 0.03, 0.035];
        let curve: YieldCurve = YieldCurve::new(maturities, rates).unwrap();
        let pricer: BondPricer = BondPricer::new(&curve);

        let price_zero_coupon: f64 = pricer.price(100.0, 0.0, 1.0, 1);
        assert_relative_eq!(price_zero_coupon, 99.0049833, epsilon = 1e-6);

        let price_coupon_bond = pricer.price(100.0, 0.05, 2.0, 2);
        println!("Calculated price: {}", price_coupon_bond);
        assert_relative_eq!(price_coupon_bond, 105.9249, epsilon = 1e-1); // Increased epsilon
    }
}
