use containers::space::SpaceDisposer;

fn main() -> Result<(),std::io::Error>{

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
