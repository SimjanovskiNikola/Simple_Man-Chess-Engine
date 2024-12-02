use crate::engine::shared::helper_func::bit_pos_utility::*;

macro_rules! make_rays {
    ($ray_fn:ident) => {
        {
            let mut rays = vec![];
            
            for row in 0..8 {
                for col in 0..8 {
                    rays.push($ray_fn(row, col));
                }
            }   

            rays
        }
    };
}

macro_rules! define_ray {
    ($name:ident, $offset_fn:expr) => {
        pub fn $name(row: i8, col: i8) -> u64 {
            let mut bitboard = 0;

            for offset in 1..8 {
                bitboard = set_bit(bitboard, $offset_fn(row, col, offset));
            }

            return bitboard;
        }
    };
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rays {
    pub n_rays: Vec<u64>,
    pub e_rays: Vec<u64>,
    pub nw_rays: Vec<u64>,
    pub ne_rays: Vec<u64>,
    pub w_rays: Vec<u64>,
    pub s_rays: Vec<u64>,
    pub sw_rays: Vec<u64>,
    pub se_rays: Vec<u64>,
}

impl Rays {
    pub fn initialize() -> Self {
        let mut n_rays: Vec<u64> = make_rays!(n_ray);
        let mut e_rays: Vec<u64> = make_rays!(e_ray);
        let mut nw_rays: Vec<u64> = make_rays!(nw_ray);
        let mut ne_rays: Vec<u64> = make_rays!(ne_ray);
        let mut w_rays: Vec<u64> = make_rays!(w_ray);
        let mut s_rays: Vec<u64> = make_rays!(s_ray);
        let mut sw_rays: Vec<u64> = make_rays!(sw_ray);
        let mut se_rays: Vec<u64> = make_rays!(se_ray);

        return Self {
            n_rays: n_rays,
            e_rays: e_rays,
            nw_rays: nw_rays,
            ne_rays: ne_rays,
            w_rays: w_rays,
            s_rays: s_rays,
            sw_rays: sw_rays,
            se_rays: se_rays,
        };
    }
}

define_ray!(n_ray, |row, col, offset| (row + offset, col));
define_ray!(e_ray, |row, col, offset| (row, col + offset));
define_ray!(nw_ray, |row, col, offset| (row + offset, col - offset));
define_ray!(ne_ray, |row, col, offset| (row + offset, col + offset));
define_ray!(w_ray, |row, col, offset| (row, col - offset));
define_ray!(s_ray, |row, col, offset| (row - offset, col));
define_ray!(sw_ray, |row, col, offset| (row - offset, col - offset));
define_ray!(se_ray, |row, col, offset| (row - offset, col + offset));

pub fn set_bit(bitboard: u64, row_col: (i8, i8)) -> u64 {
    let (row, col) = row_col;

    if row < 0 || row > 7 || col < 0 || col > 7 {
        return bitboard;
    }
    return bitboard | (1 << (col + row * 8));
}

pub fn first_hit(ray: u64, forward_ray: bool, occupancy: u64) -> Option<usize> {
    let intersection = ray & occupancy;
    if intersection == 0 {
        return None;
    }

    if forward_ray {
        return Some(bit_scan_lsb(ray & occupancy));
    } else {
        return Some(bit_scan_msb(ray & occupancy));
    }
}

// TEST: Add more tests for this function
pub fn blocked_ray_attack(
    ray: u64,
    ray_family: &Vec<u64>,
    forward_ray: bool,
    own_occupancy: u64,
    enemy_occupancy: u64
) -> u64 {
    let own_overlap = ray & own_occupancy;
    let enemy_overlap = ray & enemy_occupancy;

    let first_own_hit = first_hit(ray, forward_ray, own_overlap);
    let first_enemy_hit = first_hit(ray, forward_ray, enemy_overlap);

    match (first_own_hit, first_enemy_hit) {
        (None, None) => {
            return ray;
        }
        (None, Some(enemy_idx)) => {
            let ray_after = ray_family[enemy_idx];
            return ray ^ ray_after;
        }
        (Some(own_idx), None) => {
            let ray_after = ray_family[own_idx];
            return ray ^ (ray_after | (1 << own_idx));
        }
        (Some(own_idx), Some(enemy_idx)) => {
            let own_after = ray_family[own_idx];
            let enemy_after = ray_family[enemy_idx];
            return ray ^ (own_after | (1 << own_idx) | enemy_after);
        }
    }
}

// TESTS: Here Are the tests for the above functions
#[cfg(test)]
mod tests {
    use crate::engine::shared::helper_func::print_utility::{
        bitboard_to_string,
        print_bitboard,
    };

    use super::*;

    fn get_occupancy() -> (u64, u64) {
        let mut own_occupancy = 0;
        let mut enemy_occupancy = 0;

        for i in 0..16 {
            if i == 5 {
                continue;
            }
            own_occupancy |= 1 << i;
        }
        own_occupancy |= 1 << 22;

        for i in 48..64 {
            if i == 57 || i == 49 {
                continue;
            }
            enemy_occupancy |= 1 << i;
        }
        enemy_occupancy |= 1 << 41;
        enemy_occupancy |= 1 << 42;
        return (own_occupancy, enemy_occupancy);
    }

    #[test]
    fn test_rays() {
        let rays = Rays::initialize();
        let row = 5;
        let col = 6;
        let idx = position_to_idx(row, col, Some(true));

        let mut expected_sw_6_7 = 0;
        for i in 1..8 {
            if is_inside_board_bounds_row_col(row - i, col - i) {
                expected_sw_6_7 = set_bit(expected_sw_6_7, (row - i, col - i));
            }
        }
        assert_eq!(rays.sw_rays[idx as usize], expected_sw_6_7);

        let mut expected_w_6_7 = 0;
        for i in 1..8 {
            if is_inside_board_bounds_row_col(row, col - i) {
                expected_w_6_7 = set_bit(expected_w_6_7, (row, col - i));
            }
        }
        assert_eq!(rays.w_rays[idx as usize], expected_w_6_7);

        let mut expected_ne_6_7 = 0;
        for i in 1..8 {
            if is_inside_board_bounds_row_col(row + i, col + i) {
                expected_ne_6_7 = set_bit(expected_ne_6_7, (row + i, col + i));
            }
        }
        assert_eq!(rays.ne_rays[idx as usize], expected_ne_6_7);
    }

    #[test]
    fn test_blocked_rays() {
        let (own_occupancy, enemy_occuopancy) = get_occupancy();
        let rays = Rays::initialize();
        let row = 4;
        let col = 4;
        let idx = position_to_idx(row, col, Some(true));

        print_bitboard(own_occupancy, Some(idx));
        print_bitboard(enemy_occuopancy, Some(idx));

        let blocked_ray = blocked_ray_attack(
            rays.nw_rays[idx as usize],
            &rays.nw_rays,
            true,
            enemy_occuopancy,
            own_occupancy
        );
        print_bitboard(blocked_ray, Some(idx));
        assert_eq!(blocked_ray, 1 << (idx + 7));
    }

    #[test]
    fn print_n_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        print_bitboard(rays.n_rays[idx], Some(idx as i8));
    }
    #[test]
    fn print_ne_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        print_bitboard(rays.ne_rays[idx], Some(idx as i8));
    }
    #[test]
    fn print_e_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        print_bitboard(rays.e_rays[idx], Some(idx as i8));
    }
    #[test]
    fn print_se_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        print_bitboard(rays.se_rays[idx], Some(idx as i8));
    }
    #[test]
    fn print_s_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        print_bitboard(rays.s_rays[idx], Some(idx as i8));
    }
    #[test]
    fn print_sw_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        print_bitboard(rays.sw_rays[idx], Some(idx as i8));
    }
    #[test]
    fn print_w_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        print_bitboard(rays.w_rays[idx], Some(idx as i8));
    }
    #[test]
    fn print_nw_ray() {
        let rays = Rays::initialize();
        let idx = 43;
        print_bitboard(rays.nw_rays[idx], Some(idx as i8));
    }
}
