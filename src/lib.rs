pub struct StrTree {
    root: *mut Node,
}

struct Node {
    cnt: [usize; 26],
    son: [*mut Node; 26],
}

impl StrTree {
    pub fn new() -> Self {
        StrTree {
            root: Box::into_raw(Box::new(Node::new())),
        }
    }

    pub fn build(&mut self, str: &str) {
        let mut p = self.root;
        for c in str.chars() {
            let u = c.to_digit(26).unwrap();
            unsafe {
                if (*p).son[u as usize].is_null() {
                    (*p).son[u as usize] = Box::into_raw(Box::new(Node::new()));
                }
                p = (*p).son[u as usize];
            }
        }
        let end_char_byte = str.as_bytes()[str.len() - 1];
        unsafe { (*p).cnt[end_char_byte as usize - b'a' as usize] += 1 }
    }

    pub fn query(&self, str: &str) -> usize {
        let mut p = self.root;
        for c in str.chars() {
            let u = c.to_digit(26).unwrap();
            unsafe {
                if (*p).son[u as usize].is_null() {
                    return 0;
                }
                p = (*p).son[u as usize];
            }
        }
        let end_char_byte = str.as_bytes()[str.len() - 1];
        unsafe {
            return (*p).cnt[(end_char_byte - b'a') as usize];
        }
    }
}

impl Node {
    fn new() -> Self {
        Node {
            cnt: [0; 26],
            son: [std::ptr::null_mut(); 26],
        }
    }
}
