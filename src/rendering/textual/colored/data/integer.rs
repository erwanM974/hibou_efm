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
use crate::core::syntax::data::builtin::integer::*;
use crate::core::syntax::data::builtin::number::*;

use crate::rendering::hibou_color_palette::*;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::textual::convention::*;

impl ColoredTextable for TD_Integer {
    fn to_colored_text(&self, gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext) -> Vec<TextToPrint> {
        match self {
            TD_Integer::Value( ival ) => {
                return vec![ TextToPrint{text:ival.to_string(),color:Rgb(HC_Concrete_Value)} ];
            },
            TD_Integer::Minus( td_num ) => {
                let mut texts : Vec<TextToPrint> = Vec::new();
                texts.push( TextToPrint{text:"(".to_string(),color:Rgb(HC_Grammar_Symbol)});
                texts.push( TextToPrint{text:"-".to_string(),color:Rgb(HC_Grammar_Symbol)});
                texts.append( &mut td_num.to_colored_text(gen_ctx,exe_ctx));
                texts.push( TextToPrint{text:")".to_string(),color:Rgb(HC_Grammar_Symbol)});
                return texts;
            },
            TD_Integer::Factor( factors ) => {
                let mut texts : Vec<TextToPrint> = Vec::new();
                texts.push( TextToPrint{text:"(".to_string(),color:Rgb(HC_Grammar_Symbol)});
                let mut count : usize = 0;
                for (sign,td_num) in factors {
                    if count > 0 {
                        texts.push( TextToPrint{text:"*".to_string(),color:Rgb(HC_Grammar_Symbol)});
                    }
                    count = count +1;
                    match sign {
                        ARITH_FACTOR_SIGN::Div => {
                            texts.push( TextToPrint{text:"(1/".to_string(),color:Rgb(HC_Grammar_Symbol)});
                            texts.append( &mut td_num.to_colored_text(gen_ctx,exe_ctx));
                            texts.push( TextToPrint{text:")".to_string(),color:Rgb(HC_Grammar_Symbol)});
                        },
                        ARITH_FACTOR_SIGN::Mult => {
                            texts.append( &mut td_num.to_colored_text(gen_ctx,exe_ctx));
                        }
                    }
                }
                texts.push( TextToPrint{text:")".to_string(),color:Rgb(HC_Grammar_Symbol)});
                return texts;
            },
            TD_Integer::Add( adds ) => {
                let mut texts : Vec<TextToPrint> = Vec::new();
                texts.push( TextToPrint{text:"(".to_string(),color:Rgb(HC_Grammar_Symbol)});
                let mut count : usize = 0;
                for (sign,td_num) in adds {
                    match sign {
                        ARITH_ADD_SIGN::Plus => {
                            if count > 0 {
                                texts.push( TextToPrint{text:"+".to_string(),color:Rgb(HC_Grammar_Symbol)});
                            }
                        },
                        ARITH_ADD_SIGN::Minus => {
                            texts.push( TextToPrint{text:"-".to_string(),color:Rgb(HC_Grammar_Symbol)});
                        }
                    }
                    count = count +1;
                    texts.append( &mut td_num.to_colored_text(gen_ctx,exe_ctx));
                }
                texts.push( TextToPrint{text:")".to_string(),color:Rgb(HC_Grammar_Symbol)});
                return texts;
            },
            TD_Integer::Reference( var_ref ) => {
                return var_ref.to_colored_text(gen_ctx,exe_ctx);
            }
        }
    }
}