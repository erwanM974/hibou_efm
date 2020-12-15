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
use std::collections::HashMap;
use std::path::Path;

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
use crate::rendering::custom_draw::seqdiag::dimensions_tools::*;
use crate::rendering::sd_drawing_conf::*;
use crate::rendering::hibou_color_palette::*;
use crate::rendering::custom_draw::seqdiag::lf_coords::DrawingLifelineCoords;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::custom_draw::utils::colored_text::draw_colored_text;
/*
            DRAWING FUNCTION
*/




pub fn draw_frame(image : &mut RgbImage, img_width : &f32, img_height : &f32, max_y_shift : usize) {
    draw_filled_rect_mut(image, Rect::at(0,0).of_size(*img_width as u32,*img_height as u32), Rgb(HCP_White));
    /*
    for x in 0..max_y_shift {
        let y_pos : f32 = MARGIN + ((x as f32)*VERTICAL_SIZE);
        draw_line_segment_mut(image, (0.5,y_pos), (img_width - 0.5,y_pos), Rgb(HCP_DarkCyan));
    }*/
    /*
    draw_hollow_rect_mut(image,
                         Rect::at((MARGIN) as i32,(MARGIN) as i32).of_size((img_width -2.0*MARGIN) as u32, (img_height -2.0*MARGIN) as u32),
                         Rgb(HC_Grammar_Symbol));

    let font = FontCollection::from_bytes(HIBOU_GRAPHIC_FONT).unwrap().into_font().unwrap();

    let scale = Scale { x: FONT_WIDTH, y: FONT_HEIGHT };
    draw_text_mut(image,
                  Rgb(HC_Grammar_Symbol),
                  (MARGIN+THICKNESS+FRAGMENT_TITLE_MARGIN) as u32,
                  (MARGIN+THICKNESS) as u32,
                  scale,
                  &font,
                  &label
    );*/
}

pub fn draw_lifelines(image : &mut RgbImage,
                      lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                      inner_height : f32,
                      gen_ctx:&GeneralContext,
                      exe_ctx:&ExecutionContext,
                      init_lf_interpretations : HashMap< usize,(usize,Vec<Vec<TextToPrint>>) >) {
    // Draw Lifelines
    let lifeline_y_start :f32 = MARGIN + (2 as f32)*VERTICAL_SIZE;;
    let lifeline_y_end :f32 = MARGIN+inner_height;
    for (lf_id,lf_coords) in lf_x_widths.iter() {
        // ***
        let lf_name = exe_ctx.get_lf_name(gen_ctx,*lf_id).unwrap();
        let lf_name_span = FONT_WIDTH*(lf_name.chars().count() as f32)/2.0;
        let mut square_span_with_margin = lf_name_span + 2.0*MARGIN;

        let font = FontCollection::from_bytes(HIBOU_GRAPHIC_FONT).unwrap().into_font().unwrap();

        let scale = Scale { x: FONT_WIDTH, y: FONT_HEIGHT };

        let label = vec![TextToPrint{text:lf_name,color:Rgb(HC_Lifeline)}];
        let lf_label_centered_pos = lf_coords.x_middle - lf_name_span/2.0;
        draw_colored_text(image,&label,lf_label_centered_pos,lifeline_y_start + VERTICAL_SIZE/2.0);
        // ***
        // Draw current interpretation
        let mut yshift : usize = 2;
        // draw interpretation
        match init_lf_interpretations.get(lf_id) {
            None => {},
            Some( (_,assignments_to_print) ) => {
                for ttp in assignments_to_print {
                    let ttp_size = (TextToPrint::char_count(ttp) as f32)*FONT_WIDTH/2.0;
                    let ttp_size_with_margins = ttp_size + 2.0*MARGIN;
                    square_span_with_margin = square_span_with_margin.max(ttp_size_with_margins);
                    let ttp_text_start = lf_coords.x_middle - ttp_size/2.0;
                    let ttp_rect_start = lf_coords.x_middle - ttp_size_with_margins/2.0;
                    let ttp_y_text = lifeline_y_start + (yshift as f32)*VERTICAL_SIZE + VERTICAL_SIZE/2.0;
                    let ttp_y_rect = ttp_y_text -0.5*VERTICAL_SIZE;
                    draw_filled_rect_mut(image, Rect::at(ttp_rect_start as i32,ttp_y_rect as i32).of_size(ttp_size_with_margins as u32,(VERTICAL_SIZE*2.0) as u32), Rgb(HCP_BrightGray));
                    draw_colored_text(image,ttp,ttp_text_start,ttp_y_text);
                    yshift = yshift +2;
                }
            }
        }
        // ***
        let actor_x_start : f32 = lf_coords.x_middle - (square_span_with_margin/2.0);
        draw_hollow_rect_mut(image,
                             Rect::at(actor_x_start as i32, lifeline_y_start as i32).of_size(square_span_with_margin as u32, ((yshift as f32)*VERTICAL_SIZE) as u32),
                             Rgb(HC_Grammar_Symbol));
        // ***
        draw_line_segment_mut(image,
                              (lf_coords.x_middle, lifeline_y_start + (yshift as f32)*VERTICAL_SIZE),
                              (lf_coords.x_middle, lifeline_y_end),
                              Rgb(HC_Grammar_Symbol));
    }
}







