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

use std::env;
use std::collections::{HashMap,HashSet};
use std::cmp;

// **********

use image::{Rgb, RgbImage};
use imageproc::drawing::{
    Point,
    draw_cross_mut,
    draw_line_segment_mut,
    draw_hollow_rect_mut,
    draw_filled_rect_mut,
    draw_hollow_circle_mut,
    draw_filled_circle_mut,
    draw_convex_polygon_mut,
    draw_text_mut
};
use rusttype::{FontCollection, Scale};

// **********

use crate::core::syntax::interaction::{Interaction,ScheduleOperatorKind};
use crate::core::syntax::action::*;
use crate::core::context::general::GeneralContext;

use crate::core::context::execution::ExecutionContext;

use crate::rendering::sd_drawing_conf::*;
use crate::rendering::textual::convention::*;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::custom_draw::seqdiag::recursive_frag::*;
use crate::rendering::custom_draw::seqdiag::dimensions_tools::*;
use crate::rendering::custom_draw::seqdiag::action::draw_action;
use crate::rendering::custom_draw::seqdiag::lf_coords::DrawingLifelineCoords;
use crate::rendering::hibou_color_palette::*;
use crate::rendering::custom_draw::utils::colored_text::draw_colored_text;
// **********



// **********

pub fn draw_interaction_rec(    image : &mut RgbImage,
                                gen_ctx : &GeneralContext,
                                exe_ctx : &ExecutionContext,
                                interaction : &Interaction,
                                lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                texts_to_print : &mut Vec<Vec<TextToPrint>>,
                                nest_shift : &mut u32,
                                yshift : &mut u32)
                        -> [usize;2] { // returns left and right borders of the interaction
    match interaction {
        &Interaction::Empty => {
            return [gen_ctx.get_lf_num(),0]; // because when going up we keep the minimum on the left and maximum on the right
        },
        &Interaction::Action(ref act) => {
            *yshift = *yshift +1;
            let (new_yshift,lr_bounds) = draw_action(image,exe_ctx,act,lf_x_widths,texts_to_print,*yshift);
            *yshift = new_yshift + 1;
            return lr_bounds;
        },
        &Interaction::Seq(ref i1,ref i2) => {
            let wr1 : [usize;2] = draw_interaction_rec(image, gen_ctx, exe_ctx,i1, lf_x_widths, texts_to_print, nest_shift, yshift);
            *yshift = *yshift +1;
            let wr2 : [usize;2] = draw_interaction_rec(image,  gen_ctx, exe_ctx,i2, lf_x_widths, texts_to_print, nest_shift, yshift);
            return [ std::cmp::min(wr1[0],wr2[0]) , std::cmp::max(wr1[1],wr2[1]) ];
        },
        &Interaction::Strict(ref i1,ref i2) => {
            let mut frags = get_recursive_strict_frags(i1);
            frags.extend( get_recursive_strict_frags(i2) );
            let label = vec![TextToPrint{text:SYNTAX_STRICT.to_string(),color:Rgb(HCP_Black)}];
            return draw_n_ary_combined_fragment(image, gen_ctx, exe_ctx,frags,lf_x_widths,texts_to_print, label, nest_shift, yshift);
        },
        &Interaction::Alt(ref i1,ref i2) => {
            let mut frags = get_recursive_alt_frags(i1);
            frags.extend( get_recursive_alt_frags(i2) );
            let label = vec![TextToPrint{text:SYNTAX_ALT.to_string(),color:Rgb(HCP_Black)}];
            return draw_n_ary_combined_fragment(image, gen_ctx, exe_ctx,frags,lf_x_widths,texts_to_print, label, nest_shift, yshift);
        },
        &Interaction::Par(ref i1,ref i2) => {
            let mut frags = get_recursive_par_frags(i1);
            frags.extend( get_recursive_par_frags(i2) );
            let label = vec![TextToPrint{text:SYNTAX_PAR.to_string(),color:Rgb(HCP_Black)}];
            return draw_n_ary_combined_fragment(image, gen_ctx, exe_ctx,frags,lf_x_widths,texts_to_print, label, nest_shift, yshift);
        },
        &Interaction::Scope(_, ref i1) => {
            let mut label : Vec<TextToPrint> = Vec::new();
            label.push( TextToPrint{text:"scope{".to_string(),color:Rgb(HCP_Black)});
            let mut scope_content = texts_to_print.remove(0);
            label.append( &mut scope_content);
            label.push( TextToPrint{text:"}".to_string(),color:Rgb(HCP_Black)});
            return draw_unary_combined_fragment(image,  gen_ctx, exe_ctx,i1,lf_x_widths,texts_to_print, label, nest_shift, yshift);
        },
        &Interaction::Loop(ref lkind, ref i1) => {
            match lkind {
                ScheduleOperatorKind::Strict => {
                    let label = vec![TextToPrint{text:SYNTAX_LOOPX.to_string(),color:Rgb(HCP_Black)}];
                    return draw_unary_combined_fragment(image,  gen_ctx, exe_ctx,i1,lf_x_widths,texts_to_print, label, nest_shift, yshift);
                },
                ScheduleOperatorKind::Seq => {
                    let label = vec![TextToPrint{text:SYNTAX_LOOPH.to_string(),color:Rgb(HCP_Black)}];
                    return draw_unary_combined_fragment(image,  gen_ctx, exe_ctx,i1,lf_x_widths,texts_to_print, label, nest_shift, yshift);
                },
                ScheduleOperatorKind::Par => {
                    let label = vec![TextToPrint{text:SYNTAX_LOOPP.to_string(),color:Rgb(HCP_Black)}];
                    return draw_unary_combined_fragment(image,  gen_ctx, exe_ctx,i1,lf_x_widths,texts_to_print, label, nest_shift, yshift);
                }
            }
        }
    }
}

