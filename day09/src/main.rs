use std::fs;

fn main() {
    let input = fs::read_to_string("./day09/input/input.txt").unwrap();

    let mut checksum1 = 0;

    let part_iter = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .enumerate()
        .map(|(index, size)| {
            if index % 2 == 0 {
                DiskPart::File {
                    id: index / 2,
                    size: size as usize,
                }
            } else {
                DiskPart::Free {
                    size: size as usize,
                }
            }
        });

    let blocks = part_iter
        .clone()
        .flat_map(|part| part.as_blocks())
        .enumerate()
        .collect::<Vec<(usize, DiskBlock)>>();

    let mut block_iter = blocks.iter();

    while let Some((i, block)) = block_iter.next() {
        if let DiskBlock::File { id } = block {
            checksum1 += i * id;
        } else {
            while let Some(last_block) = block_iter.next_back() {
                if let (_, DiskBlock::File { id }) = last_block {
                    checksum1 += i * id;

                    break;
                }
            }
        }
    }

    let mut parts: Vec<DiskPart> = part_iter.collect();

    for i in (0..parts.len()).rev() {
        if let DiskPart::File { size: file_size, .. } = parts[i] {
            for j in 0..i {
                if let DiskPart::Free { size: free_size } = parts[j] {
                    if free_size >= file_size {
                        parts[j] = parts[i];
                        parts[i] = DiskPart::Free { size: file_size };
                        if free_size > file_size {
                            parts.insert(
                                j + 1,
                                DiskPart::Free { size: free_size - file_size },
                            );
                        }
                        break;
                    }
                }
            }
        }
    }
    
    let checksum2: usize = parts
        .iter()
        .flat_map(|part| part.as_blocks())
        .enumerate()
        .fold(0, |acc, (i, block)| {
            acc + match block {
                DiskBlock::Free => 0,
                DiskBlock::File { id } => id * i,
            }
        });

    println!("Part 1: {}", checksum1);
    println!("Part 2: {}", checksum2);
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum DiskPart {
    File { id: usize, size: usize },
    Free { size: usize },
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum DiskBlock {
    File { id: usize },
    Free,
}

impl DiskPart {
    fn as_blocks(self) -> impl Iterator<Item = DiskBlock> {
        let (block, size) = match self {
            DiskPart::File { id, size } => (DiskBlock::File { id }, size),
            DiskPart::Free { size } => (DiskBlock::Free, size),
        };

        (0..size).map(move |_| block)
    }
}
