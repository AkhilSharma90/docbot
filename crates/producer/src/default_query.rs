use super::Producer;

pub struct DefaultQuery {
    min_length: u32,
    max_length: u32,
    current: Vec<u8>,
    char_set: Vec<u8>,
    rolled: bool,
}

impl DefaultQuery {
    pub fn new(max_length: u32, min_length: u32) -> Self {
        let mut char_set: Vec<u8> = (b'0'..=b'9').chain(b'A'..=b'Z').collect();
        char_set.sort();
        Self {
            max_length,
            min_length,
            current: vec![char_set[0]; min_length.try_into().unwrap()],
            char_set,
            rolled: false,
        }
    }
}

impl Producer for DefaultQuery {
    fn next(&mut self) -> Result<Option<Vec<u8>>, String> {
        let mut next = self.current.clone();
        let mut stopped = false;
        for i in 0..next.len() {
            let spot = match self.char_set.binary_search(&next[i]) {
                Ok(spot) => spot,
                Err(_) => return Err("Couldn't find character in character set".to_string()),
            };
            if spot >= self.char_set.len() - 1 {
                next[i] = self.char_set[0];
            } else {
                next[i] = self.char_set[spot + 1];
                stopped = true;
                break;
            }
        }
        if !stopped {
            // We rolled every digit to every character, now we need to add a new character
            next.insert(0, self.char_set[0]);
            if next.len() > self.max_length.try_into().unwrap() {
                if self.rolled {
                    return Err("Out of elements".to_string());
                } else {
                    self.rolled = true;
                    // For debugging
                    //match String::from_utf8(self.current.clone()) {
                    //Ok(val)=>println!("Trying {}", val), _=>{}
                    //};
                    return Ok(Some(self.current.clone()));
                }
            }
        }
        let return_value = std::mem::replace(&mut self.current, next);
        // For debugging and making sure all values are tried
        //match String::from_utf8(return_value.clone()) {
        //Ok(val)=>println!("Trying {}", val), _=>{}
        //};
        Ok(Some(return_value))
    }

    fn size(&self) -> usize {
        let mut ret = 0usize;
        for len in self.min_length..=self.max_length {
            ret += self.char_set.len().pow(len);
        }
        ret
    }
}
