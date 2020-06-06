

pub struct CGroupV1;

impl CGroupV1{
    pub const CG_SUCCESS: libc::c_int = 0x0;
    pub const CG_FAIL: libc::c_int = 0xc35d;
    pub const CG_NOT_EQUAL: libc::c_int = 0xc361;
    pub const CG_CONTROLLER_NOT_EQUAL: libc::c_int = 0xc362;
    pub const CG_EOF: libc::c_int = 0xc367;



    pub fn init()->Result<(),std::io::Error>{

        unsafe {
            let ret = cgroup_init();

            match ret {
                Self::CG_SUCCESS|Self::CG_EOF => (),
                _=> return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                            .to_string_lossy().into_owned()
                ))
            }
        }
        Ok(())
    }
}





pub struct CGroupV1ControllerIter{
    controllers: ControllerCtx,
    handler: *const libc::c_void,
    index: libc::c_int,
}

impl std::iter::Iterator for CGroupV1ControllerIter{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == CGroupV1::CG_SUCCESS {
            let exists = unsafe {
                std::ffi::CStr::from_ptr(self.controllers.name.as_ptr()).to_string_lossy().into_owned()
            };

            self.index = unsafe {
                let c_hand_ptr = &(self.handler);
                cgroup_get_all_controller_next(c_hand_ptr as *const *const libc::c_void,&mut self.controllers)
            };

            Some(exists)
        }else{
            None
        }
    }
}

impl CGroupV1ControllerIter{
    pub fn init() -> Result<Self,std::io::Error> {
        let mut iter = Self{
            controllers:Default::default(),
            handler:std::ptr::null(),
            index:0
        };
        unsafe {
            let c_hand_ptr = &(iter.handler);
            let ret = cgroup_get_all_controller_begin(c_hand_ptr as *const *const libc::c_void ,&mut iter.controllers);
            match ret {
                CGroupV1::CG_SUCCESS|CGroupV1::CG_EOF => (),
                _=> return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy().into_owned()
                ))
            }
        }

        Ok(iter)
    }
}






/// About C



#[repr(C)]
#[derive(Copy)]
pub struct ControllerCtx {
    pub name: [libc::c_char; libc::FILENAME_MAX as usize],
    pub hierarchy: libc::c_int,
    pub num_cgroups: libc::c_int,
    pub enabled: libc::c_int,
}

impl Clone for ControllerCtx {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for ControllerCtx {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

extern "C" {
    // cgroup error
    pub fn cgroup_init() -> libc::c_int;
    pub fn cgroup_strerror(code:libc::c_int) -> *const libc::c_char;
    pub fn cgroup_get_last_errno() -> libc::c_int;


    // cgroup controller each
    pub fn cgroup_get_all_controller_begin(handle: *const *const libc::c_void,
                                           info: *mut ControllerCtx)
                                           -> libc::c_int;
    pub fn cgroup_get_all_controller_next(handle: *const *const libc::c_void,
                                          info: *mut ControllerCtx)
                                          -> libc::c_int;
    pub fn cgroup_get_all_controller_end(handle: *const *const libc::c_void) -> libc::c_int;


}