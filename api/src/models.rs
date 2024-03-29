use serde;
use statrs;
use statrs::distribution::Univariate;

// Parameter objects
#[derive(serde::Deserialize)]
pub struct Binomial {
    spot: f64,
    strike: f64,
    vol: f64,
    time: f64,
    rate: f64,
    div: f64,
    steps: i32,
}

#[derive(serde::Deserialize)]
pub struct BlackScholes {
    pub spot: f64,
    pub strike: f64,
    pub vol: f64,
    pub time: f64,
    pub rate: f64,
    pub div: f64,
}

impl Binomial {
    pub fn call_price(&self) -> f64 {
        let dt = self.time / self.steps as f64;
        let up = (self.vol * dt.powf(0.5)).exp();
        let p0 = (up * (-self.div * dt).exp() - (-self.rate * dt).exp()) / (up.powi(2) - 1.0);
        let p1 = (-self.rate * dt).exp() - p0;
        let mut price = vec![];
        for i in 0..=self.steps {
            price.push(self.spot * up.powi(2 * i - self.steps) - self.strike);
            price[i as usize] = f64::max(price[i as usize], 0.0);
        }
        for j in (0..self.steps).rev() {
            for i in 0..=j {
                price[i as usize] = p0 * price[(i + 1) as usize] + p1 * price[i as usize];
                let exercise = -self.strike + self.spot * up.powi(2 * i - j);
                price[i as usize] = f64::max(price[i as usize], exercise);
            }
        }

        price[0]
    }

    pub fn put_price(&self) -> f64 {
        let dt = self.time / self.steps as f64;
        let up = (self.vol * dt.powf(0.5)).exp();
        let p0 = (up * (-self.div * dt).exp() - (-self.rate * dt).exp()) / (up.powi(2) - 1.0);
        let p1 = (-self.rate * dt).exp() - p0;
        let mut price = vec![];
        for i in 0..=self.steps {
            price.push(-self.spot * up.powi(2 * i - self.steps) + self.strike);
            price[i as usize] = f64::max(price[i as usize], 0.0);
        }
        for j in (0..self.steps).rev() {
            for i in 0..=j {
                price[i as usize] = p0 * price[(i + 1) as usize] + p1 * price[i as usize];
                let exercise = self.strike - self.spot * up.powi(2 * i - j);
                price[i as usize] = f64::max(price[i as usize], exercise);
            }
        }

        price[0]
    }
}

impl BlackScholes {
    fn d1(&self) -> f64 {
        (f64::ln(self.spot / self.strike)
            + (self.rate - self.div + self.vol.powi(2) / 2.0) * self.time)
            / (self.vol * self.time.powf(0.5))
    }

    fn d2(&self) -> f64 {
        self.d1() - self.vol * self.time.powf(0.5)
    }

    pub fn call_price(&self) -> f64 {
        let normal = statrs::distribution::Normal::new(0.0, 1.0).unwrap();
        self.spot * normal.cdf(self.d1()) * (-self.div * self.time).exp()
            - self.strike * normal.cdf(self.d2()) * (-self.rate * self.time).exp()
    }

    pub fn put_price(&self) -> f64 {
        let normal = statrs::distribution::Normal::new(0.0, 1.0).unwrap();
        -self.spot * normal.cdf(-self.d1()) * (-self.div * self.time).exp()
            + self.strike * normal.cdf(-self.d2()) * (-self.rate * self.time).exp()
    }
}
