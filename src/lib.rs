use std::{fs, path::Path, error::Error, process::Command};

const TMP_FOLDER: &str = ".tmp-split";

pub fn ct_tmp() {
    if !Path::new(TMP_FOLDER).exists() {
        if let Err(err) = fs::create_dir(TMP_FOLDER) {
            panic!("Failed to create TMP_FOLDER:\n{}", err)
        }
    }
}

pub fn rm_tmp() {
    if Path::new(TMP_FOLDER).exists() {
        if let Err(err) = fs::remove_dir_all(TMP_FOLDER) {
            panic!("Failed to remove TMP_FOLDER:\n{}", err)
        }
    }
}

pub fn parse_input(filename: &str) -> (String, Vec<(String, Vec<usize>)>){
    let contents = std::fs::read_to_string(filename);

    let contents = match contents {
        Ok(cont) => cont,
        Err(err) => panic!("Failed to parse the input:\n{}", err),
    };

    let (filename, contents) = contents.split_once("\n\n").expect("Failed to parse input!");

    let parsed = contents.clone().lines()
        .map(|line| line.split_once(' ').expect("Failed to split line!"))
        .map(|(fname, range)| (fname.to_string(), 
                               range.split('-')
                               .map(|word| word.parse::<usize>()
                                    .expect("Failed to parse range values!"))
                               .collect::<Vec<usize>>()))
        .collect::<Vec<(String, Vec<usize>)>>();
    
    (filename.to_string(), parsed)
}

pub fn magic(file_path: &str, input: (String, Vec<usize>)) -> Result<(), Box<dyn Error>> {
    // Create subfolder for results
    let (fname, range) = input;
    let sub_dir = format!("{}/{}", TMP_FOLDER, fname);
    fs::create_dir(&sub_dir)?;

    Command::new("pdfseparate")
        .arg("-f").arg(range[0].to_string())
        .arg("-l").arg(range[1].to_string())
        .arg(file_path)
        .arg(sub_dir.clone() + "/page%d.pdf")
        .output()
        .expect("Failed to extract pages!");
    
    let entries = fs::read_dir(&sub_dir)?;
    let mut fnames: Vec<String> = entries.filter_map(|entry| {
        let path = entry.ok()?.path();
        if path.is_file() {
            path.file_name()?.to_str().map(|s| s.to_owned())
        } else {
            None
        }
    }).collect();

    fnames.sort_by(|a, b| {
        let a_num: usize = a["page".len()..a.len() - ".pdf".len()].parse().unwrap_or(0);
        let b_num: usize = b["page".len()..b.len() - ".pdf".len()].parse().unwrap_or(0);
        a_num.cmp(&b_num)
    });

    let mut cmd = Command::new("pdfunite");
    for fname in fnames {
        cmd.arg(sub_dir.clone() + "/" + &fname);
    }
    cmd.arg(fname + ".pdf");
    cmd.output().expect("Failed to combine pages!");

    Ok(())
}

