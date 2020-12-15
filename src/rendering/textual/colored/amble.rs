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


pub fn diagram_revr_amble(amble : &Vec<ActionAmbleItem>,
                                     gen_ctx : &GeneralContext,
                                     exe_ctx : &ExecutionContext) -> Vec<Vec<TextToPrint>> {
    let mut to_print_items : Vec<Vec<TextToPrint>> = Vec::new();
    for am_item in amble {
        match am_item {
            ActionAmbleItem::Guard( td_bool ) => {
                to_print_items.push( td_bool.to_colored_text(gen_ctx,exe_ctx) );
            },
            ActionAmbleItem::Reset( vr_id ) => {
                let mut inner : Vec<TextToPrint> = Vec::new();
                inner.push( TextToPrint{text:"reset ".to_string(),color:Rgb(HC_Grammar_Symbol)});
                let vr_name = exe_ctx.get_vr_name(gen_ctx, *vr_id).unwrap();
                inner.push(  TextToPrint{text:vr_name,color:Rgb(HC_Variable)} );
                to_print_items.push( inner );
            },
            ActionAmbleItem::Assignment( vr_id, valueornewfresh) => {
                let mut inner : Vec<TextToPrint> = Vec::new();
                let vr_name = exe_ctx.get_vr_name(gen_ctx, *vr_id).unwrap();
                inner.push(  TextToPrint{text:vr_name,color:Rgb(HC_Variable)} );
                inner.push( TextToPrint{text:":=".to_string(),color:Rgb(HC_Grammar_Symbol)});
                match valueornewfresh {
                    ValueOrNewFresh::Value( td_generic ) => {
                        inner.append( &mut td_generic.to_colored_text(gen_ctx,exe_ctx));
                    },
                    ValueOrNewFresh::NewFresh => {
                        inner.push( TextToPrint{text:SYNTAX_NEWFRESH.to_string(),color:Rgb(HC_NewFresh)} );
                    }
                }
                to_print_items.push( inner );
            }
        }
    }
    return to_print_items;
}