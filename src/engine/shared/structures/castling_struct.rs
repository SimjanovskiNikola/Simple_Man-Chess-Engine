use bitflags::bitflags;

use crate::engine::game::Game;
use crate::engine::move_generation::move_generation::*;
use crate::engine::shared::structures::square::SqPos::*;
use super::color::*;

// TODO: Needs a little reaserch because it does not sum all of this.
bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct CastlingRights: u8 {
        const NONE = 0;
        const WKINGSIDE = 1 << 0;
        const WQUEENSIDE = 1 << 1;
        const BKINGSIDE = 1 << 2;
        const BQUEENSIDE = 1 << 3;
        const ALL = 15;
    }
}

impl CastlingRights {
    pub fn val(&self) -> u8 {
        return self.bits() as u8;
    }

    pub fn idx(&self) -> usize {
        return self.bits() as usize;
    }

    pub fn add(&mut self, castle: CastlingRights) {
        *self |= castle
    }

    pub fn clear(&mut self, castle: CastlingRights) {
        *self &= !castle
    }

    pub fn all_set(&self) -> bool {
        return self.idx() == 15;
    }

    pub fn is_set(&self, castle: CastlingRights) -> bool {
        return self.val() & castle.val() != 0;
    }

    pub fn sq_empty(&self, castling: CastlingRights, own: u64, enemy: u64) -> bool {
        let occ = own | enemy;
        let resp = match castling {
            CastlingRights::WKINGSIDE => occ & ((1 << F1.idx()) | (1 << G1.idx())),
            CastlingRights::WQUEENSIDE => {
                occ & ((1 << D1.idx()) | (1 << C1.idx()) | (1 << B1.idx()))
            }
            CastlingRights::BKINGSIDE => occ & ((1 << F8.idx()) | (1 << G8.idx())),
            CastlingRights::BQUEENSIDE => {
                occ & ((1 << D8.idx()) | (1 << C8.idx()) | (1 << B8.idx()))
            }
            _ => panic!("Invalid Castling Rights"),
        };
        return resp == 0;
    }

    pub fn sq_att(&self, castle: CastlingRights, game: &Game, own: u64, enemy: u64) -> bool {
        let resp = match castle {
            CastlingRights::WKINGSIDE => {
                sq_attack(game, E1.idx(), WHITE)
                    | sq_attack(game, F1.idx(), WHITE)
                    | sq_attack(game, G1.idx(), WHITE)
            }
            CastlingRights::WQUEENSIDE => {
                sq_attack(game, E1.idx(), WHITE)
                    | sq_attack(game, D1.idx(), WHITE)
                    | sq_attack(game, C1.idx(), WHITE)
            }
            CastlingRights::BKINGSIDE => {
                sq_attack(game, E8.idx(), BLACK)
                    | sq_attack(game, F8.idx(), BLACK)
                    | sq_attack(game, G8.idx(), BLACK)
            }
            CastlingRights::BQUEENSIDE => {
                sq_attack(game, E8.idx(), BLACK)
                    | sq_attack(game, D8.idx(), BLACK)
                    | sq_attack(game, C8.idx(), BLACK)
            }
            _ => panic!("Invalid Castling Rights"),
        };
        return resp != 0;
    }

    pub fn valid(&self, castle: CastlingRights, game: &Game, own: u64, enemy: u64) -> bool {
        return self.is_set(castle)
            && self.sq_empty(castle, own, enemy)
            && !self.sq_att(castle, game, own, enemy);
    }
}
