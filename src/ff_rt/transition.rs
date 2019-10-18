use crate::game::{ActiveState, EndState, MoveKind};

#[derive(Eq, PartialEq)]
pub enum WallOrientation {
    Against,
    NotAgainst,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Separation {
    S0, S1, S2, S3, SG,
}

#[derive(Eq, PartialEq)]
pub enum Transition {
    ActiveNaiveMove,
    ActiveBounce,
    ActiveP1Push,
    ActiveP2Push,
    ActiveWall,
    EndP1Victory,
    EndP2Victory,
    EndP1Pin,
    EndP2Pin,
    EndP1Survive,
    EndP2Survive,
    EndEnergy,
}

pub fn go(
    sep: Separation,
    p1_move: MoveKind, p2_move: MoveKind,
    p1_wall: WallOrientation, p2_wall: WallOrientation)
    -> Transition
{
    use WallOrientation::*;
    use MoveKind::*;
    use Separation::*;
    use Transition::*;

    match (
        sep,
        p1_move, p2_move,
        p1_wall, p2_wall,
    ) {
        (_, NoEnergy, NoEnergy, NotAgainst, NotAgainst) => EndEnergy,
        (_, Back    , NoEnergy, NotAgainst, NotAgainst) => EndP1Survive,
        (_, Stand   , NoEnergy, NotAgainst, NotAgainst) => EndP1Survive,
        (_, Forward , NoEnergy, NotAgainst, NotAgainst) => EndP1Survive,
        (_, Lunge   , NoEnergy, NotAgainst, NotAgainst) => EndP1Survive,
        (_, NoEnergy, Back    , NotAgainst, NotAgainst) => EndP2Survive,
        (_, NoEnergy, Stand   , NotAgainst, NotAgainst) => EndP2Survive,
        (_, NoEnergy, Forward , NotAgainst, NotAgainst) => EndP2Survive,
        (_, NoEnergy, Lunge   , NotAgainst, NotAgainst) => EndP2Survive,


        //    _, a, b, _

        (S0, Back    , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S0, Stand   , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S0, Forward , Forward , NotAgainst, NotAgainst) => ActiveBounce,
        (S0, Lunge   , Lunge   , NotAgainst, NotAgainst) => ActiveBounce,

        (S0, Stand   , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S0, Forward , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S0, Lunge   , Back    , NotAgainst, NotAgainst) => EndP2Victory,
        (S0, Back    , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S0, Back    , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S0, Back    , Lunge   , NotAgainst, NotAgainst) => EndP1Victory,

        (S0, Forward , Stand   , NotAgainst, NotAgainst) => ActiveBounce,
        (S0, Lunge   , Stand   , NotAgainst, NotAgainst) => ActiveP1Push,
        (S0, Stand   , Forward , NotAgainst, NotAgainst) => ActiveBounce,
        (S0, Stand   , Lunge   , NotAgainst, NotAgainst) => ActiveP2Push,

        (S0, Lunge   , Forward , NotAgainst, NotAgainst) => ActiveP1Push,
        (S0, Forward , Lunge   , NotAgainst, NotAgainst) => ActiveP2Push,


        //    _, a, _, b, _

        (S1, Back    , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S1, Stand   , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S1, Forward , Forward , NotAgainst, NotAgainst) => ActiveBounce,
        (S1, Lunge   , Lunge   , NotAgainst, NotAgainst) => ActiveBounce,

        (S1, Stand   , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S1, Forward , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S1, Lunge   , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S1, Back    , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S1, Back    , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S1, Back    , Lunge   , NotAgainst, NotAgainst) => ActiveNaiveMove,

        (S1, Forward , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S1, Lunge   , Stand   , NotAgainst, NotAgainst) => EndP1Victory,
        (S1, Stand   , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S1, Stand   , Lunge   , NotAgainst, NotAgainst) => EndP2Victory,

        (S1, Lunge   , Forward , NotAgainst, NotAgainst) => ActiveP1Push,
        (S1, Forward , Lunge   , NotAgainst, NotAgainst) => ActiveP2Push,


        //    _, a, _, _, b, _

        (S2, Back    , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Stand   , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Forward , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Lunge   , Lunge   , NotAgainst, NotAgainst) => ActiveBounce,

        (S2, Stand   , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Forward , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Lunge   , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Back    , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Back    , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Back    , Lunge   , NotAgainst, NotAgainst) => ActiveNaiveMove,

        (S2, Forward , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Lunge   , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Stand   , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S2, Stand   , Lunge   , NotAgainst, NotAgainst) => ActiveNaiveMove,

        (S2, Lunge   , Forward , NotAgainst, NotAgainst) => EndP1Victory,
        (S2, Forward , Lunge   , NotAgainst, NotAgainst) => EndP2Victory,


        //    _, a, _, _, _, b, _

        (S3, Back    , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Stand   , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Forward , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Lunge   , Lunge   , NotAgainst, NotAgainst) => ActiveBounce,

        (S3, Stand   , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Forward , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Lunge   , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Back    , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Back    , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Back    , Lunge   , NotAgainst, NotAgainst) => ActiveNaiveMove,

        (S3, Forward , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Lunge   , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Stand   , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Stand   , Lunge   , NotAgainst, NotAgainst) => ActiveNaiveMove,

        (S3, Lunge   , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (S3, Forward , Lunge   , NotAgainst, NotAgainst) => ActiveNaiveMove,


        //    _, a, _, _, _, _, b, _

        (SG, Back    , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Stand   , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Forward , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Lunge   , Lunge   , NotAgainst, NotAgainst) => ActiveNaiveMove,

        (SG, Stand   , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Forward , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Lunge   , Back    , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Back    , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Back    , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Back    , Lunge   , NotAgainst, NotAgainst) => ActiveNaiveMove,

        (SG, Forward , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Lunge   , Stand   , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Stand   , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Stand   , Lunge   , NotAgainst, NotAgainst) => ActiveNaiveMove,

        (SG, Lunge   , Forward , NotAgainst, NotAgainst) => ActiveNaiveMove,
        (SG, Forward , Lunge   , NotAgainst, NotAgainst) => ActiveNaiveMove,




        //    a, b, _
        (S0, Stand   , Lunge   , Against   , NotAgainst) => EndP2Pin,

        (S0, Back    , Back    , Against   , NotAgainst) => ActiveWall,
        (S0, Back    , Stand   , Against   , NotAgainst) => ActiveWall,
        (S0, Back    , Forward , Against   , NotAgainst) => EndP2Pin,
        (S0, Back    , Lunge   , Against   , NotAgainst) => EndP2Pin,
        //    a, _, b, _
        (S1, Back    , Back    , Against   , NotAgainst) => ActiveWall,
        (S1, Back    , Stand   , Against   , NotAgainst) => ActiveWall,
        (S1, Back    , Forward , Against   , NotAgainst) => ActiveWall,
        (S1, Back    , Lunge   , Against   , NotAgainst) => EndP2Pin,
        //    a, _, _, b, _
        (S2, Back    , Back    , Against   , NotAgainst) => ActiveWall,
        (S2, Back    , Stand   , Against   , NotAgainst) => ActiveWall,
        (S2, Back    , Forward , Against   , NotAgainst) => ActiveWall,
        (S2, Back    , Lunge   , Against   , NotAgainst) => ActiveWall,
        //    a, _, _, _, b, _
        (S3, Back    , Back    , Against   , NotAgainst) => ActiveWall,
        (S3, Back    , Stand   , Against   , NotAgainst) => ActiveWall,
        (S3, Back    , Forward , Against   , NotAgainst) => ActiveWall,
        (S3, Back    , Lunge   , Against   , NotAgainst) => ActiveWall,
        //    a, _, _, _, _, b, _
        (SG, Back    , Back    , Against   , NotAgainst) => ActiveWall,
        (SG, Back    , Stand   , Against   , NotAgainst) => ActiveWall,
        (SG, Back    , Forward , Against   , NotAgainst) => ActiveWall,
        (SG, Back    , Lunge   , Against   , NotAgainst) => ActiveWall,

        (_, _        , _       , Against   , NotAgainst) => {
            go(sep, p1_move, p2_move, NotAgainst, NotAgainst)
        }

        //    _, a, b
        (S0, Lunge   , Stand   , NotAgainst, Against   ) => EndP1Pin,

        (S0, Back    , Back    , NotAgainst, Against   ) => ActiveWall,
        (S0, Stand   , Back    , NotAgainst, Against   ) => ActiveWall,
        (S0, Forward , Back    , NotAgainst, Against   ) => EndP1Pin,
        (S0, Lunge   , Back    , NotAgainst, Against   ) => EndP1Pin,
        //    _, a, _, b
        (S1, Back    , Back    , NotAgainst, Against   ) => ActiveWall,
        (S1, Stand   , Back    , NotAgainst, Against   ) => ActiveWall,
        (S1, Forward , Back    , NotAgainst, Against   ) => ActiveWall,
        (S1, Lunge   , Back    , NotAgainst, Against   ) => EndP1Pin,
        //    _, a, _, _, b
        (S2, Back    , Back    , NotAgainst, Against   ) => ActiveWall,
        (S2, Stand   , Back    , NotAgainst, Against   ) => ActiveWall,
        (S2, Forward , Back    , NotAgainst, Against   ) => ActiveWall,
        (S2, Lunge   , Back    , NotAgainst, Against   ) => ActiveWall,
        //    _, a, _, _, _, b
        (S3, Back    , Back    , NotAgainst, Against   ) => ActiveWall,
        (S3, Stand   , Back    , NotAgainst, Against   ) => ActiveWall,
        (S3, Forward , Back    , NotAgainst, Against   ) => ActiveWall,
        (S3, Lunge   , Back    , NotAgainst, Against   ) => ActiveWall,
        //    _, a, _, _, _, _, b
        (SG, Back    , Back    , NotAgainst, Against   ) => ActiveWall,
        (SG, Stand   , Back    , NotAgainst, Against   ) => ActiveWall,
        (SG, Forward , Back    , NotAgainst, Against   ) => ActiveWall,
        (SG, Lunge   , Back    , NotAgainst, Against   ) => ActiveWall,

        (_, _        , _       , NotAgainst, Against   ) => {
            go(sep, p1_move, p2_move, NotAgainst, NotAgainst)
        }

        //    a, _, _, _, _, b
        (SG, Back    , Back    , Against   , Against   ) => ActiveWall,
        (SG, Back    , Stand   , Against   , Against   ) => ActiveWall,
        (SG, Back    , Forward , Against   , Against   ) => ActiveWall,
        (SG, Back    , Lunge   , Against   , Against   ) => ActiveWall,
        (SG, Back    , Back    , Against   , Against   ) => ActiveWall,
        (SG, Stand   , Back    , Against   , Against   ) => ActiveWall,
        (SG, Forward , Back    , Against   , Against   ) => ActiveWall,
        (SG, Lunge   , Back    , Against   , Against   ) => ActiveWall,

        (_, _, _, Against, Against) => {
            go(sep, p1_move, p2_move, NotAgainst, NotAgainst)
        },
    }
}
