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
use std::env;
use std::collections::{HashMap,HashSet};

// **********

use image::{Rgb, RgbImage};
use imageproc::rect::Rect;
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

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::action::*;


use crate::rendering::sd_drawing_conf::*;
use crate::rendering::hibou_color_palette::*;

use crate::rendering::custom_draw::seqdiag::dimensions_tools::*;
use crate::rendering::textual::colored::colored_text::*;

use crate::rendering::custom_draw::utils::colored_text::draw_colored_text;
use crate::rendering::custom_draw::utils::arrow_heads::*;
use crate::rendering::custom_draw::seqdiag::lf_coords::DrawingLifelineCoords;

// **********

struct LfActToDraw{
    lf_id : usize,
    preamble_len : usize,
    postamble_len : usize,
    texts_to_print : Vec<Vec<TextToPrint>>,
    max_txt_width : f32
}

impl LfActToDraw {
    pub fn new(lf_id : usize,
               preamble_len : usize,
               postamble_len : usize,
               texts_to_print : Vec<Vec<TextToPrint>>,
               max_txt_width : f32) -> LfActToDraw {
        return LfActToDraw{
            lf_id,
            preamble_len,
            postamble_len,
            texts_to_print,
            max_txt_width
        }
    }
}

pub fn draw_action( image : &mut RgbImage,
                    exe_ctx: &ExecutionContext,
                    action : &ObservableAction,
                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                    texts_to_print : &mut Vec<Vec<TextToPrint>>,
                    yshift : u32) -> (u32,[usize;2]) {

    let mut max_preamble_len : usize = action.lf_act.preamble.len();
    let mut max_postamble_len : usize = action.lf_act.postamble.len();
    // ***
    let mut min_lf_id : usize = action.lf_act.lf_id;
    let mut max_lf_id : usize = action.lf_act.lf_id;
    // ***
    let msg_to_print = texts_to_print.remove(0);
    let mut lf_actions : Vec<LfActToDraw> = Vec::new();
    let mut arr_kinds : Vec<ActionArrowKind> = Vec::new();
    {
        let mut lf_act_texts : Vec<Vec<TextToPrint>> = Vec::new();
        let mut max_txt_width : f32 = (TextToPrint::char_count(&msg_to_print) as f32)*FONT_WIDTH/2.0;
        for txt_idx in 0..(action.lf_act.preamble.len() + action.lf_act.postamble.len()) {
            let got_txt = texts_to_print.remove(0);
            max_txt_width = max_txt_width.max( (TextToPrint::char_count(&got_txt) as f32)*FONT_WIDTH/2.0 );
            lf_act_texts.push( got_txt );
        }
        lf_actions.push( LfActToDraw::new(action.lf_act.lf_id,
                                          action.lf_act.preamble.len(),
                                          action.lf_act.postamble.len(),
                                          lf_act_texts,
                                          max_txt_width) );
    }
    // ***
    let msg_to_print_width : f32 = (TextToPrint::char_count(&msg_to_print) as f32)*FONT_WIDTH/2.0;
    let position_to_draw_message_text : f32;
    // ***
    match action.act_kind {
        ObservableActionKind::Emission(ref targets) => {
            for target_lf_act in targets {
                // ***
                let mut lf_act_texts : Vec<Vec<TextToPrint>> = Vec::new();
                let mut max_txt_width : f32 = 0.0;
                for txt_idx in 0..(target_lf_act.preamble.len() + target_lf_act.postamble.len()) {
                    let got_txt = texts_to_print.remove(0);
                    max_txt_width = max_txt_width.max( (TextToPrint::char_count(&got_txt) as f32)*FONT_WIDTH/2.0 );
                    lf_act_texts.push( got_txt );
                }
                // ***
                max_preamble_len = cmp::max(max_preamble_len,target_lf_act.preamble.len());
                max_postamble_len = cmp::max(max_postamble_len,target_lf_act.postamble.len());
                // ***
                min_lf_id = cmp::min(min_lf_id, target_lf_act.lf_id);
                max_lf_id = cmp::max(max_lf_id, target_lf_act.lf_id);
                // ***
                lf_actions.push( LfActToDraw::new(target_lf_act.lf_id,target_lf_act.preamble.len(),target_lf_act.postamble.len(),lf_act_texts,max_txt_width) );
            }
            if lf_actions.len() == 1 {
                {
                    let main_lf_coords = lf_x_widths.get(&action.lf_act.lf_id).unwrap();
                    position_to_draw_message_text = main_lf_coords.x_middle + (main_lf_coords.x_span_inner/4.0);
                }
                arr_kinds.push( ActionArrowKind::Emission );
                draw_action_content(image,exe_ctx,max_preamble_len,lf_actions,arr_kinds,lf_x_widths,yshift,None);
            } else if lf_actions.len() == 2 {
                {
                    let origin_lf_id = *(&action.lf_act.lf_id);
                    let origin_lf_coords = lf_x_widths.get(&origin_lf_id).unwrap();
                    let target_lf_id = (&lf_actions).get(1).unwrap().lf_id;
                    let mut anchor_lf_id : usize = target_lf_id;
                    if target_lf_id == origin_lf_id {
                        panic!("cannot draw emission then reception on the same lifeline");
                    } else if target_lf_id < origin_lf_id {
                        let mut lf_id_shift : usize = 1;
                        while !lf_x_widths.contains_key(&(origin_lf_id - lf_id_shift)) {
                            lf_id_shift = lf_id_shift + 1 ;
                        }
                        anchor_lf_id = (origin_lf_id - lf_id_shift);
                    } else if target_lf_id > origin_lf_id {
                        let mut lf_id_shift : usize = 1;
                        while !lf_x_widths.contains_key(&(origin_lf_id + lf_id_shift)) {
                            lf_id_shift = lf_id_shift + 1 ;
                        }
                        anchor_lf_id = (origin_lf_id + lf_id_shift);
                    }
                    let anchor_lf_coords = lf_x_widths.get(&anchor_lf_id).unwrap();
                    position_to_draw_message_text = (origin_lf_coords.x_middle + anchor_lf_coords.x_middle)/2.0;
                }
                arr_kinds.push( ActionArrowKind::None );
                arr_kinds.push( ActionArrowKind::None );
                let tar_lf_id = lf_actions.get(1).unwrap().lf_id;
                draw_action_content(image,exe_ctx,max_preamble_len,lf_actions,arr_kinds,lf_x_widths,yshift,Some((action.lf_act.lf_id,tar_lf_id)));
            } else {
                {
                    let main_lf_coords = lf_x_widths.get(&action.lf_act.lf_id).unwrap();
                    position_to_draw_message_text = main_lf_coords.x_middle + (main_lf_coords.x_span_inner/2.0);
                }
                arr_kinds.push( ActionArrowKind::BroadcastEmission );
                for idx in 1..lf_actions.len() {
                    arr_kinds.push( ActionArrowKind::BroadcastReception );
                }
                draw_action_content(image,exe_ctx,max_preamble_len,lf_actions,arr_kinds,lf_x_widths,yshift,None);
            }
        },
        ObservableActionKind::Reception => {
            {
                let main_lf_coords = lf_x_widths.get(&action.lf_act.lf_id).unwrap();
                position_to_draw_message_text = main_lf_coords.x_middle - (main_lf_coords.x_span_inner/4.0);
            }
            arr_kinds.push( ActionArrowKind::Reception );
            draw_action_content(image,exe_ctx,max_preamble_len,lf_actions,arr_kinds,lf_x_widths,yshift,None);
        }
    }
    // ***
    let text_y_pos = get_y_pos_from_yshift(yshift + 2*(max_preamble_len as u32));
    draw_colored_text(image,&msg_to_print,position_to_draw_message_text - msg_to_print_width/2.0,text_y_pos);
    // ***
    let new_yshift = yshift + 2 + 2*((max_preamble_len + max_postamble_len) as u32);
    return (new_yshift,[min_lf_id,max_lf_id]);
}




