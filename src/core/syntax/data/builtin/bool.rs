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

use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::action::ValueOrNewFresh;
use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::var_ref::VariableReference;
use crate::core::semantics::varmap::VarMapAble;

#[derive(Clone, PartialEq, Debug)]
pub enum Bool_Compare {
    Equal,
    Greater,
    GreaterOrEqual,
    Lower,
    LowerOrEqual,
    Different
}

#[derive(Clone, PartialEq, Debug)]
pub enum TD_Bool {
    TRUE,
    FALSE,
    AND(Vec<TD_Bool>),
    OR(Vec<TD_Bool>),
    NOT(Box<TD_Bool>),
    COMPARE(Bool_Compare, Box<TD_Generic>, Box<TD_Generic>),
    Reference(VariableReference)
}


impl TD_Bool {

    pub fn get_occuring_variables(&self) -> HashSet<usize> {
        match self {
            TD_Bool::TRUE => {
                return HashSet::new();
            },
            TD_Bool::FALSE => {
                return HashSet::new();
            },
            TD_Bool::AND(sub_bools) => {
                let mut occs : HashSet<usize> = HashSet::new();
                for sub_bool in sub_bools {
                    occs.extend(sub_bool.get_occuring_variables());
                }
                return occs;
            },
            TD_Bool::OR(sub_bools) => {
                let mut occs : HashSet<usize> = HashSet::new();
                for sub_bool in sub_bools {
                    occs.extend(sub_bool.get_occuring_variables());
                }
                return occs;
            },
            TD_Bool::NOT(sub_bool) => {
                return (*sub_bool).get_occuring_variables();
            },
            TD_Bool::COMPARE(_,first,second) => {
                let mut occs : HashSet<usize> = HashSet::new();
                occs.extend(first.get_occuring_variables());
                occs.extend(second.get_occuring_variables());
                return occs;
            },
            TD_Bool::Reference(var_ref) => {
                return var_ref.get_occuring_variables();
            }
        }
    }
}


impl VarMapAble for TD_Bool {

    fn apply_variable_mapping(&self, mapping : &HashMap<usize,usize>) -> TD_Bool {
        match self {
            TD_Bool::TRUE => {
                return TD_Bool::TRUE;
            },
            TD_Bool::FALSE => {
                return TD_Bool::FALSE;
            },
            TD_Bool::AND( sub_exprs ) => {
                let mut updated_sub_exprs : Vec<TD_Bool> = Vec::new();
                for expr in sub_exprs {
                    updated_sub_exprs.push( expr.apply_variable_mapping(mapping) );
                }
                return TD_Bool::AND(updated_sub_exprs);
            },
            TD_Bool::OR( sub_exprs ) => {
                let mut updated_sub_exprs : Vec<TD_Bool> = Vec::new();
                for expr in sub_exprs {
                    updated_sub_exprs.push( expr.apply_variable_mapping(mapping) );
                }
                return TD_Bool::OR(updated_sub_exprs);
            },
            TD_Bool::NOT( sub_expr ) => {
                return TD_Bool::NOT( Box::new(sub_expr.apply_variable_mapping(mapping)) );
            },
            TD_Bool::COMPARE(kind, expr1, expr2) => {
                let up_expr1 = expr1.apply_variable_mapping(mapping);
                let up_expr2 = expr2.apply_variable_mapping(mapping);
                return TD_Bool::COMPARE(kind.clone(),
                                            Box::new(up_expr1),
                                            Box::new(up_expr2));
            },
            TD_Bool::Reference( var_ref ) => {
                return TD_Bool::Reference( var_ref.apply_variable_mapping(mapping) );
            }
        }
    }

}

