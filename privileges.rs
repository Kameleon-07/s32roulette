pub mod admin_privileges {
    #[cfg(target_os = "windows")]
    mod windows {
        pub use winapi::um::processthreadsapi::GetCurrentProcess;
        pub use winapi::um::processthreadsapi::OpenProcessToken;
        pub use winapi::um::securitybaseapi::GetTokenInformation;
        pub use winapi::um::winnt::TokenElevation;
        pub use winapi::um::winnt::HANDLE;
        pub use winapi::um::winnt::TOKEN_ELEVATION;
        pub use winapi::um::winnt::TOKEN_QUERY;
    }

    #[cfg(target_os = "windows")]
    pub fn is_launched_as_admin() -> bool {
        unsafe {
            let mut token_handle: windows::HANDLE = std::ptr::null_mut();
            let process_handle = windows::GetCurrentProcess();
            let mut is_admin: winapi::shared::minwindef::BOOL = 0;

            if windows::OpenProcessToken(process_handle, windows::TOKEN_QUERY, &mut token_handle)
                != 0
            {
                let mut token_info: windows::TOKEN_ELEVATION = std::mem::zeroed();
                let mut return_length: u32 = 0;

                if windows::GetTokenInformation(
                    token_handle,
                    windows::TokenElevation,
                    &mut token_info as *mut _ as winapi::um::winnt::PVOID,
                    std::mem::size_of::<windows::TOKEN_ELEVATION>() as u32,
                    &mut return_length,
                ) != 0
                {
                    is_admin = token_info.TokenIsElevated as i32;
                }
            }

            if is_admin < 1  {
                false
            } else {
                true
            }
        }
    }

    #[cfg(target_os = "linux")]
    use nix::unistd::Uid;
    #[cfg(target_os = "linux")]
    pub fn is_launched_as_admin() -> bool {
        if !Uid::effective().is_root() {
            false
        } else {
            true
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    pub fn is_launched_as_admin()  -> bool {
        println!("Ayo wtf is that system stfu bro");
        false
    }
}