// **********

#[derive(Clone, PartialEq, Debug, Copy)]
enum ActionArrowKind {
    Emission,
    Reception,
    BroadcastEmission,
    BroadcastReception,
    None
}


fn draw_colored_text_centered_in_box(image: &mut RgbImage,
                                     xpos : f32,
                                     width : f32,
                                     ypos : f32,
                                    to_print : &Vec<TextToPrint>) {
    let width_with_margin = width + 2.0*MARGIN;
    let zone_rect = Rect::at((xpos-width_with_margin/2.0) as i32, (ypos - VERTICAL_SIZE)  as i32).of_size(width_with_margin as u32, 2*VERTICAL_SIZE as u32);
    draw_filled_rect_mut(image,zone_rect,Rgb(HCP_BrightGray));
    let text_width = (TextToPrint::char_count(&to_print) as f32)*FONT_WIDTH/2.0;
    draw_colored_text(image,to_print,xpos-(text_width/2.0),ypos);
}

fn draw_centered_colored_text(image: &mut RgbImage,
                              xpos : f32,
                              width : f32,
                              ypos : f32,
                              to_print : &Vec<TextToPrint>) {
    let text_width = (TextToPrint::char_count(&to_print) as f32)*FONT_WIDTH/2.0;
    draw_colored_text(image,to_print,xpos-(text_width/2.0),ypos);
}

