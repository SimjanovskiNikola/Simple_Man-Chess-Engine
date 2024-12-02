use crate::engine::shared::helper_func::bit_pos_utility::*;

const knight_attack_arr: [(i8, i8); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (2, -1),
    (2, 1),
    (1, -2),
    (1, 2),
];

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KnightAttacks {
    pub knight_rays: Vec<u64>,
}

impl KnightAttacks {
    pub fn initialize() -> Self {
        let mut attacks = vec![];
        for row in 0..8 {
            for col in 0..8 {
                let attack = knight_attacks(row, col);
                attacks.push(attack);
            }
        }
        return Self { knight_rays: attacks };
    }
}

pub fn knight_attacks(row: i8, col: i8) -> u64 {
    let mut bitboard = 0;

    for idx in 0..8 {
        let x = knight_attack_arr[idx].0;
        let y = knight_attack_arr[idx].1;
        bitboard = set_bit(bitboard, row + x, col + y);
    }

    return bitboard;
}

#[cfg(test)]
mod tests {
    use crate::engine::shared::helper_func::print_utility::bitboard_to_string;

    use super::*;

    #[test]
    fn test_knight_attacks_initialize() {
        let attacks = KnightAttacks::initialize();
    }

    #[test]
    fn test_knight_attacks() {
        let attacks = KnightAttacks::initialize();
        assert_eq!(extract_all_bits(attacks.knight_rays[0]), [10, 17]);
        assert_eq!(extract_all_bits(attacks.knight_rays[40]), [25, 34, 50, 57]);
        assert_eq!(extract_all_bits(attacks.knight_rays[17]), [0, 2, 11, 27, 32, 34]);
        assert_eq!(extract_all_bits(attacks.knight_rays[55]), [38, 45, 61]);
    }
}
