use steamlocate::SteamDir;

fn unable_locate_dir() {
    print!("Couldn't locate Steam or Vhs install dir");
}

fn main() {
    println!("Hello, world!");
    match SteamDir::locate() {
        Some(mut steamdir) => match steamdir.app(&611360) {
            Some(app) => println!("{:#?}", app),
            None => unable_locate_dir(),
        },
        None => unable_locate_dir(),
    }
}
