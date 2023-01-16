pub trait AscentProgram {
    type Output;

    fn process(&self) -> Self::Output;
}
