#![cfg_attr(not(test), no_std)]

use display_interface::{DisplayError, WriteOnlyDataCommand};

mod bit_twiddling;
mod commands;

trait WriteCommand {
    fn send<I>(&self, ifc: I) -> Result<(), DisplayError>
    where
        I: WriteOnlyDataCommand;
}
