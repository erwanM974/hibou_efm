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
use crate::core::syntax::action::ValueOrNewFresh;
use crate::core::syntax::data::generic::TD_Generic;
use crate::rendering::textual::convention::*;
use crate::rendering::hibou_color_palette::*;
use crate::rendering::textual::colored::colored_text::*;

impl ColoredTextable for ValueOrNewFresh {
    fn to_colored_text(&self, gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext) -> Vec<TextToPrint> {
        match self {
            ValueOrNewFresh::NewFresh => {
                return vec![ TextToPrint{text:SYNTAX_NEWFRESH.to_string(),color:Rgb(HC_NewFresh)} ];
            },
            ValueOrNewFresh::Value( td_generic ) => {
                return td_generic.to_colored_text(gen_ctx,exe_ctx);
            }
        }
    }
}