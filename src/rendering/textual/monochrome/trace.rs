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

use regex::Regex;
use crate::core::trace::*;
use crate::core::syntax::action::*;


pub fn trace_from_text(text : &str, lf_vec : &Vec<String>, ms_vec : &Vec<String>) -> IndexedTrace {
    let re_splitter = Regex::new( r"\." ).unwrap();
    let events_text : Vec<&str> = re_splitter.split( text ).collect();
    //println!("text : {} - splitted : {:?}",text,events_text);
    //
    let mut trace : IndexedTrace = Vec::new();
    for event_text in events_text {
        trace.push( indexed_event_from_text(event_text, lf_vec, ms_vec) );
    }
    return trace;
}




fn parse_trace_parameters(parameters_as_str : &str) -> Vec<TraceParameter> {
    let re_prms = Regex::new( r"," ).unwrap();
    let prm_str_vec : Vec<&str> = re_prms.split(parameters_as_str).collect();
    let mut prm_vec : Vec<TraceParameter> = Vec::new();
    // ***
    for prm_str in prm_str_vec {
        prm_vec.push( TraceParameter::Raw(prm_str.to_string()) );
    }
    return prm_vec;
}



fn indexed_event_from_text(my_string : &str, lf_vec : &Vec<String>, ms_vec : &Vec<String>) -> IndexedEvent {
    let emission_regex = Regex::new(REGEX_EMISSION).unwrap();
    let reception_regex = Regex::new(REGEX_RECEPTION).unwrap();
    //
    if emission_regex.is_match(&my_string) {
        let caps = emission_regex.captures(&my_string).unwrap();
        let base_act_str : &str;
        let arguments : Option<&str>;
        match caps.get(1) {
            None => {
                base_act_str = &my_string;
                arguments = None;
            }
            Some(cap1) => {
                let arguments_with_parenthesis = cap1.as_str();
                base_act_str = &my_string[..my_string.len() - arguments_with_parenthesis.len()];
                arguments = Some(&arguments_with_parenthesis[1..arguments_with_parenthesis.len()-1]);
            }
        }
        // ***
        let re_emission = Regex::new( r"!" ).unwrap();
        let (lf_id , ms_id) = get_lf_ms_ids(re_emission, base_act_str, lf_vec, ms_vec);
        // ***
        match arguments {
            None => {
                return IndexedEvent{lf_id:lf_id, evt_kind : SdEventKind::Emission, ms_id : ms_id, params : Vec::new()};
            },
            Some(parameters_as_str) => {
                let params = parse_trace_parameters(parameters_as_str);
                return IndexedEvent{lf_id:lf_id, evt_kind : SdEventKind::Emission, ms_id : ms_id, params : params};
            }
        }
    } else if reception_regex.is_match(&my_string) {
        let caps = reception_regex.captures(&my_string).unwrap();
        let base_act_str : &str;
        let arguments : Option<&str>;
        match caps.get(1) {
            None => {
                base_act_str = &my_string;
                arguments = None;
            }
            Some(cap1) => {
                let arguments_with_parenthesis = cap1.as_str();
                base_act_str = &my_string[..my_string.len() - arguments_with_parenthesis.len()];
                arguments = Some(&arguments_with_parenthesis[1..arguments_with_parenthesis.len()-1]);
            }
        }
        // ***
        let re_reception = Regex::new( r"\?" ).unwrap();
        let (lf_id , ms_id) = get_lf_ms_ids(re_reception, base_act_str, lf_vec, ms_vec);
        // ***
        match arguments {
            None => {
                return IndexedEvent{lf_id:lf_id, evt_kind : SdEventKind::Reception, ms_id : ms_id, params : Vec::new()};
            },
            Some(parameters_as_str) => {
                let params = parse_trace_parameters(parameters_as_str);
                return IndexedEvent{lf_id:lf_id, evt_kind : SdEventKind::Reception, ms_id : ms_id, params : params};
            }
        }
    } else {
        panic!("error while parsing trace");
    }
}



