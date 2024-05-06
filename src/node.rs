use std::collections::BTreeMap;

use logic_form::fol::Sort;

use crate::{
    op::{BiOp, BiOpType, TriOp, TriOpType, UniOp, UniOpType},
    parse_id,
};

#[derive(Debug)]
pub enum Node {
    Const(Const),
    UniOp(UniOp),
    BiOp(BiOp),
    TriOp(TriOp),
    State(Sort),
    Init(Init),
    Next(Next),
    Bad(u32),
    // Source(Source),
    // Drain(Drain),
    // ExtOp(ExtOp),
    // SliceOp(SliceOp),
    // Justice(Justice),
}

#[derive(Debug, Clone)]
pub struct Const {
    pub ty: ConstType,
    pub sort: Sort,
    pub value: String,
}

impl Const {
    pub fn parse<'a>(
        ty: ConstType,
        split: &mut impl Iterator<Item = &'a str>,
        sorts: &BTreeMap<u32, Sort>,
    ) -> Node {
        let sort = sorts.get(&parse_id(split)).unwrap().clone();
        let value = String::from(split.next().unwrap());
        Node::Const(Const { ty, sort, value })
    }
}

#[derive(Debug, Clone, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum ConstType {
    Const = 2,
    Constd = 10,
    Consth = 16,
}

#[derive(Debug, Clone)]
pub struct State {
    pub sort: Sort,
}

#[derive(Debug, Clone)]
pub struct Init {
    pub sort: Sort,
    pub state: u32,
    pub value: u32,
}

#[derive(Debug, Clone)]
pub struct Next {
    pub sort: Sort,
    pub state: u32,
    pub value: u32,
}

impl Node {
    pub(crate) fn parse<'a>(
        second: &str,
        mut split: impl Iterator<Item = &'a str>,
        sorts: &BTreeMap<u32, Sort>,
    ) -> Option<Node> {
        if let Ok(ty) = ConstType::try_from(second) {
            return Some(Const::parse(ty, &mut split, sorts));
        }

        // if let Ok(ty) = SourceType::try_from(second) {
        //     let sid = parse_sid(&mut split)?;
        //     return Ok(Some(Node::Source(Source { ty, sid })));
        // }

        // // drain
        // if let Ok(ty) = DrainType::try_from(second) {
        //     let rnid = parse_rnid(&mut split)?;
        //     return Ok(Some(Node::Drain(Drain { ty, rnid })));
        // }

        // // temporal
        // if let Ok(ty) = TemporalType::try_from(second) {
        //     let sid = parse_sid(&mut split)?;
        //     let state = parse_nid(&mut split)?;
        //     let value = parse_rnid(&mut split)?;
        //     return Ok(Some(Node::Temporal(Temporal {
        //         ty,
        //         sid,
        //         state,
        //         value,
        //     })));
        // }

        if let Ok(ty) = UniOpType::try_from(second) {
            let sort = sorts.get(&parse_id(&mut split)).unwrap().clone();
            let a = parse_id(&mut split);
            return Some(Node::UniOp(UniOp { sort, ty, a }));
        }

        if let Ok(ty) = BiOpType::try_from(second) {
            let sort = sorts.get(&parse_id(&mut split)).unwrap().clone();
            let a = parse_id(&mut split);
            let b = parse_id(&mut split);
            return Some(Node::BiOp(BiOp { sort, ty, a, b }));
        }

        if let Ok(ty) = TriOpType::try_from(second) {
            let sort = sorts.get(&parse_id(&mut split)).unwrap().clone();
            let a = parse_id(&mut split);
            let b = parse_id(&mut split);
            let c = parse_id(&mut split);
            return Some(Node::TriOp(TriOp { sort, ty, a, b, c }));
        }

        // // extension
        // if let Ok(ty) = ExtOpType::try_from(second) {
        //     let sid = parse_sid(&mut split)?;
        //     let a = parse_rnid(&mut split)?;
        //     let length = parse_u32(&mut split)?;
        //     return Ok(Some(Node::ExtOp(ExtOp { sid, ty, a, length })));
        // }

        // // other node types
        match second {
            // "slice" => {
            //     let sid = parse_sid(&mut split)?;
            //     let a = parse_rnid(&mut split)?;
            //     let upper_bit = parse_u32(&mut split)?;
            //     let lower_bit = parse_u32(&mut split)?;

            //     if upper_bit < lower_bit {
            //         return Err(LineError::InvalidSlice);
            //     }
            //     Node::SliceOp(SliceOp {
            //         sid,
            //         a,
            //         upper_bit,
            //         lower_bit,
            //     })
            // }
            "state" => {
                let sort = sorts.get(&parse_id(&mut split)).unwrap().clone();
                Some(Node::State(sort))
            }
            "init" => {
                let sort = sorts.get(&parse_id(&mut split)).unwrap().clone();
                let state = parse_id(&mut split);
                let value = parse_id(&mut split);
                Some(Node::Init(Init { sort, state, value }))
            }
            "next" => {
                let sort = sorts.get(&parse_id(&mut split)).unwrap().clone();
                let state = parse_id(&mut split);
                let value = parse_id(&mut split);
                Some(Node::Next(Next { sort, state, value }))
            }
            "bad" => {
                let bad = parse_id(&mut split);
                Some(Node::Bad(bad))
            }
            // "justice" => {
            //     let num = parse_u32(&mut split)?;
            //     let mut vec = Vec::new();
            //     for _ in 0..num {
            //         vec.push(parse_rnid(&mut split)?);
            //     }
            //     Node::Justice(Justice { nids: vec })
            // }
            _ => None,
        }
    }
}
