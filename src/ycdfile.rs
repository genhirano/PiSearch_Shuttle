use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Read},
};

use byteorder::{ByteOrder, LittleEndian};
use regex::Regex;

pub struct Block {
    block_no: i64,
    pos: i64,
    data: String,
}

impl Block {
    #[allow(dead_code)]
    pub fn block_no(&self) -> &i64 {
        &self.block_no
    }
    #[allow(dead_code)]
    pub fn pos(&self) -> &i64 {
        &self.pos
    }
    #[allow(dead_code)]
    pub fn data(&self) -> &String {
        &self.data
    }
}

pub struct YCDFile {
    filename: String,
    header: HashMap<String, String>,
    datareader: BufReader<File>,
    current_block_no: i64,
    block_count: i64,
    base_pos: i64,
}

impl YCDFile {
    pub fn new(filename: &str) -> io::Result<Self> {
        let file = File::open(&filename)?;

        let mut instance = Self {
            filename: filename.to_string(),
            header: HashMap::new(),
            datareader: BufReader::new(file),
            current_block_no: 1,
            block_count: 0,
            base_pos: 0,
        };

        instance.get_header_info(&filename)?;

        //ヘッダー部分を読み飛ばす
        let header_end_point: i32 = instance.header["header_end_point"].parse().unwrap();
        for _ in 0..header_end_point {
            let mut buf: [u8; 1] = [0; 1];
            instance.datareader.read(&mut buf).unwrap();
        }

        instance.block_count = instance.header["Blocksize"].parse::<i64>().unwrap() / 19;

        //19桁の倍数でない場合は、半端な1ブロックも読むように、ブロック数を+1
        if 0 != instance.header["Blocksize"].parse::<i64>().unwrap() % 19 {
            instance.block_count += 1;
        }

        instance.base_pos = (instance.header["Blocksize"].parse::<i64>().unwrap())
            * (instance.header["BlockID"].parse::<i64>().unwrap());

        Ok(instance)
    }

    #[allow(dead_code)]
        fn filename(&self) -> &String {
        &self.filename
    }
    #[allow(dead_code)]
    fn header(&self) -> &HashMap<String, String> {
        &self.header
    }
    #[allow(dead_code)]
    pub fn block_count(&self) -> i64 {
        self.block_count
    }

    fn get_header_info(&mut self, file_name: &str) -> io::Result<()> {
        /* ヘッダーの例
        #Compressed Digit File

        FileVersion:	1.1.0

        Base:	10

        FirstDigits:	3.14159265358979323846264338327950288419716939937510

        TotalDigits:	0

        Blocksize:	1000000
        BlockID:	0

        EndHeader

        */
        let mut headreader: BufReader<File> = BufReader::new(File::open(file_name)?);
        let mut vec: Vec<u8> = Vec::new();
        let mut buf: [u8; 1] = [0; 1];

        let mut header_end_point: i32 = 0;

        for i in 1..1000000 {
            //1バイトずつ読み込んでVectorに積んでいく
            match headreader.read(&mut buf)? {
                0 => break, //readメソッドが0を返した場合（つまり、読み込むデータがない場合）
                n => {
                    let buf2: &[u8] = &buf[..n]; //スライス
                    vec.push(buf2[0]);
                }
            }
            if let Some(&0) = vec.last() {
                //ヘッダーの終了マークである"0"に到達したら、そこまでをヘッダーサイズとする
                header_end_point = i;
                break;
            }
        }

        let slice: &[u8] = &vec[..];
        let s: &str = std::str::from_utf8(&slice).unwrap();

        // 連続する改行を一つの改行にまとめる
        let re = Regex::new(r"\n+").unwrap();
        let s2 = re.replace_all(s, "\n");

        // 改行文字で改行で分割して、それぞれベクターに格納
        let mut vec: Vec<String> = s2.split('\n').map(|s| s.to_string()).collect();

        // Vectorから改行のみの文字列と空の文字列を削除
        vec.retain(|x| x != "\r" && !x.is_empty());
        vec.retain(|x| x.contains(":"));

        // Vector内の各文字列から改行とタブ文字を削除
        for x in vec.iter_mut() {
            *x = x.replace("\r", "");
            *x = x.replace("\t", "");
        }

        // ヘッダー情報をHashMapに格納
        self.header = vec
            .iter()
            .map(|s| {
                let mut parts = s.splitn(2, ':');
                (
                    parts.next().unwrap().trim().to_string(),
                    parts.next().unwrap_or("").trim().to_string(),
                )
            })
            .collect();

        // ヘッダーの終了地点をHashMapに追加
        self.header
            .insert("header_end_point".to_string(), header_end_point.to_string());

        Ok(())
    }

    pub fn read_one_block(&mut self) -> Option<Block> {
        if self.block_count < self.current_block_no {
            return None;
        }

        let mut buf = [0; 8]; // 64ビット = 8バイト
        let mut num_str = match &self.datareader.read_exact(&mut buf) {
            Ok(()) => {
                let num: u64 = LittleEndian::read_u64(&buf);
                format!("{:019}", num)
            }
            Err(_e) => panic!("Failed to get header size: {}", _e)
        };

        if self.block_count == self.current_block_no {
            let blocksize: i64 = self.header["Blocksize"].parse().unwrap();
            let amari = blocksize % 19;

            num_str = num_str
                .chars()
                .take(amari.try_into().unwrap())
                .collect::<String>();
        }

        let block = Block {
            block_no: self.current_block_no,
            pos: 1 + ((self.current_block_no - 1) * 19) + self.base_pos,
            data: num_str,
        };

        self.current_block_no += 1;

        return Some(block);
    }
}
