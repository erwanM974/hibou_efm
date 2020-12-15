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
use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;

use crate::core::syntax::data::builtin::integer::TD_Integer;
use crate::core::syntax::data::builtin::bool::TD_Bool;
use crate::core::syntax::data::generic::TD_Generic;


pub fn deploy_original_action_followup(exe_ctx : &ExecutionContext,
                                       interaction : &Interaction,
                                       position : &Position,
                                       model_action : &ObservableAction,
                                       effective_parameters : &Vec<TD_Generic>) -> Interaction {
    let to_substitute : Interaction;
    match &model_action.act_kind {
        ObservableActionKind::Reception => {
            to_substitute = Interaction::Empty;
        },
        ObservableActionKind::Emission( targets ) => {
            let params = effective_parameters.iter().map(|x| ValueOrNewFresh::Value(x.clone()) ).collect();;
            to_substitute = deploy_receptions(exe_ctx,0,targets, &model_action.ms_id, &params, (model_action.original_position).as_ref().unwrap());
        }
    }
    return interaction.substitute(to_substitute,position);
}


fn deploy_receptions(exe_ctx : &ExecutionContext, index : usize, targets : &Vec<LifelineAction>, ms_id : &usize, params : &Vec<ValueOrNewFresh>, parent_original : &Vec<u32>) -> Interaction {
    let tar_len = targets.len();
    if index >= tar_len {
        return Interaction::Empty;
    } else if index == tar_len -1 {
        let lf_act = targets.get(index).unwrap();
        return deploy_lf_act(exe_ctx, lf_act, ms_id, params, parent_original,index as u32);
    } else {
        let head_lf_act = targets.get(index).unwrap();
        let head_int = deploy_lf_act(exe_ctx, head_lf_act, ms_id, params,parent_original,index as u32);
        match &head_int {
            &Interaction::Empty => {
                return deploy_receptions(exe_ctx,index+1,targets, ms_id, params,parent_original);
            },
            _ => {
                return Interaction::Par(Box::new(head_int),
                                        Box::new(deploy_receptions(exe_ctx,index+1,targets, ms_id, params,parent_original)));
            }
        }
    }
}

fn deploy_lf_act(exe_ctx : &ExecutionContext,
                 lf_act : &LifelineAction,
                 ms_id : &usize,
                 params : &Vec<ValueOrNewFresh>,
                 parent_original : &Vec<u32>,
                 index : u32) -> Interaction {
    let mut original = parent_original.clone();
    original.push(index + 1);
    return Interaction::Action( ObservableAction{
        lf_act:lf_act.clone(),
        act_kind:ObservableActionKind::Reception,
        ms_id:*ms_id,
        params:params.clone(),
        original_position:Some(original.clone()) });
}

