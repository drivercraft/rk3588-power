#![no_std]
//! # RK3588 电源管理驱动
//!
//! 本库提供了针对 RK3588 系列 SoC 的电源管理功能。
//!

extern crate alloc;

use core::{fmt, ptr::NonNull};

use crate::{registers::PmuRegs, variants::RockchipPmuInfo};

mod registers;
mod variants;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RkBoard {
    Rk3588,
}

pub struct RockchipPM {
    board: RkBoard,
    reg: PmuRegs,
    info: RockchipPmuInfo,
}

impl RockchipPM {
    pub fn new(base: NonNull<u8>, board: RkBoard) -> Self {
        Self {
            board,
            info: RockchipPmuInfo::new(board),
            reg: PmuRegs::new(base),
        }
    }
}