fn draw_unary_combined_fragment(    image : &mut RgbImage,
                                    gen_ctx : &GeneralContext,
                                    exe_ctx : &ExecutionContext,
                                    i1 : &Interaction,
                                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                    texts_to_print : &mut Vec<Vec<TextToPrint>>,
                                    label : Vec<TextToPrint>,
                                    nest_shift : &mut u32,
                                    yshift : &mut u32) -> [usize;2] {
    // draw content and gather data
    *nest_shift += 1;
    let start_y : u32 = *yshift;
    *yshift += 3;
    let lr_bounds : [usize;2] = draw_interaction_rec(image,  gen_ctx, exe_ctx,i1, lf_x_widths, texts_to_print, nest_shift, yshift);
    *yshift += 1;
    let end_y : u32 = *yshift;
    *nest_shift -= 1;
    // draw frame
    let mut y_drafts : Vec<u32> = [start_y,end_y].to_vec();
    draw_combined_fragment_frame(image, label, *nest_shift,lf_x_widths,lr_bounds[0],lr_bounds[1],y_drafts);
    return lr_bounds;
}

fn draw_n_ary_combined_fragment(  image : &mut RgbImage,
                                  gen_ctx : &GeneralContext,
                                  exe_ctx : &ExecutionContext,
                                  sub_ints : Vec<&Interaction>,
                                  lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                  texts_to_print : &mut Vec<Vec<TextToPrint>>,
                                  label : Vec<TextToPrint>,
                                  nest_shift : &mut u32,
                                  yshift : &mut u32) -> [usize;2] {
    let mut y_drafts : Vec<u32> = Vec::new();
    // draw content and gather data
    *nest_shift += 1;
    y_drafts.push(*yshift);
    *yshift += 2;
    //
    let mut min_lf_id : usize = gen_ctx.get_lf_num();
    let mut max_lf_id : usize = 0;
    for my_int in sub_ints {
        *yshift += 1;
        let lr_bounds = draw_interaction_rec(image,  gen_ctx, exe_ctx,my_int, lf_x_widths, texts_to_print, nest_shift, yshift);
        min_lf_id = cmp::min( min_lf_id, lr_bounds[0]);
        max_lf_id = cmp::max( max_lf_id, lr_bounds[1]);
        *yshift += 1;
        y_drafts.push(*yshift);
    }
    *nest_shift -= 1;
    //
    let lr_bounds: [usize;2] = [ min_lf_id, max_lf_id ];
    // draw frame
    draw_combined_fragment_frame(image,label,*nest_shift,lf_x_widths,lr_bounds[0],lr_bounds[1],y_drafts);
    return lr_bounds;
}

fn draw_combined_fragment_frame(    image : &mut RgbImage,
                                    label : Vec<TextToPrint>,
                                    nest_shift : u32,
                                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                    left_bound : usize,
                                    right_bound : usize,
                                    y_drafts : Vec<u32>) {
    let left_lf_coords = lf_x_widths.get(&left_bound).unwrap();
    let x_left : f32 = left_lf_coords.x_start + (nest_shift as f32)*FRAGMENT_PADDING;
    let right_lf_coords = lf_x_widths.get(&right_bound).unwrap();
    let x_right : f32 = (right_lf_coords.x_start + right_lf_coords.x_span_outer) - (nest_shift as f32)*FRAGMENT_PADDING;

    let mut y_coords : Vec<f32> = y_drafts.into_iter().map(|y| get_y_pos_from_yshift(y) ).collect::< Vec<f32> >();
    let y_start : f32 = y_coords[0];// + *nest_shift*FRAGMENT_PADDING;
    y_coords.drain(0..0);
    let y_end : f32 = y_coords.pop().unwrap();// - (nest_shift as f32)*FRAGMENT_PADDING;
    draw_line_segment_mut(image,
                          (x_left, y_start),
                          (x_left, y_end),
                          Rgb(HCP_Black));
    draw_line_segment_mut(image,
                          (x_right, y_start),
                          (x_right, y_end),
                          Rgb(HCP_Black));
    draw_line_segment_mut(image,
                          (x_left, y_start),
                          (x_right, y_start),
                          Rgb(HCP_Black));
    draw_line_segment_mut(image,
                          (x_left, y_end),
                          (x_right, y_end),
                          Rgb(HCP_Black));
    for y_coord in y_coords {
        draw_line_segment_mut(image,
                              (x_left, y_coord),
                              (x_right, y_coord),
                              Rgb(HCP_Black));
    }
    let font = FontCollection::from_bytes(HIBOU_GRAPHIC_FONT).unwrap().into_font().unwrap();

    let scale = Scale { x: FONT_WIDTH, y: FONT_HEIGHT };
    draw_colored_text(image,&label,x_left+FRAGMENT_TITLE_MARGIN,y_start + VERTICAL_SIZE);
}






