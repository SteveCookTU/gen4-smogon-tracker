#[cfg(windows)]
use winres::*;

#[cfg(windows)]
fn main() {
    let mut res = WindowsResource::new();
    res.set_language(0x0409);
    res.set_icon("icon.ico");
    res.set("ProductName", "SmogonSetTracker");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
