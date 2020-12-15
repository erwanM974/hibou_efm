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

use std::fs;
use std::collections::{HashSet,HashMap};
use std::collections::btree_map::BTreeMap;
use std::path::Path;

use pest::iterators::Pair;

use crate::pest::Parser;

use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::action::*;
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::var_ref::VariableReference;
use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;

use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::builtin::float::TD_Float;
use crate::core::syntax::data::builtin::integer::TD_Integer;
use crate::core::syntax::data::builtin::string::TD_String;
use crate::core::syntax::data::builtin::bool::TD_Bool;
use crate::from_text::error::HibouParsingError;
use crate::process::log::ProcessLogger;

use crate::from_text::parser::*;
use crate::from_text::data::td_type::parse_type;
use crate::from_text::interaction::parse_interaction;
use crate::from_text::data::generic::parse_data;
use crate::from_text::hibou_options::{HibouOptions,parse_hibou_options};

use crate::process::hibou_process::HibouPreFilter;
use crate::from_text::hsf_file::ProcessKind;

pub struct InterpretationItemPlan {
    pub lf_id : usize,
    pub vr_id : usize,
    pub assignment : ValueOrNewFresh
}

pub fn parse_setup(setup_pair : Pair<Rule>,
                   gen_ctx : &mut GeneralContext,
                   file_name : &str,
                   process_kind : &ProcessKind) -> Result<(Vec<InterpretationItemPlan>,HibouOptions),HibouParsingError> {
    // ***
    let mut got_section_explore_options   : bool = false;
    let mut got_section_analyze_options   : bool = false;
    let mut got_section_messages  : bool = false;
    let mut got_section_variables  : bool = false;
    let mut got_section_lifelines : bool = false;
    let mut got_section_init : bool = false;
    // ***
    let mut interpretation_plan : Vec<InterpretationItemPlan> = Vec::new();
    let mut hibou_options_opt : Option<HibouOptions> = None;
    // ***
    let mut contents = setup_pair.into_inner();
    while let Some(current_pair) = contents.next() {
        match current_pair.as_rule() {
            Rule::EXPLORE_OPTION_SECTION => {
                if got_section_explore_options {
                    return Err( HibouParsingError::HsfSetupError("several '@explore_option' sections declared".to_string()));
                }
                got_section_explore_options = true;
                match process_kind {
                    &ProcessKind::Explore => {
                        match parse_hibou_options(current_pair,file_name, process_kind) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( hoptions ) => {
                                hibou_options_opt = Some(hoptions);
                            }
                        }
                    },
                    _ => {}
                }
            },
            Rule::ANALYZE_OPTION_SECTION => {
                if got_section_analyze_options {
                    return Err( HibouParsingError::HsfSetupError("several '@analyze_option' sections declared".to_string()));
                }
                got_section_analyze_options = true;
                match process_kind {
                    &ProcessKind::Analyze => {
                        match parse_hibou_options(current_pair,file_name, process_kind) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( hoptions ) => {
                                hibou_options_opt = Some(hoptions);
                            }
                        }
                    },
                    _ => {}
                }
            },
            Rule::HIBOU_MODEL_MS_DECL => {
                if got_section_messages {
                    return Err( HibouParsingError::HsfSetupError("several '@message' sections declared".to_string()));
                }
                got_section_messages = true;
                parse_message_decl(current_pair,gen_ctx);
            },
            Rule::HIBOU_MODEL_VAR_DECL => {
                if got_section_variables {
                    return Err( HibouParsingError::HsfSetupError("several '@variable' sections declared".to_string()));
                }
                got_section_variables = true;
                parse_variable_decl(current_pair,gen_ctx);
            },
            Rule::HIBOU_MODEL_LF_DECL => {
                if got_section_lifelines {
                    return Err( HibouParsingError::HsfSetupError("several '@lifeline' sections declared".to_string()));
                }
                got_section_lifelines = true;
                parse_lifeline_decl(current_pair,gen_ctx);
            },
            Rule::HIBOU_MODEL_VAR_INIT => {
                if got_section_init {
                    return Err( HibouParsingError::HsfSetupError("several '@init' sections declared".to_string()));
                }
                got_section_init = true;
                match parse_initialization(current_pair,gen_ctx) {
                    Err(e) => {
                        return Err(e);
                    },
                    Ok( iip ) => {
                        interpretation_plan = iip;
                    }
                }
            },
            _ => {
                unreachable!();
            }
        }
    }
    match hibou_options_opt {
        None => {
            match process_kind {
                ProcessKind::Analyze => {
                    return Ok( (interpretation_plan,HibouOptions::default_analyze()) );
                },
                _ => {
                    return Ok( (interpretation_plan,HibouOptions::default_explore()) );
                }
            }
        },
        Some(hibou_options) => {
            return Ok( (interpretation_plan,hibou_options) );
        }
    }
}

