use crate::prelude::*;
use log::debug;
use std::collections::HashMap;


#[derive(Debug)]
pub struct CGroupTasks{
    pub tasks_uid: u32,
    pub c_task_uid: libc::uid_t,
    pub tasks_gid: u32,
    pub c_task_gid: libc::gid_t,
    pub ctrl_uid: u32,
    pub c_ctrl_uid: libc::uid_t,
    pub ctrl_gid: u32,
    pub c_ctrl_gid: libc::gid_t,
}


#[derive(Debug)]
pub struct CGroupBuilder<'a,'b>{
    name: &'a str,
    c_groups: *mut cgroup,
    cgroups: HashMap<&'b str,CGroupControllerBuilder<'b>>,
}


#[derive(Debug)]
pub struct CGroupControllerBuilder<'a>{
    name: &'a str,
    c_groups: *mut cgroup,
    c_groups_ctrl: *mut cgroup_controller,
}


impl<'a> CGroupControllerBuilder<'a>{
    pub fn new(name:&'a str,c_groups:*mut cgroup,c_groups_ctrl:*mut cgroup_controller)->Self{
        Self{name,c_groups,c_groups_ctrl}
    }

    pub fn add_str(&mut self,name:&str,value:&str)->Result<(),std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = std::ffi::CString::new(value)?;
            let ret = cgroup_add_value_string(self.c_groups_ctrl,c_name.as_ptr(),c_value.as_ptr());
            debug!("(CGroupControllerBuilder::add_str) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }

    pub fn add_i64(&mut self,name:&str,value:i64)->Result<(),std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = libc::c_longlong::from(value);
            let ret = cgroup_add_value_int64(self.c_groups_ctrl,c_name.as_ptr(),c_value);
            debug!("(CGroupControllerBuilder::add_i64) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }

    pub fn add_u64(&mut self,name:&str,value:u64)->Result<(),std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = libc::c_ulonglong::from(value);
            let ret = cgroup_add_value_uint64(self.c_groups_ctrl,c_name.as_ptr(),c_value);
            debug!("(CGroupControllerBuilder::add_u64) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }

    pub fn add_bool(&mut self,name:&str,value:bool)->Result<(),std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = libc::c_int::from( if value.eq(&true) {1} else {0} as i32);
            let ret = cgroup_add_value_bool(self.c_groups_ctrl,c_name.as_ptr(),c_value);
            debug!("(CGroupControllerBuilder::add_bool) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }


    pub fn set_str(&mut self,name:&str,value:&str)->Result<(),std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = std::ffi::CString::new(value)?;
            let ret = cgroup_set_value_string(self.c_groups_ctrl,c_name.as_ptr(),c_value.as_ptr());
            debug!("(CGroupControllerBuilder::set_str) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }

    pub fn set_i64(&mut self,name:&str,value:i64)->Result<(),std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = libc::c_longlong::from(value);
            let ret = cgroup_set_value_int64(self.c_groups_ctrl,c_name.as_ptr(),c_value);
            debug!("(CGroupControllerBuilder::set_i64) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }

    pub fn set_u64(&mut self,name:&str,value:u64)->Result<(),std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = libc::c_ulonglong::from(value);
            let ret = cgroup_set_value_uint64(self.c_groups_ctrl,c_name.as_ptr(),c_value);
            debug!("(CGroupControllerBuilder::set_u64) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }

    pub fn set_bool(&mut self,name:&str,value:bool)->Result<(),std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = libc::c_int::from( if value.eq(&true) {1} else {0} as i32);
            let ret = cgroup_set_value_bool(self.c_groups_ctrl,c_name.as_ptr(),c_value);
            debug!("(CGroupControllerBuilder::set_bool) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }


    pub fn get_str(&mut self,name:&str)->Result<Option<String>,std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = std::ptr::null();
            let ret = cgroup_get_value_string(self.c_groups_ctrl,c_name.as_ptr(),&c_value as *const *const libc::c_char);
            debug!("(CGroupControllerBuilder::get_str) = {}",ret);

            return match ret {
                C_CG_SUCCESS => Ok(Some(std::ffi::CStr::from_ptr(c_value)
                    .to_string_lossy()
                    .to_owned()
                    .to_string())),
                C_CG_NOT_PARAM => Ok(None),
                _ => Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ))
            };
        }
    }

    pub fn get_i64(&mut self,name:&str)->Result<Option<i64>,std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = std::ptr::null_mut();
            let ret = cgroup_get_value_int64(self.c_groups_ctrl,c_name.as_ptr(),c_value as *mut libc::c_longlong);
            debug!("(CGroupControllerBuilder::get_i64) = {}",ret);
            return match ret {
                C_CG_SUCCESS => Ok(Some(c_value as i64)),
                C_CG_NOT_PARAM => Ok(None),
                _ => Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ))
            }
        }
    }

