use crate::{CubeFace, FaceRotation, Move, RotationDirection};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// Move to perform on a cube (including slices and rotations)
pub enum ExtendedMove {
    Outer(Move),
    Slice(SliceMove),
    Wide(WideMove),
    Rotation(CubeRotation),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// Slice move. For large cubes, this rotates all inner layers.
pub enum SliceMove {
    M,
    Mp,
    M2,
    S,
    Sp,
    S2,
    E,
    Ep,
    E2,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// Axis for a slice move.
pub enum SliceMoveAxis {
    M,
    S,
    E,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// Wide move. For large cubes, this rotates all except the outer edge of the opposing side.
pub enum WideMove {
    U,
    Up,
    U2,
    F,
    Fp,
    F2,
    R,
    Rp,
    R2,
    B,
    Bp,
    B2,
    L,
    Lp,
    L2,
    D,
    Dp,
    D2,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// Rotation of the cube. Does not move any pieces but moves applied afterwards are applied with
/// rotation taken into account (for example, `x F` ends in the same cube state as `D`).
pub enum CubeRotation {
    X,
    Xp,
    X2,
    Y,
    Yp,
    Y2,
    Z,
    Zp,
    Z2,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// Rotation axis for a cube rotation.
pub enum CubeRotationAxis {
    X,
    Y,
    Z,
}

/// Operations on sequences of cube moves (including slices and rotations)
pub trait ExtendedMoveSequence: Sized {
    /// Returns the inverse of this move sequence (undoing all moves)
    fn inverse(&self) -> Vec<ExtendedMove>;

    /// Returns the human-readable string for this move sequence
    fn to_string(&self) -> String;
}

/// Context for performing extended moves. Keeps track of the current rotation of the cube.
pub struct ExtendedMoveContext<'a, C: FaceRotation> {
    cube: &'a mut C,
    face_map: CubeFaceMap,
}

struct CubeFaceMap {
    mapping: [CubeFace; 6],
}

impl ExtendedMove {
    /// Gets the inverse of the move
    pub fn inverse(&self) -> Self {
        match self {
            ExtendedMove::Outer(mv) => ExtendedMove::Outer(mv.inverse()),
            ExtendedMove::Slice(mv) => ExtendedMove::Slice(mv.inverse()),
            ExtendedMove::Wide(mv) => ExtendedMove::Wide(mv.inverse()),
            ExtendedMove::Rotation(mv) => ExtendedMove::Rotation(mv.inverse()),
        }
    }
}

impl ToString for ExtendedMove {
    fn to_string(&self) -> String {
        match self {
            ExtendedMove::Outer(mv) => mv.to_string(),
            ExtendedMove::Slice(mv) => mv.to_string(),
            ExtendedMove::Wide(mv) => mv.to_string(),
            ExtendedMove::Rotation(mv) => mv.to_string(),
        }
    }
}

impl SliceMove {
    pub fn from_axis_and_count(axis: SliceMoveAxis, count: i32) -> Option<Self> {
        let count = count % 4;
        match axis {
            SliceMoveAxis::M => match count {
                -3 => Some(SliceMove::M),
                -2 => Some(SliceMove::M2),
                -1 => Some(SliceMove::Mp),
                1 => Some(SliceMove::M),
                2 => Some(SliceMove::M2),
                3 => Some(SliceMove::Mp),
                _ => None,
            },
            SliceMoveAxis::S => match count {
                -3 => Some(SliceMove::S),
                -2 => Some(SliceMove::S2),
                -1 => Some(SliceMove::Sp),
                1 => Some(SliceMove::S),
                2 => Some(SliceMove::S2),
                3 => Some(SliceMove::Sp),
                _ => None,
            },
            SliceMoveAxis::E => match count {
                -3 => Some(SliceMove::E),
                -2 => Some(SliceMove::E2),
                -1 => Some(SliceMove::Ep),
                1 => Some(SliceMove::E),
                2 => Some(SliceMove::E2),
                3 => Some(SliceMove::Ep),
                _ => None,
            },
        }
    }

