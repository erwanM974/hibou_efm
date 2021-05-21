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
use std::fs::write;
use std::path::Path;
use clap::App;
use clap::ArgMatches;


use crate::core::context::general::GeneralContext;

use crate::core::trace::*;




use crate::rendering::custom_draw::seqdiag::interaction::draw_interaction;


use crate::rendering::process::graphic_logger::GraphicProcessLogger;

use crate::process::log::*;

use crate::process::analysis::analyze;
use crate::process::exploration::explore;
use crate::from_text::hsf_file::{ProcessKind,parse_hsf_file};
use crate::from_text::htf_file::parse_htf_file;

fn get_ascii_border() -> &'static str {
    return r#"===================="#;
}

fn get_ascii_left() -> Vec<&'static str> {
    let mut my_vec = Vec::new();
    my_vec.push(r#" ___   Holistic   "#);
    my_vec.push(r#"(o,o)  Interaction"#);
    my_vec.push(r#"{`"'}  Behavioral "#);
    my_vec.push(r#"-"-"-  Oracle     "#);
    my_vec.push(r#" \_/   Utility    "#);
    my_vec.push(r#"                  "#);
    my_vec.push(r#"   V-efm-0.1.1    "#);
    return my_vec;
}

fn print_retval(ret_print : Vec<String>) {
    let ascii_left = get_ascii_left();
    // ***
    println!("{}", get_ascii_border());
    if ret_print.len() >= ascii_left.len() {
        for i in 0..ascii_left.len() {
            println!("{}  |  {}", ascii_left[i], ret_print[i]);
        }
        for i in ascii_left.len()..ret_print.len() {
            println!("{} |  {}", " ".repeat(19),  ret_print[i]);
        }
    } else {
        for i in 0..ret_print.len() {
            println!("{}  |  {}", ascii_left[i], ret_print[i]);
        }
        for i in ret_print.len()..ascii_left.len() {
            println!("{}  |", ascii_left[i]);
        }
    }
    println!("{}", get_ascii_border());
}



pub async fn hibou_cli() -> i32 {

    let yaml = load_yaml!("hibou_cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut ret_print : Vec<String> = Vec::new();

    if let Some(matches) = matches.subcommand_matches("draw") {
        let hsf_file_path = matches.value_of("hsf").unwrap();
        match parse_hsf_file(hsf_file_path,&ProcessKind::None) {
            Err(e) => {
                ret_print.push( e.to_string() );
                print_retval(ret_print);
                return -1;
            },
            Ok( (gen_ctx,exe_ctx,my_int,hoptions) ) => {
                let spec_output_file : String;
                if matches.is_present("output") {
                    let extracted = matches.value_of("output").unwrap();
                    spec_output_file = format!("{}.png", extracted);
                } else {
                    let file_name = Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap();
                    spec_output_file = format!("{}.png", file_name);
                }
                ret_print.push( "".to_string());
                ret_print.push( "DRAWING INTERACTION".to_string());
                ret_print.push( format!("from file '{}'",hsf_file_path) );
                ret_print.push( format!("on file : {}",spec_output_file) );
                ret_print.push( "".to_string());
                draw_interaction(&spec_output_file, &my_int,&gen_ctx,&exe_ctx, &None);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("explore") {
        let hsf_file_path = matches.value_of("hsf").unwrap();
        match parse_hsf_file(hsf_file_path,&ProcessKind::Explore) {
            Err(e) => {
                ret_print.push( e.to_string() );
                print_retval(ret_print);
                return -1;
            },
            Ok( (gen_ctx,exe_ctx,my_int,hoptions) ) => {
                // ***
                ret_print.push( "".to_string());
                ret_print.push( "EXPLORING SEMANTICS".to_string());
                ret_print.push( format!("of interaction from file '{}'",hsf_file_path) );
                ret_print.push( "".to_string());
                // ***
                explore(my_int,
                        gen_ctx,exe_ctx,
                        hoptions.temporality,
                        hoptions.pre_filters,
                        hoptions.strategy,
                        hoptions.frontier_priorities,
                        hoptions.loggers).await;
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("analyze") {
        let hsf_file_path = matches.value_of("hsf").unwrap();
        match parse_hsf_file(hsf_file_path,&ProcessKind::Analyze) {
            Err(e) => {
                ret_print.push( e.to_string() );
                print_retval(ret_print);
                return -1;
            },
            Ok( (gen_ctx,exe_ctx,my_int,hoptions) ) => {
                let htf_file_path = matches.value_of("htf").unwrap();
                match parse_htf_file(htf_file_path,&gen_ctx,&hoptions.temporality) {
                    Err(e) => {
                        ret_print.push( e.to_string() );
                        print_retval(ret_print);
                        return -1;
                    },
                    Ok( multi_trace ) => {
                        ret_print.push( "ANALYZING TRACE".to_string());
                        ret_print.push( format!("from file '{}'",htf_file_path) );
                        ret_print.push( "W.R.T. INTERACTION".to_string());
                        ret_print.push( format!("from file '{}'",hsf_file_path) );
                        ret_print.push( "".to_string());

                        let verdict = analyze(my_int,
                                              multi_trace,
                                              gen_ctx,
                                              exe_ctx,
                                              hoptions.temporality,
                                              hoptions.pre_filters,
                                              hoptions.strategy,
                                              hoptions.frontier_priorities,
                                              hoptions.loggers,
                                              hoptions.goal.unwrap()).await;

                        ret_print.push( format!("verdict: '{}'", verdict.to_string() ) );
                    }
                }
            }
        }
    } else {
        ret_print.push( "".to_string() );
        ret_print.push( "TYPE help or -h to get a summary of the utilities".to_string() );
    }
    // ***
    print_retval(ret_print);
    return 0;
}

