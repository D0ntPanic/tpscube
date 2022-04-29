use crate::{
    Color, Corner, CornerPiece, Cube, CubeFace, FaceRotation, InitialCubeState, Move, RandomSource,
    RotationDirection,
};
use num_enum::TryFromPrimitive;
use std::collections::BTreeMap;
use std::convert::TryFrom;

#[cfg(not(feature = "no_solver"))]
use crate::common::MoveSequence;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
/// Identification of an edge piece. Names come from the faces of the cube this edge
/// belongs to on a solved cube, followed by a differentiator for which of the two
/// edge pieces on the side. The indexes must be chosen such that edges rotating on
/// an inner slice always inverts orientation.
pub enum Edge4x4x4 {
    URB = 0,
    URF = 1,
    UFR = 2,
    UFL = 3,
    ULF = 4,
    ULB = 5,
    UBL = 6,
    UBR = 7,
    DRF = 8,
    DRB = 9,
    DFL = 10,
    DFR = 11,
    DLB = 12,
    DLF = 13,
    DBR = 14,
    DBL = 15,
    RFD = 16,
    RFU = 17,
    LFU = 18,
    LFD = 19,
    LBD = 20,
    LBU = 21,
    RBU = 22,
    RBD = 23,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EdgePiece4x4x4 {
    pub piece: Edge4x4x4,
    pub orientation: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// A 4x4x4 cube represented in piece format (optimal for computational algorithms).
pub struct Cube4x4x4 {
    corners: [CornerPiece; 8],
    edges: [EdgePiece4x4x4; 24],
    centers: [Color; 24],
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// A 4x4x4 cube represented in face color format (easy to use, matches visuals).
pub struct Cube4x4x4Faces {
    state: [Color; 6 * 16],
}

impl Cube4x4x4 {
    pub fn from_corners_edges_and_centers(
        corners: [CornerPiece; 8],
        edges: [EdgePiece4x4x4; 24],
        centers: [Color; 24],
    ) -> Self {
        Self {
            corners,
            edges,
            centers,
        }
    }

    /// Gets the piece at a given corner
    pub fn corner_piece(&self, corner: Corner) -> CornerPiece {
        self.corners[corner as u8 as usize]
    }

    /// Gets the piece at a given edge
    pub fn edge_piece(&self, edge: Edge4x4x4) -> EdgePiece4x4x4 {
        self.edges[edge as u8 as usize]
    }

    pub fn center_color(&self, face: CubeFace, row: usize, col: usize) -> Color {
        self.centers[Self::center_idx(face, row, col)]
    }

    pub(crate) const fn center_idx(face: CubeFace, row: usize, col: usize) -> usize {
        face as u8 as usize * 4 + row * 2 + col
    }

    pub fn oll_parity(&self) -> bool {
        // OLL parity exists if the number of flipped edges is not a multiple of 4.
        // It is always a multiple of 2, so check bit 1 of the sum of orientations.
        let mut result = 0;
        for edge in &self.edges {
            result += edge.orientation;
        }
        result & 2 != 0
    }

    /// Gets this cube state in face color format
    pub fn as_faces(&self) -> Cube4x4x4Faces {
        let mut faces = Cube4x4x4Faces::new();

        // Translate corner pieces into face colors
        for corner_idx in 0..8 {
            let piece = self.corners[corner_idx];
            for i in 0..3 {
                let dest = crate::tables::corner::CUBE4_CORNER_INDICIES[corner_idx][i];
                let src = crate::tables::corner::CUBE4_CORNER_INDICIES[piece.piece as u8 as usize]
                    [(i + 3 - piece.orientation as usize) % 3];
                let face = Cube4x4x4Faces::face_for_idx(src);
                faces.state[dest] = face.color();
            }
        }

        // Translate edge pieces into face colors
        for edge_idx in 0..24 {
            let piece = self.edges[edge_idx];
            for i in 0..2 {
                let dest = crate::tables::table4x4x4::CUBE4_EDGE_INDICIES[edge_idx][i];
                let src = crate::tables::table4x4x4::CUBE4_EDGE_INDICIES
                    [piece.piece as u8 as usize][i ^ piece.orientation as usize];
                let face = Cube4x4x4Faces::face_for_idx(src);
                faces.state[dest] = face.color();
            }
        }

        // Set center piece colors
        for center_idx in 0..24 {
            let face = CubeFace::try_from((center_idx / 4) as u8).unwrap();
            let row = (center_idx / 2) % 2;
            let col = center_idx % 2;
            faces.state[Cube4x4x4Faces::idx(face, row + 1, col + 1)] =
                self.center_color(face, row, col);
        }

        faces
    }
}

impl FaceRotation for Cube4x4x4 {
    fn rotate_wide(&mut self, face: CubeFace, dir: RotationDirection, width: usize) {
        let face_idx = face as u8 as usize;
        let dir_idx = dir as u8 as usize;

        // Save existing cube state so that it can be looked up during rotation
        let old_corners = self.corners;
        let old_edges = self.edges;
        let old_centers = self.centers;

        // Apply corner movement using lookup table
        for i in 0..4 {
            let (dest, src) =
                crate::tables::corner::CUBE_CORNER_PIECE_ROTATION[dir_idx][face_idx][i];
            self.corners[dest as u8 as usize] = CornerPiece {
                piece: old_corners[src.piece as u8 as usize].piece,
                orientation: (old_corners[src.piece as u8 as usize].orientation + src.orientation)
                    % 3,
            };
        }

        // Apply outer edge movement using lookup table
        for i in 0..8 {
            let (dest, src) =
                crate::tables::table4x4x4::CUBE4_EDGE_PIECE_ROTATION[dir_idx][face_idx][i];
            self.edges[dest as u8 as usize] = EdgePiece4x4x4 {
                piece: old_edges[src.piece as u8 as usize].piece,
                orientation: (old_edges[src.piece as u8 as usize].orientation ^ src.orientation),
            };
        }

        // Apply center movement using lookup table
        for i in 0..4 {
            let src = crate::tables::table4x4x4::CUBE4_CENTER_PIECE_ROTATION[dir_idx][i];
            self.centers[face_idx * 4 + i] = old_centers[face_idx * 4 + src];
        }

        if width == 2 {
            // Wide move, apply inner slice edge movement using lookup table. Inner edge pieces
            // always invert orientation no matter which face (this is an invariant that must
            // be true for the edge piece indexes to work properly).
            let edge_offset = match dir {
                RotationDirection::CW => 3,
                RotationDirection::CCW => 1,
            };
            for i in 0..4 {
                let dest = crate::tables::table4x4x4::CUBE4_SLICED_EDGE_PIECE_ROTATION[face_idx][i];
                let src = crate::tables::table4x4x4::CUBE4_SLICED_EDGE_PIECE_ROTATION[face_idx]
                    [(i + edge_offset) % 4];
                self.edges[dest as u8 as usize] = EdgePiece4x4x4 {
                    piece: old_edges[src as u8 as usize].piece,
                    orientation: old_edges[src as u8 as usize].orientation ^ 1,
                };
            }

            // Apply inner slice center movement using lookup table
            let center_offset = match dir {
                RotationDirection::CW => 6,
                RotationDirection::CCW => 2,
            };
            for i in 0..8 {
                let dest =
                    crate::tables::table4x4x4::CUBE4_SLICED_CENTER_PIECE_ROTATION[face_idx][i];
                let src = crate::tables::table4x4x4::CUBE4_SLICED_CENTER_PIECE_ROTATION[face_idx]
                    [(i + center_offset) % 8];
                self.centers[dest] = old_centers[src];
            }
        }
    }
}

impl InitialCubeState for Cube4x4x4 {
    fn new() -> Self {
        let mut corners = [CornerPiece {
            piece: Corner::URF,
            orientation: 0,
        }; 8];
        for i in 0..8 {
            corners[i].piece = Corner::try_from(i as u8).unwrap();
        }

        let mut edges = [EdgePiece4x4x4 {
            piece: Edge4x4x4::URB,
            orientation: 0,
        }; 24];
        for i in 0..24 {
            edges[i].piece = Edge4x4x4::try_from(i as u8).unwrap();
        }

        let mut centers = [Color::White; 24];
        for i in 0..24 {
            centers[i] = CubeFace::try_from((i / 4) as u8).unwrap().color();
        }

        Self {
            corners,
            edges,
            centers,
        }
    }

    fn sourced_random<T: RandomSource>(rng: &mut T) -> Self {
        let mut cube = Self::new();

        // Randomize the corner pieces
        for i in 0..7 {
            let n = rng.next(8) as usize;
            if i != n {
                // Must swap two corners at a time to avoid parity violation
                cube.corners.swap(i, n);
                cube.corners.swap(6, 7);
            }
        }

        // Randomize the edge pieces
        for i in 0..23 {
            let n = rng.next(24) as usize;
            if i != n {
                // Must swap two edges at a time to avoid parity violation
                cube.edges.swap(i, n);
                cube.edges.swap(22, 23);
            }
        }

        // Randomize the center pieces
        for i in 0..23 {
            let n = rng.next(24) as usize;
            if i != n {
                // Must swap two edges at a time to avoid parity violation
                cube.centers.swap(i, n);
                cube.centers.swap(22, 23);
            }
        }

        // Randomize the corner orientations
        let mut corner_orientation_sum = 0;
        for i in 0..7 {
            cube.corners[i].orientation = rng.next(3) as u8;
            corner_orientation_sum += cube.corners[i].orientation;
        }

        // Randomize the edge orientations
        let mut edge_orientation_sum = 0;
        for i in 0..23 {
            cube.edges[i].orientation = rng.next(3) as u8;
            edge_orientation_sum += cube.edges[i].orientation;
        }

        // Make sure all corner orientations add up to a multiple of 3 (otherwise it is not solvable)
        cube.corners[7].orientation = (3 - (corner_orientation_sum % 3)) % 3;

        // Make sure all edge orientations add up to a multiple of 2 (otherwise it is not solvable)
        cube.edges[23].orientation = (2 - (edge_orientation_sum % 2)) % 2;

        cube
    }
}

impl Cube for Cube4x4x4 {
    fn is_solved(&self) -> bool {
        // Check corners
        for i in 0..8 {
            let correct_piece = CornerPiece {
                piece: Corner::try_from(i as u8).unwrap(),
                orientation: 0,
            };
            if self.corners[i] != correct_piece {
                return false;
            }
        }

        // Check edges
        for i in 0..24 {
            let correct_piece = EdgePiece4x4x4 {
                piece: Edge4x4x4::try_from(i as u8).unwrap(),
                orientation: 0,
            };
            if self.edges[i] != correct_piece {
                return false;
            }
        }

        // Check centers
        for i in 0..24 {
            if self.centers[i] != CubeFace::try_from((i / 4) as u8).unwrap().color() {
                return false;
            }
        }
        true
    }

    fn do_move(&mut self, mv: Move) {
        self.rotate_counted_wide(mv.face(), mv.rotation(), mv.width());
    }

    fn size(&self) -> usize {
        4
    }

    fn colors(&self) -> BTreeMap<CubeFace, Vec<Vec<Color>>> {
        self.as_faces().colors()
    }

    #[cfg(not(feature = "no_solver"))]
    fn solve(&self) -> Option<Vec<Move>> {
        unimplemented!()
    }

    #[cfg(not(feature = "no_solver"))]
    fn solve_fast(&self) -> Option<Vec<Move>> {
        unimplemented!()
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn dyn_clone(&self) -> Box<dyn Cube> {
        Box::new(self.clone())
    }
}

impl Cube4x4x4Faces {
    /// Create a cube state from a color array. The ordering of the array is the faces
    /// in order of the `Face` enumeration, with 9 elements per face. Each face is stored
    /// from top to bottom in row major order, with columns left to right.
    pub fn from_colors(state: [Color; 6 * 16]) -> Self {
        Self { state }
    }

    pub(crate) const fn face_start(face: CubeFace) -> usize {
        face as u8 as usize * 16
    }

    pub(crate) const fn face_offset(row: usize, col: usize) -> usize {
        (row * 4) + col
    }

    pub(crate) const fn idx(face: CubeFace, row: usize, col: usize) -> usize {
        Self::face_start(face) + Self::face_offset(row, col)
    }

    pub(crate) fn face_for_idx(idx: usize) -> CubeFace {
        CubeFace::try_from((idx / 16) as u8).unwrap()
    }

    /// Gets the color for a given place on the cube. For a given `face`, the `row` and
    /// `col` represent the zero-indexed position on the face to be accessed.
    pub fn color(&self, face: CubeFace, row: usize, col: usize) -> Color {
        self.state[Self::idx(face, row, col)]
    }

    /// Gets the color for a given place on the cube. For a given `face`, the `row` and
    /// `col` represent the zero-indexed position on the face to be accessed.
    pub(crate) fn color_by_idx(&self, idx: usize) -> Color {
        self.state[idx]
    }

    /// Gets the color of a specific corner (there are three colors per corner)
    pub fn corner_color(&self, corner: Corner, idx: usize) -> Color {
        self.state[crate::tables::corner::CUBE4_CORNER_INDICIES[corner as u8 as usize][idx]]
    }

    /// Gets the color of a specific edge (there are two colors per edge)
    pub fn edge_color(&self, edge: Edge4x4x4, idx: usize) -> Color {
        self.state[crate::tables::table4x4x4::CUBE4_EDGE_INDICIES[edge as u8 as usize][idx]]
    }

    /// Gets the color of a specific center piece
    pub fn center_color(&self, center: CubeFace, row: usize, col: usize) -> Color {
        self.color_by_idx(Self::idx(center, row + 1, col + 1))
    }

    /// Gets this cube state in piece format
    pub fn as_pieces(&self) -> Cube4x4x4 {
        let mut pieces = Cube4x4x4::new();

        for corner_idx in 0..8 {
            let corner_colors: [Color; 3] = [
                self.corner_color(Corner::try_from(corner_idx).unwrap(), 0),
                self.corner_color(Corner::try_from(corner_idx).unwrap(), 1),
                self.corner_color(Corner::try_from(corner_idx).unwrap(), 2),
            ];
            // Find this corner piece and orientation
            for i in 0..8 {
                if corner_colors[0] == crate::tables::corner::CUBE_CORNER_COLORS[i][0]
                    && corner_colors[1] == crate::tables::corner::CUBE_CORNER_COLORS[i][1]
                    && corner_colors[2] == crate::tables::corner::CUBE_CORNER_COLORS[i][2]
                {
                    pieces.corners[corner_idx as usize] = CornerPiece {
                        piece: Corner::try_from(i as u8).unwrap(),
                        orientation: 0,
                    };
                    break;
                } else if corner_colors[1] == crate::tables::corner::CUBE_CORNER_COLORS[i][0]
                    && corner_colors[2] == crate::tables::corner::CUBE_CORNER_COLORS[i][1]
                    && corner_colors[0] == crate::tables::corner::CUBE_CORNER_COLORS[i][2]
                {
                    pieces.corners[corner_idx as usize] = CornerPiece {
                        piece: Corner::try_from(i as u8).unwrap(),
                        orientation: 1,
                    };
                    break;
                } else if corner_colors[2] == crate::tables::corner::CUBE_CORNER_COLORS[i][0]
                    && corner_colors[0] == crate::tables::corner::CUBE_CORNER_COLORS[i][1]
                    && corner_colors[1] == crate::tables::corner::CUBE_CORNER_COLORS[i][2]
                {
                    pieces.corners[corner_idx as usize] = CornerPiece {
                        piece: Corner::try_from(i as u8).unwrap(),
                        orientation: 2,
                    };
                    break;
                }
            }
        }

        for edge_idx in 0..24 {
            let edge_colors: [Color; 2] = [
                self.edge_color(Edge4x4x4::try_from(edge_idx).unwrap(), 0),
                self.edge_color(Edge4x4x4::try_from(edge_idx).unwrap(), 1),
            ];
            // Find this edge piece and orientation. Which of the two possible edges for the color
            // combination is based on whether we are looking for an odd indexed slot, and whether
            // the detected colors are flipped. This is a simple XOR of the two conditions to
            // determine the lowest bit of the edge index.
            for i in 0..12 {
                if edge_colors[0] == crate::tables::table4x4x4::CUBE4_EDGE_COLORS[i][0]
                    && edge_colors[1] == crate::tables::table4x4x4::CUBE4_EDGE_COLORS[i][1]
                {
                    pieces.edges[edge_idx as usize] = EdgePiece4x4x4 {
                        piece: Edge4x4x4::try_from((i * 2) as u8 ^ (edge_idx as u8 & 1)).unwrap(),
                        orientation: 0,
                    };
                    break;
                } else if edge_colors[1] == crate::tables::table4x4x4::CUBE4_EDGE_COLORS[i][0]
                    && edge_colors[0] == crate::tables::table4x4x4::CUBE4_EDGE_COLORS[i][1]
                {
                    pieces.edges[edge_idx as usize] = EdgePiece4x4x4 {
                        piece: Edge4x4x4::try_from((i * 2) as u8 ^ (edge_idx as u8 & 1) ^ 1)
                            .unwrap(),
                        orientation: 1,
                    };
                    break;
                }
            }
        }

        for center_idx in 0..6 {
            pieces.centers[center_idx * 4] =
                self.center_color(CubeFace::try_from(center_idx as u8).unwrap(), 0, 0);
            pieces.centers[center_idx * 4 + 1] =
                self.center_color(CubeFace::try_from(center_idx as u8).unwrap(), 0, 1);
            pieces.centers[center_idx * 4 + 2] =
                self.center_color(CubeFace::try_from(center_idx as u8).unwrap(), 1, 0);
            pieces.centers[center_idx * 4 + 3] =
                self.center_color(CubeFace::try_from(center_idx as u8).unwrap(), 1, 1);
        }

        pieces
    }
}

impl FaceRotation for Cube4x4x4Faces {
    fn rotate_wide(&mut self, face: CubeFace, dir: RotationDirection, width: usize) {
        let face_idx = face as u8 as usize;
        let dir_idx = dir as u8 as usize;

        // Rotate colors on face itself
        let mut rotated_colors: [Color; 16] = [Color::White; 16];
        for i in 0..16 {
            rotated_colors[i] = self.state[Self::face_start(face)
                + crate::tables::table4x4x4::CUBE4_FACE_ROTATION[dir_idx][i]];
        }
        for i in 0..16 {
            self.state[Self::face_start(face) + i] = rotated_colors[i];
        }

        // Collect colors on edges and corners
        let mut adjacent_corner_colors: [[Color; 2]; 4] = [[Color::White; 2]; 4];
        for i in 0..4 {
            adjacent_corner_colors[i][0] =
                self.state[crate::tables::corner::CUBE4_CORNER_ADJACENCY[face_idx][i][0]];
            adjacent_corner_colors[i][1] =
                self.state[crate::tables::corner::CUBE4_CORNER_ADJACENCY[face_idx][i][1]];
        }

        let mut adjacent_edge_colors: [Color; 8] = [Color::White; 8];
        for i in 0..8 {
            adjacent_edge_colors[i] =
                self.state[crate::tables::table4x4x4::CUBE4_EDGE_ADJACENCY[face_idx][i]];
        }

        // Rotate colors on corners
        for i in 0..4 {
            let j = crate::tables::corner::CUBE_CORNER_ROTATION[dir_idx][i];
            self.state[crate::tables::corner::CUBE4_CORNER_ADJACENCY[face_idx][j][0]] =
                adjacent_corner_colors[i][0];
            self.state[crate::tables::corner::CUBE4_CORNER_ADJACENCY[face_idx][j][1]] =
                adjacent_corner_colors[i][1];
        }

        // Rotate colors on edges
        for i in 0..8 {
            let j = crate::tables::table4x4x4::CUBE4_EDGE_ROTATION[dir_idx][i];
            self.state[crate::tables::table4x4x4::CUBE4_EDGE_ADJACENCY[face_idx][j]] =
                adjacent_edge_colors[i];
        }

        if width == 2 {
            // Wide move, apply inner slice edge movement using lookup table. Inner edge pieces
            // always invert orientation no matter which face (this is an invariant that must
            // be true for the edge piece indexes to work properly).
            let mut slice_edge_colors: [[Color; 2]; 4] = [[Color::White; 2]; 4];
            let edge_offset = match dir {
                RotationDirection::CW => 3,
                RotationDirection::CCW => 1,
            };
            for i in 0..4 {
                let src = crate::tables::table4x4x4::CUBE4_EDGE_INDICIES
                    [crate::tables::table4x4x4::CUBE4_SLICED_EDGE_PIECE_ROTATION[face_idx][i] as u8
                        as usize];
                for j in 0..2 {
                    slice_edge_colors[i][j] = self.state[src[j]];
                }
            }
            for i in 0..4 {
                let dest = crate::tables::table4x4x4::CUBE4_EDGE_INDICIES
                    [crate::tables::table4x4x4::CUBE4_SLICED_EDGE_PIECE_ROTATION[face_idx][i] as u8
                        as usize];
                for j in 0..2 {
                    self.state[dest[j]] = slice_edge_colors[(i + edge_offset) % 4][j ^ 1];
                }
            }

            // Apply inner slice center movement using lookup table
            let mut slice_center_colors: [Color; 16] = [Color::White; 16];
            let center_offset = match dir {
                RotationDirection::CW => 6,
                RotationDirection::CCW => 2,
            };
            for i in 0..8 {
                slice_center_colors[i] = self.state
                    [crate::tables::table4x4x4::CUBE4_SLICED_CENTER_FACE_ROTATION[face_idx][i]];
            }
            for i in 0..8 {
                self.state
                    [crate::tables::table4x4x4::CUBE4_SLICED_CENTER_FACE_ROTATION[face_idx][i]] =
                    slice_center_colors[(i + center_offset) % 8];
            }
        }
    }
}

impl InitialCubeState for Cube4x4x4Faces {
    fn new() -> Self {
        let mut state = [Color::White; 6 * 16];
        for i in 0..16 {
            state[Self::face_start(CubeFace::Top) + i] = Color::White;
            state[Self::face_start(CubeFace::Front) + i] = Color::Green;
            state[Self::face_start(CubeFace::Right) + i] = Color::Red;
            state[Self::face_start(CubeFace::Back) + i] = Color::Blue;
            state[Self::face_start(CubeFace::Left) + i] = Color::Orange;
            state[Self::face_start(CubeFace::Bottom) + i] = Color::Yellow;
        }
        Self { state }
    }

    fn sourced_random<T: RandomSource>(rng: &mut T) -> Self {
        Cube4x4x4::sourced_random(rng).as_faces()
    }
}

impl Cube for Cube4x4x4Faces {
    fn is_solved(&self) -> bool {
        for face in 0..6 {
            let face = CubeFace::try_from(face).unwrap();
            for i in 0..16 {
                if self.state[Self::face_start(face) + i] != face.color() {
                    return false;
                }
            }
        }
        true
    }

    fn do_move(&mut self, mv: Move) {
        self.rotate_counted_wide(mv.face(), mv.rotation(), mv.width());
    }

    fn size(&self) -> usize {
        4
    }

    fn colors(&self) -> BTreeMap<CubeFace, Vec<Vec<Color>>> {
        let mut result = BTreeMap::new();
        for face in &[
            CubeFace::Top,
            CubeFace::Front,
            CubeFace::Right,
            CubeFace::Back,
            CubeFace::Left,
            CubeFace::Bottom,
        ] {
            let mut rows = Vec::new();
            for row in 0..4 {
                let mut cols = Vec::new();
                for col in 0..4 {
                    cols.push(self.color(*face, row, col));
                }
                rows.push(cols);
            }
            result.insert(*face, rows);
        }
        result
    }

    #[cfg(not(feature = "no_solver"))]
    fn solve(&self) -> Option<Vec<Move>> {
        self.as_pieces().solve()
    }

    #[cfg(not(feature = "no_solver"))]
    fn solve_fast(&self) -> Option<Vec<Move>> {
        self.as_pieces().solve_fast()
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn dyn_clone(&self) -> Box<dyn Cube> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for Cube4x4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_faces().fmt(f)
    }
}

impl std::fmt::Display for Cube4x4x4Faces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_state: [[char; 17]; 12] = [[' '; 17]; 12];
        const FACE_X: [usize; 6] = [4, 4, 8, 12, 0, 4];
        const FACE_Y: [usize; 6] = [0, 4, 4, 4, 4, 8];
        for face_idx in 0..6 {
            for row in 0..4 {
                for col in 0..4 {
                    let ch = match self.state
                        [Self::idx(CubeFace::try_from(face_idx).unwrap(), row, col)]
                    {
                        Color::White => 'W',
                        Color::Green => 'G',
                        Color::Red => 'R',
                        Color::Blue => 'B',
                        Color::Orange => 'O',
                        Color::Yellow => 'Y',
                    };
                    debug_state[FACE_Y[face_idx as usize] + row][FACE_X[face_idx as usize] + col] =
                        ch;
                }
            }
        }
        for row in 0..12 {
            let s: String = debug_state[row].iter().collect();
            write!(f, "{}\n", s)?;
        }
        Ok(())
    }
}

/// Generates a random scramble
#[cfg(not(feature = "no_solver"))]
pub fn scramble_4x4x4() -> Vec<Move> {
    let state = Cube4x4x4::random();
    let solution = state.solve().unwrap();
    solution.inverse()
}

/// Generates a random scramble very fast, but with more moves required than normal
#[cfg(not(feature = "no_solver"))]
pub fn scramble_4x4x4_fast() -> Vec<Move> {
    let state = Cube4x4x4::random();
    let solution = state.solve_fast().unwrap();
    solution.inverse()
}
