use crate::cube2x2x2::Cube2x2x2Faces;

// Table for rotation of a face in face color format. Each entry is the
// index on a face where the new color comes from.
pub(crate) const CUBE2_FACE_ROTATION: [[usize; 4]; 2] = [
    // CW
    [
        Cube2x2x2Faces::face_offset(1, 0),
        Cube2x2x2Faces::face_offset(0, 0),
        Cube2x2x2Faces::face_offset(1, 1),
        Cube2x2x2Faces::face_offset(0, 1),
    ],
    // CCW
    [
        Cube2x2x2Faces::face_offset(0, 1),
        Cube2x2x2Faces::face_offset(1, 1),
        Cube2x2x2Faces::face_offset(0, 0),
        Cube2x2x2Faces::face_offset(1, 0),
    ],
];
