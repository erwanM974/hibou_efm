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


use crate::core::context::general::GeneralContext;
use crate::core::syntax::action::*;

use crate::core::context::execution::ExecutionContext;


pub fn action_to_text(action : &ObservableAction, gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext) -> String {
    let mut act_str : String = String::new();
    // ***
    let lf_name = gen_ctx.get_lf_name(action.lf_act.lf_id).unwrap();
    act_str.push_str( &lf_name );
    // ***
    match &action.act_kind {
        &ObservableActionKind::Reception => {
            act_str.push_str("?");
        },
        &ObservableActionKind::Emission( ref targets ) => {
            act_str.push_str("!");
            if targets.len() > 0 {
                act_str.push_str("(");
                let mut count : usize = 0;
                for tar_lf_act in targets {
                    let lf_name = gen_ctx.get_lf_name(tar_lf_act.lf_id).unwrap();
                    act_str.push_str( &lf_name );
                    count = count +1;
                    if count < targets.len() {
                        act_str.push_str(",");
                    }
                }
                act_str.push_str(")");
            }
        }
    }
    // ***
    match gen_ctx.get_ms_name(action.ms_id) {
        Err(e) => {
            println!("WARNING action message name not found for display");
            panic!();
        },
        Ok( ms_name ) => {
            act_str.push_str( &ms_name );
        }
    }
    // ***
    let prm_num = action.params.len();
    if prm_num > 0 {
        act_str.push_str("(");
        let mut current_prm : usize = 0;
        for prm_arg in &action.params {
            act_str.push_str( &exe_ctx.get_vr_name(gen_ctx,prm_arg.vr_id).unwrap() );
            current_prm = current_prm +1;
            if current_prm < prm_num {
                act_str.push_str( "," );
            }
        }
        act_str.push_str( ")" );
    }
    // ***
    return act_str;
}