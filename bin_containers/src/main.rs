use lib_containers::space::{SpaceDisposer, SpaceBuilder};

fn main() -> Result<(),std::io::Error>{

    /*
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
     */

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
