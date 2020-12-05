const TREE: u8 = b'#';

trait QueryableMap {
    fn at(&self, x: u32, y: u32) -> Option<u8>;
}

pub struct Map {
    pub data: Vec<u8>,
    pub width: u32,
}
impl QueryableMap for Map {
    fn at(&self, x: u32, y: u32) -> Option<u8> {
        let index = (y * (self.width + 1) + x % self.width) as usize;
        if index >= self.data.len() {
            return None;
        }
        return Some(self.data[index]);
    }
}

pub fn count_encountered_trees(map: &Map, direction: (u32, u32)) -> u32 {
    let mut position: (u32, u32) = (0, 0);

    let mut tree_count = 0;
    while let Some(square)=map.at(position.0, position.1)  {
        position.0 += direction.0;
        position.1 += direction.1;
        tree_count += if square == TREE { 1 } else { 0 };
    };
    return tree_count;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn map_will_wrap_around_horizontally() {
        let map = Map{data: ".#".as_bytes().to_vec(), width: 2};
        assert_eq!(map.at(0, 0), Some(b'.'));
        assert_eq!(map.at(1, 0), Some(b'#'));
        assert_eq!(map.at(2, 0), Some(b'.'));
    }
    #[test]
    fn map_will_run_off_end_vertically() {
        let map = Map{data: ".#".as_bytes().to_vec(), width: 2};
        assert_eq!(map.at(0, 1), None);
    }
    #[test]
    fn will_count_encountered_trees() {
        let map = Map{data: ".#\n.#\n".as_bytes().to_vec(), width: 2};
        assert_eq!(count_encountered_trees(&map, (0, 1)), 0);
        assert_eq!(count_encountered_trees(&map, (1, 1)), 1);
    }
}