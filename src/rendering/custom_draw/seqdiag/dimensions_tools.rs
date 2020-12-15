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


use std::cmp;

use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::action::*;
use crate::core::syntax::interaction::{Interaction};

use crate::rendering::sd_drawing_conf::*;
use crate::rendering::custom_draw::seqdiag::recursive_frag::*;


pub fn get_interaction_max_yshift(interaction : &Interaction, exe_ctx : &ExecutionContext) -> usize {
    let mut cpt = 4;
    cpt += get_interaction_depth(interaction, exe_ctx);
    return cpt;
}

fn get_interaction_depth(interaction : &Interaction, exe_ctx : &ExecutionContext) -> usize {
    match interaction {
        &Interaction::Empty => {return  0},
        &Interaction::Action(ref act) => {
            match &act.act_kind {
                ObservableActionKind::Reception => {
                    return 4 + 2*(act.lf_act.preamble.len() + act.lf_act.postamble.len());
                },
                ObservableActionKind::Emission(targets) => {
                    let mut max_preamble_len : usize = act.lf_act.preamble.len();
                    let mut max_postamble_len : usize = act.lf_act.postamble.len();
                    // ***
                    for target in targets {
                        max_preamble_len = cmp::max(max_preamble_len, target.preamble.len());
                        max_postamble_len = cmp::max(max_postamble_len, target.postamble.len());
                    }
                    // ***
                    return 4 + 2*(max_preamble_len + max_postamble_len);
                }
            }
        },
        &Interaction::Strict(ref i1, ref i2) => {
            let mut frags = get_recursive_strict_frags(i1);
            frags.extend( get_recursive_strict_frags(i2) );
            let mut sum : usize = 2;
            for frag in frags {
                sum = sum + get_interaction_depth(frag,exe_ctx) + 2;
            }
            return sum;
        },
        &Interaction::Seq(ref i1, ref i2) => {
            return get_interaction_depth(i1,exe_ctx) + get_interaction_depth(i2,exe_ctx) + 1;
        },
        &Interaction::Alt(ref i1, ref i2) => {
            let mut frags = get_recursive_alt_frags(i1);
            frags.extend( get_recursive_alt_frags(i2) );
            let mut sum : usize = 2;
            for frag in frags {
                sum = sum + get_interaction_depth(frag,exe_ctx) + 2;
            }
            return sum;
        },
        &Interaction::Par(ref i1, ref i2) => {
            let mut frags = get_recursive_par_frags(i1);
            frags.extend( get_recursive_par_frags(i2) );
            let mut sum : usize = 2;
            for frag in frags {
                sum = sum + get_interaction_depth(frag,exe_ctx) + 2;
            }
            return sum;
        },
        &Interaction::Scope(_, ref i1) => {
            return get_interaction_depth(i1,exe_ctx) + 4;
        }
        &Interaction::Loop(_, ref i1) => {
            return get_interaction_depth(i1,exe_ctx) + 4;
        }
    }
}

pub fn get_y_pos_from_yshift(yshift : u32) -> f32 {
    return MARGIN + VERTICAL_SIZE*(yshift as f32);
}

