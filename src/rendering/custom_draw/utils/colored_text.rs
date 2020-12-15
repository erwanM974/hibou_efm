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

use image::{Rgb, RgbImage};
use rusttype::{FontCollection, Scale};
use imageproc::drawing::draw_text_mut;

use crate::rendering::textual::convention::*;
use crate::rendering::sd_drawing_conf::*;
use crate::rendering::textual::colored::colored_text::*;

pub fn draw_colored_text(image : &mut RgbImage,
                         to_print : &Vec<TextToPrint>,
                         msg_x_pos : f32,
                         msg_y_pos : f32) {
    let font = FontCollection::from_bytes(HIBOU_GRAPHIC_FONT).unwrap().into_font().unwrap();
    let scale = Scale { x: FONT_WIDTH, y: FONT_HEIGHT };

    // ***
    let mut char_count : u32 = 0;
    for txt_to_print in to_print {
        let mut my_text : String = (0..char_count).map(|_| " ").collect::<String>();
        my_text.push_str( &txt_to_print.text );
        draw_text_mut(image,
                      txt_to_print.color,
                      msg_x_pos as u32,
                      (msg_y_pos - (VERTICAL_SIZE as f32)/2.0) as u32,
                      scale,
                      &font,
                      &my_text
        );
        char_count = char_count + txt_to_print.text.chars().count() as u32;
    }
}