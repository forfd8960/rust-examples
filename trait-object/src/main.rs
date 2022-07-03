
use std::{error::Error, process::Command};

type BoxedError = Box<dyn Error + Send + Sync>;

pub trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

pub struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl<'a, 'b> Shell<'a, 'b> {
    pub fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Self { cmd, args } 
    }
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
        let output = Command::new(self.cmd).args(self.args).output()?;
        Ok(output.status.code())
    }
}

pub fn execute_generics(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

pub fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

pub fn execute_boxed_trait_object(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

fn main() {
    let cmd = Shell::new("ls", &[]);
    let result = cmd.run().unwrap();
    println!("result: {:?}", result);

    let result1 = execute_generics(&cmd);
    println!("result: {:?}", result1);

    let result2 = execute_trait_object(&cmd);
    println!("result: {:?}", result2);

    let boxed_cmd = Box::new(cmd);
    let result3 = execute_boxed_trait_object(boxed_cmd);
    println!("result: {:?}", result3);
}
