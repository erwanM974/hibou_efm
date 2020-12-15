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

use image::Rgb;
use crate::core::context::general::GeneralContext;
use crate::core::context::execution::ExecutionContext;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TextToPrint {
    pub text : String,
    pub color : Rgb<u8>
}

impl TextToPrint {
    pub fn flatten(to_print : &Vec<TextToPrint>) -> String {
        let mut flattened : String = String::new();
        for ttp in to_print {
            flattened.push_str(&ttp.text);
        }
        return flattened;
    }

    pub fn char_count(to_print : &Vec<TextToPrint>) -> usize {
        let mut count : usize = 0;
        for ttp in to_print {
            count = count + ttp.text.chars().count();
        }
        return count;
    }

    pub fn contains_sub_str(to_print : &Vec<TextToPrint>, sub_str : &str) -> bool {
        for ttp in to_print {
            if ttp.text.contains(sub_str) {
                return true;
            }
        }
        return false;
    }

    /*
    pub fn contains_one_of_sub_strs(to_print : &Vec<TextToPrint>, sub_strs : Vec<&str>) -> bool {
        for ttp in to_print {
            for sub_str in &sub_strs {
                if ttp.text.contains(sub_str) {
                    return true;
                }
            }
        }
        return false;
    }*/
}

pub trait ColoredTextable {

    fn to_colored_text(&self, gen_ctx : &GeneralContext, exe_ctx : &ExecutionContext) -> Vec<TextToPrint>;

}
