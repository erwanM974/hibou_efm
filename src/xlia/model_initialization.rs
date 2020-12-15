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

use std::collections::HashMap;
use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::data::generic::TD_Generic;
use crate::core::syntax::data::td_type::TD_DataType;
use crate::core::syntax::data::builtin::float::TD_Float;
use crate::core::syntax::data::builtin::integer::TD_Integer;
use crate::core::syntax::data::builtin::string::TD_String;
use crate::core::syntax::data::builtin::bool::TD_Bool;
use crate::core::syntax::data::var_ref::VariableReference;

use crate::tools::fold_vec_to_string;

use crate::xlia::xlia_build_name_tools::*;
use crate::xlia::data::{td_generic_to_xlia,td_bool_to_xlia};

use crate::process::hibou_process::HibouProcessTemporality;


pub fn make_lifeline_initialization_action(gen_ctx : &GeneralContext,
                                       exe_ctx : &ExecutionContext,
                                       lf_id : usize) -> String {
    let mut xlia_action_str : String = "\tmachine initialization {\n".to_string();
    // ***
    xlia_action_str.push_str("\t@moe:\n");
    xlia_action_str.push_str("\t\t@run{\n");
    xlia_action_str.push_str("\t\t// initialization of variables according to @init section of .hsf Hibou Specification File\n");
    // ***
    match exe_ctx.get_lf_interpretation(lf_id) {
        None => {},
        Some( interpretation ) => {
            for (vr_id,td_gen) in interpretation.iter() {
                xlia_action_str.push_str( &generate_xlia_vr_initialization(gen_ctx,exe_ctx,*vr_id,td_gen) );
            }
        }
    }
    // ***
    xlia_action_str.push_str("\t\t}\n");
    xlia_action_str.push_str("\t}\n");
    return xlia_action_str;
}


fn generate_xlia_vr_initialization(gen_ctx:&GeneralContext, exe_ctx:&ExecutionContext, vr_id : usize, td_gen: &TD_Generic) -> String {
    let variable_diversity_name = variable_diversity_name(gen_ctx,vr_id);
    match td_gen {
        TD_Generic::String(td_str) => {
            match td_str {
                TD_String::Reference(_) => {
                    let vr_base_for_newfresh_name = variable_base_for_newfresh_diversity_name(gen_ctx,vr_id);
                    return format!("\t\t\t{} = newfresh({});\n", variable_diversity_name, vr_base_for_newfresh_name);
                },
                _ => {
                    return format!("\t\t\t{} = {};\n",variable_diversity_name,td_generic_to_xlia(gen_ctx,td_gen));
                }/*
                TD_String::Value( raw_str ) => {
                    let value = format!("\"{}\"",raw_str);
                    return format!("\t\t\t{} = {};\n", variable_diversity_name, value) );
                },*/
            }
        },
        TD_Generic::Bool(td_bool) => {
            match td_bool {
                TD_Bool::Reference(_) => {
                    let vr_base_for_newfresh_name = variable_base_for_newfresh_diversity_name(gen_ctx,vr_id);
                    return format!("\t\t\t{} = newfresh({});\n", variable_diversity_name, vr_base_for_newfresh_name);
                },
                _ => {
                    return format!("\t\t\t{} = {};\n",variable_diversity_name,td_generic_to_xlia(gen_ctx,td_gen));
                }
            }
        },
        TD_Generic::Integer(td_int) => {
            match td_int {
                TD_Integer::Reference(_) => {
                    let vr_base_for_newfresh_name = variable_base_for_newfresh_diversity_name(gen_ctx,vr_id);
                    return format!("\t\t\t{} = newfresh({});\n", variable_diversity_name, vr_base_for_newfresh_name);
                },
                _ => {
                    return format!("\t\t\t{} = {};\n",variable_diversity_name,td_generic_to_xlia(gen_ctx,td_gen));
                }
            }
        },
        TD_Generic::Float(td_float) => {
            match td_float {
                TD_Float::Reference(_) => {
                    let vr_base_for_newfresh_name = variable_base_for_newfresh_diversity_name(gen_ctx,vr_id);
                    return format!("\t\t\t{} = newfresh({});\n", variable_diversity_name, vr_base_for_newfresh_name);
                },
                _ => {
                    return format!("\t\t\t{} = {};\n",variable_diversity_name,td_generic_to_xlia(gen_ctx,td_gen));
                }
            }
        }
    }
}

