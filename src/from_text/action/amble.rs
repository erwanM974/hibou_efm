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

use pest::iterators::{Pair,Pairs};



use crate::core::syntax::action::{ValueOrNewFresh,LifelineAction,ActionAmbleItem};
use crate::core::context::general::GeneralContext;

use crate::from_text::parser::*;
use crate::from_text::error::HibouParsingError;
use crate::from_text::data::generic::parse_data;
use crate::from_text::data::logic::parse_logic_expr;

pub fn parse_amble(gen_ctx : &GeneralContext, amble_pair : Pair<Rule>, opt_ms_id : &Option<usize>) -> Result<Vec<ActionAmbleItem>,HibouParsingError> {
    let mut amble : Vec<ActionAmbleItem> = Vec::new();
    for item_pair in amble_pair.into_inner() {
        match item_pair.as_rule() {
            Rule::SD_OPERATION => {
                match parse_operations(gen_ctx,item_pair, opt_ms_id) {
                    Err(e) => {
                        return Err(e);
                    },
                    Ok( mut operations_amble ) => {
                        amble.append( &mut operations_amble );
                    }
                }
            },
            Rule::SD_GUARD => {
                match parse_guards(gen_ctx, item_pair, opt_ms_id) {
                    Err(e) => {
                        return Err(e);
                    },
                    Ok( mut guards_amble ) => {
                        amble.append( &mut guards_amble );
                    }
                }
            },
            _ => {
                unreachable!();
            }
        }
    }
    return Ok( amble );
}


fn parse_guards(gen_ctx : &GeneralContext, guard_pairs : Pair<Rule>, opt_ms_id : &Option<usize>) -> Result<Vec<ActionAmbleItem>,HibouParsingError> {
    let mut guards : Vec<ActionAmbleItem> = Vec::new();
    for guard_pair in guard_pairs.into_inner() {
        match parse_logic_expr(gen_ctx,guard_pair, opt_ms_id) {
            Err(e) => {
                return Err(e);
            },
            Ok( td_bool ) => {
                guards.push( ActionAmbleItem::Guard(td_bool) );
            }
        }
    }
    return Ok( guards );
}

fn parse_operations(gen_ctx : &GeneralContext, operations_pairs : Pair<Rule>, opt_ms_id : &Option<usize>) -> Result<Vec<ActionAmbleItem>,HibouParsingError> {
    let mut operations : Vec<ActionAmbleItem> = Vec::new();
    for operation_pair in operations_pairs.into_inner() {
        match operation_pair.as_rule() {
            Rule::OPERATION_RESET => {
                let var_to_reset_pair = operation_pair.into_inner().next().unwrap();
                let var_to_reset_name : String = var_to_reset_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                match gen_ctx.get_vr_id(&var_to_reset_name ) {
                    None => {
                        return Err( HibouParsingError::MissingVariableDeclarationError( var_to_reset_name ) );
                    },
                    Some( vr_id ) => {
                        if gen_ctx.is_clock( vr_id ) {
                            operations.push( ActionAmbleItem::Reset(vr_id) );
                        } else {
                            return Err( HibouParsingError::ClockMisuse( format!("can only reset clocks ; tried to reset '{:}' of index '{}' which is not a clock - clocks are {:?}", var_to_reset_name,vr_id,gen_ctx.get_clocks() ) ) );
                        }
                    }
                }
            },
            Rule::OPERATION_ASSIGNMENT => {
                let mut op_content = operation_pair.into_inner();
                let vr_name : String = op_content.next().unwrap().as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let value_or_newfresh_pair = op_content.next().unwrap();
                match gen_ctx.get_vr_id(&vr_name) {
                    None => {
                        return Err( HibouParsingError::MissingVariableDeclarationError( vr_name ) );
                    },
                    Some( vr_id ) => {
                        match value_or_newfresh_pair.as_rule() {
                            Rule::TD_VALUE => {
                                match parse_data(gen_ctx,value_or_newfresh_pair, opt_ms_id) {
                                    Err(e) => {
                                        return Err(e);
                                    },
                                    Ok( td_generic )=> {
                                        operations.push( ActionAmbleItem::Assignment(vr_id, ValueOrNewFresh::Value(td_generic) ) );
                                    }
                                }
                            },
                            Rule::NEW_FRESH => {
                                operations.push( ActionAmbleItem::Assignment(vr_id, ValueOrNewFresh::NewFresh ) );
                            },
                            _ => {
                                unreachable!();
                            }
                        }
                    }
                }
            },
            _ => {
                unreachable!();
            }
        }
    }
    return Ok( operations );
}