    pub fn get_u64(&mut self,name:&str)->Result<Option<u64>,std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = std::ptr::null_mut();
            let ret = cgroup_get_value_uint64(self.c_groups_ctrl,c_name.as_ptr(),c_value as *mut libc::c_ulonglong);
            debug!("(CGroupControllerBuilder::get_u64) = {}",ret);
            return match ret {
                C_CG_SUCCESS => Ok(Some(c_value as u64)),
                C_CG_NOT_PARAM => Ok(None),
                _ => Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ))
            }
        }
    }


    pub fn get_bool(&mut self,name:&str)->Result<Option<bool>,std::io::Error>{
        unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_value = std::ptr::null_mut();
            let ret = cgroup_get_value_bool(self.c_groups_ctrl,c_name.as_ptr(),c_value as *mut libc::c_int);
            debug!("(CGroupControllerBuilder::get_bool) = {}",ret);
            return match ret {
                C_CG_SUCCESS => Ok(Some((c_value as i32) > 0)),
                C_CG_NOT_PARAM => Ok(None),
                _ => Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ))
            }
        }
    }


    pub fn get_value_name_count(&self)->i32{
        unsafe {
            let ret = cgroup_get_value_name_count(self.c_groups_ctrl);
            debug!("(CGroupControllerBuilder::get_value_name_count) = {}",ret);
            ret.to_owned()
        }
    }

    pub fn get_value_name(&self,idx:i32)->Option<String>{
        unsafe {
            let c_idx = libc::c_int::from(idx);
            let ret = cgroup_get_value_name(self.c_groups_ctrl,c_idx);
            debug!("(CGroupControllerBuilder::get_value_name) = {:?}",ret);
            if ret.is_null() {
                None
            }else {
                Some(std::ffi::CStr::from_ptr(ret).to_string_lossy().to_string())
            }
        }
    }

}


impl<'a,'b> CGroupBuilder<'a,'b>{

    pub fn new(name:&'a str)->Result<Self,std::io::Error>{
        let mut handler = Self{
            name,
            c_groups:std::ptr::null_mut(),
            cgroups:HashMap::new()
        };

        handler.c_groups = unsafe {
            let c_str = std::ffi::CString::new(handler.name)?;
            cgroup_new_cgroup(c_str.as_ptr())
        };
        debug!("(CGroupBuilder::new) = c_group - {:?} ",handler.c_groups);

        if handler.c_groups.is_null() {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }

        Ok(handler)
    }


    pub fn add_controller(&mut self, name:&'b str) ->Result<(),std::io::Error>{
        self.cgroups.insert(name,unsafe {
            let c_name = std::ffi::CString::new(name)?;
            let c_ctrl_ptr = cgroup_add_controller(self.c_groups,c_name.as_ptr());
            debug!("(CGroupBuilder::add_controller) = {} - {:?}",name,c_ctrl_ptr);
            if c_ctrl_ptr.is_null() {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
            }
            CGroupControllerBuilder::new(name,self.c_groups,c_ctrl_ptr)
        });

        Ok(())
    }


