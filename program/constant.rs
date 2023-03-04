use anchor_lang::prelude::*;

#[constant]
pub const USER_TAG: &[u8] = b"USER_STATE";

#[constant]
pub const TODO_TAG: &[u8] = b"TODO_STATE";

#[constant]
pub const TODO_STATUS_TODO: u8 = 0;

#[constant]
pub const TODO_STATUS_START: u8 = 1;

#[constant]
pub const TODO_STATUS_DONE: u8 = 2;

#[constant]
pub const TODO_STATUS_ABANDON: u8 = 3;