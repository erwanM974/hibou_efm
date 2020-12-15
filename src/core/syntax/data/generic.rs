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
use crate::core::context::general::GeneralContext;
use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::var_ref::VariableReference;
use crate::core::syntax::data::builtin::bool::TD_Bool;
use crate::core::syntax::data::builtin::integer::TD_Integer;
use crate::core::syntax::data::builtin::float::TD_Float;
use crate::core::syntax::data::builtin::string::TD_String;
use crate::core::semantics::varmap::VarMapAble;

#[derive(Clone, PartialEq, Debug)] // Eq, Hash,
pub enum TD_Generic {
    //Reference(VariableReference),
    Bool(TD_Bool),
    String(TD_String),
    Integer(TD_Integer),
    Float(TD_Float)
}

impl TD_Generic {

    pub fn get_td_type(&self) -> TD_DataType {
        match self {
            TD_Generic::Bool(_) => {
                return TD_DataType::Bool;
            },
            TD_Generic::String(_) => {
                return TD_DataType::String;
            },
            TD_Generic::Integer(_) => {
                return TD_DataType::Integer;
            },
            TD_Generic::Float(_) => {
                return TD_DataType::Float;
            }
        }
    }

    pub fn get_occuring_variables(&self) -> HashSet<usize> {
        match self {
            TD_Generic::Bool(td_bool) => {
                return td_bool.get_occuring_variables();
            },
            TD_Generic::String(td_string) => {
                return td_string.get_occuring_variables();
            },
            TD_Generic::Integer(td_int) => {
                return td_int.get_occuring_variables();
            },
            TD_Generic::Float(td_float) => {
                return td_float.get_occuring_variables();
            }
        }
    }

    pub fn as_td_bool(&self) -> TD_Bool {
        match self {
            TD_Generic::Bool(td_bool) => {
                return td_bool.clone();
            },
            _ => {
                panic!();
            }
        }
    }

    pub fn as_td_string(&self) -> TD_String {
        match self {
            TD_Generic::String(td_string) => {
                return td_string.clone();
            },
            _ => {
                panic!();
            }
        }
    }

    pub fn as_td_int(&self) -> TD_Integer {
        match self {
            TD_Generic::Integer(td_int) => {
                return td_int.clone();
            },
            _ => {
                panic!();
            }
        }
    }

    pub fn as_td_float(&self) -> TD_Float {
        match self {
            TD_Generic::Float(td_float) => {
                return td_float.clone();
            },
            _ => {
                panic!();
            }
        }
    }

}

impl VarMapAble for TD_Generic {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> TD_Generic {
        match self {/*
            TD_Generic::Reference( var_ref ) => {
                return TD_Generic::Reference( var_ref.apply_variable_mapping(mapping) );
            },*/
            TD_Generic::Bool( td_bool ) => {
                return TD_Generic::Bool( td_bool.apply_variable_mapping(mapping) );
            },
            TD_Generic::Integer( td_num ) => {
                return TD_Generic::Integer( td_num.apply_variable_mapping(mapping) );
            },
            TD_Generic::Float( td_num ) => {
                return TD_Generic::Float( td_num.apply_variable_mapping(mapping) );
            },
            TD_Generic::String( td_string ) => {
                return TD_Generic::String( td_string.apply_variable_mapping(mapping) );
            }
        }
    }
}