    pub fn axis(&self) -> SliceMoveAxis {
        match self {
            SliceMove::M | SliceMove::Mp | SliceMove::M2 => SliceMoveAxis::M,
            SliceMove::S | SliceMove::Sp | SliceMove::S2 => SliceMoveAxis::S,
            SliceMove::E | SliceMove::Ep | SliceMove::E2 => SliceMoveAxis::E,
        }
    }

    pub fn count(&self) -> i32 {
        match self {
            SliceMove::M | SliceMove::S | SliceMove::E => 1,
            SliceMove::Mp | SliceMove::Sp | SliceMove::Ep => -1,
            SliceMove::M2 | SliceMove::S2 | SliceMove::E2 => 2,
        }
    }

    /// Gets the inverse of the move
    pub fn inverse(&self) -> Self {
        Self::from_axis_and_count(self.axis(), -self.count()).unwrap()
    }
}

impl ToString for SliceMove {
    fn to_string(&self) -> String {
        match self {
            SliceMove::M => "M".to_string(),
            SliceMove::Mp => "M'".to_string(),
            SliceMove::M2 => "M2".to_string(),
            SliceMove::S => "S".to_string(),
            SliceMove::Sp => "S'".to_string(),
            SliceMove::S2 => "S2".to_string(),
            SliceMove::E => "E".to_string(),
            SliceMove::Ep => "E'".to_string(),
            SliceMove::E2 => "E2".to_string(),
        }
    }
}

impl WideMove {
    pub fn from_face_and_rotation(face: CubeFace, rotation: i32) -> Option<Self> {
        let rotation = rotation % 4;
        match face {
            CubeFace::Top => match rotation {
                -3 => Some(WideMove::U),
                -2 => Some(WideMove::U2),
                -1 => Some(WideMove::Up),
                1 => Some(WideMove::U),
                2 => Some(WideMove::U2),
                3 => Some(WideMove::Up),
                _ => None,
            },
            CubeFace::Front => match rotation {
                -3 => Some(WideMove::F),
                -2 => Some(WideMove::F2),
                -1 => Some(WideMove::Fp),
                1 => Some(WideMove::F),
                2 => Some(WideMove::F2),
                3 => Some(WideMove::Fp),
                _ => None,
            },
            CubeFace::Right => match rotation {
                -3 => Some(WideMove::R),
                -2 => Some(WideMove::R2),
                -1 => Some(WideMove::Rp),
                1 => Some(WideMove::R),
                2 => Some(WideMove::R2),
                3 => Some(WideMove::Rp),
                _ => None,
            },
            CubeFace::Back => match rotation {
                -3 => Some(WideMove::B),
                -2 => Some(WideMove::B2),
                -1 => Some(WideMove::Bp),
                1 => Some(WideMove::B),
                2 => Some(WideMove::B2),
                3 => Some(WideMove::Bp),
                _ => None,
            },
            CubeFace::Left => match rotation {
                -3 => Some(WideMove::L),
                -2 => Some(WideMove::L2),
                -1 => Some(WideMove::Lp),
                1 => Some(WideMove::L),
                2 => Some(WideMove::L2),
                3 => Some(WideMove::Lp),
                _ => None,
            },
            CubeFace::Bottom => match rotation {
                -3 => Some(WideMove::D),
                -2 => Some(WideMove::D2),
                -1 => Some(WideMove::Dp),
                1 => Some(WideMove::D),
                2 => Some(WideMove::D2),
                3 => Some(WideMove::Dp),
                _ => None,
            },
        }
    }

    pub const fn face(&self) -> CubeFace {
        match self {
            WideMove::U | WideMove::Up | WideMove::U2 => CubeFace::Top,
            WideMove::F | WideMove::Fp | WideMove::F2 => CubeFace::Front,
            WideMove::R | WideMove::Rp | WideMove::R2 => CubeFace::Right,
            WideMove::B | WideMove::Bp | WideMove::B2 => CubeFace::Back,
            WideMove::L | WideMove::Lp | WideMove::L2 => CubeFace::Left,
            WideMove::D | WideMove::Dp | WideMove::D2 => CubeFace::Bottom,
        }
    }

