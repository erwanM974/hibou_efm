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

use std::cmp;

use crate::core::syntax::position::*;
use crate::core::syntax::action::*;

#[derive(Clone, PartialEq, Debug)]
pub enum ScheduleOperatorKind {
    Strict,
    Seq,
    Par
}

#[derive(Clone, PartialEq, Debug)]
pub enum Interaction {
    Empty,
    Action(ObservableAction),
    Strict(Box<Interaction>,Box<Interaction>),
    Seq(Box<Interaction>,Box<Interaction>),
    Alt(Box<Interaction>,Box<Interaction>),
    Par(Box<Interaction>,Box<Interaction>),
    Loop(ScheduleOperatorKind,Box<Interaction>),
    Scope(Vec<usize>,Box<Interaction>)
}


impl Interaction {

    pub fn substitute(&self, subst_int : Interaction, my_pos : &Position) -> Interaction {
        match my_pos {
            Position::Epsilon => {
                return subst_int;
            },
            Position::Left(sub_pos) => {
                match self {
                    Interaction::Strict(i1,i2) => {
                        let substituted_i1 = (*i1).substitute(subst_int, &(*sub_pos) );
                        if substituted_i1 == Interaction::Empty {
                            return *i2.clone();
                        } else {
                            return Interaction::Strict( Box::new(substituted_i1), (*i2).clone() );
                        }
                    },
                    Interaction::Seq(i1,i2) => {
                        let substituted_i1 = (*i1).substitute(subst_int, &(*sub_pos) );
                        if substituted_i1 == Interaction::Empty {
                            return *i2.clone();
                        } else {
                            return Interaction::Seq( Box::new(substituted_i1), (*i2).clone() );
                        }
                    },
                    Interaction::Alt(i1,i2) => {
                        return Interaction::Alt( Box::new((*i1).substitute(subst_int, &(*sub_pos) ) ), (*i2).clone() );
                    },
                    Interaction::Par(i1,i2) => {
                        let substituted_i1 = (*i1).substitute(subst_int, &(*sub_pos) );
                        if substituted_i1 == Interaction::Empty {
                            return *i2.clone();
                        } else {
                            return Interaction::Par( Box::new(substituted_i1), (*i2).clone() );
                        }
                    },
                    Interaction::Loop(kind,i1) => {
                        let substituted_i1 = (*i1).substitute(subst_int, &(*sub_pos) );
                        if substituted_i1 == Interaction::Empty {
                            return Interaction::Empty;
                        } else {
                            return Interaction::Loop(kind.clone(), Box::new(substituted_i1) );
                        }
                    },
                    Interaction::Scope(scope,i1) => {
                        let substituted_i1 = (*i1).substitute(subst_int, &(*sub_pos) );
                        if substituted_i1 == Interaction::Empty {
                            return Interaction::Empty;
                        } else {
                            return Interaction::Scope(scope.clone(), Box::new(substituted_i1) );
                        }
                    },
                    _ => {
                        panic!("cannot substitute on a position that does not exist within the interaction");
                    }
                }
            },
            Position::Right(sub_pos) => {
                match self {
                    Interaction::Strict(i1,i2) => {
                        let substituted_i2 = (*i2).substitute(subst_int, &(*sub_pos) );
                        if substituted_i2 == Interaction::Empty {
                            return *i1.clone();
                        } else {
                            return Interaction::Strict( (*i1).clone(), Box::new(substituted_i2) );
                        }
                    },
                    Interaction::Seq(i1,i2) => {
                        let substituted_i2 = (*i2).substitute(subst_int, &(*sub_pos) );
                        if substituted_i2 == Interaction::Empty {
                            return *i1.clone();
                        } else {
                            return Interaction::Seq( (*i1).clone(), Box::new(substituted_i2) );
                        }
                    },
                    Interaction::Alt(i1,i2) => {
                        return Interaction::Alt( (*i1).clone(), Box::new((*i2).substitute(subst_int, &(*sub_pos) )) );
                    },
                    Interaction::Par(i1,i2) => {
                        let substituted_i2 = (*i2).substitute(subst_int, &(*sub_pos) );
                        if substituted_i2 == Interaction::Empty {
                            return *i1.clone();
                        } else {
                            return Interaction::Par( (*i1).clone(), Box::new(substituted_i2) );
                        }
                    },
                    _ => {
                        panic!("cannot substitute on a position that does not exist within the interaction");
                    }
                }
            }
        }
    }

