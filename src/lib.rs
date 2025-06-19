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
    Security::{
        GetTokenInformation, TokenElevation, TOKEN_ACCESS_MASK, TOKEN_ELEVATION, TOKEN_QUERY,
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

pub fn is_elevated() -> windows::core::Result<bool> {
    unsafe {
        let mut h_token: HANDLE = HANDLE(0 as _);
        let result = OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ACCESS_MASK(TOKEN_QUERY.0),
            &mut h_token,
        );
        match result {
            Ok(_) => {
                let mut token_elevation: TOKEN_ELEVATION = core::mem::zeroed();
                let mut return_length = 0;

                match GetTokenInformation(
                    h_token,
                    TokenElevation,
                    Some(&mut token_elevation as *mut _ as *mut _),
                    core::mem::size_of::<TOKEN_ELEVATION>() as u32,
                    &mut return_length,
                ) {
                    Ok(_) => {
                        CloseHandle(h_token)?;
                        Ok(token_elevation.TokenIsElevated != 0)
                    }
                    Err(e) => {
                        CloseHandle(h_token)?;
                        Err(e)
                    }
                }
            }
            Err(e) => {
                CloseHandle(h_token)?;
                Err(e)
            }
        }
    }
}
