use std::collections::{HashSet, VecDeque};
use std::error::Error;

const START_PACKET_CHARS: u32 = 4;
const START_MESSAGE_CHARS: u32 = 14;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let packet_start = get_char_index(&contents, START_PACKET_CHARS);
    println!("Packet starts: {}", packet_start);

    let message_start = get_char_index(&contents, START_MESSAGE_CHARS);
    println!("Message starts: {}", message_start);

    Ok(())
}

fn get_char_index(contents: &str, distinct_char_size: u32) -> u32 {
    let mut buf = VecDeque::new();
    for (i, c) in contents.char_indices() {
        buf.push_back(c);
        if (buf.len() as u32) == distinct_char_size {
            let chars: HashSet<_> = HashSet::from_iter(buf.iter());
            if (chars.len() as u32) == distinct_char_size {
                return (i as u32) + 1;
            }
            buf.pop_front();
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_char_index() {
        let stream = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, get_char_index(stream, START_PACKET_CHARS));
        assert_eq!(19, get_char_index(stream, START_MESSAGE_CHARS));
    }
}
