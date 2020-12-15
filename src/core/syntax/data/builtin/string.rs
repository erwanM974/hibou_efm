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

use std::collections::{HashSet,HashMap};
use std::collections::btree_map::BTreeMap;

use crate::core::context::execution::ExecutionContext;
use crate::core::error::HibouCoreError;
use crate::core::syntax::action::ValueOrNewFresh;
use crate::core::syntax::data::var_ref::VariableReference;
use crate::core::semantics::varmap::VarMapAble;
use crate::core::syntax::data::generic::TD_Generic;

#[derive(Clone, PartialEq, Eq, Hash,Debug)]
pub enum TD_String {
    Value(String),
    //Concat(Vec<TD_String>),
    Reference(VariableReference)
}

impl TD_String {

    pub fn get_occuring_variables(&self) -> HashSet<usize> {
        match self {
            TD_String::Value(_) => {
                return HashSet::new();
            },
            TD_String::Reference(var_ref) => {
                return var_ref.get_occuring_variables();
            }
        }
    }

}

impl VarMapAble for TD_String {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> TD_String {
        match self {
            TD_String::Value( val ) => {
                return TD_String::Value( val.clone() );
            },/*
            TD_String::Concat( primary, adds ) => {
                let up_primary = primary.apply_mapping(mapping);
                let mut up_adds = Vec::new();
                for add_str in adds {
                    up_adds.push( add_str.apply_mapping(mapping) );
                }
                return TD_String::Concat( Box::new(up_primary), up_adds );
            },*/
            TD_String::Reference( var_ref ) => {
                return TD_String::Reference( var_ref.apply_variable_mapping(mapping) );
            }
        }
    }

}

