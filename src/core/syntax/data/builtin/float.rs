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
pub enum TD_Float {
    Value(f64),
    Minus(Box<TD_Float>),
    Factor(Vec<(ARITH_FACTOR_SIGN,TD_Float)>),
    Add(Vec<(ARITH_ADD_SIGN,TD_Float)>),
    Reference(VariableReference)
}

impl TD_Float {

    pub fn get_occuring_variables(&self) -> HashSet<usize> {
        match self {
            TD_Float::Value(_) => {
                return HashSet::new();
            },
            TD_Float::Minus(sub_float) => {
                return (*sub_float).get_occuring_variables();
            },
            TD_Float::Factor(sub_floats) => {
                let mut occs : HashSet<usize> = HashSet::new();
                for (_,sub_float) in sub_floats {
                    occs.extend(sub_float.get_occuring_variables());
                }
                return occs;
            },
            TD_Float::Add(sub_floats) => {
                let mut occs : HashSet<usize> = HashSet::new();
                for (_,sub_float) in sub_floats {
                    occs.extend(sub_float.get_occuring_variables());
                }
                return occs;
            },
            TD_Float::Reference(var_ref) => {
                return var_ref.get_occuring_variables();
            }
        }
    }
}

impl VarMapAble for TD_Float {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> TD_Float {
        match self {
            TD_Float::Value( val ) => {
                return TD_Float::Value( *val );
            },
            TD_Float::Minus( boxed_inv ) => {
                let mapped = boxed_inv.apply_variable_mapping(mapping);
                return TD_Float::Minus( Box::new(mapped) );
            },
            TD_Float::Factor( factors ) => {
                let mut mapped_factors : Vec<(ARITH_FACTOR_SIGN,TD_Float)> = Vec::new();
                for (sign,number) in factors {
                    mapped_factors.push( (sign.clone(), number.apply_variable_mapping(mapping)) );
                }
                return TD_Float::Factor( mapped_factors );
            },
            TD_Float::Add( adds ) => {
                let mut mapped_adds : Vec<(ARITH_ADD_SIGN,TD_Float)> = Vec::new();
                for (sign,number) in adds {
                    mapped_adds.push( (sign.clone(), number.apply_variable_mapping(mapping)) );
                }
                return TD_Float::Add( mapped_adds );
            },
            TD_Float::Reference( var_ref ) => {
                return TD_Float::Reference( var_ref.apply_variable_mapping(mapping) );
            }
        }
    }
}