    /// Gets the face rotation amount in number of 90 degree clockwise rotations
    pub const fn rotation(&self) -> i32 {
        match self {
            WideMove::U | WideMove::F | WideMove::R | WideMove::B | WideMove::L | WideMove::D => 1,
            WideMove::Up
            | WideMove::Fp
            | WideMove::Rp
            | WideMove::Bp
            | WideMove::Lp
            | WideMove::Dp => -1,
            WideMove::U2
            | WideMove::F2
            | WideMove::R2
            | WideMove::B2
            | WideMove::L2
            | WideMove::D2 => 2,
        }
    }

    /// Gets the inverse of the move
    pub fn inverse(&self) -> Self {
        Self::from_face_and_rotation(self.face(), -self.rotation()).unwrap()
    }
}

impl ToString for WideMove {
    fn to_string(&self) -> String {
        match self {
            WideMove::U => "u".to_string(),
            WideMove::Up => "u'".to_string(),
            WideMove::U2 => "u2".to_string(),
            WideMove::F => "f".to_string(),
            WideMove::Fp => "f'".to_string(),
            WideMove::F2 => "f2".to_string(),
            WideMove::R => "r".to_string(),
            WideMove::Rp => "r'".to_string(),
            WideMove::R2 => "r2".to_string(),
            WideMove::B => "b".to_string(),
            WideMove::Bp => "b'".to_string(),
            WideMove::B2 => "b2".to_string(),
            WideMove::L => "l".to_string(),
            WideMove::Lp => "l'".to_string(),
            WideMove::L2 => "l2".to_string(),
            WideMove::D => "d".to_string(),
            WideMove::Dp => "d'".to_string(),
            WideMove::D2 => "d2".to_string(),
        }
    }
}

impl CubeRotation {
    pub fn from_axis_and_count(axis: CubeRotationAxis, count: i32) -> Option<Self> {
        let count = count % 4;
        match axis {
            CubeRotationAxis::X => match count {
                -3 => Some(CubeRotation::X),
                -2 => Some(CubeRotation::X2),
                -1 => Some(CubeRotation::Xp),
                1 => Some(CubeRotation::X),
                2 => Some(CubeRotation::X2),
                3 => Some(CubeRotation::Xp),
                _ => None,
            },
            CubeRotationAxis::Y => match count {
                -3 => Some(CubeRotation::Y),
                -2 => Some(CubeRotation::Y2),
                -1 => Some(CubeRotation::Yp),
                1 => Some(CubeRotation::Y),
                2 => Some(CubeRotation::Y2),
                3 => Some(CubeRotation::Yp),
                _ => None,
            },
            CubeRotationAxis::Z => match count {
                -3 => Some(CubeRotation::Z),
                -2 => Some(CubeRotation::Z2),
                -1 => Some(CubeRotation::Zp),
                1 => Some(CubeRotation::Z),
                2 => Some(CubeRotation::Z2),
                3 => Some(CubeRotation::Zp),
                _ => None,
            },
        }
    }

    pub fn axis(&self) -> CubeRotationAxis {
        match self {
            CubeRotation::X | CubeRotation::Xp | CubeRotation::X2 => CubeRotationAxis::X,
            CubeRotation::Y | CubeRotation::Yp | CubeRotation::Y2 => CubeRotationAxis::Y,
            CubeRotation::Z | CubeRotation::Zp | CubeRotation::Z2 => CubeRotationAxis::Z,
        }
    }

    pub fn count(&self) -> i32 {
        match self {
            CubeRotation::X | CubeRotation::Y | CubeRotation::Z => 1,
            CubeRotation::Xp | CubeRotation::Yp | CubeRotation::Zp => -1,
            CubeRotation::X2 | CubeRotation::Y2 | CubeRotation::Z2 => 2,
        }
    }

