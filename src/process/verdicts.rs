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


pub enum CoverageVerdict{
    Cov,
    TooShort,
    LackObs,
    Out
}

impl std::string::ToString for CoverageVerdict {

    fn to_string(&self) -> String {
        match self {
            CoverageVerdict::Cov => {
                return "Cov".to_string();
            },
            CoverageVerdict::TooShort => {
                return "TooShort".to_string();
            },
            CoverageVerdict::LackObs => {
                return "LackObs".to_string();
            },
            CoverageVerdict::Out => {
                return "Out".to_string();
            }
        }
    }

}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum GlobalVerdict {
    Fail,
    Inconc,
    WeakPass,
    Pass
}

impl std::string::ToString for GlobalVerdict {
    fn to_string(&self) -> String {
        match self {
            GlobalVerdict::Pass => {
                return "Pass".to_string();
            },
            GlobalVerdict::WeakPass => {
                return "WeakPass".to_string();
            },
            GlobalVerdict::Inconc => {
                return "Inconc".to_string();
            },
            GlobalVerdict::Fail => {
                return "Fail".to_string();
            }
        }
    }
}

pub fn update_global_verdict_from_new_coverage_verdict(glo:GlobalVerdict,cov:CoverageVerdict) -> GlobalVerdict {
    match glo {
        GlobalVerdict::Pass => {
            return GlobalVerdict::Pass;
        },
        GlobalVerdict::WeakPass => {
            match cov {
                CoverageVerdict::Cov => {
                    return GlobalVerdict::Pass;
                },
                _ => {
                    return GlobalVerdict::WeakPass;
                }
            }
        },
        GlobalVerdict::Inconc => {
            match cov {
                CoverageVerdict::Cov => {
                    return GlobalVerdict::Pass;
                },
                CoverageVerdict::TooShort => {
                    return GlobalVerdict::WeakPass;
                },
                _ => {
                    return GlobalVerdict::Inconc;
                }
            }
        },
        GlobalVerdict::Fail => {
            match cov {
                CoverageVerdict::Cov => {
                    return GlobalVerdict::Pass;
                },
                CoverageVerdict::TooShort => {
                    return GlobalVerdict::WeakPass;
                },
                CoverageVerdict::LackObs => {
                    return GlobalVerdict::Inconc;
                },
                _ => {
                    return GlobalVerdict::Fail;
                }
            }
        }
    }
}
