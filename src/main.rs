use std::{collections::HashMap, io::Result, path::PathBuf};

#[derive(Debug)]
struct Gallery<'a> {
    name: &'a str,
    pics: Vec<&'a str>,
    folder: Option<&'a str>,
}

///slice the line if it has a pipe: thats the folder to use
fn maybe_slice_line(line: &str) -> (&str, Option<&str>) {
    if let Some(loc) = line.find("|") {
        (&line[..loc], Some(&line[loc+1..]))
    } else {
        (line, None)
    }
}

fn read_data(data: &str) -> Vec<Gallery<'_>> {
    let mut current_gallery: Option<Gallery> = None;
    let mut galleries = vec![];
    for line in data.lines() {
        if let Some(gal) = &mut current_gallery {
            if line == "" {
                galleries.push(current_gallery.take().unwrap());
            } else {
                gal.pics.push(line);
            }
        } else {
            let (name, folder) = maybe_slice_line(&line);
            current_gallery = Some(Gallery {
                name, folder,
                pics: vec![],
            });
        }
    }

    if let Some(x) = current_gallery.take() {
        galleries.push(x);
    }

    galleries
}

fn read_folders(pics_dir: &str) -> HashMap<String, Vec<String>> {
    let dirs = std::fs::read_dir(&pics_dir).unwrap();

    let mut dirmap = HashMap::new();
    for nikon_dir in dirs {
        if let Ok(d) = nikon_dir {
            let name = d.file_name();
            let mut dirname = PathBuf::new();
            dirname.push(&pics_dir);
            dirname.push(&name);
            let dir = std::fs::read_dir(dirname).unwrap();
            //insert folder name + files list
            dirmap.insert(
                name.to_string_lossy().into_owned(),
                dir.map(|x| x.unwrap().file_name().to_string_lossy().into_owned())
                    .collect::<Vec<_>>(),
            );
        }
    }

    dirmap
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("./data.txt").unwrap();

    let galleries = read_data(&data);
    let pics_dir = "./picsc/";
    let out_dir = "./dist/";
    let dirs = read_folders(&pics_dir);

    for gallery in galleries {
        //this could be done in one filter map but whatever
        let dirsthatwork = dirs
            .iter()
            .filter(|(folder, files)| {
                if let Some(reqfolder) = gallery.folder {
                    if !folder.contains(reqfolder) {
                        return false;
                    }
                }

                for picnumber in &gallery.pics {
                    let ok = files.iter().any(|filename| filename.contains(picnumber));
                    if !ok {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<_>>();

        enum PicSet {
            Missing,
            Ok,
            Multiple,
        }

        let picset = match dirsthatwork.len() {
            0 => PicSet::Missing,
            1 => PicSet::Ok,
            _ => PicSet::Multiple,
        };

        match picset {
            PicSet::Missing  => println!("Missing possible structure... {}", gallery.name),
            PicSet::Ok       => println!("Ok... {}", gallery.name),
            PicSet::Multiple => println!("Multiple... {}", gallery.name),
        };

        for (folder, files) in dirsthatwork {
            let mut folder_path = PathBuf::new();
            folder_path.push(out_dir);
            folder_path.push(gallery.name);
            //if multiple match, use folder to disambiguate
            if let PicSet::Multiple = picset {
                folder_path.push(folder);
            }
            //println!("adding {} as {:#?}", gallery.name, &folder_path);
            std::fs::create_dir_all(&folder_path).unwrap();

            let mut foldersrc = PathBuf::new();
            foldersrc.push(pics_dir);
            foldersrc.push(folder);

            for (number, picname) in gallery.pics.iter().enumerate() {
                let fullpathpic = files.iter().find(|name| name.contains(picname)).unwrap();

                let mut dest = folder_path.clone();
                dest.push(format!("{:03}.jpg", number));

                let mut src = foldersrc.clone();
                src.push(fullpathpic);

                std::fs::copy(src, dest)?;
            }
        }
    }

    Ok(())
}
