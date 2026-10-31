use memchr::memmem;

pub fn split_bytes_once<'a>(data: &'a [u8], sep: &'a [u8]) -> Option<(&'a [u8], &'a [u8])> {
    match memmem::find(data, sep) {
        Some(pos) => Some((&data[..pos], &data[pos + sep.len()..])),
        None => None,
    }
}

pub fn split_bytes<'a>(data: &'a [u8], sep: &'a [u8]) -> impl Iterator<Item = &'a [u8]> {
    let mut curr_index = 0;
    let mut matches = memmem::find_iter(data, sep);
    let mut done = false;
    std::iter::from_fn(move || {
        if done {
            return None;
        }
        match matches.next() {
            Some(pos) => {
                let piece = &data[curr_index..pos];
                curr_index = pos + sep.len();
                Some(piece)
            }
            None => {
                done = true;
                Some(&data[curr_index..])
            }
        }
    })
}
