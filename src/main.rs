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



extern crate strum;

#[macro_use]
extern crate strum_macros;

extern crate rusttype;

extern crate image;

extern crate imageproc;

extern crate pest;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate clap;

extern crate tonic;

extern crate prost;

extern crate bytes;

// **********

pub mod tools;
pub mod core;
pub mod from_text;
pub mod rendering;
pub mod process;
pub mod ui;
pub mod diversity;
pub mod grpc_connect;
pub mod xlia;
// **********

use crate::ui::hibou_cli::hibou_cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    hibou_cli().await;
    return Ok(());
}


