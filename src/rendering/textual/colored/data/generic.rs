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
use crate::core::syntax::data::generic::TD_Generic;


use crate::rendering::textual::colored::colored_text::*;


impl ColoredTextable for TD_Generic {
    fn to_colored_text(&self, gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext) -> Vec<TextToPrint> {
        match self {
            TD_Generic::Bool( td_bool ) => {
                return td_bool.to_colored_text(gen_ctx,exe_ctx);
            },
            TD_Generic::Integer( td_int ) => {
                return td_int.to_colored_text(gen_ctx,exe_ctx);
            },
            TD_Generic::Float( td_float ) => {
                return td_float.to_colored_text(gen_ctx,exe_ctx);
            },
            TD_Generic::String( td_str ) => {
                return td_str.to_colored_text(gen_ctx,exe_ctx);
            }/*,
            TD_Generic::Reference( var_ref ) => {
                return var_ref.to_colored_text(gen_ctx,exe_ctx);
            }*/
        }
    }
}