use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    img: f64,
}

impl Complex {
    pub fn new(real: f64, img: f64) -> Self {
        Self { real, img }
    }
}

/*
----- 所以泛型 trait 可以让我们在需要的时候，对同一种类型的同一个 trait，有多个实现。 ----
*/

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.real + rhs.real, self.img + rhs.img)
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.real + rhs.real, self.img + rhs.img)
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        Complex::new(self.real + rhs, self.img)
    }
}

fn main() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);
    println!("c1+c2: {:?}", &c1 + &c2);
    println!("c1+6.66: {:?}", &c1 + 6.66);
    println!("c1+c2: {:?}", c1 + c2);
}