fn parse_message_decl(ms_decl_pair : Pair<Rule>, gen_ctx : &mut GeneralContext) -> Result<(),HibouParsingError> {
    for msg_decl_pair in ms_decl_pair.into_inner() {
        let mut msg_decl_content = msg_decl_pair.into_inner();
        let ms_name : String = msg_decl_content.next().unwrap().as_str().chars().filter(|c| !c.is_whitespace()).collect();
        let mut ms_spec : Vec<(TD_DataType,Option<String>)> = Vec::new();
        match msg_decl_content.next() {
            None => {},
            Some( msg_pr_decl ) => {
                for pr_decl_pair in msg_pr_decl.into_inner() {
                    match pr_decl_pair.as_rule() {
                        Rule::PRM_DECLARATION => {
                            let mut var_decl_content = pr_decl_pair.into_inner();
                            let vr_name : String = var_decl_content.next().unwrap().as_str().chars().filter(|c| !c.is_whitespace()).collect();
                            let td_type_pair = var_decl_content.next().unwrap();
                            let (td_type, is_clock) = parse_type(td_type_pair);
                            if is_clock {
                                panic!("message parameter argument cannot be a clock");
                            }
                            ms_spec.push( (td_type,Some(vr_name)) );
                        },
                        _ => {
                            let (td_type, is_clock) = parse_type(pr_decl_pair);
                            if is_clock {
                                panic!("message parameter argument cannot be a clock");
                            }
                            ms_spec.push( (td_type,None) );
                        }
                    }
                }
            }
        }
        gen_ctx.add_msg(ms_name,ms_spec);
    }
    return Ok(());
}

fn parse_variable_decl(vr_decl_pair : Pair<Rule>, gen_ctx : &mut GeneralContext ) {
    for var_decl_pair in vr_decl_pair.into_inner() {
        let mut var_decl_content = var_decl_pair.into_inner();
        let vr_name : String = var_decl_content.next().unwrap().as_str().chars().filter(|c| !c.is_whitespace()).collect();
        let td_type_pair = var_decl_content.next().unwrap();
        let (td_type, is_clock) = parse_type(td_type_pair);
        let new_vr_id = gen_ctx.get_vr_num();
        gen_ctx.add_vr( vr_name.clone(), td_type);
        if is_clock {
            gen_ctx.add_as_clock( new_vr_id );
        }
    }
}

fn parse_lifeline_decl(lf_decl_pair : Pair<Rule>, gen_ctx : &mut GeneralContext ) {
    for lf_pair in lf_decl_pair.into_inner() {
        let lf_name : String = lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        gen_ctx.add_lf(lf_name);
    }
}

fn parse_initialization(init_pair : Pair<Rule>, gen_ctx : &GeneralContext ) -> Result<Vec<InterpretationItemPlan>, HibouParsingError> {
    let mut interpretation_plan : Vec<InterpretationItemPlan> = Vec::new();
    for var_init_pair in init_pair.into_inner() {
        let mut var_init_contents = var_init_pair.into_inner();
        // ***
        let lf_id : usize;
        {
            let lf_name_pair = var_init_contents.next().unwrap();
            let lf_name : String = lf_name_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_lf_id( &lf_name) {
                None => {
                    return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name) );
                },
                Some( got_lf_id ) => {
                    lf_id = got_lf_id;
                }
            }
        }
        // ***
        let vr_id : usize;
        {
            let vr_name_pair = var_init_contents.next().unwrap();
            let vr_name : String = vr_name_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_vr_id( &vr_name) {
                None => {
                    return Err( HibouParsingError::MissingVariableDeclarationError(vr_name) );
                },
                Some( got_vr_id ) => {
                    vr_id = got_vr_id;
                }
            }
        }
        // ***
        {
            let assignment_pair = var_init_contents.next().unwrap();
            match assignment_pair.as_rule() {
                Rule::TD_VALUE => {
                    match parse_data(gen_ctx,assignment_pair,&None) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( td_generic ) => {
                            interpretation_plan.push( InterpretationItemPlan{lf_id,vr_id,assignment:ValueOrNewFresh::Value(td_generic)});
                        }
                    }
                },
                Rule::NEW_FRESH => {
                    interpretation_plan.push( InterpretationItemPlan{lf_id,vr_id,assignment:ValueOrNewFresh::NewFresh});
                },
                _ => {
                    panic!("what rule then ? : {:?}", assignment_pair.as_rule() );
                }
            }
        }
        // ***
    }
    return Ok(interpretation_plan);
}