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
use std::collections::HashMap;
use image::Rgb;

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::trace::{MultiTraceCanal,AnalysableMultiTrace,TraceAction,TraceActionKind};

use crate::rendering::textual::colored::colored_text::TextToPrint;
use crate::rendering::textual::colored::short_action::diagram_repr_atomic_trace_action;
use crate::rendering::hibou_color_palette::*;

pub fn extract_texts_on_multi_trace(gen_ctx : &GeneralContext,
                                    exe_ctx : &ExecutionContext,
                                    multi_trace : &AnalysableMultiTrace) -> Vec<Vec<TextToPrint>> {
    let mut all_texts : Vec<Vec<TextToPrint>> = Vec::new();
    for trace_canal in &multi_trace.canals {
        let mut canal_text : Vec<TextToPrint> = Vec::new();
        // ***
        canal_text.push( TextToPrint{text:"[".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        let mut remaining_len = trace_canal.lifelines.len();
        for lf_id in &trace_canal.lifelines {
            let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
            canal_text.push( TextToPrint{text:lf_name, color:Rgb(HC_Lifeline)} );
            remaining_len = remaining_len -1;
            if remaining_len > 0 {
                canal_text.push( TextToPrint{text:",".to_string(), color:Rgb(HC_Grammar_Symbol)} );
            }
        }
        canal_text.push( TextToPrint{text:"] ← ".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        // ***
        let canal_len = trace_canal.trace.len();
        if canal_len > 0 {
            let first_act: &TraceAction = trace_canal.trace.get(0).unwrap();
            canal_text.append(&mut diagram_repr_atomic_trace_action(first_act, gen_ctx, exe_ctx)  );
            if canal_len > 1 {
                canal_text.push( TextToPrint{text:format!("...+{:}",(canal_len-1)), color:Rgb(HC_Grammar_Symbol)} );
            }
            /*
            let mut remaining = canal_len;
            for act in &(trace_canal.trace) {
                canal_text.append(&mut diagram_repr_atomic_trace_action(act, gen_ctx, exe_ctx)  );
                remaining = remaining -1;
                if remaining > 0 {
                    canal_text.push( TextToPrint{text:".".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                }
            }*/
        } else {
            canal_text.push( TextToPrint{text:"ε".to_string(), color:Rgb(HCP_LightGray)} );
        }
        canal_text.push( TextToPrint{text:" ".to_string(), color:Rgb(HCP_Black)} );
        all_texts.push(canal_text);
    }
    return all_texts;
}
