use lib_containers::space::SpaceBuilder;

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
