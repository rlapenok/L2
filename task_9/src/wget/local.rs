use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{self, copy, Cursor},
    path::PathBuf,
};



pub enum Directory {
    Html,
    Image,
    Links,
    Script,
}





#[allow(dead_code)]
#[derive(Debug)]
pub struct Local {
    html: PathBuf,
    links: PathBuf,
    image: PathBuf,
    script: PathBuf,
}

impl Local {
    pub fn new(main_path: PathBuf) -> Self {
        let links = main_path.join("links");
        let script = main_path.join("script");
        let image = main_path.join("image");
        Self {
            html: main_path,
            links,
            script,
            image,
        }
    }
}
impl  Local {
    pub fn save(&self, dir: Directory, name: &str, data: &[u8]) -> io::Result<()> {
        //closure for create file for saave
        let create_file = |path: &PathBuf, name: &str| -> io::Result<(File, PathBuf)> {
            create_dir_all(path)
                .inspect_err(|err| eprintln!("Error while create dir({:?}) :{}", path, err))?;
            let path = path.join(name);
            let file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&path)
                .inspect_err(|err| eprintln!("Error while create file({:?}):{}", path, err))?;
            Ok((file, path))
        };

        match dir {
            Directory::Html => {
                let mut file = create_file(&self.html, name)?;
                let mut content = Cursor::new(data);
                copy(&mut content, &mut file.0)?;
                Ok(())
            }
            Directory::Image => {
                let mut file = create_file(&self.image, name)?;
                let mut content = Cursor::new(data);
                copy(&mut content, &mut file.0)?;
                Ok(())
            }
            Directory::Links => {
                let mut file = create_file(&self.links, name)?;
                let mut content = Cursor::new(data);
                copy(&mut content, &mut file.0)?;

                Ok(())
            }
            Directory::Script => {
                let mut file = create_file(&self.script, name)?;
                let mut content = Cursor::new(data);
                copy(&mut content, &mut file.0)?;
                Ok(())
            }
        }
    }
}
