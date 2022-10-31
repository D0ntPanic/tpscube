use crate::{CubeRotation, ExtendedMove, Move, OLLAlgorithm, PLLAlgorithm, SliceMove, WideMove};
use num_enum::TryFromPrimitive;

macro_rules! mv {
    (U) => {
        ExtendedMove::Outer(Move::U)
    };
    (Up) => {
        ExtendedMove::Outer(Move::Up)
    };
    (U2) => {
        ExtendedMove::Outer(Move::U2)
    };
    (F) => {
        ExtendedMove::Outer(Move::F)
    };
    (Fp) => {
        ExtendedMove::Outer(Move::Fp)
    };
    (F2) => {
        ExtendedMove::Outer(Move::F2)
    };
    (R) => {
        ExtendedMove::Outer(Move::R)
    };
    (Rp) => {
        ExtendedMove::Outer(Move::Rp)
    };
    (R2) => {
        ExtendedMove::Outer(Move::R2)
    };
    (B) => {
        ExtendedMove::Outer(Move::B)
    };
    (Bp) => {
        ExtendedMove::Outer(Move::Bp)
    };
    (B2) => {
        ExtendedMove::Outer(Move::B2)
    };
    (L) => {
        ExtendedMove::Outer(Move::L)
    };
    (Lp) => {
        ExtendedMove::Outer(Move::Lp)
    };
    (L2) => {
        ExtendedMove::Outer(Move::L2)
    };
    (D) => {
        ExtendedMove::Outer(Move::D)
    };
    (Dp) => {
        ExtendedMove::Outer(Move::Dp)
    };
    (D2) => {
        ExtendedMove::Outer(Move::D2)
    };
    (M) => {
        ExtendedMove::Slice(SliceMove::M)
    };
    (Mp) => {
        ExtendedMove::Slice(SliceMove::Mp)
    };
    (M2) => {
        ExtendedMove::Slice(SliceMove::M2)
    };
    (S) => {
        ExtendedMove::Slice(SliceMove::S)
    };
    (Sp) => {
        ExtendedMove::Slice(SliceMove::Sp)
    };
    (S2) => {
        ExtendedMove::Slice(SliceMove::S2)
    };
    (E) => {
        ExtendedMove::Slice(SliceMove::E)
    };
    (Ep) => {
        ExtendedMove::Slice(SliceMove::Ep)
    };
    (E2) => {
        ExtendedMove::Slice(SliceMove::E2)
    };
    (u) => {
        ExtendedMove::Wide(WideMove::U)
    };
    (up) => {
        ExtendedMove::Wide(WideMove::Up)
    };
    (u2) => {
        ExtendedMove::Wide(WideMove::U2)
    };
    (f) => {
        ExtendedMove::Wide(WideMove::F)
    };
    (fp) => {
        ExtendedMove::Wide(WideMove::Fp)
    };
    (f2) => {
        ExtendedMove::Wide(WideMove::F2)
    };
    (r) => {
        ExtendedMove::Wide(WideMove::R)
    };
    (rp) => {
        ExtendedMove::Wide(WideMove::Rp)
    };
    (r2) => {
        ExtendedMove::Wide(WideMove::R2)
    };
    (b) => {
        ExtendedMove::Wide(WideMove::B)
    };
    (bp) => {
        ExtendedMove::Wide(WideMove::Bp)
    };
    (b2) => {
        ExtendedMove::Wide(WideMove::B2)
    };
    (l) => {
        ExtendedMove::Wide(WideMove::L)
    };
    (lp) => {
        ExtendedMove::Wide(WideMove::Lp)
    };
    (l2) => {
        ExtendedMove::Wide(WideMove::L2)
    };
    (d) => {
        ExtendedMove::Wide(WideMove::D)
    };
    (dp) => {
        ExtendedMove::Wide(WideMove::Dp)
    };
    (d2) => {
        ExtendedMove::Wide(WideMove::D2)
    };
    (x) => {
        ExtendedMove::Rotation(CubeRotation::X)
    };
    (xp) => {
        ExtendedMove::Rotation(CubeRotation::Xp)
    };
    (x2) => {
        ExtendedMove::Rotation(CubeRotation::X2)
    };
    (y) => {
        ExtendedMove::Rotation(CubeRotation::Y)
    };
    (yp) => {
        ExtendedMove::Rotation(CubeRotation::Yp)
    };
    (y2) => {
        ExtendedMove::Rotation(CubeRotation::Y2)
    };
    (z) => {
        ExtendedMove::Rotation(CubeRotation::Z)
    };
    (zp) => {
        ExtendedMove::Rotation(CubeRotation::Zp)
    };
    (z2) => {
        ExtendedMove::Rotation(CubeRotation::Z2)
    };
}

macro_rules! algorithm {
    ($($mv:ident)*) => {
        [$(mv!($mv),)*].to_vec()
    }
}

/// Database of known algorithms for speed solving.
pub struct KnownAlgorithms;

