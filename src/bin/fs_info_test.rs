extern crate linux;

fn main() {
    //let info = linux::fs::info::FileSystemInfo::from_path("/sys");
    let file = linux::file::File::open_read("/").unwrap();
    let info = file.fs_info();
    println!("{:?}", info);
}
