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

use crate::tools::fold_vec_to_string;

pub fn action_diversity_name(relative_position : &Vec<u32>) -> String {
    return format!("action_{}",fold_vec_to_string(relative_position));
}

pub fn message_parameter_diversity_name(gen_ctx : &GeneralContext, ms_id : usize, pr_id : usize) -> String {
    let ms_name= gen_ctx.get_ms_name(ms_id).unwrap();
    return format!("ms_{}_pr_{}",ms_name,pr_id);
}

pub fn trace_message_parameter_diversity_name(gen_ctx : &GeneralContext, ms_id : usize, pr_id : usize) -> String {
    let ms_name= gen_ctx.get_ms_name(ms_id).unwrap();
    return format!("trace_ms_{}_pr_{}",ms_name,pr_id);
}

pub fn variable_array_index_diversity_name(gen_ctx : &GeneralContext, vr_id : usize) -> String {
    let vr_name : String = gen_ctx.get_vr_name(vr_id).unwrap();
    return format!("index_{}",vr_name);
}

pub fn variable_diversity_name(gen_ctx : &GeneralContext, vr_id : usize) -> String {
    let vr_name : String = gen_ctx.get_vr_name(vr_id).unwrap();
    let index_name = variable_array_index_diversity_name(gen_ctx,vr_id);
    return format!("lf_var_{}[{}]",vr_name,index_name);
}

pub fn variable_vector_diversity_name(gen_ctx : &GeneralContext, vr_id : usize) -> String {
    let vr_name : String = gen_ctx.get_vr_name(vr_id).unwrap();
    return format!("lf_var_{}",vr_name);
}

pub fn variable_base_for_newfresh_diversity_name(gen_ctx : &GeneralContext, vr_id : usize) -> String {
    let vr_name : String = gen_ctx.get_vr_name(vr_id).unwrap();
    return format!("var_{}",vr_name);
}