fn draw_message_arrow(image : &mut RgbImage,
                      arr_kind : &ActionArrowKind,
                     lf_x_coords : &DrawingLifelineCoords,
                     arrow_y_shift : u32) {
    let event_y_pos : f32 = get_y_pos_from_yshift(arrow_y_shift);
    match arr_kind {
        &ActionArrowKind::Emission => {
            let msg_x_left = lf_x_coords.x_middle;
            let msg_x_right= msg_x_left + lf_x_coords.x_span_inner/2.0;
            draw_arrowhead_rightward(image,msg_x_right,event_y_pos,Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (msg_x_left, event_y_pos),
                                  (msg_x_right, event_y_pos),
                                  Rgb(HCP_Black));
        },
        &ActionArrowKind::Reception => {
            let msg_x_right = lf_x_coords.x_middle;
            let msg_x_left= msg_x_right - lf_x_coords.x_span_inner/2.0;
            draw_filled_circle_mut(image, (msg_x_left as i32, event_y_pos as i32), 3, Rgb(HCP_Black));
            draw_arrowhead_rightward(image,msg_x_right,event_y_pos,Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (msg_x_left, event_y_pos),
                                  (msg_x_right, event_y_pos),
                                  Rgb(HCP_Black));
        },
        ActionArrowKind::BroadcastEmission => {
            let msg_x_left = lf_x_coords.x_middle;
            let msg_x_right= msg_x_left + lf_x_coords.x_span_inner/2.0;
            draw_double_half_ellipsis_rightward(image,msg_x_right, event_y_pos,Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (msg_x_left, event_y_pos),
                                  (msg_x_right, event_y_pos),
                                  Rgb(HCP_Black));
        },
        ActionArrowKind::BroadcastReception => {
            let msg_x_right = lf_x_coords.x_middle;
            let msg_x_left= msg_x_right - lf_x_coords.x_span_inner/2.0;
            draw_double_half_ellipsis_rightward(image, msg_x_left, event_y_pos,Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (msg_x_left, event_y_pos),
                                  (msg_x_right, event_y_pos),
                                  Rgb(HCP_Black));
        },
        ActionArrowKind::None => {}
    }
}

