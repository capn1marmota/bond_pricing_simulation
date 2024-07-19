use std::cmp::Ordering;

pub struct YieldCurve {
    maturities: Vec<f64>,
    rates: Vec<f64>,
}

impl YieldCurve {
    pub fn new(maturities: Vec<f64>, rates: Vec<f64>) -> Result<Self, &'static str> {
        if maturities.len() != rates.len() {
            return Err("Maturities and rates must have the same length");
        }
        Ok(YieldCurve { maturities, rates })
    }

    pub fn get_rate(&self, maturity: f64) -> f64 {
        match self
            .maturities
            .binary_search_by(|&x| x.partial_cmp(&maturity).unwrap_or(Ordering::Equal))
        {
            Ok(index) => self.rates[index],
            Err(index) if index == 0 => self.rates[0],
            Err(index) if index == self.maturities.len() => *self.rates.last().unwrap(),
            Err(index) => {
                let (x1, x2) = (self.maturities[index - 1], self.maturities[index]);
                let (y1, y2) = (self.rates[index - 1], self.rates[index]);
                // Linear interpolation
                y1 + (y2 - y1) * (maturity - x1) / (x2 - x1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_yield_curve() {
        let maturities: Vec<f64> = vec![1.0, 2.0, 3.0, 5.0, 10.0];
        let rates: Vec<f64> = vec![0.01, 0.02, 0.025, 0.03, 0.035];
        let curve: YieldCurve = YieldCurve::new(maturities, rates).unwrap();

        assert_relative_eq!(curve.get_rate(1.0), 0.01, epsilon = 1e-10);
        assert_relative_eq!(curve.get_rate(2.0), 0.02, epsilon = 1e-10);
        assert_relative_eq!(curve.get_rate(1.5), 0.015, epsilon = 1e-10);
        assert_relative_eq!(curve.get_rate(4.0), 0.0275, epsilon = 1e-10);
        assert_relative_eq!(curve.get_rate(0.5), 0.01, epsilon = 1e-10);
        assert_relative_eq!(curve.get_rate(15.0), 0.035, epsilon = 1e-10);
    }

    #[test]
    #[should_panic(expected = "Maturities and rates must have the same length")]
    fn test_yield_curve_mismatched_lengths() {
        let maturities: Vec<f64> = vec![1.0, 2.0, 3.0];
        let rates: Vec<f64> = vec![0.01, 0.02];
        YieldCurve::new(maturities, rates).unwrap();
    }
}
