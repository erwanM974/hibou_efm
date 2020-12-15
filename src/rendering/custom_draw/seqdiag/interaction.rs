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

use std::collections::HashSet;
use std::cmp;
use std::path::Path;
use std::collections::HashMap;

// **********

use image::Rgb;
use image::RgbImage;

// **********

use crate::core::syntax::interaction::*;
use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::trace::*;

use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::sd_drawing_conf::*;
use crate::rendering::custom_draw::seqdiag::dimensions_tools::*;
use crate::rendering::custom_draw::seqdiag::img_frame::*;
use crate::rendering::custom_draw::seqdiag::img_content::*;
use crate::rendering::custom_draw::extraction::ext_interaction::extract_texts_on_interaction;
use crate::rendering::custom_draw::extraction::ext_context::extract_texts_on_interpretation;
use crate::rendering::custom_draw::extraction::ext_multi_trace::extract_texts_on_multi_trace;
use crate::rendering::custom_draw::seqdiag::lf_coords::DrawingLifelineCoords;
use crate::rendering::hibou_color_palette::*;
use crate::rendering::custom_draw::utils::colored_text::draw_colored_text;
// **********



pub fn draw_interaction(path_str : &String,
                        interaction : &Interaction,
                        gen_ctx : &GeneralContext,
                        exe_ctx : &ExecutionContext,
                        remaining_multi_trace : &Option<AnalysableMultiTrace>) {
    let path = Path::new( path_str );
    // ***
    let (mut lf_char_widths,mut texts_to_print) = extract_texts_on_interaction(interaction,gen_ctx,exe_ctx);
    // ***
    let mut additional_y_space_for_context : usize = 0;
    let init_lf_interpretations = extract_texts_on_interpretation(gen_ctx,exe_ctx);
    let lf_keys : Vec<usize> = lf_char_widths.keys().cloned().collect();
    for lf_id in lf_keys {
        let mut lf_additional_y_space : usize = 0;
        match init_lf_interpretations.get(&lf_id) {
            None => {},
            Some( (width_from_context,lf_init_texts) ) => {
                lf_additional_y_space = lf_additional_y_space + lf_init_texts.len();
                // ***
                let lf_cwidth = lf_char_widths.get(&lf_id).unwrap();
                lf_char_widths.insert( lf_id, cmp::max(*lf_cwidth,*width_from_context) );
            }
        }
        additional_y_space_for_context = cmp::max(additional_y_space_for_context,lf_additional_y_space);
    }
    // ***
    let mut lf_x_widths : HashMap<usize,DrawingLifelineCoords> = HashMap::new();
    let mut current_x : f32 = MARGIN;
    for lf_id in 0..gen_ctx.get_lf_num() {
        match lf_char_widths.get(&lf_id) {
            None => {},
            Some(lf_char_width) => {
                let mut max_lf_char_width : usize = *lf_char_width;
                let span_inner = (HORIZONTAL_SIZE - 2.0*MARGIN).max( 2.0*MARGIN + (*lf_char_width as f32)*FONT_WIDTH/2.0 );
                let span_outer = span_inner + 2.0*MARGIN;
                let middle = current_x + (span_outer/2.0) + THICKNESS;
                lf_x_widths.insert(lf_id,DrawingLifelineCoords{x_start:current_x,
                    x_span_inner:span_inner,
                    x_span_outer:span_outer,
                    x_middle:middle});
                current_x = current_x + span_outer + MARGIN;
            }
        }
    }
    //current_x = current_x + MARGIN;
    // ***
    let max_y_shift = 2 + get_interaction_max_yshift(interaction,exe_ctx) + additional_y_space_for_context*2;
    let mut inner_height : f32 = (max_y_shift as f32)*VERTICAL_SIZE;

    // ***
    let mut img_width : f32 = current_x;
    let multi_trace_txttoprint : Option<(Vec<Vec<TextToPrint>>,f32)>;
    match remaining_multi_trace {
        None => {
            multi_trace_txttoprint = None;
        }
        Some( multi_trace ) => {
            let mt_ttp = extract_texts_on_multi_trace( gen_ctx, exe_ctx, multi_trace);
            let mut max_char_count = 0;
            for ttp in &mt_ttp {
                max_char_count = max_char_count.max(TextToPrint::char_count(ttp) );
            }
            let mt_print_width = (max_char_count as f32)*FONT_WIDTH/2.0;
            img_width = current_x + mt_print_width;
            // ***
            inner_height = inner_height.max( ((2*mt_ttp.len()) as f32)*VERTICAL_SIZE );
            // ***
            multi_trace_txttoprint = Some( (mt_ttp,mt_print_width) );
        }
    }
    // ***
    let mut path_condition_ttp : Vec<TextToPrint>;
    {
        path_condition_ttp = vec![TextToPrint{text:"π=".to_string(),color:Rgb(HCP_Black)}];
        path_condition_ttp.append(&mut (exe_ctx.get_path_condition()).to_colored_text(gen_ctx,exe_ctx) );
        let mt_print_width = (TextToPrint::char_count(&path_condition_ttp) as f32)*FONT_WIDTH/2.0;
        img_width = img_width.max(2.0*MARGIN + mt_print_width);
    }
    // ***
    let img_height : f32 = inner_height + 2.0*MARGIN;

    // Draw Frame
    let mut image = RgbImage::new( img_width as u32, img_height as u32);
    draw_frame(&mut image, &img_width, &img_height, max_y_shift);

    // Draw Path Condition
    {
        let path_condition_x_pos = img_width/2.0 - (TextToPrint::char_count(&path_condition_ttp) as f32)*FONT_WIDTH/4.0;
        draw_colored_text(&mut image,&path_condition_ttp, path_condition_x_pos,MARGIN);
    }

    // Draw Lifelines
    draw_lifelines(&mut image, &lf_x_widths, inner_height, gen_ctx, exe_ctx, init_lf_interpretations);

    // Draw Fragments
    let mut nest_shift : u32 = 1; // shift to display nested fragments
    let mut yshift : u32 = 5 + ((additional_y_space_for_context*2) as u32);

    draw_interaction_rec(&mut image,  gen_ctx, exe_ctx, interaction, &lf_x_widths, &mut texts_to_print,&mut nest_shift, &mut yshift);

    match multi_trace_txttoprint {
        None => {},
        Some( (mt_ttp,mt_print_width) ) => {
            let mut yshift : u32 = 2;
            for text in mt_ttp {
                let msg_x_pos = img_width - mt_print_width - MARGIN/2.0;
                let msg_y_pos = MARGIN + (yshift as f32)*VERTICAL_SIZE;
                draw_colored_text(&mut image,&text,msg_x_pos,msg_y_pos);
                yshift = yshift +2;
            }
        }
    }

    image.save(path).unwrap();
}