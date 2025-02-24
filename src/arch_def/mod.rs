use std::error::Error;
use crate::assembler::passes::parse::PlausibleOperator;

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
        operands: impl IntoIterator<Item = <Arch::OperandKind as OperandKind<Arch>>::Operand>,
    ) -> impl IntoIterator<Item = u8>;
    fn enumerate() -> impl IntoIterator<Item = &'static Self>;
}

pub trait OperandKind<Arch: Architecture + ?Sized> {
    type Operand: Clone;
    fn parse(&self, plausible_operator: PlausibleOperator<Arch>) -> Result<Self::Operand, Box<dyn Error>>;
    fn matches(&self, plausible_operator: &PlausibleOperator<Arch>) -> bool {
        self.parse(plausible_operator.clone()).is_ok()
    }
}

pub trait Symbol<Arch: Architecture + ?Sized>: Sized + Clone {
    fn parse(symbol: &str) -> Result<Self, Box<dyn Error>>;
}
