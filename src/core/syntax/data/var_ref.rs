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
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::builtin::float::*;
use crate::core::syntax::data::builtin::integer::*;
use crate::core::syntax::data::builtin::string::*;
use crate::core::syntax::data::builtin::bool::*;
use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::semantics::varmap::VarMapAble;

#[derive(Clone, PartialEq, Eq, Hash,Debug)]
pub enum VariableReference {
    MSG_PARAMETER(usize,usize),
    VARIABLE(usize),
    SYMBOL(usize)
}

impl VariableReference {

    pub fn get_occuring_variables(&self) -> HashSet<usize> {
        match self {
            VariableReference::VARIABLE(var_id) => {
                let mut var_set : HashSet<usize> = HashSet::new();
                var_set.insert( *var_id );
                return var_set;
            },
            _ => {
                return HashSet::new();
            }
        }
    }

}

impl VarMapAble for VariableReference {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> VariableReference {
        match self {
            VariableReference::VARIABLE( vr_id ) => {
                match mapping.get(vr_id) {
                    None => {
                        return VariableReference::VARIABLE( *vr_id );
                    },
                    Some( up_vr_id ) => {
                        return VariableReference::VARIABLE( *up_vr_id );
                    }
                }
            },
            VariableReference::SYMBOL( symb_id ) => {
                return VariableReference::SYMBOL( *symb_id );
            },
            VariableReference::MSG_PARAMETER( ms_id, pr_id ) => {
                return VariableReference::MSG_PARAMETER( *ms_id, *pr_id );
            }
        }
    }

}

