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

use image::Rgb;

use crate::core::syntax::action::*;
use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;

use crate::core::syntax::data::generic::TD_Generic;
use crate::rendering::textual::convention::*;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::hibou_color_palette::*;

use crate::rendering::textual::colored::message::{diagram_repr_message,diagram_repr_raw_message};
use crate::core::trace::{TraceAction,TraceActionKind};


pub fn diagram_repr_atomic_trace_action(action : &TraceAction,
                                        gen_ctx : &GeneralContext,
                                        exe_ctx : &ExecutionContext) -> Vec<TextToPrint> {
    let mut to_print : Vec<TextToPrint> = Vec::new();
    // ***
    match &action.delay {
        None => {
            // nothing
        },
        Some( delay_td_float ) => {
            to_print.push( TextToPrint{text:"[".to_string(),color:Rgb(HC_Grammar_Symbol)} );
            to_print.append(&mut delay_td_float.to_colored_text(gen_ctx,&exe_ctx) );
            to_print.push( TextToPrint{text:"].".to_string(),color:Rgb(HC_Grammar_Symbol)} );
        }
    }
    // ***
    {
        let lf_name: String = exe_ctx.get_lf_name(gen_ctx,action.lf_id).unwrap();
        to_print.push( TextToPrint{text:lf_name,color:Rgb(HC_Lifeline)} );
    }
    // ***
    match &action.act_kind {
        &TraceActionKind::Reception => {
            to_print.push( TextToPrint{text:SYNTAX_RECEPTION.to_string(),color:Rgb(HC_Grammar_Symbol)} );
        },
        &TraceActionKind::Emission => {
            to_print.push( TextToPrint{text:SYNTAX_EMISSION.to_string(),color:Rgb(HC_Grammar_Symbol)} );
        }
    }
    // ***
    to_print.append( &mut diagram_repr_raw_message(action.ms_id, &action.arguments,gen_ctx,exe_ctx));
    // ***
    return to_print;
}



pub fn diagram_repr_atomic_model_action(action : &ObservableAction,
                                        gen_ctx : &GeneralContext,
                                        exe_ctx : &ExecutionContext) -> Vec<TextToPrint> {
    let mut to_print : Vec<TextToPrint> = Vec::new();
    // ***
    {
        let lf_name: String = exe_ctx.get_lf_name(gen_ctx,action.lf_act.lf_id).unwrap();
        to_print.push( TextToPrint{text:lf_name,color:Rgb(HC_Lifeline)} );
    }
    // ***
    match &action.act_kind {
        &ObservableActionKind::Reception => {
            to_print.push( TextToPrint{text:SYNTAX_RECEPTION.to_string(),color:Rgb(HC_Grammar_Symbol)} );
        },
        &ObservableActionKind::Emission(_) => {
            to_print.push( TextToPrint{text:SYNTAX_EMISSION.to_string(),color:Rgb(HC_Grammar_Symbol)} );
        }
    }
    // ***
    to_print.append( &mut diagram_repr_message(action.ms_id, &action.params,gen_ctx,exe_ctx));
    // ***
    return to_print;
}