impl KnownAlgorithms {
    /// Returns the list of known algorithms for a given OLL case.
    pub fn oll(oll: OLLAlgorithm) -> Vec<Vec<ExtendedMove>> {
        match oll.as_number() {
            1 => vec![
                algorithm!(R U2 R2 F R Fp U2 Rp F R Fp),
                algorithm!(R Up R2 Dp r Up rp D R2 U Rp),
                algorithm!(R U2 R2 F R Fp U2 Rp F R Fp),
                algorithm!(R U Bp R B R2 Up Rp F R Fp),
                algorithm!(r U Rp U Rp r2 Up Rp U Rp r2 U2 rp),
            ],
            2 => vec![
                algorithm!(R Up R2 Dp r U rp D R2 U Rp),
                algorithm!(F R U Rp Up S R U Rp Up fp),
                algorithm!(r U rp U2 R U2 Rp U2 r Up rp),
                algorithm!(F R U Rp Up Fp f R U Rp Up fp),
                algorithm!(Rp U2 r Up rp U2 r U rp U2 R),
                algorithm!(F R U rp Up R U Rp Mp Up Fp),
            ],
            3 => vec![
                algorithm!(Rp F2 R2 U2 Rp F R U2 R2 F2 R),
                algorithm!(f R U Rp Up fp Up F R U Rp Up Fp),
                algorithm!(rp R2 U Rp U r U2 rp U Mp),
                algorithm!(M R U Rp U r U2 rp U Mp),
                algorithm!(F U R Up Rp Fp U F R U Rp Up Fp),
                algorithm!(rp R U Rp F2 R U Lp U L Mp),
            ],
            4 => vec![
                algorithm!(Rp F2 R2 U2 Rp Fp R U2 R2 F2 R),
                algorithm!(f R U Rp Up fp U F R U Rp Up Fp),
                algorithm!(M Up r U2 rp Up R Up Rp Mp),
                algorithm!(l L2 Up L Up lp U2 l Up Mp),
                algorithm!(Mp Rp Up R Up rp U2 r Up M),
                algorithm!(F U R Up Rp Fp Up F R U Rp Up Fp),
                algorithm!(M Up r U2 rp Up R Up R2 r),
            ],
            5 => vec![
                algorithm!(rp U2 R U Rp U r),
                algorithm!(lp U2 L U Lp U l),
                algorithm!(Rp F2 r U rp F R),
                algorithm!(rp U2 R U Rp U r),
                algorithm!(F R U Rp Up Fp Up F R U Rp Up Fp),
            ],
            6 => vec![
                algorithm!(r U2 Rp Up R Up rp),
                algorithm!(l U2 Lp Up L Up lp),
                algorithm!(xp D R2 Up Rp U Rp Dp x),
                algorithm!(R U R2 F R F2 U F),
            ],
            7 => vec![
                algorithm!(r U Rp U R U2 rp),
                algorithm!(l U Lp U L U2 lp),
                algorithm!(r U rp U R Up Rp r Up rp),
                algorithm!(F Rp Fp R U2 R U2 Rp),
            ],
            8 => vec![
                algorithm!(rp Up R Up Rp U2 r),
                algorithm!(lp Up L Up Lp U2 l),
                algorithm!(R U2 Rp U2 Rp F R Fp),
                algorithm!(Rp Fp r Up rp F2 R),
            ],
            9 => vec![
                algorithm!(R U Rp Up Rp F R2 U Rp Up Fp),
                algorithm!(R U2 Rp Up Sp R Up Rp S),
                algorithm!(Fp Up F r Up rp U r U rp),
                algorithm!(Lp Up L Up L Fp Lp F Lp U2 L),
                algorithm!(rp R2 U2 Rp Up R Up Rp Up Mp),
                algorithm!(Rp Up R Up Rp U Rp F R Fp U R),
            ],
            10 => vec![
                algorithm!(R U Rp U Rp F R Fp R U2 Rp),
                algorithm!(F U Fp Rp F R Up Rp Fp R),
                algorithm!(Mp Rp U2 R U Rp U R U M),
                algorithm!(R U Rp U Rp F R Fp R U2 Rp),
                algorithm!(Lp Up L U L Fp L2 Up L U F),
                algorithm!(R U Rp y Rp F R Up Rp Fp R),
            ],
            11 => vec![
                algorithm!(rp R2 U Rp U R U2 Rp U Mp),
                algorithm!(S R U Rp U R U2 Rp U2 Sp),
                algorithm!(Sp U2 R U Rp U R U2 Rp S),
                algorithm!(r U Rp U Rp F R Fp R U2 rp),
                algorithm!(M R U Rp U R U2 Rp U Mp),
                algorithm!(Fp Lp Up L U F U F R U Rp Up Fp),
                algorithm!(Fp Lp Up L U F Up Fp Lp Up L U F),
            ],
            12 => vec![
                algorithm!(Mp Rp Up R Up Rp U2 R Up M),
                algorithm!(S Rp Up R Up Rp U2 R U2 Sp),
                algorithm!(F R U Rp Up Fp U F R U Rp Up Fp),
                algorithm!(r R2 Up R Up Rp U2 R Up R rp),
                algorithm!(l L2 Up L Up Lp U2 L Up Mp),
                algorithm!(M U2 Rp Up R Up Rp U2 R U Mp),
                algorithm!(M Lp Up L Up Lp U2 L Up Mp),
                algorithm!(lp Up L Up L Fp Lp F Lp U2 l),
            ],
            13 => vec![
                algorithm!(F U R U2 Rp Up R U Rp Fp),
                algorithm!(F U R Up R2 Fp R U R Up Rp),
                algorithm!(r Up rp Up r U rp Fp U F),
                algorithm!(r Up rp Up r U rp y Lp U L),
                algorithm!(r Up rp Up r U rp yp Rp U R),
            ],
            14 => vec![
                algorithm!(Rp F R U Rp Fp R F Up Fp),
                algorithm!(r U Rp Up rp F R2 U Rp Up Fp),
                algorithm!(Fp Up Lp U L2 F Lp Up Lp U L),
                algorithm!(lp U l U lp Up l F Up Fp),
                algorithm!(rp U r U rp Up r y R Up Rp),
                algorithm!(Rp F R U Rp Fp R yp R Up Rp),
                algorithm!(lp U l U lp Up l yp R Up Rp),
                algorithm!(Fp Up Lp U2 L U Lp Up L F),
                algorithm!(Fp Up rp F r2 U rp Up rp F r),
            ],
            15 => vec![
                algorithm!(rp Up r Rp Up R U rp U r),
                algorithm!(lp Up l Lp Up L U lp U l),
                algorithm!(Rp Fp R Lp Up L U Rp F R),
                algorithm!(rp Up Mp Up R U rp U r),
            ],
            16 => vec![
                algorithm!(r U rp R U Rp Up r Up rp),
                algorithm!(r U M U Rp Up r Up rp),
                algorithm!(Rp F R U Rp Up Fp R Up Rp U2 R),
                algorithm!(l U lp L U Lp Up l Up lp),
            ],
            17 => vec![
                algorithm!(R U Rp U Rp F R Fp U2 Rp F R Fp),
                algorithm!(F Rp Fp R U Sp R Up Rp S),
                algorithm!(F Rp Fp R2 rp U R Up Rp Up Mp),
                algorithm!(Fp r U rp Up S rp F r Sp),
                algorithm!(f R U Rp Up fp Up R U Rp Up Rp F R Fp),
            ],
            18 => vec![
                algorithm!(R U2 R2 F R Fp U2 Mp U R Up rp),
                algorithm!(F Sp R Up Rp S R U2 Rp Up Fp),
                algorithm!(r U Rp U R U2 r2 Up R Up Rp U2 r),
                algorithm!(R D rp Up r Dp Rp Up R2 F R Fp R),
                algorithm!(rp Up R U Mp Up R2 F R Fp U R),
                algorithm!(F R U Rp d Rp U2 Rp F R Fp),
                algorithm!(F R U Rp U yp Rp U2 Rp F R Fp),
                algorithm!(F Rp Fp R U2 F Rp Fp R Up R Up Rp),
            ],
            19 => vec![
                algorithm!(M U R U Rp Up Mp Rp F R Fp),
                algorithm!(Sp R U Rp S Up Rp F R Fp),
                algorithm!(r U2 Rp Up R Up r2 U2 R U Rp U r),
                algorithm!(rp R U R U Rp Up Mp Rp F R Fp),
                algorithm!(Rp U2 F R U Rp Up F2 U2 F R),
            ],
            20 => vec![
                algorithm!(r U Rp Up M2 U R Up Rp Up Mp),
                algorithm!(Sp R U Rp S Up Mp U R Up rp),
                algorithm!(M U R U Rp Up M2 U R Up rp),
                algorithm!(S Rp Up R U R U R Up Rp Sp),
                algorithm!(R U Rp Up Rp F R Fp R U2 R2 F R Fp R U2 Rp),
                algorithm!(Mp U Mp U Mp U Mp Up Mp U Mp U Mp U Mp),
                algorithm!(rp R U R U Rp Up r2 R2 U R Up rp),
                algorithm!(Mp Up Mp Up Mp Up Mp U Mp Up Mp Up Mp Up Mp),
                algorithm!(Mp U2 M U2 Mp U M U2 Mp U2 M),
            ],
            21 => vec![
                algorithm!(R U Rp U R Up Rp U R U2 Rp),
                algorithm!(R U2 Rp Up R U Rp Up R Up Rp),
                algorithm!(F R U Rp Up R U Rp Up R U Rp Up Fp),
                algorithm!(Rp U2 R U Rp Up R U Rp U R),
                algorithm!(Rp Up R Up Rp U R Up Rp U2 R),
                algorithm!(F R Up Rp U R U2 Rp Up R U Rp Up Fp),
                algorithm!(R U Rp U R U Lp U Rp Up L),
            ],
            22 => vec![
                algorithm!(R U2 R2 Up R2 Up R2 U2 R),
                algorithm!(Rp U2 R2 U R2 U R2 U2 Rp),
                algorithm!(f R U Rp Up Sp R U Rp Up Fp),
                algorithm!(f R U Rp Up fp F R U Rp Up Fp),
            ],
            23 => vec![
                algorithm!(R2 D Rp U2 R Dp Rp U2 Rp),
                algorithm!(R2 Dp R U2 Rp D R U2 R),
                algorithm!(R U Rp U R U2 R2 Up R Up Rp U2 R),
                algorithm!(R U Rp Up R Up Rp U2 R Up Rp U2 R U Rp),
                algorithm!(R U2 Rp Bp U R U Rp Up B),
            ],
            24 => vec![
                algorithm!(r U Rp Up rp F R Fp),
                algorithm!(Rp Fp r U R Up rp F),
                algorithm!(Rp Up Rp Dp R U Rp D R2),
                algorithm!(R U R D Rp Up R Dp R2),
                algorithm!(xp R U Rp D R Up Rp Dp x),
                algorithm!(lp Up L U R Up rp F),
                algorithm!(L F Rp Fp Lp F R Fp),
                algorithm!(r U Rp Up Lp U R Up xp),
            ],
            25 => vec![
                algorithm!(Fp r U Rp Up rp F R),
                algorithm!(F Rp Fp r U R Up rp),
                algorithm!(x Rp U R Dp Rp Up R D xp),
                algorithm!(xp R Up Rp D R U Rp Dp x),
                algorithm!(Rp F R Bp Rp Fp R B),
                algorithm!(R U2 Rp Up R U Rp Up R U Rp Up R Up Rp),
                algorithm!(lp Up Lp U R Up L U xp),
                algorithm!(Lp R U Rp Up L U R Up Rp),
            ],
            26 => vec![
                algorithm!(R U2 Rp Up R Up Rp),
                algorithm!(Rp Up R Up Rp U2 R),
                algorithm!(Lp Up L Up Lp U2 L),
                algorithm!(L U2 Lp Up L Up Lp),
                algorithm!(Rp U L Up R U Lp),
                algorithm!(Lp U R Up L U Rp),
            ],
            27 => vec![
                algorithm!(R U Rp U R U2 Rp),
                algorithm!(Rp U2 R U Rp U R),
                algorithm!(Lp U2 L U Lp U L),
                algorithm!(L U Lp U L U2 Lp),
                algorithm!(R Up Lp U Rp Up L),
            ],
            28 => vec![
                algorithm!(r U Rp Up M U R Up Rp),
                algorithm!(Mp U M U2 Mp U M),
                algorithm!(r U Rp Up rp R U R Up Rp),
                algorithm!(Rp F R S Rp Fp R Sp),
                algorithm!(Mp Up M U2 Mp Up M),
                algorithm!(M U Mp U2 M U Mp),
                algorithm!(M Up Mp U2 M Up Mp),
            ],
            29 => vec![
                algorithm!(r2 Dp r U rp D r2 Up rp Up r),
                algorithm!(R U Rp Up R Up Rp Fp Up F R U Rp),
                algorithm!(Sp R U Rp Up Rp F R Fp U S),
                algorithm!(M U R U Rp Up Rp F R Fp Mp),
                algorithm!(l D lp U l Dp l2 U l Up lp Up l),
                algorithm!(Rp F R Fp R U2 Rp Up Fp Up F),
            ],
            30 => vec![
                algorithm!(rp Dp r Up rp D r2 Up rp U r U rp),
                algorithm!(F U R U2 Rp Up R U2 Rp Up Fp),
                algorithm!(F Rp F R2 Up Rp Up R U Rp F2),
                algorithm!(Sp Rp Up R f Rp U R Up Fp),
                algorithm!(M Up Lp Up L U L Fp Lp F Mp),
                algorithm!(R2 U Rp Bp R Up R2 U R B Rp),
                algorithm!(f R U R2 Up Rp U R2 Up Rp fp),
            ],
            31 => vec![
                algorithm!(Rp Up F U R Up Rp Fp R),
                algorithm!(S R U Rp Up fp Up F),
                algorithm!(Sp rp Fp r U r Up rp f),
                algorithm!(Sp Lp Up L U L Fp Lp f),
                algorithm!(F Rp Fp R U R U Rp Up R Up Rp),
            ],
            32 => vec![
                algorithm!(S R U Rp Up Rp F R fp),
                algorithm!(L U Fp Up Lp U L F Lp),
                algorithm!(R U Bp Up Rp U R B Rp),
                algorithm!(Rp F R Fp Up r Up rp U r U rp),
                algorithm!(F U R Up Fp r U Rp Up rp),
                algorithm!(R U2 Rp Up Fp U F R Up Rp),
                algorithm!(R d Lp dp Rp U l U lp),
            ],
            33 => vec![
                algorithm!(R U Rp Up Rp F R Fp),
                algorithm!(Lp Up L U L Fp Lp F),
                algorithm!(F R Up Rp U R U Rp Fp),
            ],
            34 => vec![
                algorithm!(f R fp Up rp Up R U Mp),
                algorithm!(R U R2 Up Rp F R U R Up Fp),
                algorithm!(R U Rp Up Bp Rp F R Fp B),
                algorithm!(F R U Rp Up Rp Fp r U R Up rp),
                algorithm!(R U Rp Up x Dp Rp U R Up D xp),
                algorithm!(R U Rp Up yp rp Up R U Mp),
                algorithm!(f R fp Up rp Up R U Mp),
            ],
            35 => vec![
                algorithm!(R U2 R2 F R Fp R U2 Rp),
                algorithm!(R2 F R Fp R U2 Rp U R U2 Rp Up R),
                algorithm!(f R U Rp Up fp R U Rp U R U2 Rp),
            ],
            36 => vec![
                algorithm!(Lp Up L Up Lp U L U L Fp Lp F),
                algorithm!(R U R2 Fp Up F U R2 U2 Rp),
                algorithm!(R U Rp Fp R U Rp Up Rp F R Up Rp F R Fp),
                algorithm!(Rp Fp Up F2 U R Up Rp Fp R),
                algorithm!(R U Rp Up Fp U2 F U R U Rp),
                algorithm!(Rp Up R Up Rp U R U l Up Rp U x),
                algorithm!(Rp Up R Up Rp U R U R y Rp Fp R),
                algorithm!(Rp Up R Up Rp U R U R Bp Rp B),
            ],
            37 => vec![
                algorithm!(F Rp Fp R U R Up Rp),
                algorithm!(F R Up Rp Up R U Rp Fp),
                algorithm!(Fp r U rp Up rp F r),
            ],
            38 => vec![
                algorithm!(R U Rp U R Up Rp Up Rp F R Fp),
                algorithm!(F R Up Rp S Up R U Rp fp),
            ],
            39 => vec![
                algorithm!(fp r U rp Up rp F r S),
                algorithm!(L Fp Lp Up L U F Up Lp),
                algorithm!(R U Rp Fp Up F U R U2 Rp),
                algorithm!(fp L F Lp Up Lp U L S),
                algorithm!(R Bp Rp Up R U B Up Rp),
                algorithm!(F R U Rp Up Fp Rp Up R Up Rp U2 R),
            ],
            40 => vec![
                algorithm!(Rp F R U Rp Up Fp U R),
                algorithm!(f Rp Fp R U R Up Rp Sp),
            ],
            41 => vec![
                algorithm!(R U Rp U R U2 Rp F R U Rp Up Fp),
                algorithm!(F U R2 D Rp Up R Dp R2 Fp),
                algorithm!(S Up Rp Fp Up F U R Sp),
                algorithm!(Rp Fp Up F R Sp Rp U R S),
                algorithm!(f R U Rp Up fp Up R U Rp U R U2 Rp),
                algorithm!(R Up Rp U2 R U y R Up Rp Up Fp),
            ],
            42 => vec![
                algorithm!(Rp Up R Up Rp U2 R F R U Rp Up Fp),
                algorithm!(F Sp R U Rp Up Fp U S),
                algorithm!(Rp Up F2 up R U Rp D R2 B),
                algorithm!(Rp F R Fp Rp F R Fp R U Rp Up R U Rp),
                algorithm!(F Rp Fp R U2 Rp Up R2 Up R2 U2 R),
                algorithm!(M U F R U Rp Up Fp Mp),
                algorithm!(Lp U L U2 Lp Up yp Lp U L U F),
                algorithm!(Lp Up L Up Lp U2 L Fp Lp Up L U F),
            ],
            43 => vec![
                algorithm!(Rp Up Fp U F R),
                algorithm!(Fp Up Lp U L F),
                algorithm!(fp Lp Up L U f),
                algorithm!(rp Fp Up F U r),
                algorithm!(Bp Up Rp U R B),
            ],
            44 => vec![
                algorithm!(f R U Rp Up fp),
                algorithm!(F U R Up Rp Fp),
                algorithm!(R U B Up Bp Rp),
            ],
            45 => vec![
                algorithm!(F R U Rp Up Fp),
                algorithm!(Rp Fp Up F U R),
                algorithm!(f U R Up Rp fp),
                algorithm!(Fp Lp Up L U F),
            ],
            46 => vec![
                algorithm!(Rp Up Rp F R Fp U R),
                algorithm!(rp Up F Rp Fp R U r),
            ],
            47 => vec![
                algorithm!(Fp Lp Up L U Lp Up L U F),
                algorithm!(F Rp Fp R U2 R Up Rp U R U2 Rp),
                algorithm!(Rp Up Rp F R Fp Rp F R Fp U R),
                algorithm!(Rp Up lp U R Up Rp U R Up xp U R),
            ],
            48 => vec![
                algorithm!(F R U Rp Up R U Rp Up Fp),
                algorithm!(f U R Up Rp U R Up Rp fp),
            ],
            49 => vec![
                algorithm!(r Up r2 U r2 U r2 Up r),
                algorithm!(l Up l2 U l2 U l2 Up l),
                algorithm!(R Bp R2 F R2 B R2 Fp R),
                algorithm!(Rp F Rp Fp R2 U2 Bp R B Rp),
            ],
            50 => vec![
                algorithm!(rp U r2 Up r2 Up r2 U rp),
                algorithm!(lp U l2 Up l2 Up l2 U lp),
                algorithm!(Rp F R2 Bp R2 Fp R2 B Rp),
                algorithm!(R U2 Rp Up R Up Rp F R U Rp Up Fp),
            ],
            51 => vec![
                algorithm!(F U R Up Rp U R Up Rp Fp),
                algorithm!(f R U Rp Up R U Rp Up fp),
                algorithm!(Rp Up Rp F R Fp R Up Rp U2 R),
            ],
            52 => vec![
                algorithm!(Rp Fp Up F Up R U Rp U R),
                algorithm!(R U Rp U R Up B Up Bp Rp),
                algorithm!(R U Rp U R dp R Up Rp Fp),
                algorithm!(Rp Up R Up Rp U Fp U F R),
                algorithm!(R U Rp U R Up y R Up Rp Fp),
                algorithm!(Rp Up R Up Rp d Rp U R B),
                algorithm!(Rp Up R Up Rp U yp Rp U R B),
            ],
            53 => vec![
                algorithm!(rp Up R Up Rp U R Up Rp U2 r),
                algorithm!(lp Up L Up Lp U L Up Lp U2 l),
                algorithm!(rp U2 R U Rp Up R U Rp U r),
                algorithm!(lp U2 L U Lp Up L U Lp U l),
                algorithm!(F R U Rp Up Fp R U Rp Up Rp F R Fp),
            ],
            54 => vec![
                algorithm!(r U Rp U R Up Rp U R U2 rp),
                algorithm!(r U2 Rp Up R U Rp Up R Up rp),
                algorithm!(r U rp R U Rp Up R U Rp Up r Up rp),
            ],
            55 => vec![
                algorithm!(Rp F U R Up R2 Fp R2 U Rp Up R),
                algorithm!(R U2 R2 Up R Up Rp U2 F R Fp),
                algorithm!(Rp F R U R Up R2 Fp R2 Up Rp U R U Rp),
                algorithm!(F Up R2 D Rp U2 R Dp R2 U Fp),
                algorithm!(r U2 R2 F R Fp U2 rp F R Fp),
            ],
            56 => vec![
                algorithm!(r U rp U R Up Rp Mp U R U2 rp),
                algorithm!(r U rp U R Up Rp U R Up Rp r Up rp),
                algorithm!(F R U Rp Up R Fp r U Rp Up rp),
                algorithm!(rp Up r Up Rp U R Up Rp U R rp U r),
                algorithm!(r U rp U R Up Rp U R Up Mp Up rp),
                algorithm!(Rp Fp R Up rp F r Mp Up rp F2 R),
                algorithm!(f R U Rp Up fp F R U Rp Up R U Rp Up Fp),
                algorithm!(F R U Rp Up Fp r U Rp Up rp F R Fp),
            ],
            57 => vec![
                algorithm!(R U Rp Up Mp U R Up rp),
                algorithm!(S Rp F R Sp Rp Fp R),
                algorithm!(R Up Rp Sp R U Rp S),
                algorithm!(R U Rp Sp R Up Rp S),
                algorithm!(Mp Up M U Mp Up M Up Mp U2 M),
                algorithm!(Mp U Mp U Mp U2 M U M U M),
                algorithm!(R U Rp Up r Rp U R Up rp),
                algorithm!(Mp U Mp U Mp U Mp U2 Mp U Mp U Mp U Mp),
            ],
            _ => Vec::new(),
        }
    }

