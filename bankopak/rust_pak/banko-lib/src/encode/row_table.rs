use banko::Row;
use combinations::Combination;
use std::u8;
use typenum::consts::*;

pub struct RowTable {
    table: [[[[[[[[[u8; 2]; 2]; 2]; 2]; 2]; 2]; 2]; 2]; 2],
}

impl RowTable {
    #[inline]
    pub fn get(&self, index: Row) -> Option<u8> {
        let result = self.table[index[0].is_some() as usize][index[1].is_some() as usize]
            [index[2].is_some() as usize][index[3].is_some() as usize][index[4].is_some() as usize]
            [index[5].is_some() as usize][index[6].is_some() as usize][index[7].is_some() as usize]
            [index[8].is_some() as usize];

        if result == u8::MAX {
            None
        } else {
            Some(result)
        }
    }
}

impl Default for RowTable {
    #[inline]
    fn default() -> RowTable {
        let mut table = [[[[[[[[[u8::MAX; 2]; 2]; 2]; 2]; 2]; 2]; 2]; 2]; 2];

        for (ndx, comb) in Combination::<U8, U5>::new().enumerate() {
            let mut pos = [0; 9];
            for c in comb {
                pos[c as usize] = 1;
            }

            table[pos[0]][pos[1]][pos[2]][pos[3]][pos[4]][pos[5]][pos[6]][pos[7]][pos[8]] =
                ndx as u8;
        }

        RowTable { table }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU8;

    #[test]
    fn test_row_table() {
        let row_table = RowTable::default();
        let mut seen = 0;
        for n in 0..=0b111111111 {
            let mut index = [None; 9];
            let mut count = 0;
            for k in 0..9 {
                if n & (1 << k) != 0 {
                    index[k] = NonZeroU8::new(1);
                    count += 1;
                }
            }

            if count != 5 {
                assert!(row_table.get(index).is_none());
            } else {
                assert_eq!(row_table.get(index), Some(seen));
                seen += 1;
            }
        }
    }
}