fn draw_lf_act(image : &mut RgbImage,
               atd : &LfActToDraw,
               arr_kind : &ActionArrowKind,
               lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
               yshift: u32,
               max_preamble_len : usize) {
    let lf_x_coords = lf_x_widths.get(&atd.lf_id).unwrap();
    // ***
    let mut lf_yshift : u32 = yshift + 2*(max_preamble_len as u32) - 2*(atd.preamble_len as u32);
    // ***
    if atd.preamble_len + atd.postamble_len > 0 {
        let zone_width : f32 = atd.max_txt_width + 2.0*MARGIN;
        let zone_left : i32 = (lf_x_coords.x_middle-zone_width/2.0) as i32;
        // ***
        let zone_top : i32 = get_y_pos_from_yshift(lf_yshift - 1) as i32;
        let height_yshift : usize = (atd.preamble_len + atd.postamble_len)*2 + 4;
        // ***
        let zone_height : f32 = (height_yshift as f32)*VERTICAL_SIZE;
        // ***
        let zone_rect = Rect::at( zone_left, zone_top).of_size(zone_width as u32, zone_height as u32);
        draw_filled_rect_mut(image,zone_rect,Rgb(HCP_BrightGray));
    }
    // ***
    let mut current_txt_id : usize = 0;
    // ***
    for id in 0..atd.preamble_len {
        let to_print = atd.texts_to_print.get(current_txt_id).unwrap();
        current_txt_id = current_txt_id +1;
        draw_centered_colored_text(image, lf_x_coords.x_middle,atd.max_txt_width,get_y_pos_from_yshift(lf_yshift),to_print);
        lf_yshift = lf_yshift +2;
    }
    // ***
    lf_yshift = lf_yshift +2;
    let arrow_y_shift = lf_yshift;
    lf_yshift = lf_yshift +1;
    // ***
    for id in 0..atd.postamble_len {
        let to_print = atd.texts_to_print.get(current_txt_id).unwrap();
        current_txt_id = current_txt_id +1;
        draw_centered_colored_text(image, lf_x_coords.x_middle,atd.max_txt_width,get_y_pos_from_yshift(lf_yshift),to_print);
        lf_yshift = lf_yshift +2;
    }
    // ***
    // we do it afterwards so that the gray squares are not painted over the arrows but stay under
    draw_message_arrow(image,arr_kind,lf_x_coords,arrow_y_shift);
}


fn draw_action_content( image : &mut RgbImage,
                        exe_ctx : &ExecutionContext,
                        max_preamble_len : usize,
                        lf_actions : Vec<LfActToDraw>,
                        arr_kinds : Vec<ActionArrowKind>,
                        lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                        yshift: u32,
                        draw_passing : Option<(usize,usize)>) {
    // ***
    for idx in 0..lf_actions.len() {
        let atd : &LfActToDraw = lf_actions.get(idx).unwrap();
        let arr_kind : &ActionArrowKind = arr_kinds.get(idx).unwrap();
        draw_lf_act(image,atd,arr_kind,lf_x_widths,
                    yshift,max_preamble_len);
    }
    // ***
    match draw_passing {
        None => {},
        Some( (origin_lf_id, target_lf_id) ) => {
            let arrow_y_pos = get_y_pos_from_yshift(yshift + 2 + 2*(max_preamble_len as u32));
            let msg_x_orig : f32 = lf_x_widths.get(&origin_lf_id).unwrap().x_middle;
            let msg_x_targ : f32 = lf_x_widths.get(&target_lf_id).unwrap().x_middle;
            if origin_lf_id < target_lf_id {
                draw_arrowhead_rightward(image,msg_x_targ,arrow_y_pos,Rgb(HCP_Black));
            } else {
                draw_arrowhead_leftward(image,msg_x_targ,arrow_y_pos,Rgb(HCP_Black));
            }
            draw_line_segment_mut(image,
                                  (msg_x_targ, arrow_y_pos),
                                  (msg_x_orig, arrow_y_pos),
                                  Rgb(HCP_Black));
        }
    }
}







