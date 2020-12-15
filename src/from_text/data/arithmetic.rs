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

use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::builtin::number::{ARITH_FACTOR_SIGN, ARITH_ADD_SIGN};
use crate::core::syntax::data::builtin::integer::TD_Integer;
use crate::core::syntax::data::builtin::float::TD_Float;
use crate::core::syntax::data::var_ref::VariableReference;

use crate::from_text::error::HibouParsingError;
use crate::from_text::parser::*;

#[derive(Clone, PartialEq, Eq)]
pub enum ParsingNumberKind {
    Integer,
    Float
}

pub fn parse_arith_expr(gen_ctx : &GeneralContext, arth_pair : Pair<Rule>, opt_ms_id : &Option<usize>) -> Result<TD_Generic,HibouParsingError> {
    let mut pairs = arth_pair.into_inner();
    let factor_pair = pairs.next().unwrap();
    // ***
    let as_raw_string_for_error = factor_pair.as_str().to_string();
    // ***
    match parse_factor_expr(gen_ctx,factor_pair,opt_ms_id) {
        Err(e) => {
            return Err(e);
        },
        Ok( factor_td_gen ) => {
            match factor_td_gen {
                TD_Generic::Integer( factor_td_int ) => {
                    let mut adds : Vec<(ARITH_ADD_SIGN,TD_Integer)> = Vec::new();
                    adds.push( (ARITH_ADD_SIGN::Plus,factor_td_int) );
                    while let Some(next_add) = pairs.next() {
                        match parse_add_expr(gen_ctx,next_add,opt_ms_id) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( (sign,add_td_gen) ) => {
                                match add_td_gen {
                                    TD_Generic::Integer( add_td_int ) => {
                                        adds.push( (sign,add_td_int) );
                                    },
                                    _ => {
                                        return Err( HibouParsingError::MalformedArithmeticExpression( as_raw_string_for_error ) );
                                    }
                                }
                            }
                        }
                    }
                    if adds.len() == 1 {
                        let (_,prim) = adds.get(0).unwrap();
                        return Ok( TD_Generic::Integer(prim.clone()) );
                    } else {
                        return Ok( TD_Generic::Integer( TD_Integer::Add(adds) ) );
                    }
                },
                TD_Generic::Float( factor_td_float ) => {
                    let mut adds : Vec<(ARITH_ADD_SIGN,TD_Float)> = Vec::new();
                    adds.push( (ARITH_ADD_SIGN::Plus,factor_td_float) );
                    while let Some(next_add) = pairs.next() {
                        match parse_add_expr(gen_ctx,next_add,opt_ms_id) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( (sign,add_td_gen) ) => {
                                match add_td_gen {
                                    TD_Generic::Float( add_td_float ) => {
                                        adds.push( (sign,add_td_float) );
                                    },
                                    _ => {
                                        return Err( HibouParsingError::MalformedArithmeticExpression( as_raw_string_for_error ) );
                                    }
                                }
                            }
                        }
                    }
                    if adds.len() == 1 {
                        let(_,prim) = adds.get(0).unwrap();
                        return Ok( TD_Generic::Float(prim.clone()) );
                    } else {
                        return Ok( TD_Generic::Float( TD_Float::Add(adds) ) );
                    }
                },
                _ => {
                    return Err( HibouParsingError::MalformedArithmeticExpression( as_raw_string_for_error ) );
                }
            }
        }
    }
}

fn parse_add_expr(gen_ctx : &GeneralContext, add_pair : Pair<Rule>, opt_ms_id : &Option<usize>) -> Result<(ARITH_ADD_SIGN,TD_Generic),HibouParsingError> {
    let mut pairs = add_pair.into_inner();
    let sign_pair = pairs.next().unwrap();
    let factor_pair = pairs.next().unwrap();
    match parse_factor_expr(gen_ctx,factor_pair,opt_ms_id) {
        Err(e) => {
            return Err(e);
        },
        Ok( factor_td_gen ) => {
            match sign_pair.as_rule() {
                Rule::ARITH_PLUS => {
                    return Ok( (ARITH_ADD_SIGN::Plus,factor_td_gen) );
                },
                Rule::ARITH_MINUS => {
                    return Ok( (ARITH_ADD_SIGN::Minus,factor_td_gen) );
                },
                _ => {
                    unreachable!();
                }
            }
        }
    }
}