    /// Returns the list of known algorithms for a given PLL case.
    pub fn pll(pll: PLLAlgorithm) -> Vec<Vec<ExtendedMove>> {
        match pll {
            PLLAlgorithm::Aa => vec![
                algorithm!(x Rp U Rp D2 R Up Rp D2 R2 xp),
                algorithm!(xp R2 D2 Rp Up R D2 Rp U Rp x),
                algorithm!(x L2 D2 Lp Up L D2 Lp U Lp xp),
                algorithm!(Rp Bp R2 D Rp Up R Dp Rp U Rp B R),
                algorithm!(lp U Rp D2 R Up Rp D2 R2 xp),
                algorithm!(Rp F Rp B2 R Fp Rp B2 R2),
                algorithm!(xp Rp D Rp U2 R Dp Rp U2 R2 x),
                algorithm!(r U rp Up rp F r2 Up rp Up r U rp Fp),
                algorithm!(R U Rp Fp r U Rp Up rp F R2 Up Rp),
                algorithm!(xp Lp U Lp D2 L Up Lp D2 L2 x),
                algorithm!(F U R2 D Rp Up R Dp Rp U Rp Up Fp),
                algorithm!(Rp Dp R U2 Rp D R Up Rp Dp R Up Rp D R),
                algorithm!(Lp B Lp F2 L Bp Lp F2 L2),
            ],
            PLLAlgorithm::Ab => vec![
                algorithm!(x R2 D2 R U Rp D2 R Up R xp),
                algorithm!(x L Up L D2 Lp U L D2 L2 xp),
                algorithm!(xp R Up R D2 Rp U R D2 R2 x),
                algorithm!(Rp Bp R Up R D Rp U R Dp R2 B R),
                algorithm!(lp Rp D2 R U Rp D2 R Up R xp),
                algorithm!(xp L2 D2 L U Lp D2 L Up L x),
                algorithm!(Rp Dp R U2 Rp D R U Rp Dp R U Rp D R),
                algorithm!(l Up R D2 Rp U R D2 R2 x),
                algorithm!(R2 B2 R F Rp B2 R Fp R),
                algorithm!(r Up L D2 Lp U L D2 L2 xp),
                algorithm!(xp R2 U2 R D Rp U2 R Dp R x),
                algorithm!(R Bp R F2 Rp B R F2 R2),
            ],
            PLLAlgorithm::E => vec![
                algorithm!(xp R Up Rp D R U Rp Dp R U Rp D R Up Rp Dp x),
                algorithm!(Rp Up Rp Dp R Up Rp D R U Rp Dp R U Rp D R2),
                algorithm!(xp Lp U L Dp Lp Up L D Lp Up L Dp Lp U L D x),
                algorithm!(R2 U Rp Up y R U Rp Up R U Rp Up R U Rp yp R Up R2),
                algorithm!(z U2 R2 F R U Rp Up R U Rp Up R U Rp Up Fp R2 U2 zp),
                algorithm!(R2 U R2 U D R2 Up R2 U R2 Up Dp R2 U R2 U2 R2),
                algorithm!(xp R Up Rp D R U Rp u2 Rp U R D Rp Up R xp),
                algorithm!(Fp r U Rp Up rp F R U2 r U Rp Up rp F R Fp),
                algorithm!(F Rp Fp r U R Up rp F R Fp r U Rp Up rp),
                algorithm!(L Up R D2 Rp U R Lp Up L D2 Lp U Rp),
            ],
            PLLAlgorithm::F => vec![
                algorithm!(Rp Up Fp R U Rp Up Rp F R2 Up Rp Up R U Rp U R),
                algorithm!(Rp U R Up R2 Fp Up F U R F Rp Fp R2),
                algorithm!(Rp F R fp Rp F R2 U Rp Up Rp Fp R2 U Rp S),
                algorithm!(R2 F R Fp Rp Up Fp U F R2 U Rp Up R),
                algorithm!(Rp U2 Rp dp Rp Fp R2 Up Rp U Rp F R Up F),
                algorithm!(Mp U2 L Fp R U2 rp U rp R2 U2 R2 x),
                algorithm!(Rp U R Up R2 yp Rp Up R U y x R U Rp Up R2 xp),
            ],
            PLLAlgorithm::Ga => vec![
                algorithm!(R2 U Rp U Rp Up R Up R2 D Up Rp U R Dp),
                algorithm!(R2 u Rp U Rp Up R up R2 yp Rp U R),
                algorithm!(Dp R2 U Rp U Rp Up R Up R2 Up D Rp U R),
                algorithm!(R U Rp Fp R U Rp Up Rp F R Up Rp F R2 Up Rp Up R U Rp Fp),
                algorithm!(R2 u Rp U Rp Up R up R2 y Lp U L),
                algorithm!(R2 u Rp U Rp Up R up R2 Fp U F),
                algorithm!(L2 F2 Lp U2 Lp U2 L Fp Lp Up L U L Fp L2),
            ],
            PLLAlgorithm::Gb => vec![
                algorithm!(Rp Up R y R2 u Rp U R Up R up R2),
                algorithm!(Fp Up F R2 u Rp U R Up R up R2),
                algorithm!(Rp dp F R2 u Rp U R Up R up R2),
                algorithm!(D Rp Up R U Dp R2 U Rp U R Up R Up R2),
                algorithm!(Rp Up R U Dp R2 U Rp U R Up R Up R2 D),
                algorithm!(R U Rp Fp r U Rp Up rp F R Fp R U Rp Up Rp F R2 Up Rp),
                algorithm!(Lp Up L yp R2 u Rp U R Up R up R2),
            ],
            PLLAlgorithm::Gc => vec![
                algorithm!(R2 Up R Up R U Rp U R2 Dp U R Up Rp D),
                algorithm!(R2 F2 R U2 R U2 Rp F R U Rp Up Rp F R2),
                algorithm!(R2 up R Up R U Rp u R2 f Rp fp),
                algorithm!(L2 up L Up L U Lp u L2 y L Up Lp),
                algorithm!(D R2 Up R Up R U Rp U R2 Dp U R Up Rp),
                algorithm!(R Dp R2 Up R2 U Rp U R U2 Rp U Rp U R2 D Rp),
                algorithm!(R2 up R Up R U Rp u R2 y R Up Rp),
            ],
            PLLAlgorithm::Gd => vec![
                algorithm!(R U Rp Up D R2 Up R Up Rp U Rp U R2 Dp),
                algorithm!(Dp R U Rp Up D R2 Up R Up Rp U Rp U R2),
                algorithm!(R2 Fp R U R Up Rp Fp R U2 Rp U2 Rp F2 R2),
                algorithm!(R U Rp yp R2 up R Up Rp U Rp u R2),
                algorithm!(f R fp R2 up R Up Rp U Rp u R2),
                algorithm!(R U Rp Fp R U Rp U R Up Rp Up Rp F R2 U Rp Up R Up Rp),
                algorithm!(L U Lp B2 Dp R Up Rp U Rp u R2),
            ],
            PLLAlgorithm::H => vec![
                algorithm!(M2 U M2 U2 M2 U M2),
                algorithm!(M2 Up M2 U2 M2 Up M2),
                algorithm!(R2 U2 R U2 R2 U2 R2 U2 R U2 R2),
                algorithm!(R2 S2 R2 Up R2 S2 R2),
                algorithm!(R2 U2 R2 U2 R2 Up R2 U2 R2 U2 R2),
                algorithm!(M2 U2 M2 U M2 U2 M2),
                algorithm!(M2 U2 M2 Up M2 U2 M2),
                algorithm!(R2 U2 Rp U2 R2 U2 R2 U2 Rp U2 R2),
            ],
            PLLAlgorithm::Ja => vec![
                algorithm!(x R2 F R Fp R U2 rp U r U2 xp),
                algorithm!(Lp Up L F Lp Up L U L Fp L2 U L),
                algorithm!(Rp U Lp U2 R Up Rp U2 R L),
                algorithm!(Mp Rp F R Fp R U2 rp U r U2 rp),
                algorithm!(rp F Rp F2 r Up rp F2 r R),
                algorithm!(Rp U2 R U Rp U2 L Up R U Lp),
                algorithm!(Lp U2 L U Lp U2 R Up L U Rp),
                algorithm!(F Up Rp F R2 Up Rp Up R U Rp Fp R U Rp Fp),
                algorithm!(x U2 rp Up r U2 lp U Rp Up R2 x2),
                algorithm!(Lp U Rp z R2 U Rp Up R2 U D zp),
                algorithm!(z Up R Dp R2 U Rp Up R2 U D Rp zp),
                algorithm!(L Up Rp U Lp U2 R Up Rp U2 R),
                algorithm!(r R2 F R Fp R U2 rp U r U2 rp),
                algorithm!(Rp U2 R U Rp z R2 U Rp D R Up zp),
                algorithm!(R Up Lp U Rp U2 L Up Lp U2 L),
            ],
            PLLAlgorithm::Jb => vec![
                algorithm!(R U Rp Fp R U Rp Up Rp F R2 Up Rp),
                algorithm!(rp F R Fp r U2 Rp U R U2 Rp),
                algorithm!(R U2 Rp Up R U2 Lp U Rp Up L),
                algorithm!(R U2 Rp Up R U2 Lp U Rp Up r xp),
                algorithm!(Lp U R Up L U2 Rp U R U2 Rp),
                algorithm!(L Up R U2 Lp U L U2 Rp Lp),
            ],
            PLLAlgorithm::Na => vec![
                algorithm!(R U Rp U R U Rp Fp R U Rp Up Rp F R2 Up Rp U2 R Up Rp),
                algorithm!(Fp R U Rp Up Rp F R2 F Up Rp Up R U Fp Rp),
                algorithm!(R F Up Rp U R U Fp R2 Fp R U R Up Rp F),
                algorithm!(rp D r U2 rp D r U2 rp D r U2 rp D r U2 rp D r),
                algorithm!(L Up R U2 Lp U Rp L Up R U2 Lp U Rp),
                algorithm!(z U Rp D R2 Up R Dp U Rp D R2 Up R Dp zp),
                algorithm!(R Up L U2 Rp U Lp R Up L U2 Rp U Lp),
                algorithm!(z Rp U Rp D R2 Up R U Dp Rp D R2 Up R Dp zp),
            ],
            PLLAlgorithm::Nb => vec![
                algorithm!(rp Dp F r Up rp Fp D r2 U rp Up rp F r Fp),
                algorithm!(Rp U R Up Rp Fp Up F R U Rp F Rp Fp R Up R),
                algorithm!(Lp Up L Up Lp Up L F Lp Up L U L Fp L2 U L U2 Lp U L),
                algorithm!(Rp U R Up Rp Fp Up F R U Rp Up R Up f R fp),
                algorithm!(Rp U Lp U2 R Up L Rp U Lp U2 R Up L),
                algorithm!(Rp U Rp F R Fp R Up Rp Fp U F R U Rp Up R),
                algorithm!(z Dp R Up R2 D Rp U Dp R Up R2 D Rp U zp),
                algorithm!(Dp R2 D R Dp R2 U Rp D U2 Rp U2 R U2 R),
                algorithm!(r D rp U2 r D rp U2 r D rp U2 r D rp U2 r D rp),
                algorithm!(z Up R Dp R2 U Rp D Up R Dp R2 U Rp D zp),
                algorithm!(Lp U Rp U2 L Up R Lp U Rp U2 L Up R),
            ],
            PLLAlgorithm::Ra => vec![
                algorithm!(R Up Rp Up R U R D Rp Up R Dp Rp U2 Rp),
                algorithm!(R U Rp Fp R U2 Rp U2 Rp F R U R U2 Rp),
                algorithm!(L U2 Lp U2 L Fp Lp Up L U L F L2),
                algorithm!(R Up R2 Dp R U Rp D R Up R Up Rp U R U Rp),
                algorithm!(R U2 Rp U2 R Bp Rp Up R U R B R2),
            ],
            PLLAlgorithm::Rb => vec![
                algorithm!(Rp U2 R U2 Rp F R U Rp Up Rp Fp R2),
                algorithm!(R2 F R U R Up Rp Fp R U2 Rp U2 R),
                algorithm!(Rp U2 Rp Dp R Up Rp D R U R Up Rp Up R),
                algorithm!(Rp U R U Rp Up Rp Dp R U Rp D R U2 R),
            ],
            PLLAlgorithm::T => vec![
                algorithm!(R U Rp Up Rp F R2 Up Rp Up R U Rp Fp),
                algorithm!(R U Rp Up Rp F R2 Up Rp U Fp Lp U L),
                algorithm!(R2 U R2 Up R2 Up D R2 Up R2 U R2 Dp),
                algorithm!(Lp Up L U L Fp L2 U L U Lp Up L F),
            ],
            PLLAlgorithm::Ua => vec![
                algorithm!(R U Rp U Rp Up R2 Up Rp U Rp U R),
                algorithm!(M2 U M U2 Mp U M2),
                algorithm!(R2 Up Sp U2 S Up R2),
                algorithm!(R Up R U R U R Up Rp Up R2),
                algorithm!(M2 U Mp U2 M U M2),
                algorithm!(R2 Up Rp Up R U R U R Up R),
                algorithm!(F2 Up L Rp F2 Lp R Up F2),
                algorithm!(M2 up Mp u2 Mp up M2),
                algorithm!(L Up L U L U L Up Lp Up L2),
            ],
            PLLAlgorithm::Ub => vec![
                algorithm!(Rp U Rp Up Rp Up Rp U R U R2),
                algorithm!(M2 Up M U2 Mp Up M2),
                algorithm!(R2 U R U Rp Up Rp Up Rp U Rp),
                algorithm!(M2 Up Mp U2 M Up M2),
                algorithm!(R2 Up S R2 Sp R2 U R2),
                algorithm!(R2 U R2 S R2 Sp Up R2),
                algorithm!(R2 U F Bp R2 Fp B U R2),
                algorithm!(Lp U Lp Up Lp Up Lp U L U L2),
                algorithm!(M2 u Mp u2 Mp u M2),
                algorithm!(F2 U L Rp F2 Lp R U F2),
                algorithm!(L2 U L U Lp Up Lp Up Lp U Lp),
            ],
            PLLAlgorithm::V => vec![
                algorithm!(Rp U Rp Up R Dp Rp D Rp U Dp R2 Up R2 D R2),
                algorithm!(Rp U Rp Up y Rp Fp R2 Up Rp U Rp F R F),
                algorithm!(R Up R U Rp D R Dp R Up D R2 U R2 Dp R2),
                algorithm!(Rp U R Up Rp fp Up R U2 Rp Up R Up Rp f R),
                algorithm!(Rp U Rp dp Rp Fp R2 Up Rp U Rp F R F),
                algorithm!(R2 Dp R2 U R2 D Up R Dp R D Rp U R Up R),
                algorithm!(Rp U lp fp Rp F Rp Fp R2 Up Rp U l F),
                algorithm!(Rp fp R U Rp U R U2 Rp U f R U Rp Up R),
                algorithm!(U2 Rp Up Fp S U F2 Up Bp R2 Up Rp D Rp U zp),
                algorithm!(Rp U R Up xp U R U2 Rp Up R Up Rp U2 R U Rp Up x),
                algorithm!(Fp U Fp Up Rp Fp R2 Up Rp U Rp F R F),
                algorithm!(z Dp R2 D R2 U Rp Dp R Up R U Rp D R Up zp),
                algorithm!(R U2 Rp D R Up R Up R U R2 D Rp Up R D2),
                algorithm!(Rp U2 R U2 L Up Rp U Lp U L Up R U Lp),
                algorithm!(R2 Up B2 U B2 R Dp R D Rp U R Up R),
            ],
            PLLAlgorithm::Y => vec![
                algorithm!(F R Up Rp Up R U Rp Fp R U Rp Up Rp F R Fp),
                algorithm!(F Rp F R2 Up Rp Up R U Rp Fp R U Rp Up Fp),
                algorithm!(R2 Up R2 Up R2 U Rp Fp R U R2 Up Rp F R),
            ],
            PLLAlgorithm::Z => vec![
                algorithm!(M2 U M2 U Mp U2 M2 U2 Mp),
                algorithm!(Mp Up M2 Up M2 Up Mp U2 M2),
                algorithm!(M2 Up M2 Up Mp U2 M2 U2 Mp),
                algorithm!(M2 U2 M U M2 U M2 U M),
                algorithm!(Mp U M2 U M2 U Mp U2 M2),
                algorithm!(M2 U2 M Up M2 Up M2 Up M),
                algorithm!(Rp Up R Up R U R Up Rp U R U R2 Up Rp),
                algorithm!(M2 u M2 up S M2 Sp),
                algorithm!(Rp Up R2 U R U Rp Up R U R Up R Up Rp),
                algorithm!(R U Rp U Rp Up Rp U R Up Rp Up R2 U R),
                algorithm!(M2 Up Mp U2 M2 U2 Mp U M2),
                algorithm!(M2 U2 M Up M2 Up M2 Up M),
            ],
        }
    }
}

