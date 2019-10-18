pub const GAME_FIELD_SIZE: u8 = 12;
pub const STARTING_ENERGY: u32 = 10000;

pub struct Game {
    turns: Vec<Turn>,
    end: EndState,
}

pub struct Turn {
    state: ActiveState,
    moves: MovePair,
}

pub struct ActiveState {
    p1: PlayerState,
    p2: PlayerState,
}

pub struct PlayerState {
    pos: u8,
    energy: u32,
}

pub enum EndState {
    P1Victory(ActiveState),
    P2Victory(ActiveState),
    P1Survive(ActiveState),
    P2Survive(ActiveState),
    EnergyTie(ActiveState),
}

pub struct MovePair {
    p1: Move,
    p2: Move,
}

pub struct Move {
    kind: MoveKind,
    energy_spent: u32,
}

#[derive(Eq, PartialEq)]
pub enum MoveKind {
    Back,
    Stand,
    Forward,
    Lunge,
    OutOfEnergy,
}

pub enum NextGameState {
    ActiveState(ActiveState),
    EndState(EndState),
}

pub struct DecisionState {
    p1_dist_from_wall: u8,
    p2_dist_from_wall: u8,
    separation_dist: u8,
    p1_energy: u32,
    p2_energy: u32,
}

impl DecisionState {
    pub fn assert(&self) {
        let dist_sum = self.p1_dist_from_wall
            .checked_add(self.p2_dist_from_wall)
            .expect("dist_sum")
            .checked_add(self.separation_dist)
            .expect("dist_sum");
        assert_eq!(dist_sum, GAME_FIELD_SIZE);

        assert!(self.p1_energy > 0);
        assert!(self.p2_energy > 0);
    }
}

impl ActiveState {
    #[allow(unused_comparisons)]
    pub fn assert(&self) {
        assert!(self.p1.pos >= 0);
        assert!(self.p2.pos >= 0);
        assert!(self.p1.pos < GAME_FIELD_SIZE);
        assert!(self.p2.pos < GAME_FIELD_SIZE);
        assert!(self.p1.pos != self.p2.pos);
        assert!(self.p1.pos < self.p2.pos);

        assert!(self.p1.energy > 0);
        assert!(self.p2.energy > 0);
        assert!(self.p1.energy <= STARTING_ENERGY);
        assert!(self.p2.energy <= STARTING_ENERGY);
    }

    pub fn decision_state(&self) -> DecisionState {
        self.assert();

        let ds = DecisionState {
            p1_dist_from_wall: self.p1.pos,
            p2_dist_from_wall: (GAME_FIELD_SIZE - 1).checked_sub(self.p2.pos).expect("p2_pos_from_wall"),
            separation_dist: self.p2.pos.checked_sub(self.p1.pos).expect("separation_dist"),
            p1_energy: self.p1.energy,
            p2_energy: self.p2.energy,
        };

        ds.assert();

        ds
    }

    pub fn make_move(self, moves: MovePair) -> (Turn, NextGameState) {
        self.assert();

        let ds = self.decision_state();

        assert!(ds.p1_energy >= moves.p1.energy_spent);
        assert!(ds.p2_energy >= moves.p2.energy_spent);
        assert!(ds.p1_energy - moves.p1.energy_spent > 0 || moves.p1.kind == MoveKind::OutOfEnergy);
        assert!(ds.p2_energy - moves.p2.energy_spent > 0 || moves.p2.kind == MoveKind::OutOfEnergy);

        panic!()
    }
}
