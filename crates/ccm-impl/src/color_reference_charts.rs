use crate::color::SRgbColor;

#[rustfmt::skip]
pub const COLOR_REFERENCE_CHARTS: &[(&str, (usize, usize), &[SRgbColor])] = &[
    ("charttu-colorchecker",            ( 6,  4), &CHARTTU_COLORCHECKER           ),
    ("chinese-generic",                 ( 6,  4), &CHINESE_GENERIC                ),
    ("datacolor-spydercheckr",          ( 8,  6), &DATACOLOR_SPYDERCHECKR         ),
    ("xrite-colorchecker-classic-2009", ( 6,  4), &XRITE_COLORCHECKER_CLASSIC_2009),
    ("xrite-colorchecker-classic-2014", ( 6,  4), &XRITE_COLORCHECKER_CLASSIC_2014),
    ("xrite-colorchecker-classic-old",  ( 6,  4), &XRITE_COLORCHECKER_CLASSIC_OLD ),
    ("xrite-colorchecker-sg-2014",      (14, 10), &XRITE_COLORCHECKER_SG_2014     ),
    ("xrite-colorchecker-sg-old",       (14, 10), &XRITE_COLORCHECKER_SG_OLD      ),
];

#[rustfmt::skip]
pub const XRITE_COLORCHECKER_CLASSIC_2009: [SRgbColor; 24] = [
    [115,  82,  68], [194, 150, 130], [ 98, 122, 157], [ 87, 108,  67], [133, 128, 177], [103, 189, 170],
    [214, 126,  44], [ 80,  91, 166], [193,  90,  99], [ 94,  60, 108], [157, 188,  64], [224, 163,  46],
    [ 56,  61, 150], [ 70, 148,  73], [175,  54,  60], [231, 199,  31], [187,  86, 149], [  8, 133, 161],
    [243, 243, 242], [200, 200, 200], [160, 160, 160], [122, 122, 121], [ 85,  85,  85], [ 52,  52,  52],
];

#[rustfmt::skip]
pub const XRITE_COLORCHECKER_CLASSIC_2014: [SRgbColor; 24] = [
    [118,  79,  65], [200, 143, 127], [ 83, 121, 155], [ 95, 108,  64], [125, 128, 174], [ 93, 188, 172],
    [229, 124,  49], [ 45,  91, 167], [200,  81,  95], [ 90,  59, 104], [165, 186,  61], [234, 160,  41],
    [  0,  64, 145], [ 69, 146,  71], [181,  54,  56], [245, 198,  23], [191,  81, 146], [  0, 133, 165],
    [242, 242, 235], [201, 202, 201], [161, 163, 163], [121, 121, 121], [ 83,  84,  85], [ 49,  50,  50],
];

#[rustfmt::skip]
pub const XRITE_COLORCHECKER_CLASSIC_OLD: [SRgbColor; 24] = [
    [118,  81,  67], [202, 147, 129], [ 83, 123, 156], [ 93, 107,  65], [123, 129, 176], [ 90, 190, 172],
    [230, 123,  47], [ 43,  93, 170], [201,  83,  97], [ 90,  59, 105], [167, 188,  65], [237, 161,  41],
    [  0,  66, 147], [ 75, 148,  75], [183,  49,  56], [247, 197,  27], [192,  86, 150], [  0, 137, 170],
    [245, 245, 243], [200, 202, 202], [161, 163, 163], [121, 121, 122], [ 82,  84,  86], [ 48,  49,  51],
];

