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
use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::action::*;

use crate::core::syntax::position::{Position,SYNTAX_POSITION_EPSILON,SYNTAX_POSITION_LEFT,SYNTAX_POSITION_RIGHT};
use crate::rendering::textual::colored::colored_text::TextToPrint;
use crate::rendering::textual::colored::amble::diagram_revr_amble;
use crate::rendering::textual::colored::message::diagram_repr_message;
use crate::rendering::hibou_color_palette::*;

pub fn extract_texts_on_interaction(interaction : &Interaction,
                     gen_ctx : &GeneralContext,
                     exe_ctx : &ExecutionContext) -> (HashMap<usize,usize>,Vec<Vec<TextToPrint>>) {
    match interaction {
        Interaction::Empty => {
            return (HashMap::new(),Vec::new());
        },
        Interaction::Action( act ) => {
            // ***
            let mut lf_char_spaces : HashMap<usize,usize> = HashMap::new();
            let mut texts : Vec<Vec<TextToPrint>> = Vec::new();
            // ***
            let msg_to_print = diagram_repr_message(act.ms_id,&act.params,gen_ctx,exe_ctx);
            match lf_char_spaces.get(&act.lf_act.lf_id) {
                None => {
                    lf_char_spaces.insert(*(&act.lf_act.lf_id),TextToPrint::char_count(&msg_to_print));
                },
                Some( current_max_char ) => {
                    lf_char_spaces.insert(*(&act.lf_act.lf_id),cmp::max(*current_max_char,TextToPrint::char_count(&msg_to_print)));
                }
            }
            texts.push( msg_to_print );
            // ***
            {
                let (main_lf_chars, mut main_lf_to_print) = extract_texts_on_pre_post_amble(&act.lf_act.preamble, &act.lf_act.postamble, gen_ctx, exe_ctx);
                match lf_char_spaces.get(&act.lf_act.lf_id) {
                    None => {
                        lf_char_spaces.insert(*(&act.lf_act.lf_id),main_lf_chars);
                    },
                    Some( current_max_char ) => {
                        lf_char_spaces.insert(*(&act.lf_act.lf_id),cmp::max(*current_max_char,main_lf_chars ));
                    }
                }
                texts.append(&mut main_lf_to_print);
            }
            // ***
            match &act.act_kind {
                ObservableActionKind::Reception => {},
                ObservableActionKind::Emission( targets ) => {
                    for tar_lf_act in targets {
                        let (tar_lf_chars, mut tar_lf_to_print) = extract_texts_on_pre_post_amble(&tar_lf_act.preamble, &tar_lf_act.postamble,gen_ctx,exe_ctx);
                        match lf_char_spaces.get(&tar_lf_act.lf_id) {
                            None => {
                                lf_char_spaces.insert(tar_lf_act.lf_id,tar_lf_chars);
                            },
                            Some( current_max_char ) => {
                                lf_char_spaces.insert(tar_lf_act.lf_id,cmp::max(*current_max_char,tar_lf_chars));
                            }
                        }
                        texts.append(&mut tar_lf_to_print);
                    }
                }
            }
            // ***
            return (lf_char_spaces,texts);
        },
        Interaction::Strict(i1,i2) => {
            //let (height,lf_widths,texts) = extract_texts_in_binary_operator(i1,i2,gen_ctx,exe_ctx);
            return extract_texts_in_binary_operator(i1,i2,gen_ctx,exe_ctx);
        },
        Interaction::Seq(i1,i2) => {
            return extract_texts_in_binary_operator(i1,i2,gen_ctx,exe_ctx);
        },
        Interaction::Alt(i1,i2) => {
            return extract_texts_in_binary_operator(i1,i2,gen_ctx,exe_ctx);
        },
        Interaction::Par(i1,i2) => {
            return extract_texts_in_binary_operator(i1,i2,gen_ctx,exe_ctx);
        },
        Interaction::Loop(_,i1) => {
            return extract_texts_on_interaction(i1,gen_ctx,exe_ctx);
        },
        Interaction::Scope(vr_ids_vec,i1) => {
            let mut scope_text : Vec<TextToPrint> = Vec::new();
            let mut counter : usize = 0;
            for vr_id in vr_ids_vec {
                let vr_label: String = exe_ctx.get_vr_name(gen_ctx,*vr_id).unwrap();
                scope_text.push( TextToPrint{text:vr_label,color:Rgb(HC_Variable)} );
                counter = counter +1;
                if counter < vr_ids_vec.len() {
                    scope_text.push( TextToPrint{text:",".to_string(),color:Rgb(HC_Grammar_Symbol)} );
                }
            }
            let (inner_space,mut inner_texts) =  extract_texts_on_interaction(i1,gen_ctx,exe_ctx);
            let mut texts : Vec<Vec<TextToPrint>> = Vec::new();
            texts.push(scope_text);
            texts.append( &mut inner_texts);
            return (inner_space,texts);
        }
    }
}

fn extract_texts_in_binary_operator(i1:&Interaction,i2:&Interaction,gen_ctx:&GeneralContext,exe_ctx:&ExecutionContext) -> (HashMap<usize,usize>,Vec<Vec<TextToPrint>>) {
    let (mut i1_spaces,mut i1_texts) = extract_texts_on_interaction(i1,gen_ctx,exe_ctx);
    let (i2_spaces,mut i2_texts) = extract_texts_on_interaction(i2,gen_ctx,exe_ctx);
    // ***
    for (lf_id,space) in i2_spaces.iter() {
        match i1_spaces.get(lf_id) {
            None => {
                i1_spaces.insert( *lf_id, *space);
            },
            Some( other_space ) => {
                i1_spaces.insert( *lf_id, cmp::max(*space,*other_space));
            }
        }
    }
    // ***
    i1_texts.append(&mut i2_texts);
    return (i1_spaces,i1_texts);
}

fn extract_texts_on_pre_post_amble(preamble : &Vec<ActionAmbleItem>,
                                   postamble : &Vec<ActionAmbleItem>,
                                    gen_ctx : &GeneralContext,
                                    exe_ctx : &ExecutionContext) -> (usize,Vec<Vec<TextToPrint>>) {
    let mut max_on_lf : usize = 1; // min 1 because lf appears
    let mut to_print : Vec<Vec<TextToPrint>> = Vec::new();
    // ***
    let preamble_texts = diagram_revr_amble(preamble,gen_ctx,exe_ctx);
    for am_item_txt in preamble_texts {
        max_on_lf = cmp::max(max_on_lf, TextToPrint::char_count(&am_item_txt));
        to_print.push(am_item_txt);
    }
    // ***
    let postamble_texts = diagram_revr_amble(postamble,gen_ctx,exe_ctx);
    for am_item_txt in postamble_texts {
        max_on_lf = cmp::max(max_on_lf, TextToPrint::char_count(&am_item_txt));
        to_print.push(am_item_txt);
    }
    // ***
    return (max_on_lf,to_print);
}