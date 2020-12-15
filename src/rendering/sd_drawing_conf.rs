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

pub const HIBOU_GRAPHIC_FONT: &'static [u8] = include_bytes!("DejaVuSansMono.ttf");

// **********
pub const BASE_HORIZONTAL_SIZE : f32 = 50.0; // 100.0
pub const BASE_VERTICAL_SIZE : f32 = 5.5; //11.0; // 12.5
pub const BASE_MARGIN : f32 = BASE_VERTICAL_SIZE; // 10.0
pub const BASE_FRAGMENT_PADDING : f32 = 5.0;
pub const BASE_FRAGMENT_TITLE_MARGIN : f32 = 1.0;
// **********
//pub const BASE_ACTOR_WIDTH : f32 = BASE_HORIZONTAL_SIZE/3.0;
//pub const BASE_ACTOR_HEIGHT : f32 = 10.0;
// **********
pub const BASE_THICKNESS : f32 = 1.0;
pub const BASE_FONT_HEIGHT : f32 = 12.4;
pub const BASE_EVAL_X_PADDING : f32 = BASE_HORIZONTAL_SIZE/3.5;
pub const BASE_EVAL_HEIGHT : f32 = BASE_VERTICAL_SIZE/3.0;
pub const BASE_ARROW_HEAD_LENGTH : f32 = 5.0;
pub const BASE_FRONTIER_CIRCLE_RADIUS : f32 = 5.0;
// **********
const SCALE_FACTOR : f32 = 2.0;
// **********
pub const MARGIN : f32 = BASE_MARGIN*SCALE_FACTOR;
pub const HORIZONTAL_SIZE : f32 = BASE_HORIZONTAL_SIZE*SCALE_FACTOR;
pub const VERTICAL_SIZE : f32 = BASE_VERTICAL_SIZE*SCALE_FACTOR;
pub const FRAGMENT_PADDING : f32 = BASE_FRAGMENT_PADDING*SCALE_FACTOR;
pub const FRAGMENT_TITLE_MARGIN : f32 = BASE_FRAGMENT_TITLE_MARGIN*SCALE_FACTOR;
// **********
//pub const ACTOR_WIDTH : f32 = BASE_ACTOR_WIDTH*SCALE_FACTOR;
//pub const ACTOR_HEIGHT : f32 = BASE_ACTOR_HEIGHT*SCALE_FACTOR;
// **********
pub const THICKNESS : f32 = BASE_THICKNESS*SCALE_FACTOR;
pub const FONT_HEIGHT : f32 = BASE_FONT_HEIGHT*SCALE_FACTOR;
pub const FONT_X_PROPORTION : f32 = 1.0;
pub const FONT_WIDTH : f32 = FONT_HEIGHT*FONT_X_PROPORTION;
pub const EVAL_X_PADDING : f32 = BASE_EVAL_X_PADDING*SCALE_FACTOR;
pub const EVAL_HEIGHT : f32 = BASE_EVAL_HEIGHT*SCALE_FACTOR;
pub const ARROW_HEAD_LENGTH : f32 = BASE_ARROW_HEAD_LENGTH*SCALE_FACTOR;
pub const FRONTIER_CIRCLE_RADIUS : f32 = BASE_FRONTIER_CIRCLE_RADIUS*SCALE_FACTOR;
// **********