use cgmath::Vector4;
use alloc::vec::Vec;

pub fn cell_600_verts() -> Vec<Vector4<f32>> {
    vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(-1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, -1.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, -1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
        Vector4::new(0.0, 0.0, 0.0, -1.0),
        Vector4::new(0.0, 0.309017, 0.5, 0.809015),
        Vector4::new(0.0, 0.309017, -0.5, 0.809015),
        Vector4::new(0.0, -0.309017, 0.5, 0.809015),
        Vector4::new(0.0, 0.309017, 0.5, -0.809015),
        Vector4::new(0.0, -0.309017, -0.5, 0.809015),
        Vector4::new(0.0, 0.309017, -0.5, -0.809015),
        Vector4::new(0.0, -0.309017, 0.5, -0.809015),
        Vector4::new(0.0, -0.309017, -0.5, -0.809015),
        Vector4::new(0.0, 0.809015, 0.309017, 0.5),
        Vector4::new(0.0, 0.809015, -0.309017, 0.5),
        Vector4::new(0.0, -0.809015, 0.309017, 0.5),
        Vector4::new(0.0, 0.809015, 0.309017, -0.5),
        Vector4::new(0.0, -0.809015, -0.309017, 0.5),
        Vector4::new(0.0, 0.809015, -0.309017, -0.5),
        Vector4::new(0.0, -0.809015, 0.309017, -0.5),
        Vector4::new(0.0, -0.809015, -0.309017, -0.5),
        Vector4::new(0.0, 0.5, 0.809015, 0.309017),
        Vector4::new(0.0, 0.5, -0.809015, 0.309017),
        Vector4::new(0.0, -0.5, 0.809015, 0.309017),
        Vector4::new(0.0, 0.5, 0.809015, -0.309017),
        Vector4::new(0.0, -0.5, -0.809015, 0.309017),
        Vector4::new(0.0, 0.5, -0.809015, -0.309017),
        Vector4::new(0.0, -0.5, 0.809015, -0.309017),
        Vector4::new(0.0, -0.5, -0.809015, -0.309017),
        Vector4::new(0.5, 0.809015, 0.0, 0.309017),
        Vector4::new(-0.5, 0.809015, 0.0, 0.309017),
        Vector4::new(0.5, -0.809015, 0.0, 0.309017),
        Vector4::new(0.5, 0.809015, 0.0, -0.309017),
        Vector4::new(-0.5, -0.809015, 0.0, 0.309017),
        Vector4::new(-0.5, 0.809015, 0.0, -0.309017),
        Vector4::new(0.5, -0.809015, 0.0, -0.309017),
        Vector4::new(-0.5, -0.809015, 0.0, -0.309017),
        Vector4::new(0.309017, 0.5, 0.0, 0.809015),
        Vector4::new(-0.309017, 0.5, 0.0, 0.809015),
        Vector4::new(0.309017, -0.5, 0.0, 0.809015),
        Vector4::new(0.309017, 0.5, 0.0, -0.809015),
        Vector4::new(-0.309017, -0.5, 0.0, 0.809015),
        Vector4::new(-0.309017, 0.5, 0.0, -0.809015),
        Vector4::new(0.309017, -0.5, 0.0, -0.809015),
        Vector4::new(-0.309017, -0.5, 0.0, -0.809015),
        Vector4::new(0.809015, 0.309017, 0.0, 0.5),
        Vector4::new(-0.809015, 0.309017, 0.0, 0.5),
        Vector4::new(0.809015, -0.309017, 0.0, 0.5),
        Vector4::new(0.809015, 0.309017, 0.0, -0.5),
        Vector4::new(-0.809015, -0.309017, 0.0, 0.5),
        Vector4::new(-0.809015, 0.309017, 0.0, -0.5),
        Vector4::new(0.809015, -0.309017, 0.0, -0.5),
        Vector4::new(-0.809015, -0.309017, 0.0, -0.5),
        Vector4::new(0.309017, 0.0, 0.809015, 0.5),
        Vector4::new(-0.309017, 0.0, 0.809015, 0.5),
        Vector4::new(0.309017, 0.0, -0.809015, 0.5),
        Vector4::new(0.309017, 0.0, 0.809015, -0.5),
        Vector4::new(-0.309017, 0.0, -0.809015, 0.5),
        Vector4::new(-0.309017, 0.0, 0.809015, -0.5),
        Vector4::new(0.309017, 0.0, -0.809015, -0.5),
        Vector4::new(-0.309017, 0.0, -0.809015, -0.5),
        Vector4::new(0.809015, 0.0, 0.5, 0.309017),
        Vector4::new(-0.809015, 0.0, 0.5, 0.309017),
        Vector4::new(0.809015, 0.0, -0.5, 0.309017),
        Vector4::new(0.809015, 0.0, 0.5, -0.309017),
        Vector4::new(-0.809015, 0.0, -0.5, 0.309017),
        Vector4::new(-0.809015, 0.0, 0.5, -0.309017),
        Vector4::new(0.809015, 0.0, -0.5, -0.309017),
        Vector4::new(-0.809015, 0.0, -0.5, -0.309017),
        Vector4::new(0.5, 0.0, 0.309017, 0.809015),
        Vector4::new(-0.5, 0.0, 0.309017, 0.809015),
        Vector4::new(0.5, 0.0, -0.309017, 0.809015),
        Vector4::new(0.5, 0.0, 0.309017, -0.809015),
        Vector4::new(-0.5, 0.0, -0.309017, 0.809015),
        Vector4::new(-0.5, 0.0, 0.309017, -0.809015),
        Vector4::new(0.5, 0.0, -0.309017, -0.809015),
        Vector4::new(-0.5, 0.0, -0.309017, -0.809015),
        Vector4::new(0.809015, 0.5, 0.309017, 0.0),
        Vector4::new(-0.809015, 0.5, 0.309017, 0.0),
        Vector4::new(0.809015, 0.5, -0.309017, 0.0),
        Vector4::new(0.809015, -0.5, 0.309017, 0.0),
        Vector4::new(-0.809015, 0.5, -0.309017, 0.0),
        Vector4::new(-0.809015, -0.5, 0.309017, 0.0),
        Vector4::new(0.809015, -0.5, -0.309017, 0.0),
        Vector4::new(-0.809015, -0.5, -0.309017, 0.0),
        Vector4::new(0.5, 0.309017, 0.809015, 0.0),
        Vector4::new(-0.5, 0.309017, 0.809015, 0.0),
        Vector4::new(0.5, 0.309017, -0.809015, 0.0),
        Vector4::new(0.5, -0.309017, 0.809015, 0.0),
        Vector4::new(-0.5, 0.309017, -0.809015, 0.0),
        Vector4::new(-0.5, -0.309017, 0.809015, 0.0),
        Vector4::new(0.5, -0.309017, -0.809015, 0.0),
        Vector4::new(-0.5, -0.309017, -0.809015, 0.0),
        Vector4::new(0.309017, 0.809015, 0.5, 0.0),
        Vector4::new(-0.309017, 0.809015, 0.5, 0.0),
        Vector4::new(0.309017, 0.809015, -0.5, 0.0),
        Vector4::new(0.309017, -0.809015, 0.5, 0.0),
        Vector4::new(-0.309017, 0.809015, -0.5, 0.0),
        Vector4::new(-0.309017, -0.809015, 0.5, 0.0),
        Vector4::new(0.309017, -0.809015, -0.5, 0.0),
        Vector4::new(-0.309017, -0.809015, -0.5, 0.0),
        Vector4::new(0.5, 0.5, 0.5, 0.5),
        Vector4::new(-0.5, 0.5, 0.5, 0.5),
        Vector4::new(0.5, 0.5, -0.5, 0.5),
        Vector4::new(0.5, -0.5, 0.5, 0.5),
        Vector4::new(0.5, 0.5, 0.5, -0.5),
        Vector4::new(-0.5, 0.5, -0.5, 0.5),
        Vector4::new(-0.5, -0.5, 0.5, 0.5),
        Vector4::new(-0.5, 0.5, 0.5, -0.5),
        Vector4::new(0.5, -0.5, -0.5, 0.5),
        Vector4::new(0.5, 0.5, -0.5, -0.5),
        Vector4::new(0.5, -0.5, 0.5, -0.5),
        Vector4::new(-0.5, -0.5, -0.5, 0.5),
        Vector4::new(-0.5, 0.5, -0.5, -0.5),
        Vector4::new(-0.5, -0.5, 0.5, -0.5),
        Vector4::new(0.5, -0.5, -0.5, -0.5),
        Vector4::new(-0.5, -0.5, -0.5, -0.5),
    ]
}

