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

pub static SYNTAX_POSITION_LEFT: &'static str = "1";
pub static SYNTAX_POSITION_RIGHT: &'static str = "2";
pub static SYNTAX_POSITION_EPSILON: &'static str = "ε";

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Position {
    Left(Box<Position>),
    Right(Box<Position>),
    Epsilon
}


impl Position {

    pub fn from_vec(pos_vec : &mut Vec<u32>) -> Position {
        if pos_vec.len() == 0 {
            return Position::Epsilon;
        } else {
            let first = pos_vec.remove(0);
            let continuation = Position::from_vec(pos_vec);
            if first == 1 {
                return Position::Left( Box::new(continuation) );
            } else if first == 2 {
                return Position::Right( Box::new(continuation) );
            } else {
                panic!();
            }
        }
    }

}

use core::cmp::Ordering;

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            &Position::Epsilon => {
                match other {
                    &Position::Epsilon => {
                        return Ordering::Equal;
                    },
                    _ => {
                        return Ordering::Less;
                    }
                }
            },
            &Position::Left(ref in_self) => {
                match other {
                    &Position::Left(ref in_other) => {
                        return (**in_self).cmp( &**in_other );
                    },
                    &Position::Right(ref in_other) => {
                        return Ordering::Less;
                    },
                    &Position::Epsilon => {
                        return Ordering::Greater;
                    }
                }
            },
            &Position::Right(ref in_self) => {
                match other {
                    &Position::Left(ref in_other) => {
                        return Ordering::Greater;
                    },
                    &Position::Right(ref in_other) => {
                        return (**in_self).cmp( &**in_other );
                    },
                    &Position::Epsilon => {
                        return Ordering::Greater;
                    }
                }
            }
        }
    }
}

/*
impl Position {

    pub fn as_text(&self) -> String {
        match self {
            Position::Left(ref in_self) => {
                let mut my_string = SYNTAX_POSITION_LEFT.to_string();
                my_string.push_str( &(*in_self).as_text() );
                return my_string;
            },
            Position::Right(ref in_self) => {
                let mut my_string = SYNTAX_POSITION_RIGHT.to_string();
                my_string.push_str( &(*in_self).as_text() );
                return my_string;
            },
            Position::Epsilon => {
                return SYNTAX_POSITION_EPSILON.to_string();
            }
        }
    }

}






*/
