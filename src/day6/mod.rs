use crate::Solution;
use itertools::Itertools;

mod input;

const START_OF_PACKET_MARKER_SIZE: usize = 4;
const START_OF_MESSAGE_MARKER_SIZE: usize = 14;

pub struct Day6;

impl Solution for Day6 {
    fn day(&self) -> u8 {
        6
    }

    fn part_one(&self) -> String {
        format!(
            "Number of read characters to get start-of-packet marker: {}",
            find_start_of_packet_marker_position(input::INPUT).unwrap_or(usize::MAX),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Number of read characters to get start-of-message marker: {}",
            find_start_of_message_marker_position(input::INPUT).unwrap_or(usize::MAX),
        )
    }
}

struct SliceSignalIterator<'a> {
    signal: &'a str,
    slice_size: usize,
    position: usize,
}

impl<'a> SliceSignalIterator<'a> {
    pub fn new(signal: &'a str, slice_size: usize) -> Self {
        Self {
            signal,
            slice_size,
            position: 0,
        }
    }
}

impl<'a> Iterator for SliceSignalIterator<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position + self.slice_size < self.signal.len() {
            let start = self.position;
            let end = start + self.slice_size;
            self.position += 1;
            Some((end, &self.signal[start..end]))
        } else {
            None
        }
    }
}

fn find_unique_chars_marker_position(signal: &str, marker_size: usize) -> Option<usize> {
    SliceSignalIterator::new(signal, marker_size)
        .find(|(_, slice)| slice.chars().all_unique())
        .map(|(read, _)| read)
}

fn find_start_of_packet_marker_position(signal: &str) -> Option<usize> {
    find_unique_chars_marker_position(signal, START_OF_PACKET_MARKER_SIZE)
}

fn find_start_of_message_marker_position(signal: &str) -> Option<usize> {
    find_unique_chars_marker_position(signal, START_OF_MESSAGE_MARKER_SIZE)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_of_packet_example_1() {
        let result = find_start_of_packet_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

        assert_eq!(result, Some(7));
    }

    #[test]
    fn start_of_packet_example_2() {
        let result = find_start_of_packet_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz");

        assert_eq!(result, Some(5));
    }

    #[test]
    fn start_of_message_example1() {
        let result = find_start_of_message_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

        assert_eq!(result, Some(19));
    }
}
