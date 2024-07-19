use bond_pricing_simulation::{BondPricer, MonteCarlo, YieldCurve};

fn main() {
    let maturities = vec![1.0, 2.0, 3.0, 5.0, 10.0];
    let rates = vec![0.01, 0.02, 0.025, 0.03, 0.035];
    let curve = YieldCurve::new(maturities, rates).unwrap();
    let pricer = BondPricer::new(&curve);
    let mc = MonteCarlo::new(&pricer, 100000);

    let face_value = 100.0;
    let coupon_rate = 0.05;
    let maturity = 5.0;
    let payments_per_year = 2;

    let theoretical_price = pricer.price(face_value, coupon_rate, maturity, payments_per_year);
    let simulated_price = mc.simulate_price(face_value, coupon_rate, maturity, payments_per_year);

    println!("Theoretical price: {:.4}", theoretical_price);
    println!("Simulated price: {:.4}", simulated_price);
}