    /// Gets the inverse of the move
    pub fn inverse(&self) -> Self {
        Self::from_axis_and_count(self.axis(), -self.count()).unwrap()
    }
}

impl ToString for CubeRotation {
    fn to_string(&self) -> String {
        match self {
            CubeRotation::X => "x".to_string(),
            CubeRotation::Xp => "x'".to_string(),
            CubeRotation::X2 => "x2".to_string(),
            CubeRotation::Y => "y".to_string(),
            CubeRotation::Yp => "y'".to_string(),
            CubeRotation::Y2 => "y2".to_string(),
            CubeRotation::Z => "z".to_string(),
            CubeRotation::Zp => "z'".to_string(),
            CubeRotation::Z2 => "z2".to_string(),
        }
    }
}

impl ExtendedMoveSequence for Vec<ExtendedMove> {
    fn inverse(&self) -> Vec<ExtendedMove> {
        self.as_slice().inverse()
    }

    fn to_string(&self) -> String {
        self.as_slice().to_string()
    }
}

struct NullCube;

impl FaceRotation for NullCube {
    fn rotate_wide(&mut self, _face: CubeFace, _dir: RotationDirection, _width: usize) {}
}

impl ExtendedMoveSequence for &[ExtendedMove] {
    fn inverse(&self) -> Vec<ExtendedMove> {
        // Perform moves to find ending cube rotation
        let mut cube = NullCube;
        let mut context = ExtendedMoveContext::new(&mut cube);
        context.do_moves(self);

        // Starting moves are to place the cube in the correct rotation
        let mut result: Vec<ExtendedMove> = context
            .rotation()
            .iter()
            .map(|mv| ExtendedMove::Rotation(*mv))
            .collect();

        // The rest of the moves are the inverse sequence of the original
        let mut moves = self.iter().rev().map(|mv| mv.inverse()).collect();
        result.append(&mut moves);
        result
    }

    fn to_string(&self) -> String {
        let moves: Vec<String> = self.iter().map(|mv| mv.to_string()).collect();
        moves.join(" ")
    }
}

impl CubeFaceMap {
    fn new() -> Self {
        Self {
            mapping: [
                CubeFace::Top,
                CubeFace::Front,
                CubeFace::Right,
                CubeFace::Back,
                CubeFace::Left,
                CubeFace::Bottom,
            ],
        }
    }

    /// Translates a face accounting for any past rotation of the cube. The result is the face
    /// representing the original position of the face before any rotations, and is what needs
    /// to be used to apply moves to the cube state (which is not aware of rotations).
    fn translate(&self, face: CubeFace) -> CubeFace {
        self.mapping[face as u8 as usize]
    }

    /// Apply a cube rotation based on the axis and number of clockwise rotations.
    fn rotate(&mut self, axis: CubeRotationAxis, count: i32) {
        let count = (4 + (count % 4)) % 4;
        for _ in 0..count {
            match axis {
                CubeRotationAxis::X => self.rotate_x(),
                CubeRotationAxis::Y => self.rotate_y(),
                CubeRotationAxis::Z => self.rotate_z(),
            }
        }
    }

    /// Rotate in the X direction (same as R) once clockwise
    fn rotate_x(&mut self) {
        self.mapping = [
            self.mapping[CubeFace::Front as u8 as usize],
            self.mapping[CubeFace::Bottom as u8 as usize],
            self.mapping[CubeFace::Right as u8 as usize],
            self.mapping[CubeFace::Top as u8 as usize],
            self.mapping[CubeFace::Left as u8 as usize],
            self.mapping[CubeFace::Back as u8 as usize],
        ];
    }

    /// Rotate in the Y direction (same as U) once clockwise
    fn rotate_y(&mut self) {
        self.mapping = [
            self.mapping[CubeFace::Top as u8 as usize],
            self.mapping[CubeFace::Right as u8 as usize],
            self.mapping[CubeFace::Back as u8 as usize],
            self.mapping[CubeFace::Left as u8 as usize],
            self.mapping[CubeFace::Front as u8 as usize],
            self.mapping[CubeFace::Bottom as u8 as usize],
        ];
    }