#[repr(u8)]
#[derive(Clone, Debug, Copy, PartialEq, Eq, TryFromPrimitive)]
pub enum AlgorithmType {
    OLL = 0,
    PLL = 1,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Algorithm {
    OLL(OLLAlgorithm),
    PLL(PLLAlgorithm),
}

impl Algorithm {
    pub(crate) fn to_type_and_number(&self) -> (AlgorithmType, u8) {
        match self {
            Algorithm::OLL(oll) => (AlgorithmType::OLL, oll.as_number()),
            Algorithm::PLL(pll) => (AlgorithmType::PLL, pll.to_index() as u8),
        }
    }

    pub(crate) fn from_type_and_number(alg_type: AlgorithmType, num: u8) -> Option<Self> {
        match alg_type {
            AlgorithmType::OLL => Some(Algorithm::OLL(OLLAlgorithm::from_number(num))),
            AlgorithmType::PLL => match PLLAlgorithm::from_index(num as usize) {
                Some(pll) => Some(Algorithm::PLL(pll)),
                None => None,
            },
        }
    }
}

impl ToString for Algorithm {
    fn to_string(&self) -> String {
        match self {
            Algorithm::OLL(oll) => oll.to_string(),
            Algorithm::PLL(pll) => pll.to_str().into(),
        }
    }
}
