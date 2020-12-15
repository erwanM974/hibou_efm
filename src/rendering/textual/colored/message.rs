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

use crate::rendering::textual::convention::*;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::hibou_color_palette::*;
use crate::core::syntax::data::generic::TD_Generic;

pub fn diagram_repr_message(ms_id : usize,
                            params : &Vec<ValueOrNewFresh>,
                            gen_ctx : &GeneralContext,
                            exe_ctx : &ExecutionContext) -> Vec<TextToPrint> {
    let mut to_print: Vec<TextToPrint> = Vec::new();
    // ***
    match gen_ctx.get_ms_name(ms_id) {
        Err(_) => {
            println!("WARNING action message name not found for display");
            panic!();
        },
        Ok( ms_name ) => {
            to_print.push( TextToPrint{text:ms_name,color:Rgb(HC_Message)} );
        }
    }
    // ***
    let prm_num = params.len();
    if prm_num > 0 {
        to_print.push( TextToPrint{text:"(".to_string(),color:Rgb(HC_Grammar_Symbol)} );
        let mut current_prm : usize = 0;
        for value_or_new_fresh in params {
            match value_or_new_fresh {
                ValueOrNewFresh::Value( td_generic ) => {
                    to_print.append( &mut td_generic.to_colored_text(gen_ctx,exe_ctx));
                },
                ValueOrNewFresh::NewFresh => {
                    to_print.push( TextToPrint{text:SYNTAX_NEWFRESH.to_string(),color:Rgb(HC_NewFresh)} );
                }
            }
            current_prm = current_prm +1;
            if current_prm < prm_num {
                to_print.push( TextToPrint{text:",".to_string(),color:Rgb(HC_Grammar_Symbol)} );
            }
        }
        to_print.push( TextToPrint{text:")".to_string(),color:Rgb(HC_Grammar_Symbol)} );
    }
    return to_print;
}


pub fn diagram_repr_raw_message(ms_id : usize,
                            arguments : &Vec<TD_Generic>,
                            gen_ctx : &GeneralContext,
                            exe_ctx : &ExecutionContext) -> Vec<TextToPrint> {
    let mut to_print: Vec<TextToPrint> = Vec::new();
    // ***
    match gen_ctx.get_ms_name(ms_id) {
        Err(_) => {
            println!("WARNING action message name not found for display");
            panic!();
        },
        Ok( ms_name ) => {
            to_print.push( TextToPrint{text:ms_name,color:Rgb(HC_Message)} );
        }
    }
    // ***
    let prm_num = arguments.len();
    if prm_num > 0 {
        to_print.push( TextToPrint{text:"(".to_string(),color:Rgb(HC_Grammar_Symbol)} );
        let mut current_prm : usize = 0;
        for td_generic in arguments {
            to_print.append( &mut td_generic.to_colored_text(gen_ctx,exe_ctx));
            current_prm = current_prm +1;
            if current_prm < prm_num {
                to_print.push( TextToPrint{text:",".to_string(),color:Rgb(HC_Grammar_Symbol)} );
            }
        }
        to_print.push( TextToPrint{text:")".to_string(),color:Rgb(HC_Grammar_Symbol)} );
    }
    return to_print;
}