use crate::ycdfile::{Block, YCDFile};
use std::ffi::OsStr;
use std::{fs, vec};

pub struct YCDBlock {
    pos: i64,
    data: String,
}

impl YCDBlock {
    pub fn pos(&self) -> &i64 {
        &self.pos
    }
    pub fn data(&self) -> &String {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //200,000,000,000(2,000億)読むだけ＝40分
    //  9,500,000,000
    //106,400,000,000


    #[test]
    fn test03() {
        let mut ycd: YCD = YCD::new("pi/1000000");
        //let mut ycd: YCD = YCD::new("H:/Pi/200000000000");

        loop {
            match ycd.get_next_block() {
                Some(block) => {
                    let first_30_chars: String = block.data.chars().take(30).collect();
                    let last_30_chars: String = block
                        .data
                        .chars()
                        .rev()
                        .take(30)
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect();

                    println!(
                        "{:?}, {:?}, {:?}, {:?}",
                        block.pos,
                        first_30_chars,
                        last_30_chars,
                        block.data.len(),
                    );
                }
                None => {
                    println!("おわり");
                    break;
                }
            }
        }
    }

    #[test]
    fn test04() {
        //let mut ycd: YCD = YCD::new("pi/1000000");
        let mut ycd: YCD = YCD::new("H:/Pi/200000000000");

        let target = "1415926";

        loop {
            match ycd.get_next_block() {
                Some(block) => match block.data.find(target) {
                    Some(pos) => println!(
                        "'{}' found at position {}",
                        target,
                        (pos as i64 + block.pos)
                    ),
                    None => continue,
                },
                None => {
                    println!("おわり");
                    break;
                }
            }
        }
    }
}

pub struct YCD {
    filenamelist: Vec<String>,
    current_ycd: Option<YCDFile>,
    current_ycd_no: usize,
    is_end_reached: bool,
    beforelast: String,
}

impl YCD {
    pub fn new(target_folder: &str) -> Self {
        let filelist = get_ycd_filename_list(target_folder);

        Self {
            filenamelist: filelist,
            current_ycd: None,
            current_ycd_no: 0,
            is_end_reached: false,
            beforelast: std::iter::repeat("@").take(100).collect(),
        }
    }

    pub fn read_one_block(&mut self) -> Option<Block> {
        match &mut self.current_ycd {
            None => {
                if self.current_ycd_no != 0 {
                    panic!("current_ycd_no is not 0");
                }
                self.current_ycd =
                    Some(YCDFile::new(&self.filenamelist[self.current_ycd_no]).unwrap());
                return self.current_ycd.as_mut().unwrap().read_one_block();
            }
            Some(ycd_file) => {
                let block = ycd_file.read_one_block();

                if block.is_none() {
                    self.current_ycd_no += 1;

                    if self.filenamelist.len() <= self.current_ycd_no {
                        return None;
                    }

                    self.current_ycd =
                        Some(YCDFile::new(&self.filenamelist[self.current_ycd_no]).unwrap());
                    return self.current_ycd.as_mut().unwrap().read_one_block();
                }
                return block;
            }
        }
    }

    #[allow(dead_code)]
    pub fn filenamelist(&self) -> &Vec<String> {
        return &self.filenamelist;
    }

    pub fn get_next_block(&mut self) -> Option<YCDBlock> {
        if self.is_end_reached {
            return None;
        }

        let mut vec: Vec<String> = Vec::new();
        let mut start_pos: i64 = -1;

        for _i in 0..9999 {
            match self.read_one_block() {
                Some(block) => {
                    vec.push(block.data().to_string());
                    if -1 == start_pos {
                        start_pos = *block.pos() - self.beforelast.len() as i64;
                    };
                }
                None => {
                    self.is_end_reached = true;
                    break;
                }
            }
        }

        let mut data: String = self.beforelast.clone();
        data.push_str(&vec.join(""));

        self.beforelast = data
            .chars()
            .rev()
            .take(self.beforelast.len())
            .collect::<String>()
            .chars()
            .rev()
            .collect();
        return Some(YCDBlock {
            pos: start_pos,
            data: data,
        });
    }
}

fn get_ycd_filename_list(target_folder: &str) -> vec::Vec<String> {
    // 指定フォルダ内のファイルを読み取る
    let entries = fs::read_dir(target_folder).expect("Unable to read directory");

    // ファイル名と対応する番号を格納するベクタ
    let mut files: Vec<(String, u32)> = Vec::new();

    for entry in entries {
        let entry = entry.expect("Unable to get entry");
        let path = entry.path();

        // ファイル名を取得
        if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
            // "data_00.txt" から "data_99.txt" のパターンにマッチするかチェック
            if file_name.starts_with("Pi - Dec - Chudnovsky -") && file_name.ends_with(".ycd") {
                // 途中の数字部分を取得
                let num_part = &file_name[23..file_name.len() - 4].trim();

                // 数字部分が0から99の範囲内かチェック
                if let Ok(num) = num_part.parse::<u32>() {
                    if num < 999 {
                        files.push((file_name.to_string(), num));
                    }
                }
            }
        }
    }

    // ファイル名を小さい順にソート
    files.sort_by_key(|k| k.1);

    //戻り値のvecを作る
    let mut vec: Vec<String> = Vec::new();
    for (file, _) in files {
        vec.push(target_folder.to_string() + "/" + &file.to_string());
    }

    return vec;
}

