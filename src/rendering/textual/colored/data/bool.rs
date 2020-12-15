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
use crate::core::syntax::data::builtin::bool::*;

use crate::rendering::hibou_color_palette::*;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::textual::convention::*;

impl ColoredTextable for TD_Bool {
    fn to_colored_text(&self, gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext) -> Vec<TextToPrint> {
        match self {
            TD_Bool::TRUE => {
                return vec![ TextToPrint{text:SYNTAX_LOGIC_TRUE.to_string(),color:Rgb(HC_Concrete_Value)} ];
            },
            TD_Bool::FALSE => {
                return vec![ TextToPrint{text:SYNTAX_LOGIC_FALSE.to_string(),color:Rgb(HC_Concrete_Value)} ];
            },
            TD_Bool::AND( bool_vec ) => {
                let mut texts : Vec<TextToPrint> = Vec::new();
                texts.push( TextToPrint{text:"(".to_string(),color:Rgb(HC_Grammar_Symbol)});
                for idx in 0..bool_vec.len() {
                    let td_bool = bool_vec.get(idx).unwrap();
                    texts.append(&mut td_bool.to_colored_text(gen_ctx,exe_ctx));
                    if idx < bool_vec.len()-1 {
                        texts.push( TextToPrint{text:SYNTAX_LOGIC_AND.to_string(),color:Rgb(HC_Grammar_Symbol)});
                    }
                }
                texts.push( TextToPrint{text:")".to_string(),color:Rgb(HC_Grammar_Symbol)});
                return texts;
            },
            TD_Bool::OR( bool_vec ) => {
                let mut texts : Vec<TextToPrint> = Vec::new();
                texts.push( TextToPrint{text:"(".to_string(),color:Rgb(HC_Grammar_Symbol)});
                for idx in 0..bool_vec.len() {
                    let td_bool = bool_vec.get(idx).unwrap();
                    texts.append(&mut td_bool.to_colored_text(gen_ctx,exe_ctx));
                    if idx < bool_vec.len()-1 {
                        texts.push( TextToPrint{text:SYNTAX_LOGIC_OR.to_string(),color:Rgb(HC_Grammar_Symbol)});
                    }
                }
                texts.push( TextToPrint{text:")".to_string(),color:Rgb(HC_Grammar_Symbol)});
                return texts;
            },
            TD_Bool::NOT( td_bool ) => {
                let mut texts : Vec<TextToPrint> = Vec::new();
                texts.push( TextToPrint{text:"(".to_string(),color:Rgb(HC_Grammar_Symbol)});
                texts.push( TextToPrint{text:SYNTAX_LOGIC_NOT.to_string(),color:Rgb(HC_Grammar_Symbol)});
                texts.append(&mut td_bool.to_colored_text(gen_ctx,exe_ctx));
                texts.push( TextToPrint{text:")".to_string(),color:Rgb(HC_Grammar_Symbol)});
                return texts;
            },
            TD_Bool::COMPARE( kind, td_generic1, td_generic2 ) => {
                let mut texts : Vec<TextToPrint> = Vec::new();
                texts.push( TextToPrint{text:"(".to_string(),color:Rgb(HC_Grammar_Symbol)});
                texts.append(&mut td_generic1.to_colored_text(gen_ctx,exe_ctx));
                match kind {
                    Bool_Compare::Different => {
                        texts.push( TextToPrint{text:SYNTAX_COMPARE_Diff.to_string(),color:Rgb(HC_Grammar_Symbol)});
                    },
                    Bool_Compare::Equal => {
                        texts.push( TextToPrint{text:SYNTAX_COMPARE_Eq.to_string(),color:Rgb(HC_Grammar_Symbol)});
                    },
                    Bool_Compare::Greater => {
                        texts.push( TextToPrint{text:SYNTAX_COMPARE_Gr.to_string(),color:Rgb(HC_Grammar_Symbol)});
                    },
                    Bool_Compare::GreaterOrEqual => {
                        texts.push( TextToPrint{text:SYNTAX_COMPARE_GrEq.to_string(),color:Rgb(HC_Grammar_Symbol)});
                    },
                    Bool_Compare::Lower => {
                        texts.push( TextToPrint{text:SYNTAX_COMPARE_Lr.to_string(),color:Rgb(HC_Grammar_Symbol)});
                    },
                    Bool_Compare::LowerOrEqual => {
                        texts.push( TextToPrint{text:SYNTAX_COMPARE_LrEq.to_string(),color:Rgb(HC_Grammar_Symbol)});
                    }
                }
                texts.append(&mut td_generic2.to_colored_text(gen_ctx,exe_ctx));
                texts.push( TextToPrint{text:")".to_string(),color:Rgb(HC_Grammar_Symbol)});
                return texts;
            },
            TD_Bool::Reference( var_ref ) => {
                return var_ref.to_colored_text(gen_ctx,exe_ctx);
            }
        }
    }
}