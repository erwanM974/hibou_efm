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


use crate::process::verdicts::CoverageVerdict;

use crate::rendering::graphviz::common::GraphvizColor;


impl CoverageVerdict {
    pub fn get_verdict_color(&self) -> GraphvizColor {
        match self {
            CoverageVerdict::Cov => {
                return GraphvizColor::blue3;
            },
            CoverageVerdict::TooShort => {
                return GraphvizColor::cyan3;
            },
            CoverageVerdict::LackObs => {
                return GraphvizColor::orangered3;
            },
            CoverageVerdict::Out => {
                return GraphvizColor::red3;
            }
        }
    }
}





