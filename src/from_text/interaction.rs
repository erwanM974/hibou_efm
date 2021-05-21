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


use crate::from_text::parser::*;


use crate::core::syntax::action::*;
use crate::core::context::general::GeneralContext;
use crate::core::syntax::interaction::{Interaction,ScheduleOperatorKind};

use crate::from_text::error::HibouParsingError;
use crate::from_text::action::action::{parse_emission,parse_reception};


pub fn parse_interaction(gen_ctx : &GeneralContext, sd_interaction_pair : Pair<Rule>) -> Result<Interaction,HibouParsingError> {
    let sd_content_pair = sd_interaction_pair.into_inner().next().unwrap();
    match sd_content_pair.as_rule() {
        Rule::SD_EMPTY_INTERACTION => {
            return Ok( Interaction::Empty );
        },
        Rule::SD_ACTION_RECEPTION => {
            match parse_reception(gen_ctx,&mut sd_content_pair.into_inner()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( observable_action ) => {
                    return Ok( Interaction::Action(observable_action) );
                }
            }
        },
        Rule::SD_ACTION_EMISSION => {
            match parse_emission(gen_ctx,&mut sd_content_pair.into_inner()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( observable_action ) => {
                    return Ok( Interaction::Action(observable_action) );
                }
            }
        },
        Rule::SD_STRICT_INT => {
            match get_nary_sub_interactions(gen_ctx, sd_content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(BinaryOperatorKind::Strict,&mut sub_ints) );
                }
            }
        },
        Rule::SD_SEQ_INT => {
            match get_nary_sub_interactions(gen_ctx, sd_content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(BinaryOperatorKind::Seq,&mut sub_ints) );
                }
            }
        },
        Rule::SD_ALT_INT => {
            match get_nary_sub_interactions(gen_ctx, sd_content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(BinaryOperatorKind::Alt,&mut sub_ints) );
                }
            }
        },
        Rule::SD_PAR_INT => {
            match get_nary_sub_interactions(gen_ctx, sd_content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(BinaryOperatorKind::Par,&mut sub_ints) );
                }
            }
        },
        Rule::SD_LOOP_INT => {
            let mut loop_content = sd_content_pair.into_inner();
            let loop_kind_pair = loop_content.next().unwrap();
            match parse_interaction(gen_ctx,loop_content.next().unwrap()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( sub_int ) => {
                    match loop_kind_pair.as_rule() {
                        Rule::SD_LOOPX => {
                            return Ok( Interaction::Loop(ScheduleOperatorKind::Strict,Box::new(sub_int)) );
                        },
                        Rule::SD_LOOPH => {
                            return Ok( Interaction::Loop(ScheduleOperatorKind::Seq,Box::new(sub_int)) );
                        },
                        Rule::SD_LOOPP => {
                            return Ok( Interaction::Loop(ScheduleOperatorKind::Par,Box::new(sub_int)) );
                        },
                        _ => {
                            unreachable!();
                        }
                    }
                }
            }
        },
        Rule::SD_SCOPE_INT => {
            let mut scope_content = sd_content_pair.into_inner();
            scope_content.next(); // get rid of the operator name
            // ***
            let mut scoped_vr_ids : Vec<usize> = Vec::new();
            let mut scoped_parameters = scope_content.next().unwrap().into_inner();
            // ***
            for scoped_var_pair in scoped_parameters {
                let scoped_var_name : String = scoped_var_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                match gen_ctx.get_vr_id( &scoped_var_name ) {
                    None => {
                        return Err( HibouParsingError::MissingVariableDeclarationError( scoped_var_name ) );
                    },
                    Some( vr_id ) => {
                        scoped_vr_ids.push( vr_id );
                    }
                }
            }
            // ***
            match parse_interaction(gen_ctx,scope_content.next().unwrap()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( parsed_sub_int ) => {
                    return Ok( Interaction::Scope( scoped_vr_ids, Box::new(parsed_sub_int) ) );
                }
            }
        },/*
        Rule::SD_CREATE_INT => {
            let mut create_content = sd_content_pair.into_inner();
            create_content.next(); // get rid of the operator name
            // ***
            let mut create_lf_ids : Vec<(usize,Option<usize>)> = Vec::new();
            let mut create_parameters = create_content.next().unwrap().into_inner();
            // ***
            for create_lf_pair in create_parameters {
                match create_lf_pair.as_rule() {
                    Rule::SD_CREATE_IN_GROUP => {
                        let mut create_in_group_contents = create_lf_pair.into_inner();
                        let create_lf_name_pair = create_in_group_contents.next().unwrap();
                        let create_lf_name : String = create_lf_name_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                        let create_gp_name_pair = create_in_group_contents.next().unwrap();
                        let create_gp_name : String = create_gp_name_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                        let lgr_id = gen_ctx.add_lf_group(create_gp_name);
                        let lf_id = gen_ctx.add_lf(create_lf_name);
                        create_lf_ids.push( (lf_id,Some(lgr_id)) );
                    },
                    Rule::SD_LIFELINE_OR_GROUP => {
                        let create_lf_name : String = create_lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                        let lf_id = gen_ctx.add_lf(create_lf_name);
                        create_lf_ids.push((lf_id,None) );
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", create_lf_pair.as_rule() );
                    }
                }
            }
            // ***
            match parse_interaction(gen_ctx,create_content.next().unwrap()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( parsed_sub_int ) => {
                    return Ok( Interaction::Create( create_lf_ids, Box::new(parsed_sub_int) ) );
                }
            }
        },*/
        _ => {
            panic!("what rule then ? : {:?}", sd_content_pair.as_rule());
        }
    }
}

fn get_nary_sub_interactions(gen_ctx : &GeneralContext, sd_content_pair : Pair<Rule>) -> Result<Vec<Interaction>,HibouParsingError> {
    let mut strict_content = sd_content_pair.into_inner();
    strict_content.next(); // get rid of the operator name
    let mut sub_ints : Vec<Interaction> = Vec::new();
    for sub_interaction in strict_content {
        match parse_interaction(gen_ctx,sub_interaction) {
            Err(e) => {
                return Err(e);
            },
            Ok( parsed_sub_int ) => {
                sub_ints.push( parsed_sub_int );
            }
        }
    }
    return Ok( sub_ints );
}

enum BinaryOperatorKind {
    Strict,
    Seq,
    Par,
    Alt
}

fn fold_interactions_in_binary_operator(op_kind : BinaryOperatorKind, sub_ints : &mut Vec<Interaction>) -> Interaction {
    assert!(sub_ints.len() > 0);
    if sub_ints.len() == 1 {
        return sub_ints.remove(0);
    } else {
        let first_int = sub_ints.remove(0);
        match op_kind {
            BinaryOperatorKind::Strict => {
                return Interaction::Strict( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Seq => {
                return Interaction::Seq( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Alt => {
                return Interaction::Alt( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Par => {
                return Interaction::Par( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            }
        }
    }
}





