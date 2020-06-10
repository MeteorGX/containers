///! Manual: (http://www.libcg.sourceforge.net/html)



use log::{debug};


pub const CG_NOT_MOUNT:i32 = 0xc351;
pub const CG_DELETE_IGNORE_MIGRATION:i32 = 0x01;
pub const CG_DELETE_RECURSIVE:i32 = 0x02;

pub const C_CG_SUCCESS: libc::c_int = 0x0;
pub const C_CG_NOT_MOUNT: libc::c_int = 0xc351;
pub const C_CG_FAIL: libc::c_int = 0xc35d;
pub const C_CG_NOT_PARAM: libc::c_int = 0xc35f;
pub const C_CG_NOT_EQUAL: libc::c_int = 0xc361;
pub const C_CG_CONTROLLER_NOT_EQUAL: libc::c_int = 0xc362;
pub const C_CG_EOF: libc::c_int = 0xc367;


pub struct CGroupInitializer;
impl CGroupInitializer{
    pub fn init()->Result<(),std::io::Error>{
        unsafe {
            let ret = cgroup_init();
            debug!("(CGroupInitializer::init) = {}",ret);
            match ret {
                C_CG_SUCCESS|C_CG_EOF => (),
                _ => return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ))
            }
        }
        Ok(())
    }

    pub fn get_subsys_mount_point(ctrl:&str)->Result<String,std::io::Error>{
        unsafe {
            let c_ctrl = std::ffi::CString::new(ctrl)?;
            let c_point = std::ptr::null();
            let ret = cgroup_get_subsys_mount_point(
                c_ctrl.as_ptr(),
                &c_point as *const *const libc::c_char);

            debug!("(CGroupInitializer::get_subsys_mount_point) = {}",ret);

            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
            Ok(std::ffi::CStr::from_ptr(c_point)
                .to_string_lossy()
                .to_owned()
                .to_string())
        }
    }
}






#[repr(C)]
#[derive(Copy)]
pub struct CGroupControllerData {
    pub name: [libc::c_char; libc::FILENAME_MAX as usize],
    pub hierarchy: libc::c_int,
    pub num_cgroups: libc::c_int,
    pub enabled: libc::c_int,
}

