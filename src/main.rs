use std::io::Result;

#[derive(Debug)]
struct Gallery<'a> {
    name: &'a str,
    pics: Vec<&'a str>,
}

fn main() -> Result<()> {
    let data = std::fs::read_to_string("./data.txt").unwrap();

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

    dbg!(&galleries);

    Ok(())
}