pub fn cell_600_faces() -> Vec<usize> {
    vec![
        0, 48, 50, 0, 48, 64, 0, 50, 64, 48, 50, 64, 0, 48, 66, 0, 50, 66, 48,
        50, 66, 0, 48, 80, 0, 64, 80, 48, 64, 80, 0, 48, 82, 0, 66, 82, 48, 66,
        82, 0, 80, 82, 48, 80, 82, 0, 50, 83, 0, 64, 83, 50, 64, 83, 0, 50, 86,
        0, 66, 86, 50, 66, 86, 0, 83, 86, 50, 83, 86, 0, 51, 54, 0, 51, 67, 0,
        54, 67, 51, 54, 67, 0, 51, 70, 0, 54, 70, 51, 54, 70, 0, 51, 80, 0, 67,
        80, 51, 67, 80, 0, 51, 82, 0, 70, 82, 51, 70, 82, 51, 80, 82, 0, 54,
        83, 0, 67, 83, 54, 67, 83, 0, 54, 86, 0, 70, 86, 54, 70, 86, 54, 83,
        86, 0, 64, 67, 64, 67, 80, 64, 67, 83, 0, 66, 70, 66, 70, 82, 66, 70,
        86, 1, 49, 52, 1, 49, 65, 1, 52, 65, 49, 52, 65, 1, 49, 68, 1, 52, 68,
        49, 52, 68, 1, 49, 81, 1, 65, 81, 49, 65, 81, 1, 49, 84, 1, 68, 84, 49,
        68, 84, 1, 81, 84, 49, 81, 84, 1, 52, 85, 1, 65, 85, 52, 65, 85, 1, 52,
        87, 1, 68, 87, 52, 68, 87, 1, 85, 87, 52, 85, 87, 1, 53, 55, 1, 53, 69,
        1, 55, 69, 53, 55, 69, 1, 53, 71, 1, 55, 71, 53, 55, 71, 1, 53, 81, 1,
        69, 81, 53, 69, 81, 1, 53, 84, 1, 71, 84, 53, 71, 84, 53, 81, 84, 1,
        55, 85, 1, 69, 85, 55, 69, 85, 1, 55, 87, 1, 71, 87, 55, 71, 87, 55,
        85, 87, 1, 65, 69, 65, 69, 81, 65, 69, 85, 1, 68, 71, 68, 71, 84, 68,
        71, 87, 2, 24, 27, 2, 24, 88, 2, 27, 88, 24, 27, 88, 2, 24, 89, 2, 27,
        89, 24, 27, 89, 2, 24, 56, 2, 24, 57, 2, 56, 57, 24, 56, 57, 2, 56, 88,
        24, 56, 88, 2, 57, 89, 24, 57, 89, 2, 26, 30, 2, 26, 91, 2, 30, 91, 26,
        30, 91, 2, 26, 93, 2, 30, 93, 26, 30, 93, 2, 26, 56, 2, 26, 57, 26, 56,
        57, 2, 56, 91, 26, 56, 91, 2, 57, 93, 26, 57, 93, 2, 27, 59, 2, 27, 61,
        2, 59, 61, 27, 59, 61, 2, 59, 88, 27, 59, 88, 2, 61, 89, 27, 61, 89, 2,
        30, 59, 2, 30, 61, 30, 59, 61, 2, 59, 91, 30, 59, 91, 2, 61, 93, 30,
        61, 93, 2, 88, 91, 56, 88, 91, 2, 89, 93, 57, 89, 93, 59, 88, 91, 61,
        89, 93, 3, 25, 29, 3, 25, 90, 3, 29, 90, 25, 29, 90, 3, 25, 92, 3, 29,
        92, 25, 29, 92, 3, 25, 58, 3, 25, 60, 3, 58, 60, 25, 58, 60, 3, 58, 90,
        25, 58, 90, 3, 60, 92, 25, 60, 92, 3, 28, 31, 3, 28, 94, 3, 31, 94, 28,
        31, 94, 3, 28, 95, 3, 31, 95, 28, 31, 95, 3, 28, 58, 3, 28, 60, 28, 58,
        60, 3, 58, 94, 28, 58, 94, 3, 60, 95, 28, 60, 95, 3, 29, 62, 3, 29, 63,
        3, 62, 63, 29, 62, 63, 3, 62, 90, 29, 62, 90, 3, 63, 92, 29, 63, 92, 3,
        31, 62, 3, 31, 63, 31, 62, 63, 3, 62, 94, 31, 62, 94, 3, 63, 95, 31,
        63, 95, 3, 90, 94, 58, 90, 94, 3, 92, 95, 60, 92, 95, 62, 90, 94, 63,
        92, 95, 4, 16, 17, 4, 16, 32, 4, 17, 32, 16, 17, 32, 4, 16, 33, 4, 17,
        33, 16, 17, 33, 4, 16, 96, 4, 32, 96, 16, 32, 96, 4, 16, 97, 4, 33, 97,
        16, 33, 97, 4, 96, 97, 16, 96, 97, 4, 17, 98, 4, 32, 98, 17, 32, 98, 4,
        17, 100, 4, 33, 100, 17, 33, 100, 4, 98, 100, 17, 98, 100, 4, 19, 21,
        4, 19, 35, 4, 21, 35, 19, 21, 35, 4, 19, 37, 4, 21, 37, 19, 21, 37, 4,
        19, 96, 4, 35, 96, 19, 35, 96, 4, 19, 97, 4, 37, 97, 19, 37, 97, 19,
        96, 97, 4, 21, 98, 4, 35, 98, 21, 35, 98, 4, 21, 100, 4, 37, 100, 21,
        37, 100, 21, 98, 100, 4, 32, 35, 32, 35, 96, 32, 35, 98, 4, 33, 37, 33,
        37, 97, 33, 37, 100, 5, 18, 20, 5, 18, 34, 5, 20, 34, 18, 20, 34, 5,
        18, 36, 5, 20, 36, 18, 20, 36, 5, 18, 99, 5, 34, 99, 18, 34, 99, 5, 18,
        101, 5, 36, 101, 18, 36, 101, 5, 99, 101, 18, 99, 101, 5, 20, 102, 5,
        34, 102, 20, 34, 102, 5, 20, 103, 5, 36, 103, 20, 36, 103, 5, 102, 103,
        20, 102, 103, 5, 22, 23, 5, 22, 38, 5, 23, 38, 22, 23, 38, 5, 22, 39,
        5, 23, 39, 22, 23, 39, 5, 22, 99, 5, 38, 99, 22, 38, 99, 5, 22, 101, 5,
        39, 101, 22, 39, 101, 22, 99, 101, 5, 23, 102, 5, 38, 102, 23, 38, 102,
        5, 23, 103, 5, 39, 103, 23, 39, 103, 23, 102, 103, 5, 34, 38, 34, 38,
        99, 34, 38, 102, 5, 36, 39, 36, 39, 101, 36, 39, 103, 6, 8, 10, 6, 8,
        72, 6, 10, 72, 8, 10, 72, 6, 8, 73, 6, 10, 73, 8, 10, 73, 6, 8, 40, 6,
        8, 41, 6, 40, 41, 8, 40, 41, 6, 40, 72, 8, 40, 72, 6, 41, 73, 8, 41,
        73, 6, 9, 12, 6, 9, 74, 6, 12, 74, 9, 12, 74, 6, 9, 76, 6, 12, 76, 9,
        12, 76, 6, 9, 40, 6, 9, 41, 9, 40, 41, 6, 40, 74, 9, 40, 74, 6, 41, 76,
        9, 41, 76, 6, 10, 42, 6, 10, 44, 6, 42, 44, 10, 42, 44, 6, 42, 72, 10,
        42, 72, 6, 44, 73, 10, 44, 73, 6, 12, 42, 6, 12, 44, 12, 42, 44, 6, 42,
        74, 12, 42, 74, 6, 44, 76, 12, 44, 76, 6, 72, 74, 40, 72, 74, 6, 73,
        76, 41, 73, 76, 42, 72, 74, 44, 73, 76, 7, 11, 14, 7, 11, 75, 7, 14,
        75, 11, 14, 75, 7, 11, 77, 7, 14, 77, 11, 14, 77, 7, 11, 43, 7, 11, 45,
        7, 43, 45, 11, 43, 45, 7, 43, 75, 11, 43, 75, 7, 45, 77, 11, 45, 77, 7,
        13, 15, 7, 13, 78, 7, 15, 78, 13, 15, 78, 7, 13, 79, 7, 15, 79, 13, 15,
        79, 7, 13, 43, 7, 13, 45, 13, 43, 45, 7, 43, 78, 13, 43, 78, 7, 45, 79,
        13, 45, 79, 7, 14, 46, 7, 14, 47, 7, 46, 47, 14, 46, 47, 7, 46, 75, 14,
        46, 75, 7, 47, 77, 14, 47, 77, 7, 15, 46, 7, 15, 47, 15, 46, 47, 7, 46,
        78, 15, 46, 78, 7, 47, 79, 15, 47, 79, 7, 75, 78, 43, 75, 78, 7, 77,
        79, 45, 77, 79, 46, 75, 78, 47, 77, 79, 8, 10, 56, 8, 10, 57, 8, 56,
        57, 10, 56, 57, 8, 56, 72, 10, 56, 72, 8, 57, 73, 10, 57, 73, 8, 16,
        24, 8, 16, 104, 8, 24, 104, 16, 24, 104, 8, 16, 105, 8, 24, 105, 16,
        24, 105, 8, 16, 40, 8, 16, 41, 16, 40, 41, 8, 40, 104, 16, 40, 104, 8,
        41, 105, 16, 41, 105, 8, 24, 56, 8, 24, 57, 8, 56, 104, 24, 56, 104, 8,
        57, 105, 24, 57, 105, 8, 72, 104, 40, 72, 104, 8, 73, 105, 41, 73, 105,
        56, 72, 104, 57, 73, 105, 9, 12, 58, 9, 12, 60, 9, 58, 60, 12, 58, 60,
        9, 58, 74, 12, 58, 74, 9, 60, 76, 12, 60, 76, 9, 17, 25, 9, 17, 106, 9,
        25, 106, 17, 25, 106, 9, 17, 109, 9, 25, 109, 17, 25, 109, 9, 17, 40,
        9, 17, 41, 17, 40, 41, 9, 40, 106, 17, 40, 106, 9, 41, 109, 17, 41,
        109, 9, 25, 58, 9, 25, 60, 9, 58, 106, 25, 58, 106, 9, 60, 109, 25, 60,
        109, 9, 74, 106, 40, 74, 106, 9, 76, 109, 41, 76, 109, 58, 74, 106, 60,
        76, 109, 10, 18, 26, 10, 18, 107, 10, 26, 107, 18, 26, 107, 10, 18,
        110, 10, 26, 110, 18, 26, 110, 10, 18, 42, 10, 18, 44, 18, 42, 44, 10,
        42, 107, 18, 42, 107, 10, 44, 110, 18, 44, 110, 10, 26, 56, 10, 26, 57,
        10, 56, 107, 26, 56, 107, 10, 57, 110, 26, 57, 110, 10, 72, 107, 42,
        72, 107, 10, 73, 110, 44, 73, 110, 56, 72, 107, 57, 73, 110, 11, 14,
        59, 11, 14, 61, 11, 59, 61, 14, 59, 61, 11, 59, 75, 14, 59, 75, 11, 61,
        77, 14, 61, 77, 11, 19, 27, 11, 19, 108, 11, 27, 108, 19, 27, 108, 11,
        19, 111, 11, 27, 111, 19, 27, 111, 11, 19, 43, 11, 19, 45, 19, 43, 45,
        11, 43, 108, 19, 43, 108, 11, 45, 111, 19, 45, 111, 11, 27, 59, 11, 27,
        61, 11, 59, 108, 27, 59, 108, 11, 61, 111, 27, 61, 111, 11, 75, 108,
        43, 75, 108, 11, 77, 111, 45, 77, 111, 59, 75, 108, 61, 77, 111, 12,
        20, 28, 12, 20, 112, 12, 28, 112, 20, 28, 112, 12, 20, 115, 12, 28,
        115, 20, 28, 115, 12, 20, 42, 12, 20, 44, 20, 42, 44, 12, 42, 112, 20,
        42, 112, 12, 44, 115, 20, 44, 115, 12, 28, 58, 12, 28, 60, 12, 58, 112,
        28, 58, 112, 12, 60, 115, 28, 60, 115, 12, 74, 112, 42, 74, 112, 12,
        76, 115, 44, 76, 115, 58, 74, 112, 60, 76, 115, 13, 15, 62, 13, 15, 63,
        13, 62, 63, 15, 62, 63, 13, 62, 78, 15, 62, 78, 13, 63, 79, 15, 63, 79,
        13, 21, 29, 13, 21, 113, 13, 29, 113, 21, 29, 113, 13, 21, 116, 13, 29,
        116, 21, 29, 116, 13, 21, 43, 13, 21, 45, 21, 43, 45, 13, 43, 113, 21,
        43, 113, 13, 45, 116, 21, 45, 116, 13, 29, 62, 13, 29, 63, 13, 62, 113,
        29, 62, 113, 13, 63, 116, 29, 63, 116, 13, 78, 113, 43, 78, 113, 13,
        79, 116, 45, 79, 116, 62, 78, 113, 63, 79, 116, 14, 22, 30, 14, 22,
        114, 14, 30, 114, 22, 30, 114, 14, 22, 117, 14, 30, 117, 22, 30, 117,
        14, 22, 46, 14, 22, 47, 22, 46, 47, 14, 46, 114, 22, 46, 114, 14, 47,
        117, 22, 47, 117, 14, 30, 59, 14, 30, 61, 14, 59, 114, 30, 59, 114, 14,
        61, 117, 30, 61, 117, 14, 75, 114, 46, 75, 114, 14, 77, 117, 47, 77,
        117, 59, 75, 114, 61, 77, 117, 15, 23, 31, 15, 23, 118, 15, 31, 118,
        23, 31, 118, 15, 23, 119, 15, 31, 119, 23, 31, 119, 15, 23, 46, 15, 23,
        47, 23, 46, 47, 15, 46, 118, 23, 46, 118, 15, 47, 119, 23, 47, 119, 15,
        31, 62, 15, 31, 63, 15, 62, 118, 31, 62, 118, 15, 63, 119, 31, 63, 119,
        15, 78, 118, 46, 78, 118, 15, 79, 119, 47, 79, 119, 62, 78, 118, 63,
        79, 119, 16, 17, 40, 16, 32, 40, 17, 32, 40, 16, 17, 41, 16, 33, 41,
        17, 33, 41, 16, 24, 96, 16, 24, 97, 24, 96, 97, 16, 96, 104, 24, 96,
        104, 16, 97, 105, 24, 97, 105, 16, 32, 104, 32, 40, 104, 32, 96, 104,
        16, 33, 105, 33, 41, 105, 33, 97, 105, 17, 25, 98, 17, 25, 100, 25, 98,
        100, 17, 98, 106, 25, 98, 106, 17, 100, 109, 25, 100, 109, 17, 32, 106,
        32, 40, 106, 32, 98, 106, 17, 33, 109, 33, 41, 109, 33, 100, 109, 18,
        20, 42, 18, 34, 42, 20, 34, 42, 18, 20, 44, 18, 36, 44, 20, 36, 44, 18,
        26, 99, 18, 26, 101, 26, 99, 101, 18, 99, 107, 26, 99, 107, 18, 101,
        110, 26, 101, 110, 18, 34, 107, 34, 42, 107, 34, 99, 107, 18, 36, 110,
        36, 44, 110, 36, 101, 110, 19, 21, 43, 19, 35, 43, 21, 35, 43, 19, 21,
        45, 19, 37, 45, 21, 37, 45, 19, 27, 96, 19, 27, 97, 27, 96, 97, 19, 96,
        108, 27, 96, 108, 19, 97, 111, 27, 97, 111, 19, 35, 108, 35, 43, 108,
        35, 96, 108, 19, 37, 111, 37, 45, 111, 37, 97, 111, 20, 28, 102, 20,
        28, 103, 28, 102, 103, 20, 102, 112, 28, 102, 112, 20, 103, 115, 28,
        103, 115, 20, 34, 112, 34, 42, 112, 34, 102, 112, 20, 36, 115, 36, 44,
        115, 36, 103, 115, 21, 29, 98, 21, 29, 100, 29, 98, 100, 21, 98, 113,
        29, 98, 113, 21, 100, 116, 29, 100, 116, 21, 35, 113, 35, 43, 113, 35,
        98, 113, 21, 37, 116, 37, 45, 116, 37, 100, 116, 22, 23, 46, 22, 38,
        46, 23, 38, 46, 22, 23, 47, 22, 39, 47, 23, 39, 47, 22, 30, 99, 22, 30,
        101, 30, 99, 101, 22, 99, 114, 30, 99, 114, 22, 101, 117, 30, 101, 117,
        22, 38, 114, 38, 46, 114, 38, 99, 114, 22, 39, 117, 39, 47, 117, 39,
        101, 117, 23, 31, 102, 23, 31, 103, 31, 102, 103, 23, 102, 118, 31,
        102, 118, 23, 103, 119, 31, 103, 119, 23, 38, 118, 38, 46, 118, 38,
        102, 118, 23, 39, 119, 39, 47, 119, 39, 103, 119, 24, 27, 96, 24, 88,
        96, 27, 88, 96, 24, 27, 97, 24, 89, 97, 27, 89, 97, 24, 88, 104, 56,
        88, 104, 24, 89, 105, 57, 89, 105, 88, 96, 104, 89, 97, 105, 25, 29,
        98, 25, 90, 98, 29, 90, 98, 25, 29, 100, 25, 92, 100, 29, 92, 100, 25,
        90, 106, 58, 90, 106, 25, 92, 109, 60, 92, 109, 90, 98, 106, 92, 100,
        109, 26, 30, 99, 26, 91, 99, 30, 91, 99, 26, 30, 101, 26, 93, 101, 30,
        93, 101, 26, 91, 107, 56, 91, 107, 26, 93, 110, 57, 93, 110, 91, 99,
        107, 93, 101, 110, 27, 88, 108, 59, 88, 108, 27, 89, 111, 61, 89, 111,
        88, 96, 108, 89, 97, 111, 28, 31, 102, 28, 94, 102, 31, 94, 102, 28,
        31, 103, 28, 95, 103, 31, 95, 103, 28, 94, 112, 58, 94, 112, 28, 95,
        115, 60, 95, 115, 94, 102, 112, 95, 103, 115, 29, 90, 113, 62, 90, 113,
        29, 92, 116, 63, 92, 116, 90, 98, 113, 92, 100, 116, 30, 91, 114, 59,
        91, 114, 30, 93, 117, 61, 93, 117, 91, 99, 114, 93, 101, 117, 31, 94,
        118, 62, 94, 118, 31, 95, 119, 63, 95, 119, 94, 102, 118, 95, 103, 119,
        32, 35, 80, 32, 35, 82, 32, 80, 82, 35, 80, 82, 32, 80, 96, 35, 80, 96,
        32, 82, 98, 35, 82, 98, 32, 40, 48, 32, 48, 104, 40, 48, 104, 32, 48,
        106, 40, 48, 106, 32, 48, 80, 32, 48, 82, 32, 80, 104, 48, 80, 104, 32,
        82, 106, 48, 82, 106, 80, 96, 104, 82, 98, 106, 33, 37, 81, 33, 37, 84,
        33, 81, 84, 37, 81, 84, 33, 81, 97, 37, 81, 97, 33, 84, 100, 37, 84,
        100, 33, 41, 49, 33, 49, 105, 41, 49, 105, 33, 49, 109, 41, 49, 109,
        33, 49, 81, 33, 49, 84, 33, 81, 105, 49, 81, 105, 33, 84, 109, 49, 84,
        109, 81, 97, 105, 84, 100, 109, 34, 38, 83, 34, 38, 86, 34, 83, 86, 38,
        83, 86, 34, 83, 99, 38, 83, 99, 34, 86, 102, 38, 86, 102, 34, 42, 50,
        34, 50, 107, 42, 50, 107, 34, 50, 112, 42, 50, 112, 34, 50, 83, 34, 50,
        86, 34, 83, 107, 50, 83, 107, 34, 86, 112, 50, 86, 112, 83, 99, 107,
        86, 102, 112, 35, 43, 51, 35, 51, 108, 43, 51, 108, 35, 51, 113, 43,
        51, 113, 35, 51, 80, 35, 51, 82, 35, 80, 108, 51, 80, 108, 35, 82, 113,
        51, 82, 113, 80, 96, 108, 82, 98, 113, 36, 39, 85, 36, 39, 87, 36, 85,
        87, 39, 85, 87, 36, 85, 101, 39, 85, 101, 36, 87, 103, 39, 87, 103, 36,
        44, 52, 36, 52, 110, 44, 52, 110, 36, 52, 115, 44, 52, 115, 36, 52, 85,
        36, 52, 87, 36, 85, 110, 52, 85, 110, 36, 87, 115, 52, 87, 115, 85,
        101, 110, 87, 103, 115, 37, 45, 53, 37, 53, 111, 45, 53, 111, 37, 53,
        116, 45, 53, 116, 37, 53, 81, 37, 53, 84, 37, 81, 111, 53, 81, 111, 37,
        84, 116, 53, 84, 116, 81, 97, 111, 84, 100, 116, 38, 46, 54, 38, 54,
        114, 46, 54, 114, 38, 54, 118, 46, 54, 118, 38, 54, 83, 38, 54, 86, 38,
        83, 114, 54, 83, 114, 38, 86, 118, 54, 86, 118, 83, 99, 114, 86, 102,
        118, 39, 47, 55, 39, 55, 117, 47, 55, 117, 39, 55, 119, 47, 55, 119,
        39, 55, 85, 39, 55, 87, 39, 85, 117, 55, 85, 117, 39, 87, 119, 55, 87,
        119, 85, 101, 117, 87, 103, 119, 40, 48, 72, 40, 48, 74, 48, 72, 74,
        48, 72, 104, 48, 74, 106, 41, 49, 73, 41, 49, 76, 49, 73, 76, 49, 73,
        105, 49, 76, 109, 42, 50, 72, 42, 50, 74, 50, 72, 74, 50, 72, 107, 50,
        74, 112, 43, 51, 75, 43, 51, 78, 51, 75, 78, 51, 75, 108, 51, 78, 113,
        44, 52, 73, 44, 52, 76, 52, 73, 76, 52, 73, 110, 52, 76, 115, 45, 53,
        77, 45, 53, 79, 53, 77, 79, 53, 77, 111, 53, 79, 116, 46, 54, 75, 46,
        54, 78, 54, 75, 78, 54, 75, 114, 54, 78, 118, 47, 55, 77, 47, 55, 79,
        55, 77, 79, 55, 77, 117, 55, 79, 119, 48, 50, 72, 48, 64, 72, 50, 64,
        72, 48, 50, 74, 48, 66, 74, 50, 66, 74, 48, 64, 104, 64, 72, 104, 64,
        80, 104, 48, 66, 106, 66, 74, 106, 66, 82, 106, 49, 52, 73, 49, 65, 73,
        52, 65, 73, 49, 52, 76, 49, 68, 76, 52, 68, 76, 49, 65, 105, 65, 73,
        105, 65, 81, 105, 49, 68, 109, 68, 76, 109, 68, 84, 109, 50, 64, 107,
        64, 72, 107, 64, 83, 107, 50, 66, 112, 66, 74, 112, 66, 86, 112, 51,
        54, 75, 51, 67, 75, 54, 67, 75, 51, 54, 78, 51, 70, 78, 54, 70, 78, 51,
        67, 108, 67, 75, 108, 67, 80, 108, 51, 70, 113, 70, 78, 113, 70, 82,
        113, 52, 65, 110, 65, 73, 110, 65, 85, 110, 52, 68, 115, 68, 76, 115,
        68, 87, 115, 53, 55, 77, 53, 69, 77, 55, 69, 77, 53, 55, 79, 53, 71,
        79, 55, 71, 79, 53, 69, 111, 69, 77, 111, 69, 81, 111, 53, 71, 116, 71,
        79, 116, 71, 84, 116, 54, 67, 114, 67, 75, 114, 67, 83, 114, 54, 70,
        118, 70, 78, 118, 70, 86, 118, 55, 69, 117, 69, 77, 117, 69, 85, 117,
        55, 71, 119, 71, 79, 119, 71, 87, 119, 56, 64, 72, 56, 64, 104, 56, 64,
        107, 56, 64, 88, 56, 64, 91, 64, 88, 91, 64, 88, 104, 64, 91, 107, 57,
        65, 73, 57, 65, 105, 57, 65, 110, 57, 65, 89, 57, 65, 93, 65, 89, 93,
        65, 89, 105, 65, 93, 110, 58, 66, 74, 58, 66, 106, 58, 66, 112, 58, 66,
        90, 58, 66, 94, 66, 90, 94, 66, 90, 106, 66, 94, 112, 59, 67, 75, 59,
        67, 108, 59, 67, 114, 59, 67, 88, 59, 67, 91, 67, 88, 91, 67, 88, 108,
        67, 91, 114, 60, 68, 76, 60, 68, 109, 60, 68, 115, 60, 68, 92, 60, 68,
        95, 68, 92, 95, 68, 92, 109, 68, 95, 115, 61, 69, 77, 61, 69, 111, 61,
        69, 117, 61, 69, 89, 61, 69, 93, 69, 89, 93, 69, 89, 111, 69, 93, 117,
        62, 70, 78, 62, 70, 113, 62, 70, 118, 62, 70, 90, 62, 70, 94, 70, 90,
        94, 70, 90, 113, 70, 94, 118, 63, 71, 79, 63, 71, 116, 63, 71, 119, 63,
        71, 92, 63, 71, 95, 71, 92, 95, 71, 92, 116, 71, 95, 119, 64, 67, 88,
        64, 80, 88, 67, 80, 88, 64, 67, 91, 64, 83, 91, 67, 83, 91, 80, 88,
        104, 83, 91, 107, 65, 69, 89, 65, 81, 89, 69, 81, 89, 65, 69, 93, 65,
        85, 93, 69, 85, 93, 81, 89, 105, 85, 93, 110, 66, 70, 90, 66, 82, 90,
        70, 82, 90, 66, 70, 94, 66, 86, 94, 70, 86, 94, 82, 90, 106, 86, 94,
        112, 80, 88, 108, 83, 91, 114, 68, 71, 92, 68, 84, 92, 71, 84, 92, 68,
        71, 95, 68, 87, 95, 71, 87, 95, 84, 92, 109, 87, 95, 115, 81, 89, 111,
        85, 93, 117, 82, 90, 113, 86, 94, 118, 84, 92, 116, 87, 95, 119, 80,
        88, 96, 81, 89, 97, 82, 90, 98, 83, 91, 99, 84, 92, 100, 85, 93, 101,
        86, 94, 102, 87, 95, 103,
    ]
}