    /// Rotate in the Z direction (same as F) once clockwise
    fn rotate_z(&mut self) {
        self.mapping = [
            self.mapping[CubeFace::Left as u8 as usize],
            self.mapping[CubeFace::Front as u8 as usize],
            self.mapping[CubeFace::Top as u8 as usize],
            self.mapping[CubeFace::Back as u8 as usize],
            self.mapping[CubeFace::Bottom as u8 as usize],
            self.mapping[CubeFace::Right as u8 as usize],
        ];
    }
}

impl<'a, C: FaceRotation> ExtendedMoveContext<'a, C> {
    /// Creates an extended move context for applying extended moves. `cube` is the cube state
    /// to apply the moves to.
    pub fn new(cube: &'a mut C) -> Self {
        Self {
            cube,
            face_map: CubeFaceMap::new(),
        }
    }

    /// Perform a single outer block move
    fn outer_move(&mut self, mv: Move) {
        let face = mv.face();
        let rotation = mv.rotation();
        let width = mv.width();
        self.cube
            .rotate_counted_wide(self.face_map.translate(face), rotation, width);
    }

    /// Perform a single slice move
    fn slice_move(&mut self, mv: SliceMove) {
        let axis = mv.axis();
        let count = mv.count();
        match axis {
            SliceMoveAxis::M => {
                self.cube
                    .rotate_counted(self.face_map.translate(CubeFace::Left), -count);
                self.cube
                    .rotate_counted(self.face_map.translate(CubeFace::Right), count);
                self.face_map.rotate(CubeRotationAxis::X, -count);
            }
            SliceMoveAxis::S => {
                self.cube
                    .rotate_counted(self.face_map.translate(CubeFace::Front), -count);
                self.cube
                    .rotate_counted(self.face_map.translate(CubeFace::Back), count);
                self.face_map.rotate(CubeRotationAxis::Z, count);
            }
            SliceMoveAxis::E => {
                self.cube
                    .rotate_counted(self.face_map.translate(CubeFace::Top), -count);
                self.cube
                    .rotate_counted(self.face_map.translate(CubeFace::Bottom), count);
                self.face_map.rotate(CubeRotationAxis::Y, -count);
            }
        }
    }

    /// Perform a single wide move
    fn wide_move(&mut self, mv: WideMove) {
        let face = mv.face();
        let rotation = mv.rotation();
        self.cube
            .rotate_counted(self.face_map.translate(face.opposite()), rotation);
        match face {
            CubeFace::Top => self.face_map.rotate(CubeRotationAxis::Y, rotation),
            CubeFace::Front => self.face_map.rotate(CubeRotationAxis::Z, rotation),
            CubeFace::Right => self.face_map.rotate(CubeRotationAxis::X, rotation),
            CubeFace::Back => self.face_map.rotate(CubeRotationAxis::Z, -rotation),
            CubeFace::Left => self.face_map.rotate(CubeRotationAxis::X, -rotation),
            CubeFace::Bottom => self.face_map.rotate(CubeRotationAxis::Y, -rotation),
        }
    }

    /// Perform a single cube rotation
    fn rotation_move(&mut self, rotation: CubeRotation) {
        self.face_map.rotate(rotation.axis(), rotation.count());
    }

    /// Perform a single move
    pub fn do_move(&mut self, mv: ExtendedMove) {
        match mv {
            ExtendedMove::Outer(mv) => self.outer_move(mv),
            ExtendedMove::Slice(mv) => self.slice_move(mv),
            ExtendedMove::Wide(mv) => self.wide_move(mv),
            ExtendedMove::Rotation(rotation) => self.rotation_move(rotation),
        }
    }

    /// Perform a sequence of moves
    pub fn do_moves(&mut self, seq: &[ExtendedMove]) {
        for mv in seq {
            self.do_move(*mv);
        }
    }

    /// Returns the original position of the given face. For example, after an `x` rotation,
    /// the `Top` face will return `Front` (the green face).
    pub fn face(&self, face: CubeFace) -> CubeFace {
        self.face_map.translate(face)
    }