fn parse_primary_expr(gen_ctx : &GeneralContext, primary_pair : Pair<Rule>, opt_ms_id : &Option<usize>) -> Result<TD_Generic,HibouParsingError> {
    let mut contents = primary_pair.into_inner();
    let first_pair = contents.next().unwrap();
    // ***
    let as_raw_string_for_error = first_pair.as_str().to_string();
    // ***
    match first_pair.as_rule() {
        Rule::ARITH_EXPR => {
            return parse_arith_expr(gen_ctx,first_pair,opt_ms_id);
        },
        Rule::ARITH_FLOAT => {
            let content_str : String = first_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            let my_val : f64 = content_str.parse::<f64>().unwrap();
            let td_float = TD_Float::Value( my_val );
            return Ok( TD_Generic::Float(td_float) );
        },
        Rule::ARITH_INTEGER => {
            let content_str : String = first_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            let my_val : i64 = content_str.parse::<i64>().unwrap();
            let td_int = TD_Integer::Value( my_val );
            return Ok( TD_Generic::Integer(td_int) );
        },
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
                            let content = first_pair.into_inner().next().unwrap();
                            match content.as_rule() {
                                Rule::PRM_LABEL => {
                                    let prm_label : String  = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                                    let mut idx : usize = 0;
                                    for (pr_type,opt_pr_name) in ms_spec {
                                        if let Some(got_pr_name) = opt_pr_name {
                                            if got_pr_name == prm_label {
                                                let var_ref = VariableReference::MSG_PARAMETER(*ms_id,idx);
                                                match pr_type {
                                                    TD_DataType::Integer => {
                                                        return Ok( TD_Generic::Integer(TD_Integer::Reference(var_ref)) );
                                                    },
                                                    TD_DataType::Float => {
                                                        return Ok( TD_Generic::Float(TD_Float::Reference(var_ref)) );
                                                    },
                                                    _ => {
                                                        return Err( HibouParsingError::MalformedArithmeticExpression( as_raw_string_for_error ) );
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
                                            return Err(HibouParsingError::ParameterUsageError( format!("no parameter number '{}' in message '{}'",pr_id,gen_ctx.get_ms_name(*ms_id).unwrap()) ));
                                        },
                                        Some( (pr_type,opt_pr_name) ) => {
                                            let var_ref = VariableReference::MSG_PARAMETER(*ms_id,pr_id);
                                            match pr_type {
                                                TD_DataType::Integer => {
                                                    return Ok( TD_Generic::Integer(TD_Integer::Reference(var_ref)) );
                                                },
                                                TD_DataType::Float => {
                                                    return Ok( TD_Generic::Float(TD_Float::Reference(var_ref)) );
                                                },
                                                _ => {
                                                    return Err( HibouParsingError::MalformedArithmeticExpression( as_raw_string_for_error ) );
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
            let content_str : String = first_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_vr_id(&content_str) {
                None => {
                    return Err( HibouParsingError::MissingVariableDeclarationError( content_str ) );
                },
                Some( vr_id ) => {
                    let var_ref = VariableReference::VARIABLE(vr_id);
                    match gen_ctx.get_vr_type( vr_id ).unwrap() {
                        TD_DataType::Integer => {
                            return Ok( TD_Generic::Integer(TD_Integer::Reference(var_ref)) );
                        },
                        TD_DataType::Float => {
                            return Ok( TD_Generic::Float(TD_Float::Reference(var_ref)) );
                        },
                        _ => {
                            return Err( HibouParsingError::MalformedArithmeticExpression( as_raw_string_for_error ) );
                        }
                    }
                }
            }
        },
        Rule::ARITH_MINUS => {
            let second_pair = contents.next().unwrap();
            match parse_primary_expr(gen_ctx,second_pair,opt_ms_id) {
                Err(e) => {
                    return Err(e);
                },
                Ok( td_gen ) => {
                    match td_gen {
                        TD_Generic::Integer(td_int) => {
                            return Ok( TD_Generic::Integer(TD_Integer::Minus(Box::new(td_int))) );
                        },
                        TD_Generic::Float(td_float) => {
                            return Ok( TD_Generic::Float(TD_Float::Minus(Box::new(td_float))) );
                        },
                        _ => {
                            return Err( HibouParsingError::MalformedArithmeticExpression( as_raw_string_for_error ) );
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

fn parse_factor_expr(gen_ctx : &GeneralContext, factor_pair : Pair<Rule>, opt_ms_id : &Option<usize>) -> Result<TD_Generic,HibouParsingError> {
    let mut pairs = factor_pair.into_inner();
    let primary_pair = pairs.next().unwrap();
    // ***
    let as_raw_string_for_error = primary_pair.as_str().to_string();
    // ***
    match parse_primary_expr(gen_ctx,primary_pair,opt_ms_id) {
        Err(e) => {
            return Err(e);
        },
        Ok( primary_td_gen ) => {
            match primary_td_gen {
                TD_Generic::Integer( primary_td_int ) => {
                    let mut exprs : Vec<(ARITH_FACTOR_SIGN,TD_Integer)> = Vec::new();
                    exprs.push( (ARITH_FACTOR_SIGN::Mult,primary_td_int) );
                    while let Some(next_exp) = pairs.next() {
                        match parse_mult_expr(gen_ctx,next_exp,opt_ms_id) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( (sign,mult_td_gen) ) => {
                                match mult_td_gen {
                                    TD_Generic::Integer( mult_td_int ) => {
                                        exprs.push( (sign,mult_td_int) );
                                    },
                                    _ => {
                                        return Err( HibouParsingError::MalformedArithmeticExpression( as_raw_string_for_error ) );
                                    }
                                }
                            }
                        }
                    }
                    if exprs.len() == 1 {
                        let (_,prim) = exprs.get(0).unwrap();
                        return Ok( TD_Generic::Integer(prim.clone()) );
                    } else {
                        return Ok( TD_Generic::Integer(TD_Integer::Factor(exprs)));
                    }
                },
                TD_Generic::Float( primary_td_float ) => {
                    let mut exprs : Vec<(ARITH_FACTOR_SIGN,TD_Float)> = Vec::new();
                    exprs.push( (ARITH_FACTOR_SIGN::Mult,primary_td_float) );
                    while let Some(next_exp) = pairs.next() {
                        match parse_mult_expr(gen_ctx,next_exp,opt_ms_id) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( (sign,mult_td_gen) ) => {
                                match mult_td_gen {
                                    TD_Generic::Float( mult_td_float ) => {
                                        exprs.push( (sign,mult_td_float) );
                                    },
                                    _ => {
                                        return Err( HibouParsingError::MalformedArithmeticExpression( as_raw_string_for_error ) );
                                    }
                                }
                            }
                        }
                    }
                    if exprs.len() == 1 {
                        let (_,prim) = exprs.get(0).unwrap();
                        return Ok( TD_Generic::Float( prim.clone() ) );
                    } else {
                        return Ok( TD_Generic::Float(TD_Float::Factor(exprs)));
                    }
                },
                _ => {
                    return Err( HibouParsingError::MalformedArithmeticExpression( as_raw_string_for_error ) );
                }
            }
        }
    }
}

fn parse_mult_expr(gen_ctx : &GeneralContext, mult_pair : Pair<Rule>, opt_ms_id : &Option<usize>) -> Result<(ARITH_FACTOR_SIGN,TD_Generic),HibouParsingError> {
    let mut pairs = mult_pair.into_inner();
    let sign_pair = pairs.next().unwrap();
    let primary_pair = pairs.next().unwrap();
    match parse_primary_expr(gen_ctx,primary_pair,opt_ms_id) {
        Err(e) => {
            return Err(e);
        },
        Ok( primary_td_gen ) => {
            match sign_pair.as_rule() {
                Rule::ARITH_MULT => {
                    return Ok( (ARITH_FACTOR_SIGN::Mult,primary_td_gen) );
                },
                Rule::ARITH_DIV => {
                    return Ok( (ARITH_FACTOR_SIGN::Div,primary_td_gen) );
                },
                _ => {
                    unreachable!();
                }
            }
        }
    }
}