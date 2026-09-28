//! Checks if the current Windows process is elevated.
//! Returns true if the process is elevated, false if not.
//! ## Example
//! ```rust
//! use check_elevation::is_elevated;
//!
//! if is_elevated().expect("Failed to get elevation status.") {
//!     println!("Running as administrator.");
//! } else {
//!     println!("Not running as administrator.");
//! }
//! ```
//!
//! made with ♥  by h4rl
//! uses bsd-2-clause license

#![no_std]

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    Security::{GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation},
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

pub fn is_elevated() -> windows::core::Result<bool> {
    unsafe {
        let mut h_token = HANDLE(0 as _);
        if let Err(error) = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &raw mut h_token) {
            CloseHandle(h_token)?;
            return Err(error);
        }

        let mut token_elevation = core::mem::zeroed::<TOKEN_ELEVATION>();
        if let Err(error) = GetTokenInformation(
            h_token,
            TokenElevation,
            Some(core::ptr::addr_of_mut!(token_elevation).cast()),
            u32::try_from(size_of::<TOKEN_ELEVATION>())?,
            &mut 0,
        ) {
            CloseHandle(h_token)?;
            return Err(error);
        }

        CloseHandle(h_token)?;
        Ok(token_elevation.TokenIsElevated != 0)
    }
}
