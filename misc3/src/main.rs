use std::{ffi::OsStr, fs, io, path::Path};

fn copy_dir_to(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.is_dir() {
        fs::create_dir(dst)?;
    }

    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
    }

    Ok(())
}

fn copy_to(src: &Path, src_type: &fs::FileType, dst: &Path) -> io::Result<()> {
    if src_type.is_file() {
        fs::copy(src, dst)?;
    } else if src_type.is_dir() {
        copy_dir_to(src, dst)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("don't know how to copy: {}", src.display()),
        ));
    }
    Ok(())
}

fn main() {
    assert_eq!(
        Path::new("/home/fwolfe/program.txt").parent(),
        Some(Path::new("/home/fwolfe"))
    );

    assert_eq!(
        Path::new("/home/fwolfe/program.txt").file_name(),
        Some(OsStr::new("program.txt"))
    );

    let file = Path::new("/home/jimb/calendars/calendar-18x18.pdf");
    assert_eq!(
        file.ancestors().collect::<Vec<_>>(),
        vec![
            Path::new("/home/jimb/calendars/calendar-18x18.pdf"),
            Path::new("/home/jimb/calendars"),
            Path::new("/home/jimb"),
            Path::new("/home"),
            Path::new("/")
        ]
    );

    for entry_result in Path::new("src").read_dir().unwrap() {
        let entry = entry_result.unwrap();
        println!("{}", entry.file_name().to_string_lossy());
    }

    let _ = copy_dir_to(Path::new("src"), Path::new("src"));
}
