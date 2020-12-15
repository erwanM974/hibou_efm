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



// **********

use image::{Rgb,RgbImage};
use imageproc::drawing::{draw_line_segment_mut};
use imageproc::drawing::draw_cubic_bezier_curve_mut;

// **********

use crate::rendering::sd_drawing_conf::ARROW_HEAD_LENGTH;

// **********

pub fn draw_double_half_ellipsis_leftward(image : &mut RgbImage, x_pos : f32, y_pos : f32, my_color : Rgb<u8>) {
    draw_cubic_bezier_curve_mut(image,
                                (x_pos - 0.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 0.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 1.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 1.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.5*(ARROW_HEAD_LENGTH as f32)),
                                my_color);
    draw_cubic_bezier_curve_mut(image,
                                (x_pos - 0.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 0.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 1.0*(ARROW_HEAD_LENGTH as f32), y_pos - 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 1.0*(ARROW_HEAD_LENGTH as f32), y_pos + 0.25*(ARROW_HEAD_LENGTH as f32)),
                                my_color);
}

pub fn draw_double_half_ellipsis_rightward(image : &mut RgbImage, x_pos : f32, y_pos : f32, my_color : Rgb<u8>) {
    draw_cubic_bezier_curve_mut(image,
                                (x_pos + 0.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 0.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 1.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 1.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.5*(ARROW_HEAD_LENGTH as f32)),
                                my_color);
    draw_cubic_bezier_curve_mut(image,
                                (x_pos + 0.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 0.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 1.0*(ARROW_HEAD_LENGTH as f32), y_pos - 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 1.0*(ARROW_HEAD_LENGTH as f32), y_pos + 0.25*(ARROW_HEAD_LENGTH as f32)),
                                my_color);
}

pub fn draw_arrowhead_rightward(image : &mut RgbImage, x_pos : f32, y_pos : f32, my_color : Rgb<u8>) {
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos - (ARROW_HEAD_LENGTH as f32), y_pos - (ARROW_HEAD_LENGTH as f32)),
                          my_color);
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos - (ARROW_HEAD_LENGTH as f32), y_pos + (ARROW_HEAD_LENGTH as f32)),
                          my_color);
}

pub fn draw_arrowhead_leftward(image : &mut RgbImage, x_pos : f32, y_pos : f32, my_color : Rgb<u8>) {
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos + (ARROW_HEAD_LENGTH as f32), y_pos - (ARROW_HEAD_LENGTH as f32)),
                          my_color);
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos + (ARROW_HEAD_LENGTH as f32), y_pos + (ARROW_HEAD_LENGTH as f32)),
                          my_color);
}

// **********

