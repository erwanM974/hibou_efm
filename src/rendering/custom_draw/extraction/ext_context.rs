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
use std::collections::btree_map::BTreeMap;
use image::Rgb;

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;

use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::textual::colored::amble::diagram_revr_amble;
use crate::rendering::textual::colored::message::diagram_repr_message;
use crate::rendering::hibou_color_palette::*;


pub fn extract_texts_on_interpretation(gen_ctx : &GeneralContext,
                                    exe_ctx : &ExecutionContext) -> HashMap< usize,(usize,Vec<Vec<TextToPrint>>) > {
    let mut lfs_extractions : HashMap< usize,(usize,Vec<Vec<TextToPrint>>) > = HashMap::new();
    for lf_id in 0..gen_ctx.get_lf_num() {
        match exe_ctx.get_lf_interpretation(lf_id) {
            None => {},
            Some( lf_interpretation ) => {
                let mut init_texts_vec : Vec<Vec<TextToPrint>> = Vec::new();
                let mut max_text_size = 0;
                for (vr_id,td_generic) in lf_interpretation {
                    let mut to_print : Vec<TextToPrint> = Vec::new();
                    let vr_name = exe_ctx.get_vr_name(gen_ctx,*vr_id).unwrap();
                    to_print.push( TextToPrint{text:vr_name,color:Rgb(HC_Variable)});
                    to_print.push( TextToPrint{text:"=".to_string(),color:Rgb(HC_Grammar_Symbol)});
                    to_print.append( &mut td_generic.to_colored_text(gen_ctx,exe_ctx));
                    max_text_size = cmp::max(max_text_size, TextToPrint::char_count(&to_print));
                    init_texts_vec.push(to_print );
                }
                if init_texts_vec.len() > 0 {
                    lfs_extractions.insert(lf_id, (max_text_size,init_texts_vec));
                }
            }
        }
    }
    return lfs_extractions;
}