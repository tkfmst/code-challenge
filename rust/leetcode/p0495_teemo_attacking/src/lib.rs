/// 495. Teemo Attacking
/// Easy
///
/// Our hero Teemo is attacking an enemy Ashe with poison attacks! When Teemo attacks Ashe, Ashe gets poisoned for a exactly duration seconds. More formally, an attack at second t will mean Ashe is poisoned during the inclusive time interval [t, t + duration - 1]. If Teemo attacks again before the poison effect ends, the timer for it is reset, and the poison effect will end duration seconds after the new attack.
///
/// You are given a non-decreasing integer array timeSeries, where timeSeries[i] denotes that Teemo attacks Ashe at second timeSeries[i], and an integer duration.
///
/// Return the total number of seconds that Ashe is poisoned.
///
///
/// Example 1:
///
/// Input: timeSeries = [1,4], duration = 2
/// Output: 4
/// Explanation: Teemo's attacks on Ashe go as follows:
/// - At second 1, Teemo attacks, and Ashe is poisoned for seconds 1 and 2.
/// - At second 4, Teemo attacks, and Ashe is poisoned for seconds 4 and 5.
/// Ashe is poisoned for seconds 1, 2, 4, and 5, which is 4 seconds in total.
///
/// Example 2:
///
/// Input: timeSeries = [1,2], duration = 2
/// Output: 3
/// Explanation: Teemo's attacks on Ashe go as follows:
/// - At second 1, Teemo attacks, and Ashe is poisoned for seconds 1 and 2.
/// - At second 2 however, Teemo attacks again and resets the poison timer. Ashe is poisoned for seconds 2 and 3.
/// Ashe is poisoned for seconds 1, 2, and 3, which is 3 seconds in total.
///
///
/// Constraints:
/// * 1 <= timeSeries.length <= 10^4
/// * 0 <= timeSeries[i], duration <= 10^7
/// * timeSeries is sorted in non-decreasing order.

pub struct Solution {}

// use std::collections::HashSet;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut time = 0;
        let mut cursol = -1;

        for t in time_series {
            if cursol >= t {
                time += duration - (cursol - t) - 1;
            } else {
                time += duration
            }
            cursol = t + duration - 1;
        }

        time
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_poisoned_duration_1() {
        let time_series = vec![1, 4];
        let duration = 2;
        let output = 4;
        assert_eq!(
            Solution::find_poisoned_duration(time_series, duration),
            output
        );
    }

    #[test]
    fn test_find_poisoned_duration_2() {
        let time_series = vec![1, 2];
        let duration = 2;
        let output = 3;
        assert_eq!(
            Solution::find_poisoned_duration(time_series, duration),
            output
        );
    }

    #[test]
    fn test_find_poisoned_duration_3() {
        let time_series = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let duration = 100000;
        let output = 100008;
        assert_eq!(
            Solution::find_poisoned_duration(time_series, duration),
            output
        );
    }

    #[test]
    fn test_find_poisoned_duration_4() {
        let time_series = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
            109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
            126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
            143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
            160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
            177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193,
            194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
            211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227,
            228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
            245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261,
            262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278,
            279, 280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295,
            296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312,
            313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 328, 329,
            330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346,
            347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363,
            364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380,
            381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397,
            398, 399, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414,
            415, 416, 417, 418, 419, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431,
            432, 433, 434, 435, 436, 437, 438, 439, 440, 441, 442, 443, 444, 445, 446, 447, 448,
            449, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465,
            466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 480, 481, 482,
            483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499,
            500,
        ];
        let duration = 1;
        let output = 501;
        assert_eq!(
            Solution::find_poisoned_duration(time_series, duration),
            output
        );
    }
}
