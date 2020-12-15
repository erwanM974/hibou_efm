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
use crate::from_text::setup::{InterpretationItemPlan,parse_setup};

pub static HIBOU_MODEL_FILE_EXTENSION : &'static str = "hsf";

pub enum ProcessKind {
    Explore,
    Analyze,
    None
}

pub fn parse_hsf_file(file_path : &str, process_kind : &ProcessKind) -> Result<(GeneralContext,ExecutionContext,Interaction,HibouOptions),HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_MODEL_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_MODEL_FILE_EXTENSION.to_string()));
    }
    let file_name : &str = path_object.file_stem().unwrap().to_str().unwrap();
    match fs::read_to_string(file_path) {
        Ok( unparsed_hsf_str ) => {
            return parse_hsf_string(unparsed_hsf_str, file_name, process_kind);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}

pub fn parse_hsf_string(sd_string : String,
                        name : &str,
                        process_kind : &ProcessKind) -> Result<(GeneralContext,ExecutionContext,Interaction,HibouOptions),HibouParsingError> {
    match SDParser::parse(Rule::HSF_PEST_FILE, &sd_string) {
        Ok( ref mut sd_cfg_pair ) => {
            let mut content = sd_cfg_pair.next().unwrap().into_inner();
            let first_pair = content.next().unwrap();
            match first_pair.as_rule() {
                Rule::HIBOU_MODEL_SETUP => {
                    let second_pair = content.next().unwrap();
                    return parse_sd(second_pair,Some(first_pair),name,process_kind);
                },
                Rule::SD_INTERACTION => {
                    return parse_sd(first_pair, None,name,process_kind);
                },
                _ => {
                    unreachable!();
                }
            }
        },
        Err(e) => {
            return Err( HibouParsingError::MatchError(e.to_string()) );
        }
    }
}


fn parse_sd(interaction_pair : Pair<Rule>,
            setup_pair_opt : Option< Pair<Rule> >,
            name : &str,
            process_kind : &ProcessKind)
        -> Result<(GeneralContext,ExecutionContext,Interaction,HibouOptions),HibouParsingError> {
    let mut gen_ctx = GeneralContext::new();
    let interpretation_plan : Vec<InterpretationItemPlan>;
    let hibou_options : HibouOptions;
    match setup_pair_opt {
        None => {
            interpretation_plan = Vec::new();
            match process_kind {
                ProcessKind::Analyze => {
                    hibou_options = HibouOptions::default_analyze();
                },
                _ => {
                    hibou_options = HibouOptions::default_explore();
                }
            }
        },
        Some( setup_pair ) => {
            match parse_setup(setup_pair, &mut gen_ctx, name, process_kind) {
                Err(e) => {
                    return Err(e);
                },
                Ok( (intplan, hoptions) ) => {
                    interpretation_plan = intplan;
                    hibou_options = hoptions;
                }
            }
        }
    }
    match parse_interaction(&gen_ctx, interaction_pair) {
        Err(e) => {
            return Err(e);
        },
        Ok( interaction ) => {
            let mut interpretation : BTreeMap<usize, BTreeMap<usize,TD_Generic> > = BTreeMap::new();
            let mut symb_count : usize = 1;
            let mut symb_types : BTreeMap<usize,TD_DataType> = BTreeMap::new();
            for iip in interpretation_plan {
                let mut interpretation_on_lf : BTreeMap<usize,TD_Generic>;
                match interpretation.get(&iip.lf_id) {
                    None => {
                        interpretation_on_lf = BTreeMap::new();
                    },
                    Some( got_lf_interpretation ) => {
                        interpretation_on_lf = got_lf_interpretation.clone();
                    }
                }
                // ***
                match &iip.assignment {
                    ValueOrNewFresh::Value( ref td_generic ) => {
                        interpretation_on_lf.insert( iip.vr_id, td_generic.clone() );
                    },
                    ValueOrNewFresh::NewFresh => {
                        match gen_ctx.get_vr_type(iip.vr_id) {
                            Err(e) => {
                                panic!();
                            },
                            Ok( vr_type ) => {
                                let sy_ref = VariableReference::SYMBOL(symb_count);
                                match vr_type {
                                    TD_DataType::Float => {
                                        interpretation_on_lf.insert( iip.vr_id, TD_Generic::Float( TD_Float::Reference(sy_ref)) );
                                    },
                                    TD_DataType::Integer => {
                                        interpretation_on_lf.insert( iip.vr_id, TD_Generic::Integer( TD_Integer::Reference(sy_ref)) );
                                    },
                                    TD_DataType::Bool => {
                                        interpretation_on_lf.insert( iip.vr_id, TD_Generic::Bool( TD_Bool::Reference(sy_ref)) );
                                    },
                                    TD_DataType::String => {
                                        interpretation_on_lf.insert( iip.vr_id, TD_Generic::String( TD_String::Reference(sy_ref)) );
                                    }
                                }
                                symb_types.insert(symb_count, vr_type);
                                symb_count = symb_count +1;
                            }
                        }
                    }
                }
                interpretation.insert( iip.lf_id, interpretation_on_lf);
            }
            let exe_ctx = ExecutionContext::new(&gen_ctx,interpretation,symb_count);
            return Ok( (gen_ctx,exe_ctx,interaction.decorate_with_initial_positions(Vec::new() ),hibou_options) );
        }
    }
}