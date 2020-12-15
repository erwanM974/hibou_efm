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

use crate::core::syntax::data::td_type::TD_DataType;

use crate::core::error::HibouCoreError;

#[derive(Clone, PartialEq, Debug)]
pub struct GeneralContext {
    lf_groups : Vec<String>,
    lf_names : Vec<String>,
    ms_specs : Vec< (String,Vec<(TD_DataType,Option<String>)>) >,
    vr_names : Vec<String>,
    vr_types : Vec<TD_DataType>,
    clocks : HashSet<usize>
}



impl GeneralContext {

    pub fn new() -> GeneralContext {
        return GeneralContext{
            lf_groups:Vec::new(),
            lf_names:Vec::new(),
            ms_specs:Vec::new(),
            vr_names:Vec::new(),
            vr_types:Vec::new(),
            clocks:HashSet::new()};
    }

    pub fn get_lf_names(&self) -> &Vec<String> {
        return &self.lf_names;
    }

    pub fn get_vr_names(&self) -> &Vec<String> {
        return &self.vr_names;
    }

    pub fn get_vr_types(&self) -> &Vec<TD_DataType> {
        return &self.vr_types;
    }

    pub fn get_ms_specs(&self) -> &Vec<(String, Vec<(TD_DataType, Option<String>)>)> {
        return &self.ms_specs;
    }

    pub fn get_clocks(&self) -> &HashSet<usize> {
        return &self.clocks;
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn add_as_clock(&mut self, vr_id : usize) {
        self.clocks.insert(vr_id);
    }

    pub fn add_lf_group(&mut self, lgr_name : String) -> usize {
        match self.get_lf_id( &lgr_name ) {
            None => {
                match self.get_lgr_id(&lgr_name) {
                    None => {
                        self.lf_groups.push(lgr_name );
                        return self.lf_groups.len() - 1;
                    },
                    Some(lgr_id) => {
                        return lgr_id;
                    }
                }
            },
            _ => {
                panic!("trying to add a group with same name as a known lifeline");
            }
        }
    }

    pub fn add_lf(&mut self, lf_name : String) -> usize {
        match self.get_lgr_id( &lf_name ) {
            None => {
                match self.get_lf_id(&lf_name) {
                    None => {
                        self.lf_names.push(lf_name);
                        return self.lf_names.len() - 1;
                    },
                    Some(lf_id) => {
                        return lf_id;
                    }
                }
            },
            _ => {
                panic!("trying to add a lifeline with same name as a known group");
            }
        }
    }

    pub fn add_msg(&mut self, ms_name : String, ms_spec : Vec<(TD_DataType,Option<String>)>) -> usize {
        for (got_ms_name,got_ms_spec) in &self.ms_specs {
            if got_ms_name == &ms_name {
                panic!();
            }
        }
        self.ms_specs.push( (ms_name, ms_spec) );
        return self.ms_specs.len() - 1;
    }

    pub fn add_vr(&mut self, vr_name : String, vr_type : TD_DataType) {
        self.vr_names.push(vr_name);
        self.vr_types.push(vr_type);
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lgr_id(&self, lgr_name : &str) -> Option<usize> {
        return self.lf_groups.iter().position(|gn| gn == lgr_name);
    }

    pub fn get_lf_id(&self, lf_name : &str) -> Option<usize> {
        return self.lf_names.iter().position(|r| r == lf_name);
    }

    pub fn get_vr_id(&self, vr_name : &str) -> Option<usize> {
        return self.vr_names.iter().position(|r| r == vr_name);
    }

    pub fn get_ms_id(&self, ms_name : &str) -> Option<usize> {
        return self.ms_specs.iter().position(|(n,s)| n == ms_name);
    }

    pub fn is_clock(&self, vr_id : usize) -> bool {
        return self.clocks.contains( &vr_id );
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_num(&self) -> usize {
        return self.lf_names.len();
    }

    pub fn get_ms_num(&self) -> usize {
        return self.ms_specs.len();
    }

    pub fn get_vr_num(&self) -> usize {
        return self.vr_names.len();
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lgr_name(&self, lgr_id : usize) -> Result<String,HibouCoreError> {
        match self.lf_groups.get(lgr_id) {
            None => {
                return Err( HibouCoreError::UnknownLifelineGroup(lgr_id) );
            },
            Some( got_str ) => {
                return Ok( got_str.to_string() );
            }
        }
    }

    pub fn get_lf_name(&self, lf_id : usize) -> Result<String,HibouCoreError> {
        match self.lf_names.get(lf_id) {
            None => {
                return Err( HibouCoreError::UnknownLifeline(lf_id) );
            },
            Some( got_str ) => {
                return Ok( got_str.to_string() );
            }
        }
    }

    pub fn get_ms_name(&self, ms_id : usize) -> Result<String,HibouCoreError> {
        match self.ms_specs.get(ms_id) {
            None => {
                return Err( HibouCoreError::UnknownMessage(ms_id) );
            },
            Some( (ms_name, ms_spec) ) => {
                return Ok( ms_name.to_string() );
            }
        }
    }

    pub fn get_ms_spec(&self, ms_id : usize) -> Result<Vec<(TD_DataType,Option<String>)>,HibouCoreError> {
        match self.ms_specs.get(ms_id) {
            None => {
                return Err( HibouCoreError::UnknownMessage(ms_id) );
            },
            Some( (ms_name, ms_spec) ) => {
                return Ok( ms_spec.clone() );
            }
        }
    }

    pub fn get_pr_type(&self, ms_id : usize, pr_id : usize) -> Result<TD_DataType,HibouCoreError> {
        match self.ms_specs.get(ms_id) {
            None => {
                return Err( HibouCoreError::UnknownMessage(ms_id) );
            },
            Some( (ms_name, ms_spec) ) => {
                match ms_spec.get(pr_id) {
                    None => {
                        return Err( HibouCoreError::UnknownParameter(pr_id) );
                    },
                    Some( (pr_type,opt_pr_name) ) => {
                        return Ok(pr_type.clone());
                    }
                }
            }
        }
    }

    pub fn get_vr_name(&self, vr_id : usize) -> Result<String,HibouCoreError> {
        match self.vr_names.get(vr_id) {
            None => {
                return Err( HibouCoreError::UnknownParameter(vr_id) );
            },
            Some(my_str) => {
                return Ok( my_str.to_string() )
            }
        }
    }

    pub fn get_vr_type(&self, vr_id : usize) -> Result<TD_DataType,HibouCoreError> {
        match self.vr_types.get(vr_id) {
            None => {
                return Err( HibouCoreError::UnknownParameter(vr_id) );
            },
            Some(my_type) => {
                return Ok( my_type.clone() )
            }
        }
    }

}
