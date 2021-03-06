# containers
Micro Container Server In Rust


### Create Container

#### Package:
```toml
cgroup_rs = { path ="../cgroup_rs", version="*" }
log = "*"
env_logger = "*"
```

#### Code:

```rust
use log::{debug};
use cgroup_rs::space::SpaceBuilder;
use cgroup_rs::prelude::{CGroupInitializer, CG_NOT_MOUNT};
use cgroup_rs::cgroup::CGroupBuilder;


fn main() -> Result<(),std::io::Error>{
    env_logger::init();

    // Initialize Cgroup
    match CGroupInitializer::init() {
        Ok(_) => (),
        Err(e) if e
            .kind()
            .eq(&std::io::Error::from_raw_os_error(CG_NOT_MOUNT).kind()) =>{

            // Mouth Space
            let mut space = SpaceBuilder::new();
            space
                .set_target_path("/dev/shm/cgroup")
                .set_src_path("cgroup")
                .set_type_name("cgroup")
                .set_opts("cpu");

            if !space.exists() {
                space.mount()?;
            }

        },
        Err(e) => return Err(e),
    }


    let mount_point = CGroupInitializer::get_subsys_mount_point("cpu")?;
    debug!("[CPU] Mount Point = {}",mount_point);

    //build cgroup
    let mut cgroup = CGroupBuilder::new("mini-container")?;
    cgroup.add_controller("cpu")?;

    if let Some(cg) = cgroup.get_mut_controller("cpu") {
        println!("CGROUP = {:?}",cg);
    }
    cgroup.create(0)?;
    cgroup.attach_shell("/")?;

    Ok(())
}
```

#### Check

```plain
$ cat /proc/self/cgroup 
```


![Attach_Shell](others/attach_shell.png)

> `20:cpu:/mini-container`
>> This is the execution container we created 

> To be continued