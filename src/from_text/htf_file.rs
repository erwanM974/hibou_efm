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

use std::path::Path;
use std::collections::{HashMap,HashSet};
use std::fs;
use pest::Parser;
use pest::iterators::{Pair,Pairs};
use std::iter::FromIterator;

use crate::core::trace::*;
use crate::core::context::general::GeneralContext;

use crate::core::syntax::data::generic::TD_Generic;
use crate::from_text::error::HibouParsingError;

use crate::from_text::parser::*;

use crate::process::hibou_process::HibouProcessTemporality;

use crate::from_text::data::generic::parse_data;

use crate::core::syntax::data::builtin::float::TD_Float;

pub static HIBOU_TRACE_FILE_EXTENSION : &'static str = "hxtf";

pub fn parse_htf_file(file_path : &str,
                      gen_ctx : &GeneralContext,
                      temporality : &HibouProcessTemporality) -> Result<AnalysableMultiTrace,HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_TRACE_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_TRACE_FILE_EXTENSION.to_string()));
    }
    let file_name : &str = path_object.file_stem().unwrap().to_str().unwrap();
    match fs::read_to_string(file_path) {
        Ok( unparsed_htf_str ) => {
            return multitrace_from_text(&unparsed_htf_str, gen_ctx, temporality);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}

fn complete_canals_up_to_defined_lifelines(canals : &mut Vec<MultiTraceCanal>, gen_ctx : &GeneralContext) {
    let mut rem_lifelines : HashSet<usize> = HashSet::from_iter((0..gen_ctx.get_lf_num()).collect::<Vec<usize>>().iter().cloned());
    for canal in canals.iter() {
        for lf_id in &canal.lifelines {
            rem_lifelines.remove(lf_id);
        }
    }
    // ***
    for lf_id in rem_lifelines {
        let lifelines : HashSet<usize> = HashSet::from_iter( vec![lf_id].iter().cloned() );
        let trace : Vec<TraceAction> = Vec::new();
        canals.push( MultiTraceCanal{lifelines,trace})
    }
    // ***
}

pub fn multitrace_from_text(multitrace_str : &String,
                            gen_ctx : &GeneralContext,
                            temporality : &HibouProcessTemporality) -> Result<AnalysableMultiTrace,HibouParsingError> {
    match SDParser::parse(Rule::HTF_PEST_FILE, multitrace_str) {
        Err(e) => {
            panic!(e);
        },
        Ok( ref mut htf_pair ) => {
            let mut content = htf_pair.next().unwrap().into_inner();
            let first_pair : Pair<Rule> = content.next().unwrap();
            match first_pair.as_rule() {
                Rule::TRACE => {
                    let mut canals : Vec<MultiTraceCanal> = Vec::new();
                    match trace_canal_from_pair(first_pair,gen_ctx,&HashSet::new(),temporality) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( trace_canal ) => {
                            canals.push( trace_canal );
                        }
                    }
                    complete_canals_up_to_defined_lifelines(&mut canals,gen_ctx);
                    return Ok( AnalysableMultiTrace::new(canals) );
                },
                Rule::MULTI_TRACE => {
                    let mut unavailable_lifelines : HashSet<usize> = HashSet::new();
                    let mut canals : Vec<MultiTraceCanal> = Vec::new();
                    for trace_pair in first_pair.into_inner() {
                        match trace_canal_from_pair(trace_pair,gen_ctx,&unavailable_lifelines,temporality) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( trace_canal ) => {
                                unavailable_lifelines = unavailable_lifelines.union( &trace_canal.lifelines ).cloned().collect();
                                canals.push( trace_canal );
                            }
                        }
                    }
                    complete_canals_up_to_defined_lifelines(&mut canals,gen_ctx);
                    return Ok( AnalysableMultiTrace::new(canals) );
                },
                _ => {
                    panic!("what rule then ? : {:?}", first_pair.as_rule() );
                }
            }
        }
    }
}

