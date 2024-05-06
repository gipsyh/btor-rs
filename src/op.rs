use logic_form::fol::Sort;

#[derive(Debug, Clone, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum UniOpType {
    Not,
    Inc,
    Dec,
    Neg,
    Redand,
    Redor,
    Redxor,
}

#[derive(Debug, Clone)]
pub struct UniOp {
    pub sort: Sort,
    pub ty: UniOpType,
    pub a: u32,
}

#[derive(Debug, Clone, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum BiOpType {
    Iff,
    Implies,
    Eq,
    Neq,
    Sgt,
    Ugt,
    Sgte,
    Ugte,
    Slt,
    Ult,
    Slte,
    Ulte,
    And,
    Nand,
    Nor,
    Or,
    Xnor,
    Xor,
    Rol,
    Ror,
    Sll,
    Sra,
    Srl,
    Add,
    Mul,
    Sdiv,
    Udiv,
    Smod,
    Srem,
    Urem,
    Sub,
    Saddo,
    Uaddo,
    Sdivo,
    Udivo,
    Smulo,
    Umulo,
    Ssubo,
    Usubo,
    Concat,
    Read,
}

#[derive(Debug, Clone)]
pub struct BiOp {
    pub sort: Sort,
    pub ty: BiOpType,
    pub a: u32,
    pub b: u32,
}

#[derive(Debug, Clone, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum TriOpType {
    Ite,
    Write,
}

#[derive(Debug, Clone)]
pub struct TriOp {
    pub sort: Sort,
    pub ty: TriOpType,
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

// /// Extension operation type.
// #[derive(Debug, Clone, strum::EnumString, strum::Display)]
// #[strum(serialize_all = "lowercase")]
// pub enum ExtOpType {
//     Sext,
//     Uext,
// }

// /// Extension operation node.
// #[derive(Debug, Clone)]
// pub struct ExtOp {
//     /// Result sort.
//     pub sid: Sid,
//     /// Operation type.
//     pub ty: ExtOpType,
//     /// Operand right-side node id.
//     pub a: Rnid,
//     /// Length of extension.
//     pub length: u32,
// }

// /// Slice operation node.
// #[derive(Debug, Clone)]
// pub struct SliceOp {
//     /// Result sort.
//     pub sid: Sid,
//     /// Operand right-side node id.
//     pub a: Rnid,
//     /// Upper bit of slice (inclusive).
//     ///
//     /// Guaranteed to be greater or equal to lower bit after parsing.
//     pub upper_bit: u32,
//     /// Lower bit of slice (inclusive).
//     ///
//     /// Guaranteed to be lesser or equal to upper bit after parsing.
//     pub lower_bit: u32,
// }
