use crate::prelude::*;

/// # Overlay
impl<'a> Discord<'a> {
    pub fn overlay_enabled(&mut self) -> bool {
        let mut enabled = false;

        unsafe {
            ffi!(self
                .get_overlay_manager()
                .is_enabled(&mut enabled as *mut _))
        }

        enabled
    }

    pub fn overlay_opened(&mut self) -> bool {
        let mut opened = false;

        unsafe { ffi!(self.get_overlay_manager().is_locked(&mut opened as *mut _)) }

        opened
    }

    pub fn set_overlay_opened<F>(&mut self, opened: bool, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_overlay_manager().set_locked(
                opened,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }

    pub fn open_invite_overlay<F>(&mut self, action: Action, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_overlay_manager().open_activity_invite(
                action.to_sys(),
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }

    pub fn open_guild_invite_overlay<S, F>(&mut self, code: S, callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<()>),
    {
        let code = std::ffi::CString::new(code.as_ref()).unwrap();

        unsafe {
            ffi!(self.get_overlay_manager().open_guild_invite(
                code.as_ptr(),
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }

    pub fn open_voice_settings<F>(&mut self, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_overlay_manager().open_voice_settings(
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(across_ffi::callbacks::result::<F>)
            ))
        }
    }
}