#[rustfmt::skip]
pub const XRITE_COLORCHECKER_SG_2014: [SRgbColor; 140] = [
    [246, 246, 242], [119, 118, 117], [ 24,  24,  25], [246, 246, 242], [119, 118, 117], [ 24,  24,  25], [246, 246, 242], [119, 118, 117], [ 23,  24,  25], [246, 246, 242], [119, 118, 117], [ 23,  24,  25], [119, 118, 117], [246, 246, 242],
    [ 23,  24,  25], [143,  28,  95], [ 63,  37,  75], [198, 211, 225], [115,  64,  43], [201, 141, 120], [ 68, 112, 151], [ 78,  97,  39], [113, 119, 168], [ 74, 185, 169], [247, 206, 180], [102,  28,  43], [192,  30,  85], [119, 118, 117],
    [119, 118, 117], [181, 132, 181], [ 93,  91, 160], [243, 204, 213], [232, 121,   4], [  0,  81, 166], [199,  65,  79], [ 74,  31,  92], [165, 186,  44], [234, 156,   0], [196, 233, 212], [210,  15,  22], [ 82,  19,  53], [ 24,  24,  25],
    [246, 246, 242], [114,  33, 138], [  0,  52, 103], [166, 222, 212], [  0,  42, 140], [ 57, 143,  55], [182,   0,  24], [245, 197,   0], [193,  68, 142], [  0, 128, 166], [216, 208, 222], [209, 121, 141], [193,   4,  48], [246, 246, 242],
    [ 24,  24,  25], [  0, 129, 202], [ 17, 157, 201], [244, 204, 199], [246, 246, 242], [198, 197, 196], [158, 158, 158], [119, 118, 117], [ 82,  82,  81], [ 38,  39,  39], [172, 216, 225], [220, 123, 120], [239,  54,  45], [119, 118, 117],
    [119, 118, 117], [  0, 163, 194], [  0,  56,  80], [208, 216, 159], [ 23,  24,  25], [ 71,  71,  71], [ 92,  93,  93], [145, 145, 145], [186, 185, 185], [222, 223, 223], [172, 173, 173], [254, 109,   0], [255, 185,   0], [ 24,  24,  25],
    [246, 245, 241], [  0,  58,  59], [ 88, 151, 203], [216, 128,  87], [242, 176, 147], [192, 144, 107], [143,  94,  61], [201, 155, 135], [161,  88,  37], [211, 137, 108], [107, 107, 105], [199, 184,   0], [254, 198,   0], [246, 246, 242],
    [ 23,  23,  24], [  0, 165, 168], [  0, 140, 139], [203, 141, 122], [246, 161, 136], [192, 146, 126], [195, 147, 129], [198, 145, 124], [121,  74,  42], [215, 146, 112], [ 49,  49,  49], [181, 147,  40], [182, 182,   0], [119, 118, 117],
    [119, 118, 117], [ 56,  46,  30], [ 68, 163, 110], [  0, 141,  97], [  0,  57,  38], [  4, 164, 131], [115, 159,  71], [ 29, 141,  39], [ 53, 170,  60], [200, 137,  55], [154, 156,  45], [162, 191,  18], [ 77,  44,  29], [ 23,  24,  25],
    [246, 245, 241], [119, 118, 117], [ 24,  24,  25], [246, 245, 241], [119, 118, 117], [ 23,  24,  25], [246, 246, 242], [119, 118, 117], [ 23,  23,  25], [246, 246, 242], [119, 118, 117], [ 23,  24,  25], [119, 118, 117], [246, 245, 241],
];

#[rustfmt::skip]
pub const XRITE_COLORCHECKER_SG_OLD: [SRgbColor; 140] = [
    [244, 246, 244], [118, 118, 118], [ 20,  20,  21], [244, 246, 244], [118, 118, 118], [ 20,  21,  21], [244, 246, 244], [118, 118, 118], [ 20,  21,  22], [244, 246, 244], [118, 118, 118], [ 21,  21,  22], [118, 118, 118], [244, 245, 243],
    [ 20,  20,  21], [141,  28,  95], [ 64,  44,  80], [198, 212, 225], [111,  63,  41], [199, 139, 120], [ 68, 114, 150], [ 78,  96,  37], [113, 120, 169], [ 80, 185, 168], [245, 205, 181], [ 99,  25,  42], [188,  32,  85], [118, 118, 118],
    [118, 118, 118], [178, 131, 180], [ 92,  91, 159], [239, 202, 212], [229, 115,   0], [  0,  81, 164], [196,  65,  80], [ 72,  34,  92], [164, 183,  39], [233, 154,   0], [196, 235, 213], [204,   4,  23], [ 82,  28,  60], [ 20,  20,  21],
    [244, 245, 244], [108,  30, 129], [  0,  52, 102], [169, 223, 213], [  0,  42, 135], [ 55, 141,  52], [179,   0,  27], [245, 194,   0], [188,  69, 142], [  0, 130, 163], [216, 208, 223], [207, 120, 141], [190,   0,  43], [244, 246, 244],
    [ 20,  20,  21], [  0, 129, 199], [ 16, 157, 200], [243, 204, 200], [244, 245, 244], [197, 197, 197], [158, 158, 158], [118, 118, 118], [ 83,  83,  83], [ 38,  40,  41], [170, 218, 225], [216, 121, 119], [236,  51,  47], [118, 118, 118],
    [118, 118, 118], [  0, 163, 191], [  0,  56,  78], [210, 214, 159], [ 21,  21,  21], [ 72,  72,  73], [ 94,  95,  95], [145, 145, 146], [185, 185, 185], [222, 224, 224], [173, 173, 174], [252, 104,   0], [255, 182,   0], [ 20,  20,  21],
    [244, 245, 244], [  0,  56,  55], [ 86, 152, 204], [216, 126,  86], [239, 174, 148], [193, 144, 108], [141,  92,  59], [200, 154, 134], [159,  86,  40], [209, 135, 108], [108, 108, 108], [198, 180,   0], [254, 195,   0], [244, 245, 244],
    [ 20,  20,  21], [  0, 165, 166], [  0, 142, 136], [202, 142, 123], [242, 157, 136], [192, 146, 126], [195, 144, 127], [197, 145, 125], [120,  73,  42], [212, 144, 111], [ 49,  49,  49], [182, 145,  40], [183, 181,   0], [118, 118, 118],
    [118, 118, 118], [ 59,  50,  39], [ 73, 163, 108], [  0, 140,  93], [ 13,  61,  44], [  9, 164, 128], [117, 159,  70], [ 37, 140,  37], [ 61, 169,  57], [199, 135,  56], [154, 154,  42], [164, 187,   0], [ 75,  40,  25], [ 20,  20,  21],
    [243, 245, 243], [118, 118, 118], [ 20,  21,  21], [244, 245, 244], [118, 118, 118], [ 20,  21,  21], [244, 245, 244], [118, 118, 118], [ 20,  21,  21], [244, 245, 244], [118, 118, 118], [ 20,  21,  21], [118, 118, 118], [244, 245, 243],
];

