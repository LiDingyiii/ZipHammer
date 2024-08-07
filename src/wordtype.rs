use rand::{self, Rng};

#[derive(Clone, Copy,Debug)]
pub enum WordType {
    Number,
    Letter,
    Special,
}

impl WordType {
    // pub fn create_wordtypes(password_length: u8,pconfig:PasswordConfig) -> Vec<WordType> {
    //     /*
    //        创建密码格式
    //     */
    //     let mut wordtypes_res: Vec<WordType> = Vec::new();
    //     let wordtypes: Vec<WordType> = pconfig.types;
    //     for _ in 0..password_length {
    //         let rand_seek = wordtypes.len();
    //         let select = rand::thread_rng().gen_range(0..rand_seek);
    //         if let Some(wt) = wordtypes.get(select) {
    //             wordtypes_res.push(*wt);
    //         };
    //     }
    //     wordtypes_res
    // }
    pub fn create_until(self: &Self,b:bool) -> u8 {
        // 创建密码元素
        match self {
            WordType::Number => self.create_number(),
            WordType::Letter => self.create_letter(b),
            WordType::Special => self.create_special(),
        }
    }

    pub fn create_number(self: &Self) -> u8 {
        rand::thread_rng().gen_range( 48..=57)
    }

    pub fn create_letter(self: &Self, b:bool) -> u8 {
        /*
           p:是否包含大小写
         */
        if b {
            if rand::thread_rng().gen_range(1..10) % 2 == 0 {
                self.create_upper_letter()
            } else {
                self.create_low_letter()
            }
        } else {
            self.create_low_letter()
        }
    }

    pub fn create_low_letter(self: &Self) -> u8 {
        rand::thread_rng().gen_range(97..=122)
    }

    pub fn create_upper_letter(self: &Self) -> u8 {
        rand::thread_rng().gen_range(65..=90)
    }

    pub fn create_special(self: &Self) -> u8 {
        let r1 = 32..=64;
        let r2 = 91..=96;
        let r3 = 123..=126;
    
        let ranges = [r1, r2, r3]; // 将范围放入数组中
    
        // 获取一个随机索引来选择范围
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..ranges.len());
    
        // 根据随机索引选择范围，并生成随机值
        let random_value = match index {
            0 => rng.gen_range(ranges[0].clone()),
            1 => rng.gen_range(ranges[1].clone()),
            2 => rng.gen_range(ranges[2].clone()),
            _ => unreachable!(), // 理论上不会执行到这里，因为索引总是0、1或2
        };
        random_value as u8
    }
}