    pub fn decorate_with_initial_positions(&self, prefix : Vec<u32>) -> Interaction {
        match &self {
            &Interaction::Empty => {
                return Interaction::Empty;
            }, &Interaction::Action(ref act ) => {
                let mut new_act = act.clone();
                new_act.original_position = Some(prefix);
                return Interaction::Action( new_act )
            }, &Interaction::Strict(ref i1, ref i2) => {
                let mut left = prefix.clone();
                left.push(1);
                let new_i1 = i1.decorate_with_initial_positions(left);
                let mut right = prefix.clone();
                right.push(2);
                let new_i2 = i2.decorate_with_initial_positions(right);
                return Interaction::Strict(Box::new(new_i1),Box::new(new_i2));
            }, &Interaction::Seq(ref i1, ref i2) => {
                let mut left = prefix.clone();
                left.push(1);
                let new_i1 = i1.decorate_with_initial_positions(left);
                let mut right = prefix.clone();
                right.push(2);
                let new_i2 = i2.decorate_with_initial_positions(right);
                return Interaction::Seq(Box::new(new_i1),Box::new(new_i2));
            }, &Interaction::Par(ref i1, ref i2) => {
                let mut left = prefix.clone();
                left.push(1);
                let new_i1 = i1.decorate_with_initial_positions(left);
                let mut right = prefix.clone();
                right.push(2);
                let new_i2 = i2.decorate_with_initial_positions(right);
                return Interaction::Par(Box::new(new_i1),Box::new(new_i2));
            }, &Interaction::Alt(ref i1, ref i2) => {
                let mut left = prefix.clone();
                left.push(1);
                let new_i1 = i1.decorate_with_initial_positions(left);
                let mut right = prefix.clone();
                right.push(2);
                let new_i2 = i2.decorate_with_initial_positions(right);
                return Interaction::Alt(Box::new(new_i1),Box::new(new_i2));
            }, &Interaction::Loop(ref lkind, ref i1) => {
                let mut left = prefix.clone();
                left.push(1);
                let new_i1 = i1.decorate_with_initial_positions(left);
                return Interaction::Loop(lkind.clone(),Box::new(new_i1));
            }, &Interaction::Scope(ref scope, ref i1) => {
                let mut left = prefix.clone();
                left.push(1);
                let new_i1 = i1.decorate_with_initial_positions(left);
                return Interaction::Scope(scope.clone(),Box::new(new_i1));
            }
        }
    }

    pub fn as_leaf(&self) -> &ObservableAction {
        match self {
            Interaction::Action(act) => {
                return act;
            },
            _ => {
                panic!("called as_leaf on something that's not a leaf : {:?}", self);
            }
        }
    }

    pub fn get_sub_interaction(&self, my_pos : &Position) -> &Interaction {
        match my_pos {
            Position::Epsilon => {
                return self;
            },
            Position::Left(sub_pos) => {
                match self {
                    &Interaction::Seq(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Loop(_ , ref i1) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Scope(_, ref i1) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    _ => {
                        panic!();
                    }
                }
            },
            Position::Right(sub_pos) => {
                match self {
                    &Interaction::Seq(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }

    pub fn express_empty(&self) -> bool {
        match self {
            &Interaction::Empty => {
                return true;
            }, &Interaction::Action(_) => {
                return false;
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.express_empty() || i2.express_empty();
            }, &Interaction::Loop(_, _) => {
                return true;
            }, &Interaction::Scope(_, ref i1) => {
                return i1.express_empty();
            }
        }
    }

    pub fn loop_depth(&self) -> u32 {
        match self {
            &Interaction::Empty => {
                return 0;
            }, &Interaction::Action(_) => {
                return 0;
            }, &Interaction::Strict(ref i1, ref i2) => {
                return cmp::max(i1.loop_depth(),i2.loop_depth());
            }, &Interaction::Seq(ref i1, ref i2) => {
                return cmp::max(i1.loop_depth(),i2.loop_depth());
            }, &Interaction::Alt(ref i1, ref i2) => {
                return cmp::max(i1.loop_depth(),i2.loop_depth());
            }, &Interaction::Par(ref i1, ref i2) => {
                return cmp::max(i1.loop_depth(),i2.loop_depth());
            }, &Interaction::Loop(_, ref i1) => {
                return 1 + i1.loop_depth();
            }, &Interaction::Scope(_, ref i1) => {
                return i1.loop_depth();
            }
        }
    }

    pub fn get_loop_depth_at_pos(&self, my_pos : &Position) -> u32 {
        match my_pos {
            Position::Epsilon => {
                return 0;
            },
            Position::Left(sub_pos) => {
                match self {
                    &Interaction::Alt(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Seq(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Loop(_, ref i1) => {
                        return 1 + i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Scope(_, ref i1) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    _ => {
                        panic!("undefined pos");
                    }
                }
            },
            Position::Right(sub_pos) => {
                match self {
                    &Interaction::Alt(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Seq(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    _ => {
                        panic!("undefined pos");
                    }
                }
            }
        }
    }

    pub fn avoids(&self, lf_id : usize) -> bool {
        match self {
            &Interaction::Empty => {
                return true;
            }, &Interaction::Action(ref act) => {
                if act.occupation_after().contains(&lf_id) {
                    return false;
                } else {
                    return true;
                }
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.avoids(lf_id) && i2.avoids(lf_id);
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.avoids(lf_id) && i2.avoids(lf_id);
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.avoids(lf_id) && i2.avoids(lf_id);
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.avoids(lf_id) || i2.avoids(lf_id);
            }, &Interaction::Loop(_, _) => {
                return true;
            }, &Interaction::Scope(_, ref i1) => {
                return i1.avoids(lf_id);
            }
        }
    }

}



