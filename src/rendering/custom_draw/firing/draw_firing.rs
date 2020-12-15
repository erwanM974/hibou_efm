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
// **********

use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;
use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;

use crate::rendering::sd_drawing_conf::*;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::textual::colored::short_action::{diagram_repr_atomic_model_action,diagram_repr_atomic_trace_action};

use crate::rendering::hibou_color_palette::*;
use crate::rendering::textual::monochrome::position::position_to_text;
use crate::rendering::custom_draw::utils::colored_text::draw_colored_text;

use crate::core::trace::{TraceAction,TraceActionKind};
// **********



/*
pub fn draw_firing_on_trace_action(path_str : &String,
                                   action_position : &Position,
                                   action : &TraceAction,
                                   gen_ctx : &GeneralContext,
                                   exe_ctx:&ExecutionContext) {
    let mut ttp = diagram_repr_atomic_trace_action(action,gen_ctx,exe_ctx);
    // ***
    ttp.push( TextToPrint{text:"@p".to_string(),color:Rgb(HCP_StandardPurple)} );
    ttp.push( TextToPrint{text:position_to_text(action_position),color:Rgb(HCP_Black)} );
    // ***
    let text_lines : Vec<Vec<TextToPrint>> = vec![ttp];
    // ***
    draw_firing(path_str,text_lines,gen_ctx,exe_ctx);
}*/

pub fn draw_firing_on_model_action(path_str : &String,
                                   action_position : &Position,
                                   action : &ObservableAction,
                                   gen_ctx : &GeneralContext,
                                   exe_ctx:&ExecutionContext) {
    let mut ttp = diagram_repr_atomic_model_action(action,gen_ctx,exe_ctx);
    // ***
    ttp.push( TextToPrint{text:"@p".to_string(),color:Rgb(HCP_StandardPurple)} );
    ttp.push( TextToPrint{text:position_to_text(action_position),color:Rgb(HCP_Black)} );
    // ***
    let text_lines : Vec<Vec<TextToPrint>> = vec![ttp];
    // ***
    draw_firing(path_str,text_lines,gen_ctx,exe_ctx);
}

pub fn draw_firing_on_trace_against_model_action(path_str : &String,
                                   action_position : &Position,
                                   trace_action : &TraceAction,
                                   model_action : &ObservableAction,
                                   gen_ctx : &GeneralContext,
                                   exe_ctx:&ExecutionContext) {
    let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
    // ***
    {
        let mut ttp : Vec<TextToPrint> = Vec::new();
        ttp.append(&mut diagram_repr_atomic_model_action(model_action,gen_ctx,exe_ctx) );
        ttp.push( TextToPrint{text:"@p".to_string(),color:Rgb(HCP_StandardPurple)} );
        ttp.push( TextToPrint{text:position_to_text(action_position),color:Rgb(HCP_Black)} );
        text_lines.push(ttp);
    }
    // ***
    {
        let mut ttp : Vec<TextToPrint> = vec![TextToPrint{text:"↔".to_string(),color:Rgb(HCP_StandardPurple)}];
        ttp.append(&mut diagram_repr_atomic_trace_action(trace_action,gen_ctx,exe_ctx) );
        text_lines.push( ttp );
    }
    // ***
    draw_firing(path_str,text_lines,gen_ctx,exe_ctx);
}

fn draw_firing(path_str : &String,
               text_lines : Vec<Vec<TextToPrint>>,
               gen_ctx : &GeneralContext,
               exe_ctx:&ExecutionContext) {
    let path = Path::new( path_str );
    // ***
    let line_lens : Vec<usize> = text_lines.iter().map(|x| TextToPrint::char_count(x) ).collect();
    let max_x_shift = *line_lens.iter().max().unwrap();
    // ***
    let img_width : f32 = 2.0*MARGIN + (max_x_shift as f32)*FONT_WIDTH/2.0;
    let img_height : f32 = 2.0*MARGIN + (((text_lines.len()*2)-1) as f32)*VERTICAL_SIZE;

    // Draw Frame
    let mut image = RgbImage::new( img_width as u32, img_height as u32);
    draw_filled_rect_mut(&mut image, Rect::at(0,0).of_size(img_width as u32,img_height as u32), Rgb(HCP_White));
    // Draw Fragments
    let mut yshift : u32 = 0;
    for text in text_lines {
        let msg_x_pos = (img_width/2.0) - ( (TextToPrint::char_count(&text) as f32)*FONT_WIDTH/4.0 );
        let msg_y_pos = MARGIN + (yshift as f32)*VERTICAL_SIZE;
        draw_colored_text(&mut image,&text,msg_x_pos,msg_y_pos);
        yshift = yshift +2;
    }
    // ***
    image.save(path).unwrap();
}



