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

/*

HIBOU Color Palette
My color picks for a palette with:
 - a constant Lightness L
 - a constant Saturation S
in the HSL representation of colors

All colors have :
    - a lightness of :
        + 20 for the "Dark" version
        + 30 for the "Standard" version
        + 50 for the "Light" version
        + 65 for the "Bright" version
    - a saturation of :
        + 70 for the "Dark" version
        + 70 for the "Standard" version
        + 70 for the "Light" version
        + 90 for the "Bright" version

Different hues were selected :
    - 0 for "Red"
    - 30 for "Orange"
    - 60 for "Yellow"
    - 120 for "Green"
    - 180 for "Cyan"
    - 240 for "Blue"
    - 280 for "Purple"
    - 310 for "Pink"

For use with image::Rgb from the "image" crate

*/

pub const HCP_White : [u8;3] = [255u8,  255u8,  255u8];
pub const HCP_Black : [u8;3] = [0u8, 0u8, 0u8];

pub const HCP_DarkRed : [u8;3] = [86u8, 15u8, 15u8];
pub const HCP_StandardRed : [u8;3] = [130u8, 22u8, 22u8];
pub const HCP_LightRed : [u8;3] = [216u8, 38u8, 38u8];
pub const HCP_BrightRed : [u8;3] = [246u8, 85u8, 85u8];
pub const HCP_DarkOrange : [u8;3] = [86u8, 51u8, 15u8];
pub const HCP_StandardOrange : [u8;3] = [130u8, 76u8, 22u8];
pub const HCP_LightOrange : [u8;3] = [216u8, 127u8, 38u8];
pub const HCP_BrightOrange : [u8;3] = [246u8, 165u8, 85u8];
pub const HCP_DarkYellow : [u8;3] = [86u8, 86u8, 15u8];
pub const HCP_StandardYellow : [u8;3] = [130u8, 130u8, 22u8];
pub const HCP_LightYellow : [u8;3] = [216u8, 216u8, 38u8];
pub const HCP_BrightYellow : [u8;3] = [246u8, 246u8, 85u8];
pub const HCP_DarkGreen : [u8;3] = [15u8, 86u8, 15u8];
pub const HCP_StandardGreen : [u8;3] = [22u8, 130u8, 22u8];
pub const HCP_LightGreen : [u8;3] = [38u8, 216u8, 38u8];
pub const HCP_BrightGreen : [u8;3] = [85u8, 246u8, 85u8];
pub const HCP_DarkCyan : [u8;3] = [15u8, 86u8, 86u8];
pub const HCP_StandardCyan : [u8;3] = [22u8, 130u8, 130u8];
pub const HCP_LightCyan : [u8;3] = [38u8, 216u8, 216u8];
pub const HCP_BrightCyan : [u8;3] = [85u8, 246u8, 246u8];
pub const HCP_DarkBlue : [u8;3] = [15u8, 15u8, 86u8];
pub const HCP_StandardBlue : [u8;3] = [22u8, 22u8, 130u8];
pub const HCP_LightBlue : [u8;3] = [38u8, 38u8, 216u8];
pub const HCP_BrightBlue : [u8;3] = [85u8, 85u8, 246u8];
pub const HCP_DarkPurple : [u8;3] = [62u8, 15u8, 86u8];
pub const HCP_StandardPurple : [u8;3] = [94u8, 22u8, 130u8];
pub const HCP_LightPurple : [u8;3] = [157u8, 38u8, 216u8];
pub const HCP_BrightPurple : [u8;3] = [192u8, 85u8, 246u8];
pub const HCP_DarkPink : [u8;3] = [86u8, 15u8, 74u8];
pub const HCP_StandardPink : [u8;3] = [130u8, 22u8, 112u8];
pub const HCP_LightPink : [u8;3] = [216u8, 38u8, 186u8];
pub const HCP_BrightPink : [u8;3] = [246u8, 85u8, 219u8];

pub const HCP_DarkGray : [u8;3] = [51u8, 51u8, 51u8];
pub const HCP_StandardGray : [u8;3] = [76u8, 76u8, 76u8];
pub const HCP_LightGray : [u8;3] = [127u8, 127u8, 127u8];
pub const HCP_BrightGray : [u8;3] = [165u8, 165u8, 165u8];


pub const HC_LifelineGroup : [u8;3] = HCP_StandardCyan;
pub const HC_Lifeline : [u8;3] = HCP_StandardBlue;
pub const HC_MessageParameter : [u8;3] = HCP_StandardGreen;
pub const HC_Message : [u8;3] = HCP_DarkGreen; //HCP_StandardGreen; HCP_StandardPurple;
pub const HC_Grammar_Symbol : [u8;3] = HCP_Black;
pub const HC_Concrete_Value : [u8;3] = HCP_StandardGray;
pub const HC_Variable : [u8;3] = HCP_StandardRed;
pub const HC_Symbol : [u8;3] = HCP_StandardOrange;
pub const HC_NewFresh : [u8;3] = HCP_StandardPurple;

//pub const HC_Concrete_Value_Secondary : [u8;3] = HCP_StandardGray;
