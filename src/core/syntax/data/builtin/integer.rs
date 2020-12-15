/********************************************************************************
 * Copyright (c) 2020 Erwan Mahe (github.com/erwanM974)
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
 * which is available at https://www.apache.org/licenses/LICENSE-2.0.
 *
 * SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
 ********************************************************************************/

use std::collections::{HashMap,HashSet};
use std::collections::btree_map::BTreeMap;

use crate::core::context::execution::ExecutionContext;
use crate::core::error::HibouCoreError;
use crate::core::syntax::action::ValueOrNewFresh;
use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::var_ref::VariableReference;
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::semantics::varmap::VarMapAble;
use crate::core::syntax::data::builtin::number::*;


#[derive(Clone, PartialEq, Debug)]
pub enum TD_Integer {
    Value(i64),
    Minus(Box<TD_Integer>),
    Factor(Vec<(ARITH_FACTOR_SIGN,TD_Integer)>),
    Add(Vec<(ARITH_ADD_SIGN,TD_Integer)>),
    Reference(VariableReference)
}

impl TD_Integer {

    pub fn get_occuring_variables(&self) -> HashSet<usize> {
        match self {
            TD_Integer::Value(_) => {
                return HashSet::new();
            },
            TD_Integer::Minus(sub_int) => {
                return (*sub_int).get_occuring_variables();
            },
            TD_Integer::Factor(sub_ints) => {
                let mut occs : HashSet<usize> = HashSet::new();
                for (_,sub_int) in sub_ints {
                    occs.extend(sub_int.get_occuring_variables());
                }
                return occs;
            },
            TD_Integer::Add(sub_ints) => {
                let mut occs : HashSet<usize> = HashSet::new();
                for (_,sub_int) in sub_ints {
                    occs.extend(sub_int.get_occuring_variables());
                }
                return occs;
            },
            TD_Integer::Reference(var_ref) => {
                return var_ref.get_occuring_variables();
            }
        }
    }
}

impl VarMapAble for TD_Integer {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> TD_Integer {
        match self {
            TD_Integer::Value( val ) => {
                return TD_Integer::Value( *val );
            },
            TD_Integer::Minus( boxed_inv ) => {
                let mapped = boxed_inv.apply_variable_mapping(mapping);
                return TD_Integer::Minus( Box::new(mapped) );
            },
            TD_Integer::Factor( factors ) => {
                let mut mapped_factors : Vec<(ARITH_FACTOR_SIGN,TD_Integer)> = Vec::new();
                for (sign,number) in factors {
                    mapped_factors.push( (sign.clone(), number.apply_variable_mapping(mapping)) );
                }
                return TD_Integer::Factor( mapped_factors  );
            },
            TD_Integer::Add( adds ) => {
                let mut mapped_adds : Vec<(ARITH_ADD_SIGN,TD_Integer)> = Vec::new();
                for (sign,number) in adds {
                    mapped_adds.push( (sign.clone(), number.apply_variable_mapping(mapping)) );
                }
                return TD_Integer::Add( mapped_adds );
            },
            TD_Integer::Reference( var_ref ) => {
                return TD_Integer::Reference( var_ref.apply_variable_mapping(mapping) );
            }
        }
    }
}

