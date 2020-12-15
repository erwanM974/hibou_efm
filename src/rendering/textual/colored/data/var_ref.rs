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

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::data::var_ref::VariableReference;

use crate::rendering::hibou_color_palette::*;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::textual::convention::*;

impl ColoredTextable for VariableReference {
    fn to_colored_text(&self, gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext) -> Vec<TextToPrint> {
        match self {
            VariableReference::MSG_PARAMETER( ms_id, pr_id ) => {
                let text = format!("${:}",pr_id.to_string());
                return vec![ TextToPrint{text:text,color:Rgb(HC_MessageParameter)} ];
            },
            VariableReference::VARIABLE( vr_id ) => {
                let vr_name = exe_ctx.get_vr_name(gen_ctx, *vr_id).unwrap();
                return vec![ TextToPrint{text:vr_name,color:Rgb(HC_Variable)} ];
            },
            VariableReference::SYMBOL( symb_id ) => {
                let text = format!("{:}{:}",SYNTAX_NEWFRESH,symb_id.to_string());
                return vec![ TextToPrint{text:text,color:Rgb(HC_Symbol)} ];
            }
        }
    }
}