    pub fn get_controller(&self,name:&'b str)->Option<&CGroupControllerBuilder<'b>>{
        self.cgroups.get(name)
    }


    pub fn get_mut_controller(&mut self,name:&'b str)->Option<&mut CGroupControllerBuilder<'b>>{
        self.cgroups.get_mut(name)
    }


    pub fn free_controllers(&self){
        if !self.c_groups.is_null() {
            unsafe {
                cgroup_free_controllers(self.c_groups);
            }
        }
    }

    pub fn free(&self){
        if !self.c_groups.is_null() {
            unsafe {
                let c_group = self.c_groups;
                let c_group_ptr =  c_group as *const *const cgroup;
                cgroup_free(c_group_ptr);
            }
        }
    }


    pub fn create(&self,ignore_ownership:i32)->Result<(),std::io::Error>{

        unsafe {
            let c_ignore_ownership = libc::c_int::from(ignore_ownership);
            let ret = cgroup_create_cgroup(self.c_groups,c_ignore_ownership);
            debug!("(CGroupBuilder::create) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }


    pub fn create_from_parent(&self,ignore_ownership:i32)->Result<(),std::io::Error>{
        unsafe {
            let c_ignore_ownership = libc::c_int::from(ignore_ownership);
            let ret = cgroup_create_cgroup_from_parent(self.c_groups,c_ignore_ownership);
            debug!("(CGroupBuilder::create_from_parent) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }


    pub fn modify(&self)->Result<(),std::io::Error>{
        unsafe {
            let ret = cgroup_modify_cgroup(self.c_groups);
            debug!("(CGroupBuilder::modify) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }


    pub fn delete(&self,ignore_migration:i32)->Result<(),std::io::Error>{
        unsafe {
            let c_ignore_migration = libc::c_int::from(ignore_migration);
            let ret = cgroup_delete_cgroup(self.c_groups,c_ignore_migration);
            debug!("(CGroupBuilder::delete) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }


    pub fn delete_ext(&self,flags:i32)->Result<(),std::io::Error>{
        unsafe {
            let c_flags = libc::c_int::from(flags);
            let ret = cgroup_delete_cgroup_ext(self.c_groups,c_flags);
            debug!("(CGroupBuilder::delete_ext) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }


    pub fn get_tasks(&self)->Result<CGroupTasks,std::io::Error>{
        unsafe {
            let mut c_task_uid: libc::uid_t = 0;
            let mut c_task_gid: libc::gid_t = 0;
            let mut c_ctrl_uid: libc::uid_t = 0;
            let mut c_ctrl_gid: libc::gid_t = 0;

            let ret = cgroup_get_uid_gid(self.c_groups,
                               &mut c_task_uid as *mut libc::uid_t,
                               &mut c_task_gid as *mut libc::gid_t,
                               &mut c_ctrl_uid as *mut libc::uid_t,
                               &mut c_ctrl_gid as *mut libc::gid_t
            );
            debug!("(CGroupBuilder::get_tasks) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }

            Ok(CGroupTasks{
                c_task_uid,
                tasks_uid: c_task_uid as u32,
                c_task_gid,
                tasks_gid: c_task_gid as u32,
                c_ctrl_uid,
                ctrl_uid: c_ctrl_uid as u32,
                c_ctrl_gid,
                ctrl_gid: c_ctrl_gid as u32
            })
        }
    }

    pub fn set_tasks(&self,tasks:&CGroupTasks)->Result<(),std::io::Error>{
        unsafe {
            let c_task_uid: libc::uid_t = tasks.c_task_uid;
            let c_task_gid: libc::gid_t = tasks.c_task_gid;
            let c_ctrl_uid: libc::uid_t = tasks.c_ctrl_uid;
            let c_ctrl_gid: libc::gid_t = tasks.c_ctrl_gid;

            let ret = cgroup_set_uid_gid(self.c_groups,
                                         c_task_uid,
                                         c_task_gid,
                                         c_ctrl_uid,
                                         c_ctrl_gid
            );

            debug!("(CGroupBuilder::set_tasks) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
            Ok(())
        }
    }


    pub fn attach_task(&self)->Result<(),std::io::Error>{
        unsafe {
            let ret = cgroup_attach_task(self.c_groups);
            debug!("(CGroupBuilder::attach_task) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }

    pub fn attach_task_pid(&self,pid:i32)->Result<(),std::io::Error>{
        unsafe {
            let c_pid = libc::pid_t::from(pid);
            let ret = cgroup_attach_task_pid(self.c_groups,c_pid);
            debug!("(CGroupBuilder::attach_task_pid) = {}",ret);
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    std::ffi::CStr::from_ptr(cgroup_strerror(ret))
                        .to_string_lossy()
                        .into_owned()
                ));
            }
        }
        Ok(())
    }

    pub fn attach_shell(&self,root:&str)->Result<(),std::io::Error>{
        let c_exec = std::ffi::CString::new("/bin/sh")?;
        unsafe {
            let c_root = std::ffi::CString::new(root)?;
            let ret = libc::chroot(c_root.as_ptr());
            if ret != C_CG_SUCCESS {
                return Err(std::io::Error::new(
                    std::io::Error::from_raw_os_error(ret).kind(),
                    "Failed by change root path in CGroup"
                ));
            }


            let pid = libc::fork();
            if pid < 0 {
                return Err(std::io::Error::from(std::io::ErrorKind::AddrNotAvailable));
            }else if pid > 0 {
                let mut status: libc::c_int = 0;
                let c_pid = libc::wait(&mut status);
                debug!("Parent = {}",c_pid);
            }else {
                self.attach_task_pid(libc::getpid())?;
                debug!("Child = {}",libc::getpid());
                let ret = libc::execl(c_exec.as_ptr(),std::ptr::null());
                if ret != C_CG_SUCCESS {
                    return Err(std::io::Error::new(
                        std::io::Error::from_raw_os_error(ret).kind(),
                        "Failed by Attach CGroup Shell"
                    ));
                }
            }
        }
        Ok(())
    }
}




