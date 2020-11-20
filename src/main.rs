use std::{collections::HashMap, io::Result, path::PathBuf};

#[derive(Debug)]
struct Gallery<'a> {
    name: &'a str,
    pics: Vec<&'a str>,
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
            current_gallery = Some(Gallery {
                name: line,
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
    let pics_dir = "./pics/";
    let out_dir = "./dist/";
    let dirs = read_folders(&pics_dir);

    for gallery in galleries {
        let dirsthatwork = dirs
            .iter()
            .filter(|(_folder, files)| {
                for picnumber in &gallery.pics {
                    let ok = files.iter().any(|filename| filename.contains(picnumber));
                    if !ok {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<_>>();

        //let dirnames = dirsthatwork.iter().map(|x| x.0).collect::<Vec<_>>();

        for (folder, files) in dirsthatwork {
            let mut folder_path = PathBuf::new();
            folder_path.push(out_dir);
            folder_path.push(gallery.name);
            folder_path.push(folder);
            println!("adding {} as {:#?}", gallery.name, &folder_path);
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