    /// Returns the required moves to arrive at the current rotation of the cube starting
    /// from the initial state (white on top, green in front).
    pub fn rotation(&self) -> Vec<CubeRotation> {
        let mut result = Vec::new();
        match self.face(CubeFace::Top) {
            CubeFace::Top => match self.face(CubeFace::Front) {
                CubeFace::Front => (),
                CubeFace::Right => result.push(CubeRotation::Y),
                CubeFace::Left => result.push(CubeRotation::Yp),
                CubeFace::Back => result.push(CubeRotation::Y2),
                _ => unreachable!(),
            },
            CubeFace::Front => {
                result.push(CubeRotation::X);
                match self.face(CubeFace::Front) {
                    CubeFace::Bottom => (),
                    CubeFace::Right => result.push(CubeRotation::Y),
                    CubeFace::Left => result.push(CubeRotation::Yp),
                    CubeFace::Top => result.push(CubeRotation::Y2),
                    _ => unreachable!(),
                }
            }
            CubeFace::Right => {
                result.push(CubeRotation::Zp);
                match self.face(CubeFace::Front) {
                    CubeFace::Front => (),
                    CubeFace::Bottom => result.push(CubeRotation::Y),
                    CubeFace::Top => result.push(CubeRotation::Yp),
                    CubeFace::Back => result.push(CubeRotation::Y2),
                    _ => unreachable!(),
                }
            }
            CubeFace::Back => {
                result.push(CubeRotation::Xp);
                match self.face(CubeFace::Front) {
                    CubeFace::Top => (),
                    CubeFace::Right => result.push(CubeRotation::Y),
                    CubeFace::Left => result.push(CubeRotation::Yp),
                    CubeFace::Bottom => result.push(CubeRotation::Y2),
                    _ => unreachable!(),
                }
            }
            CubeFace::Left => {
                result.push(CubeRotation::Z);
                match self.face(CubeFace::Front) {
                    CubeFace::Front => (),
                    CubeFace::Top => result.push(CubeRotation::Y),
                    CubeFace::Bottom => result.push(CubeRotation::Yp),
                    CubeFace::Back => result.push(CubeRotation::Y2),
                    _ => unreachable!(),
                }
            }
            CubeFace::Bottom => {
                result.push(CubeRotation::X2);
                match self.face(CubeFace::Front) {
                    CubeFace::Back => (),
                    CubeFace::Right => result.push(CubeRotation::Y),
                    CubeFace::Left => result.push(CubeRotation::Yp),
                    CubeFace::Front => result.push(CubeRotation::Y2),
                    _ => unreachable!(),
                }
            }
        }
        result
    }

    /// Returns the required moves to arrive at initial cube rotation given the current
    /// rotation of the cube. Applying these moves will result in the white face being
    /// on top, and the green face being in front.
    pub fn inverse_rotation(&self) -> Vec<CubeRotation> {
        self.rotation()
            .iter()
            .rev()
            .map(|mv| mv.inverse())
            .collect()
    }

    /// Returns the required moves to arrive at the current rotation of the cube starting
    /// from the initial state (white on top). This function only matches the top and
    /// bottom faces, ignoring the sides.
    pub fn rotation_top_only(&self) -> Vec<CubeRotation> {
        match self.face(CubeFace::Top) {
            CubeFace::Top => Vec::new(),
            CubeFace::Front => vec![CubeRotation::X],
            CubeFace::Right => vec![CubeRotation::Zp],
            CubeFace::Back => vec![CubeRotation::Xp],
            CubeFace::Left => vec![CubeRotation::Z],
            CubeFace::Bottom => vec![CubeRotation::X2],
        }
    }

    /// Returns the required moves to arrive at initial cube rotation given the current
    /// rotation of the cube. Applying these moves will result in the white face being
    /// on top. This function only matches the top and bottom faces, ignoring the sides.
    pub fn inverse_rotation_top_only(&self) -> Vec<CubeRotation> {
        self.rotation_top_only()
            .iter()
            .rev()
            .map(|mv| mv.inverse())
            .collect()
    }
}
