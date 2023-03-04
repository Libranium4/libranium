use anchor_lang::prelude::*;
use std::vec::*;

#[account]
#[derive(Default)]
pub struct User {
    pub authority: Pubkey,
    pub task_last: u8,
    pub task_count: u8,
}

#[account]
#[derive(Default)]
pub struct Task {
    pub authority: Pubkey,
    pub idx: u8,
    pub content: String,
    pub status: u8,
}