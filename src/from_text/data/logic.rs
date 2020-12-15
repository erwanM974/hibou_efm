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
use crate::core::syntax::data::builtin::bool::{TD_Bool,Bool_Compare};
use crate::core::syntax::data::var_ref::VariableReference;

use crate::from_text::error::HibouParsingError;
use crate::from_text::parser::*;
use crate::from_text::data::generic::parse_data;

pub fn parse_logic_expr(gen_ctx : &GeneralContext, logic_pair : Pair<Rule>, opt_ms_id : &Option<usize>) -> Result<TD_Bool,HibouParsingError> {
    let content = logic_pair.into_inner().next().unwrap();
    match content.as_rule() {
        Rule::LOGIC_NOT => {
            let inner = content.into_inner().next().unwrap();
            match parse_logic_expr(gen_ctx, inner, opt_ms_id) {
                Err(e) => {
                    return Err(e);
                },
                Ok( inner_td_bool ) => {
                    return Ok( TD_Bool::NOT(Box::new(inner_td_bool)));
                }
            }
        },
        Rule::LOGIC_AND => {
            return parse_AND_OR_expr(gen_ctx,content, ParsingLogicKind::AND, opt_ms_id);
        },
        Rule::LOGIC_OR => {
            return parse_AND_OR_expr(gen_ctx,content, ParsingLogicKind::OR, opt_ms_id);
        },
        Rule::LOGIC_CMP => {
            return parse_CMP_expr(gen_ctx, content, opt_ms_id);
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
                            let content = content.into_inner().next().unwrap();
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
                                                        return Ok( TD_Bool::Reference(var_ref) );
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
                                            return Err(HibouParsingError::ParameterUsageError( format!("no parameter number '{}' in message '{}'",pr_id,gen_ctx.get_ms_name(*ms_id).unwrap()) ));
                                        },
                                        Some( (pr_type,opt_pr_name) ) => {
                                            let var_ref = VariableReference::MSG_PARAMETER(*ms_id,pr_id);
                                            match pr_type {
                                                TD_DataType::Bool => {
                                                    return Ok( TD_Bool::Reference(var_ref) );
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
            let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_vr_id(&content_str) {
                None => {
                    return Err( HibouParsingError::MissingVariableDeclarationError( content_str ) );
                },
                Some( vr_id ) => {
                    let var_ref = VariableReference::VARIABLE(vr_id);
                    match gen_ctx.get_vr_type( vr_id ).unwrap() {
                        TD_DataType::Bool => {
                            return Ok( TD_Bool::Reference(var_ref) );
                        },
                        _ => {
                            return Err( HibouParsingError::MalformedLogicExpression( format!("variable '{}' is not a boolean and appears in a boolean expression",content_str) ));
                        }
                    }
                }
            }
        },
        Rule::LOGIC_FALSE => {
            return Ok( TD_Bool::FALSE );
        },
        Rule::LOGIC_TRUE => {
            return Ok( TD_Bool::TRUE );
        },
        _ => {
            unreachable!();
        }
    }
}

enum ParsingLogicKind {
    AND,
    OR
}


fn parse_AND_OR_expr(gen_ctx : &GeneralContext, logic_pair : Pair<Rule>, kind : ParsingLogicKind, opt_ms_id : &Option<usize>) -> Result<TD_Bool,HibouParsingError> {
    let mut pairs = logic_pair.into_inner();
    let primary_pair = pairs.next().unwrap();
    match parse_logic_expr(gen_ctx,primary_pair, opt_ms_id) {
        Err(e) => {
            return Err(e);
        },
        Ok( primary_expr ) => {
            let mut exprs : Vec<TD_Bool> = Vec::new();
            exprs.push( primary_expr );
            while let Some(next_exp) = pairs.next() {
                match parse_logic_expr(gen_ctx, pairs.next().unwrap(), opt_ms_id) {
                    Err(e) => {
                        return Err(e);
                    },
                    Ok( sub_expr ) => {
                        exprs.push( sub_expr );
                    }
                }
            }
            match kind {
                ParsingLogicKind::AND => {
                    return Ok( TD_Bool::AND(exprs) );
                },
                ParsingLogicKind::OR => {
                    return Ok( TD_Bool::OR(exprs) );
                }
            }
        }
    }
}

fn parse_CMP_expr(gen_ctx : &GeneralContext, logic_pair : Pair<Rule>, opt_ms_id : &Option<usize>) -> Result<TD_Bool,HibouParsingError> {
    let mut inner = logic_pair.into_inner();
    let val1_pair = inner.next().unwrap();
    let cmp_operator_pair = inner.next().unwrap();
    let val2_pair = inner.next().unwrap();
    match parse_data(gen_ctx,val1_pair,opt_ms_id) {
        Err(e) => {
            return Err(e);
        },
        Ok( val1 ) => {
            match parse_data(gen_ctx, val2_pair,opt_ms_id) {
                Err(e) => {
                    return Err(e);
                },
                Ok( val2 ) => {
                    match cmp_operator_pair.as_rule() {
                        Rule::LOGIC_SYMB_Diff => {
                            return Ok( TD_Bool::COMPARE(Bool_Compare::Different, Box::new(val1), Box::new(val2)) );
                        },
                        Rule::LOGIC_SYMB_E => {
                            return Ok( TD_Bool::COMPARE(Bool_Compare::Equal, Box::new(val1), Box::new(val2)) );
                        },
                        Rule::LOGIC_SYMB_GoE => {
                            return Ok( TD_Bool::COMPARE(Bool_Compare::GreaterOrEqual, Box::new(val1), Box::new(val2)) );
                        },
                        Rule::LOGIC_SYMB_G => {
                            return Ok( TD_Bool::COMPARE(Bool_Compare::Greater, Box::new(val1), Box::new(val2)) );
                        },
                        Rule::LOGIC_SYMB_LoE => {
                            return Ok( TD_Bool::COMPARE(Bool_Compare::LowerOrEqual, Box::new(val1), Box::new(val2)) );
                        },
                        Rule::LOGIC_SYMB_L => {
                            return Ok( TD_Bool::COMPARE(Bool_Compare::Lower, Box::new(val1), Box::new(val2)) );
                        },
                        _ => {
                            unreachable!();
                        }
                    }
                }
            }
        }
    }
}

