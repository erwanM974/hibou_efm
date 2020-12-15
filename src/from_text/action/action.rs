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



use crate::core::syntax::action::*;
use crate::core::context::general::GeneralContext;

use crate::from_text::parser::*;
use crate::from_text::error::HibouParsingError;

use crate::from_text::data::generic::parse_data;
use crate::from_text::action::lf_act::parse_lifeline_action;

use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::td_type::TD_DataType;

fn parse_message_parameters(gen_ctx : &GeneralContext,
                            msg_param_pair : Pair<Rule>,
                            ms_name : &String,
                            ms_specs : Vec<(TD_DataType,Option<String>)>) -> Result<Vec<ValueOrNewFresh>,HibouParsingError> {
    let mut params : Vec<ValueOrNewFresh> = Vec::new();
    let mut current_id : usize = 0;
    for value_or_new_fresh_pair in msg_param_pair.into_inner() {
        match value_or_new_fresh_pair.as_rule() {
            Rule::TD_VALUE => {
                match parse_data(gen_ctx,value_or_new_fresh_pair,&None) {
                    Err(e) => {
                        return Err(e);
                    },
                    Ok( carried_data ) => {
                        match ms_specs.get(current_id) {
                            None => {
                                return Err( HibouParsingError::UnknownMessageParameter(  ms_name.clone(), current_id ) );
                            },
                            Some( (expected_type,_) ) => {
                                let got_type = carried_data.get_td_type();
                                if &got_type != expected_type {
                                    return Err( HibouParsingError::WrongMessageParameterType(expected_type.clone(),
                                                                                             got_type,
                                                                                             ms_name.clone(),
                                                                                             format!("{:?}", ms_specs) ) );
                                }
                                params.push( ValueOrNewFresh::Value(carried_data) );
                            }
                        }
                    }
                }
            },
            Rule::NEW_FRESH => {
                params.push( ValueOrNewFresh::NewFresh );
            },
            _ => {
                panic!("what rule then ? : {:?}", value_or_new_fresh_pair.as_rule() );
            }
        }
        current_id = current_id + 1;
    }
    let arg_count = params.len();
    if arg_count != ms_specs.len() {
        return Err( HibouParsingError::WrongMessageParametersNumber(ms_specs.len(), arg_count,ms_name.clone()) );
    }
    return Ok( params );
}

pub fn parse_reception(gen_ctx : &GeneralContext, contents : &mut Pairs<Rule>) -> Result<ObservableAction,HibouParsingError> {
    let message_name_pair = contents.next().unwrap();
    let message_name : String = message_name_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    match gen_ctx.get_ms_id( &message_name ) {
        None => {
            return Err( HibouParsingError::MissingMessageDeclarationError(message_name) );
        },
        Some( ms_id ) => {
            let ms_specs = gen_ctx.get_ms_spec(ms_id).unwrap();
            // ***
            let params : Vec<ValueOrNewFresh>;
            let lf_act : LifelineAction;
            let next_pair =  contents.next().unwrap();
            let lf_pair : Pair<Rule>;
            match next_pair.as_rule() {
                Rule::SD_MESSAGE_PARAMETERS => {
                    match parse_message_parameters(gen_ctx, next_pair, &message_name, ms_specs) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( msg_parameters ) => {
                            params = msg_parameters;
                        }
                    }
                    lf_pair = contents.next().unwrap();
                },
                Rule::SD_LIFELINE_ACTION => {
                    params = Vec::new();
                    lf_pair = next_pair;
                },
                _ => {
                    unreachable!();
                }
            }
            // ***
            match parse_lifeline_action(gen_ctx, lf_pair, &Some(ms_id)) {
                Err(e) => {
                    return Err(e);
                },
                Ok( lf_act ) => {
                    let reception_act = ObservableAction{
                        lf_act:lf_act,
                        act_kind:ObservableActionKind::Reception,
                        ms_id:ms_id,
                        params:params,
                        original_position:None};
                    return Ok( reception_act );
                }
            }
        }
    }
}

pub fn parse_emission(gen_ctx : &GeneralContext, contents : &mut Pairs<Rule>) -> Result<ObservableAction,HibouParsingError> {
    // ***
    match parse_lifeline_action(gen_ctx, contents.next().unwrap(),&None) {
        Err(e) => {
            return Err(e);
        },
        Ok( lf_act ) => {
            let message_name : String = contents.next().unwrap().as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_ms_id( &message_name ) {
                None => {
                    return Err( HibouParsingError::MissingMessageDeclarationError( message_name ) );
                },
                Some( ms_id ) => {
                    let mut targets : Vec<LifelineAction>;
                    let params : Vec<ValueOrNewFresh>;
                    match contents.next() {
                        None => {
                            targets = Vec::new();
                            params = Vec::new();
                        },
                        Some( next_pair ) => {
                            let target_pair_opt : Option<Pair<Rule>>;
                            match next_pair.as_rule() {
                                Rule::SD_MESSAGE_PARAMETERS => {
                                    let ms_specs = gen_ctx.get_ms_spec(ms_id).unwrap();
                                    match parse_message_parameters(gen_ctx, next_pair, &message_name, ms_specs) {
                                        Err(e) => {
                                            return Err(e);
                                        },
                                        Ok( msg_parameters ) => {
                                            params = msg_parameters;
                                        }
                                    }
                                    target_pair_opt = contents.next();
                                },
                                Rule::SD_EMISSION_TARGET => {
                                    params = Vec::new();
                                    target_pair_opt = Some(next_pair);
                                },
                                _ => {
                                    unreachable!();
                                }
                            }
                            // ***
                            match target_pair_opt {
                                None => {
                                    targets = Vec::new();
                                },
                                Some( target_pair ) => {
                                    targets = Vec::new();
                                    let inner_target_pair = target_pair.into_inner().next().unwrap();
                                    match inner_target_pair.as_rule() {
                                        Rule::SD_LIFELINE_ACTION => {
                                            match parse_lifeline_action(gen_ctx,inner_target_pair,&Some(ms_id)) {
                                                Err(e) => {
                                                    return Err(e);
                                                },
                                                Ok(target_lf_act) => {
                                                    targets.push( target_lf_act );
                                                }
                                            }
                                        },
                                        Rule::SD_EMISSION_TARGETS => {
                                            for targ_name in inner_target_pair.into_inner() {
                                                match parse_lifeline_action(gen_ctx,targ_name, &Some(ms_id)) {
                                                    Err(e) => {
                                                        return Err(e);
                                                    },
                                                    Ok(target_lf_act) => {
                                                        targets.push( target_lf_act );
                                                    }
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!();
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let emission_act = ObservableAction{
                        lf_act:lf_act,
                        act_kind:ObservableActionKind::Emission(targets),
                        ms_id:ms_id,
                        params:params,
                        original_position:None};
                    return Ok( emission_act );
                }
            }
        }
    }
}