pub fn trace_canal_from_pair(trace_pair : Pair<Rule>,
                             gen_ctx : &GeneralContext,
                             unavailable_lifelines : &HashSet<usize>,
                             temporality : &HibouProcessTemporality) -> Result<MultiTraceCanal,HibouParsingError> {
    let mut trace : Vec<TraceAction> = Vec::new();
    let mut lifelines : HashSet<usize> = HashSet::new();
    // ***
    let mut content = trace_pair.into_inner();
    // ***
    match content.next() {
        None => {},
        Some( first_pair ) => {
            match first_pair.as_rule() {
                Rule::CANAL_ANY => {
                    match inner_trace_from_pairs(&mut content,gen_ctx,unavailable_lifelines,&mut trace, &mut lifelines, true,temporality) {
                        Ok( _ ) => {
                            // do nothing
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    };
                },
                Rule::CANAL_ALL => {
                    let all_lfs : HashSet<usize> = HashSet::from_iter((0..gen_ctx.get_lf_num()).collect::<Vec<usize>>().iter().cloned());
                    lifelines = all_lfs;
                    match inner_trace_from_pairs(&mut content,gen_ctx,unavailable_lifelines,&mut trace, &mut lifelines, true,temporality) {
                        Ok( _ ) => {
                            // do nothing
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    };
                },
                Rule::CANAL_LIFELINES => {
                    for trace_lf_pair in first_pair.into_inner() {
                        let lf_name : String  = trace_lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                        match gen_ctx.get_lf_id(&lf_name) {
                            None => {
                                return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name));
                            },
                            Some( lf_id ) => {
                                lifelines.insert(lf_id);
                            }
                        }
                    }
                    match inner_trace_from_pairs(&mut content,gen_ctx,unavailable_lifelines,&mut trace, &mut lifelines, false,temporality) {
                        Ok( _ ) => {
                            // do nothing
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    };
                },
                _ => {
                    panic!("what rule then ? : {:?}", first_pair.as_rule() );
                }
            }
        }
    }
    // ***
    return Ok( MultiTraceCanal{lifelines,trace} );
}

pub fn inner_trace_from_pairs(content : &mut Pairs<Rule>,
                              gen_ctx : &GeneralContext,
                              unavailable_lifelines : &HashSet<usize>,
                              trace : &mut Vec<TraceAction>,
                              lifelines : &mut HashSet<usize>,
                              add_lfs : bool,
                              temporality : &HibouProcessTemporality) -> Result<(),HibouParsingError> {
    for action_pair in content {
        match trace_action_from_text(action_pair,gen_ctx,temporality) {
            Err(e) => {
                return Err(e);
            },
            Ok( action ) => {
                if unavailable_lifelines.contains(&action.lf_id) {
                    return Err( HibouParsingError::NonDisjointTraceComponents );
                } else {
                    if add_lfs {
                        lifelines.insert( action.lf_id);
                    }
                }
                trace.push( action );
            }
        }
    }
    return Ok( () );
}


fn trace_action_from_text(action_pair : Pair<Rule>,
                          gen_ctx : &GeneralContext,
                          temporality : &HibouProcessTemporality) -> Result<TraceAction,HibouParsingError> {
    let original_pair_as_string = action_pair.as_str().to_string();
    // ***
    let mut raw_delay_opt : Option<f64> = None;
    // ***
    let mut contents = action_pair.into_inner();
    // ***
    let first_pair : Pair<Rule> = contents.next().unwrap();
    let lf_pair : Pair<Rule>;
    match first_pair.as_rule() {
        Rule::TRACE_DELAY => {
            let inner_float_pair = first_pair.into_inner().next().unwrap();
            let content_str : String = inner_float_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            raw_delay_opt = Some( content_str.parse::<f64>().unwrap() );
            lf_pair = contents.next().unwrap()
        },
        Rule::TRACE_LIFELINE => {
            lf_pair = first_pair;
        },
        _ => {
            panic!();
        }
    }
    // ***
    match untimed_trace_action_from_text(lf_pair, &mut contents, gen_ctx) {
        Err(e) => {
            return Err(e);
        },
        Ok( (lf_id,act_kind,ms_id,arguments) ) => {
            match raw_delay_opt {
                None => {
                    match temporality {
                        HibouProcessTemporality::UnTimed => {
                            return Ok( TraceAction{delay:None,lf_id,act_kind,ms_id,arguments} );
                        },
                        HibouProcessTemporality::Timed => {
                            return Err( HibouParsingError::TimedTraceAbsentDelay(original_pair_as_string) );
                        }
                    }
                },
                Some( raw_delay ) => {
                    match temporality {
                        HibouProcessTemporality::UnTimed => {
                            println!("WARNNG : unused timed trace delay information due to analysis in untimed mode");
                            return Ok( TraceAction{delay:None,lf_id,act_kind,ms_id,arguments} );
                        },
                        HibouProcessTemporality::Timed => {
                            return Ok( TraceAction{delay:Some(TD_Float::Value(raw_delay)),lf_id,act_kind,ms_id,arguments} );
                        }
                    }
                }
            }
        }
    }
}

fn untimed_trace_action_from_text(  lf_pair : Pair<Rule>,
                                    action_contents : &mut Pairs<Rule>,
                                    gen_ctx : &GeneralContext) -> Result<(usize,TraceActionKind,usize,Vec<TD_Generic>),HibouParsingError> {
    // ***
    let lf_name : String  = lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    match gen_ctx.get_lf_id(&lf_name) {
        None => {
            return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name) );
        },
        Some( lf_id ) => {
            // ***
            let act_kind_pair : Pair<Rule> = action_contents.next().unwrap();
            let act_kind : TraceActionKind;
            match act_kind_pair.as_rule() {
                Rule::TRACE_EMISSION_SYMBOL => {
                    act_kind = TraceActionKind::Emission;
                },
                Rule::TRACE_RECEPTION_SYMBOL => {
                    act_kind = TraceActionKind::Reception;
                },
                _ => {
                    panic!("what rule then ? : {:?}", act_kind_pair.as_rule() );
                }
            }
            // ***
            let ms_pair : Pair<Rule> = action_contents.next().unwrap();
            let ms_name : String  = ms_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_ms_id(&ms_name) {
                None => {
                    return Err( HibouParsingError::MissingMessageDeclarationError(ms_name) );
                },
                Some( ms_id ) => {
                    let ms_specs = gen_ctx.get_ms_spec(ms_id).unwrap();
                    // ***
                    let mut arg_count : usize = 0;
                    let mut arguments : Vec<TD_Generic> = Vec::new();
                    match action_contents.next() {
                        None => {},
                        Some( arguments_pair ) => {
                            for arg_pair in arguments_pair.into_inner() {
                                let argument_string : String = arg_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                                // ***
                                match SDParser::parse(Rule::TD_VALUE, &argument_string) {
                                    Err(e) => {
                                        panic!(e);
                                    },
                                    Ok( mut argument_as_td_val ) => {
                                        match ms_specs.get(arg_count) {
                                            None => {
                                                return Err( HibouParsingError::UnknownMessageParameter(ms_name,arg_count) );
                                            },
                                            Some( (expected_type ,_) ) => {
                                                let mut content = argument_as_td_val.next().unwrap();
                                                let trace_action_param_td_gen : TD_Generic = parse_data(gen_ctx,content,&Some(ms_id)).unwrap();
                                                let got_type = trace_action_param_td_gen.get_td_type();
                                                if &got_type != expected_type {
                                                    return Err( HibouParsingError::WrongMessageParameterType(expected_type.clone(), got_type, ms_name, format!("{:?}", ms_specs) ) );
                                                }
                                                arguments.push(trace_action_param_td_gen);
                                                arg_count = arg_count +1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // ***
                    if arg_count != ms_specs.len() {
                        return Err( HibouParsingError::WrongMessageParametersNumber(ms_specs.len(), arg_count,ms_name) );
                    }
                    // ***
                    return Ok( (lf_id,act_kind,ms_id,arguments) );
                }
            }
        }
    }
}








