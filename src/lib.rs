pub use triangle::Triangle;

mod triangle {
    #[derive(Debug, Clone, Copy)]
    pub struct Triangle {
        a: f64,
        b: f64,
        c: f64,
    }

    impl Triangle {
        pub fn new(a: f64, b: f64, c: f64) -> Result<Triangle, &'static str> {
            if a <= 0.0 || b <= 0.0 || c <= 0.0 {
                return Err("All sides must be positive and non-zero");
            } else if (a + b <= c) || (b + c <= a) || (c + a <= b) {
                return Err("Sum of any two sides should be greater than third");
            } else {
                Ok(Triangle { a, b, c })
            }
        }

        pub fn perimeter(&self) -> f64 {
            self.a + self.b + self.c
        }

        pub fn area(&self) -> f64 {
            let s: f64 = self.perimeter() / 2.0;
            (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
        }
    }
}
