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

pub fn action_diversity_fqn(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, relative_position : &Vec<u32>) -> String {
    let lf_name = exe_ctx.get_lf_name(gen_ctx,lf_id).unwrap();
    return format!("{}.action_{}",lf_name,fold_vec_to_string(relative_position));
}

pub fn trace_action_compare_diversity_fqn(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, ms_id : usize) -> String {
    let lf_name = exe_ctx.get_lf_name(gen_ctx,lf_id).unwrap();
    let ms_name= gen_ctx.get_ms_name(ms_id).unwrap();
    return format!("{}.action_compare_ms_{}",lf_name,ms_name);
}

pub fn open_scopes_action_diversity_fqn(gen_ctx : &GeneralContext, lf_id : usize) -> String {
    let lf_name = gen_ctx.get_lf_name(lf_id).unwrap();
    return format!("{}.action_open_scopes",lf_name);
}

pub fn message_parameter_diversity_fqn(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, ms_id : usize, pr_id : usize) -> String {
    let lf_name = exe_ctx.get_lf_name(gen_ctx,lf_id).unwrap();
    let ms_name= gen_ctx.get_ms_name(ms_id).unwrap();
    return format!("{}.ms_{}_pr_{}",lf_name,ms_name,pr_id);
}

pub fn trace_message_parameter_diversity_fqn(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, ms_id : usize, pr_id : usize) -> String {
    let lf_name = exe_ctx.get_lf_name(gen_ctx,lf_id).unwrap();
    let ms_name= gen_ctx.get_ms_name(ms_id).unwrap();
    return format!("{}.trace_ms_{}_pr_{}",lf_name,ms_name,pr_id);
}

pub fn trace_delay_diversity_fqn(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize) -> String {
    let lf_name = exe_ctx.get_lf_name(gen_ctx,lf_id).unwrap();
    return format!("{}.trace_delay",lf_name);
}

pub fn variable_diversity_fqn(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, vr_id : usize) -> String {
    let lf_name = exe_ctx.get_lf_name(gen_ctx,lf_id).unwrap();
    let (parent_var_name, var_index) = exe_ctx.get_vr_parent_name_and_child_id(gen_ctx,vr_id).unwrap();
    return format!("{}.lf_var_{}[{}]",lf_name,parent_var_name,var_index);
}

pub fn varindex_diversity_fqn(gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext, lf_id : usize, vr_id : usize) -> String {
    let lf_name = exe_ctx.get_lf_name(gen_ctx,lf_id).unwrap();
    let (parent_var_name,_) = exe_ctx.get_vr_parent_name_and_child_id(gen_ctx,vr_id).unwrap();
    return format!("{}.index_{}",lf_name,parent_var_name);
}