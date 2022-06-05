use std::{marker::PhantomData};


#[derive(Debug, Default)]
pub struct Equation<IterMethod> {
    current: u32,
    _method: PhantomData<IterMethod>,
}

#[derive(Debug, Default)]
pub struct Liner;


#[derive(Debug, Default)]
pub struct Quadratic;

impl Iterator for Equation<Liner> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None
        }

        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u16::MAX as u32 {
            return None
        }

        Some(self.current * self.current)
    }
}

// It is very necessary to use generic data structures 
// to unify the same logic and 
// use the specific types of generic parameters to handle changing logic.
fn main() {
    let mut equation = Equation::<Liner>::default();
    println!("liner equation:");
    for _i in 0..3 {
        println!("{:?}", equation.next());
    }

    let equation1 = Equation::<Quadratic>::default();
    println!("quadratic equation:");
    for _i in 0..3 {
        println!("{:?}", equation.next());
    }
}