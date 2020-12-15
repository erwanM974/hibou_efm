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

use pest::Parser;
use pest::iterators::{Pair,Pairs};


use crate::core::context::general::GeneralContext;

use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::builtin::string::TD_String;
use crate::core::syntax::data::builtin::bool::TD_Bool;
use crate::core::syntax::data::builtin::number::{ARITH_FACTOR_SIGN, ARITH_ADD_SIGN};
use crate::core::syntax::data::builtin::integer::TD_Integer;
use crate::core::syntax::data::builtin::float::TD_Float;
use crate::core::syntax::data::var_ref::VariableReference;

use crate::from_text::error::HibouParsingError;
use crate::from_text::parser::*;
use crate::from_text::data::arithmetic::{ParsingNumberKind,parse_arith_expr};
use crate::from_text::data::logic::parse_logic_expr;

pub fn parse_data(gen_ctx : &GeneralContext,
                  td_value : Pair<Rule>,
                  opt_ms_id : &Option<usize>) -> Result<TD_Generic,HibouParsingError> {
    let data_pair = td_value.into_inner().next().unwrap();
    match data_pair.as_rule() {
        Rule::MSG_PRM_REF => {
            match opt_ms_id {
                None => {
                    return Err( HibouParsingError::ParameterUsageError("need contextual message to parse data with message parameter reference".to_string()));
                },
                Some( ms_id ) => {
                    match gen_ctx.get_ms_spec(*ms_id) {
                        Err(e) => {
                            panic!();
                        },
                        Ok( ms_spec ) => {
                            let content = data_pair.into_inner().next().unwrap();
                            match content.as_rule() {
                                Rule::PRM_LABEL => {
                                    let prm_label : String  = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                                    let mut idx : usize = 0;
                                    for (pr_type,opt_pr_name) in ms_spec {
                                        if let Some(got_pr_name) = opt_pr_name {
                                            if got_pr_name == prm_label {
                                                let var_ref = VariableReference::MSG_PARAMETER(*ms_id,idx);
                                                match pr_type {
                                                    TD_DataType::Bool => {
                                                        return Ok( TD_Generic::Bool( TD_Bool::Reference(var_ref) ) );
                                                    },
                                                    TD_DataType::Integer => {
                                                        return Ok( TD_Generic::Integer( TD_Integer::Reference(var_ref) ) );
                                                    },
                                                    TD_DataType::Float => {
                                                        return Ok( TD_Generic::Float( TD_Float::Reference(var_ref) ) );
                                                    },
                                                    TD_DataType::String => {
                                                        return Ok( TD_Generic::String( TD_String::Reference(var_ref) ) );
                                                    },
                                                    _ => {
                                                        panic!();
                                                    }
                                                }
                                            }
                                        }
                                        idx = idx +1;
                                    }
                                    return Err(HibouParsingError::ParameterUsageError( format!("no parameter '{}' in message '{}'",prm_label,gen_ctx.get_ms_name(*ms_id).unwrap()) ));
                                },
                                Rule::ARITH_INTEGER => {
                                    let pr_id : usize = content.as_str().parse::<usize>().unwrap();
                                    match ms_spec.get(pr_id) {
                                        None => {
                                            return Err(HibouParsingError::ParameterUsageError(format!("missing parameter declaration")));
                                        },
                                        Some( (pr_type,opt_pr_name) ) => {
                                            let var_ref = VariableReference::MSG_PARAMETER(*ms_id,pr_id);
                                            match pr_type {
                                                TD_DataType::Bool => {
                                                    return Ok( TD_Generic::Bool( TD_Bool::Reference(var_ref) ) );
                                                },
                                                TD_DataType::Integer => {
                                                    return Ok( TD_Generic::Integer( TD_Integer::Reference(var_ref) ) );
                                                },
                                                TD_DataType::Float => {
                                                    return Ok( TD_Generic::Float( TD_Float::Reference(var_ref) ) );
                                                },
                                                TD_DataType::String => {
                                                    return Ok( TD_Generic::String( TD_String::Reference(var_ref) ) );
                                                },
                                                _ => {
                                                    panic!();
                                                }
                                            }
                                        }
                                    }
                                },
                                _ => {
                                    panic!("what rule then ? : {:?}", content.as_rule() );
                                }
                            }
                        }
                    }
                }
            }
        },
        Rule::VAR_LABEL => {
            let var_name : String = data_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_vr_id( &var_name) {
                None => {
                    return Err( HibouParsingError::MissingVariableDeclarationError(var_name) );
                },
                Some( vr_id ) => {
                    match gen_ctx.get_vr_type(vr_id) {
                        Err(_) => {
                            panic!();
                        },
                        Ok( var_type ) => {
                            let var_ref = VariableReference::VARIABLE(vr_id);
                            match var_type {
                                TD_DataType::Bool => {
                                    return Ok( TD_Generic::Bool( TD_Bool::Reference(var_ref) ) );
                                },
                                TD_DataType::Integer => {
                                    return Ok( TD_Generic::Integer( TD_Integer::Reference(var_ref) ) );
                                },
                                TD_DataType::Float => {
                                    return Ok( TD_Generic::Float( TD_Float::Reference(var_ref) ) );
                                },
                                TD_DataType::String => {
                                    return Ok( TD_Generic::String( TD_String::Reference(var_ref) ) );
                                },
                                _ => {
                                    panic!();
                                }
                            }
                        }
                    }
                }
            }
        },
        Rule::STRING_content => {
            let string_content = data_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            let string_value = TD_String::Value( string_content );
            return Ok( TD_Generic::String( string_value ));
        },
        Rule::ARITH_EXPR => {
            return parse_arith_expr(gen_ctx, data_pair,opt_ms_id);
        },
        Rule::LOGIC_EXPR => {
            match parse_logic_expr(gen_ctx, data_pair, opt_ms_id) {
                Err(e) => {
                    return Err(e);
                },
                Ok( td_bool ) => {
                    return Ok( TD_Generic::Bool(td_bool) );
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", data_pair.as_rule() );
        }
    }
}