#[rustfmt::skip]
pub const CHARTTU_COLORCHECKER: [SRgbColor; 24] = [
    [117,  82,  68], [198, 144, 129], [ 93, 121, 157], [ 92, 109,  65], [133, 128, 177], [ 97, 189, 172],
    [230, 129,  55], [ 73,  91, 167], [195,  81,  97], [ 96,  60, 106], [161, 189,  64], [227, 161,  37],
    [ 39,  61, 144], [ 65, 150,  75], [181,  55,  60], [236, 200,  25], [192,  82, 149], [  1, 134, 166],
    [241, 242, 236], [201, 203, 203], [161, 163, 163], [120, 120, 121], [ 86,  86,  87], [ 50,  51,  52],
];

#[rustfmt::skip]
pub const CHINESE_GENERIC: [SRgbColor; 24] = [
    [115,  82,  69], [204, 161, 141], [101, 134, 179], [ 89, 109,  61], [141, 137, 194], [132, 228, 208],
    [249, 118,  35], [ 80,  91, 182], [222,  91, 125], [ 91,  63, 123], [173, 232,  91], [255, 164,  26],
    [ 44,  56, 142], [ 74, 148,  81], [179,  42,  50], [250, 226,  21], [191,  81, 160], [  6, 142, 172],
    [252, 252, 252], [230, 230, 230], [200, 200, 200], [143, 143, 142], [100, 100, 100], [ 50,  50,  50],
];

// sRGB channel values for Patch 2G (Blueprint) are incorrect in the Datacolor docs.
// I fixed it by converting the corresponding Lab color into sRGB for that color patch.
#[rustfmt::skip]
pub const DATACOLOR_SPYDERCHECKR: [SRgbColor; 48] = [
    [210, 121, 117], [218, 203, 201], [237, 206, 186], [241, 233, 229], [249, 242, 238], [  0, 127, 159], [222, 118,  32], [ 98, 187, 166],
    [216, 179,  90], [203, 205, 196], [211, 175, 133], [229, 222, 220], [202, 198, 195], [192,  75, 145], [ 58,  88, 159], [126, 125, 174],
    [127, 175, 120], [206, 203, 208], [193, 149,  91], [182, 178, 176], [161, 157, 154], [245, 205,   0], [195,  79,  95], [ 82, 106,  60],
    [ 66, 157, 179], [ 66,  57,  58], [139,  93,  61], [139, 136, 135], [122, 118, 116], [186,  26,  51], [ 83,  58, 106], [ 87, 120, 155],
    [116, 147, 194], [ 54,  61,  56], [ 74,  55,  46], [100,  99,  97], [ 80,  80,  78], [ 57, 146,  64], [157, 188,  54], [197, 145, 125],
    [190, 121, 154], [ 63,  60,  69], [ 57,  54,  56], [ 63,  61,  62], [ 43,  41,  43], [ 25,  55, 135], [238, 158,  25], [112,  76,  60],
];