impl Clone for CGroupControllerData {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for CGroupControllerData {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}




#[allow(non_camel_case_types)]
pub enum cgroup {}
#[allow(non_camel_case_types)]
pub enum cgroup_controller {}

pub type CGroup = cgroup;
pub type CGroupController = cgroup_controller;


extern "C" {

    // cgroup error
    pub fn cgroup_init() -> libc::c_int;
    pub fn cgroup_get_subsys_mount_point(ctrl:*const libc::c_char,
                                         handle: *const *const libc::c_char)
                                         -> libc::c_int;


    // errors
    pub fn cgroup_strerror(code:libc::c_int) -> *const libc::c_char;
    pub fn cgroup_get_last_errno() -> libc::c_int;


    // cgroup creator
    pub fn cgroup_new_cgroup(name:*const libc::c_char) -> *mut cgroup;
    pub fn cgroup_create_cgroup(cgroup: *const cgroup, ignore_ownership: libc::c_int) -> libc::c_int;
    pub fn cgroup_create_cgroup_from_parent(cgroup: *const cgroup,
                                            ignore_ownership: libc::c_int)
                                            -> libc::c_int;
    pub fn cgroup_add_controller(cgroup: *const cgroup,name:*const libc::c_char) -> *mut cgroup_controller;


    // cgroup modifies
    pub fn cgroup_modify_cgroup(cgroup: *const cgroup) -> libc::c_int;


    // cgroup deleter
    pub fn cgroup_delete_cgroup(cgroup: *const cgroup,ignore_migration:libc::c_int) -> libc::c_int;
    pub fn cgroup_delete_cgroup_ext(cgroup: *const cgroup, flags: libc::c_int) -> libc::c_int;



    // free
    pub fn cgroup_free(cgroup: *const *const cgroup);
    pub fn cgroup_free_controllers(cgroup: *const cgroup);


    // change param in controller
    pub fn cgroup_add_value_string(cgroup: *const cgroup_controller,
                                   name:*const libc::c_char,
                                   value:*const libc::c_char)
                                   -> libc::c_int;
    pub fn cgroup_add_value_int64(cgroup: *const cgroup_controller,
                               name:*const libc::c_char,
                               value:libc::c_longlong)
                               -> libc::c_int;
    pub fn cgroup_add_value_uint64(cgroup: *const cgroup_controller,
                                name:*const libc::c_char,
                                value:libc::c_ulonglong)
                                -> libc::c_int;
    pub fn cgroup_add_value_bool(cgroup: *const cgroup_controller,
                                name:*const libc::c_char,
                                value:libc::c_int)
                                -> libc::c_int;



    pub fn cgroup_get_value_string(cgroup: *const cgroup_controller,
                                   name:*const libc::c_char,
                                   value:*const *const libc::c_char)
                                   -> libc::c_int;
    pub fn cgroup_get_value_int64(cgroup: *const cgroup_controller,
                                name:*const libc::c_char,
                                value:*mut libc::c_longlong)
                                -> libc::c_int;
    pub fn cgroup_get_value_uint64(cgroup: *const cgroup_controller,
                                name:*const libc::c_char,
                                value:*mut libc::c_ulonglong)
                                -> libc::c_int;
    pub fn cgroup_get_value_bool(cgroup: *const cgroup_controller,
                                 name:*const libc::c_char,
                                 value:*mut libc::c_int)
                                 -> libc::c_int;



    pub fn cgroup_set_value_string(cgroup: *const cgroup_controller,
                                   name:*const libc::c_char,
                                   value:*const libc::c_char)
                                   -> libc::c_int;
    pub fn cgroup_set_value_int64(cgroup: *const cgroup_controller,
                                name:*const libc::c_char,
                                value:libc::c_longlong)
                                -> libc::c_int;
    pub fn cgroup_set_value_uint64(cgroup: *const cgroup_controller,
                                name:*const libc::c_char,
                                value:libc::c_ulonglong)
                                -> libc::c_int;
    pub fn cgroup_set_value_bool(cgroup: *const cgroup_controller,
                                 name:*const libc::c_char,
                                 value:libc::c_int)
                                 -> libc::c_int;



    pub fn cgroup_get_value_name_count(cgroup: *const cgroup_controller)-> libc::c_int;
    pub fn cgroup_get_value_name(cgroup: *const cgroup_controller,idx:libc::c_int)-> *mut libc::c_char;






    pub fn cgroup_get_uid_gid(
        cgroup: *const cgroup,
        task_uid: *mut libc::uid_t,
        task_gid: *mut libc::gid_t,
        ctrl_uid: *mut libc::uid_t,
        ctrl_gid: *mut libc::gid_t,
    ) -> libc::c_int;

    pub fn cgroup_set_uid_gid(
        cgroup: *const cgroup,
        task_uid: libc::uid_t,
        task_gid: libc::gid_t,
        ctrl_uid: libc::uid_t,
        ctrl_gid: libc::gid_t,
    ) -> libc::c_int;


    // cgroup controller each
    pub fn cgroup_get_all_controller_begin(handle: *const *const libc::c_void,
                                           info: *mut CGroupControllerData)
                                           -> libc::c_int;
    pub fn cgroup_get_all_controller_next(handle: *const *const libc::c_void,
                                          info: *mut CGroupControllerData)
                                          -> libc::c_int;
    pub fn cgroup_get_all_controller_end(handle: *const *const libc::c_void) -> libc::c_int;



    // manipulation with task
    pub fn cgroup_attach_task(cgroup: *const cgroup) -> libc::c_int;
    pub fn cgroup_attach_task_pid(cgroup: *const cgroup,pid: libc::pid_t) -> libc::c_int;
    pub fn cgroup_get_current_controller_path(
        id: libc::pid_t,
        controller: *const libc::c_char,
        current_path: *const *const libc::c_char
    ) -> libc::c_int;

}