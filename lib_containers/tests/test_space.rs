use lib_containers::space::{SpaceBuilder,SpaceDisposer};

#[test]
fn space_worker() -> Result<(),std::io::Error>{

    let mut builder = SpaceBuilder::new();
    builder
        .set_target_path("/tmp/mem")
        .set_src_path("tmpfs")
        .set_type_name("tmpfs")
        .set_opts("mode=1777")
        .set_flags(0)
        .mount()?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    builder.umount(SpaceBuilder::MNT_DETACH|SpaceBuilder::MNT_FORCE)?;

    Ok(())
}


#[test]
fn cgroup2_test() -> Result<(),std::io::Error>{
    let mut builder = SpaceBuilder::new();

    builder
        .set_target_path("/mnt/cgroup2")
        .set_src_path("none")
        .set_type_name("cgroup2")
        .mount()?;

    std::thread::sleep(std::time::Duration::from_secs(5));
    builder
        .umount(SpaceBuilder::MNT_DETACH|SpaceBuilder::MNT_FORCE)?;

    Ok(())
}


#[test]
fn disposer_test() -> Result<(),std::io::Error>{
    let entries = std::fs::read_dir("/")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    println!("ROOT:{:?}",entries);

    let disposer = SpaceDisposer::from("/mem_vm")?;
    disposer.update()?;


    let entries = std::fs::read_dir("/")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    println!("CHROOT :{:?}",entries);
    Ok(())
}
