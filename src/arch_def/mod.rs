use std::error::Error;

pub trait Architecture: Clone {
    type Instruction: Instruction<Self>;
    type OperandKind: OperandKind<Self>;
    type Symbol: Symbol<Self>;
}

pub trait Instruction<Arch: Architecture + ?Sized>: Clone + Copy
where
    Self: 'static,
{
    fn name(&self) -> &str;
    fn operands(&self) -> impl IntoIterator<Item = Arch::OperandKind>;
    fn emit(
        &self,
        operands: impl IntoIterator<Item = Arch::Symbol, IntoIter = impl ExactSizeIterator>,
    ) -> String;
    fn enumerate() -> impl IntoIterator<Item = &'static Self>;
}

pub trait OperandKind<Arch: Architecture + ?Sized> {
    type Operand;
    fn parse(&self, s: &str) -> Result<Self::Operand, Box<dyn Error>>;
}

pub trait Symbol<Arch: Architecture + ?Sized>: Sized + Clone {
    fn parse(symbol: &str) -> Result<Self, Box<dyn Error>>;
}
