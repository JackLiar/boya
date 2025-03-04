use crate::M;
pub const GAP1: f64 = 0.0012;
pub const GAP2: f64 = 0.0006;
pub const GAP3: f64 = 0.0392;

#[rustfmt::skip]
pub const FG: [[[f64; 10]; 4]; 2] = [
    [
        [ 0.2570, 0.2780, 0.2800, 0.2736, 0.2757, 0.2764, 0.2675, 0.2678, 0.2779, 0.2647 ],
        [ 0.2142, 0.2194, 0.2331, 0.2230, 0.2272, 0.2252, 0.2148, 0.2123, 0.2115, 0.2096 ],
        [ 0.1670, 0.1523, 0.1567, 0.1580, 0.1601, 0.1569, 0.1589, 0.1555, 0.1474, 0.1571 ],
        [ 0.1238, 0.0925, 0.0798, 0.0923, 0.0890, 0.0828, 0.1010, 0.0988, 0.0872, 0.1060 ],
    ],
    [
        [ 0.2360, 0.2405, 0.2499, 0.2495, 0.2517, 0.2591, 0.2636, 0.2625, 0.2551, 0.2310 ],
        [ 0.1285, 0.0925, 0.0779, 0.1060, 0.1183, 0.1176, 0.1277, 0.1268, 0.1193, 0.1211 ],
        [ 0.0981, 0.0589, 0.0401, 0.0654, 0.0761, 0.0728, 0.0841, 0.0826, 0.0776, 0.0891 ],
        [ 0.0923, 0.0486, 0.0287, 0.0498, 0.0526, 0.0482, 0.0621, 0.0636, 0.0584, 0.0794 ],
    ],
];

#[rustfmt::skip]
pub const FG_SUM: [[f64; 10]; 2] = [
    [
        0.2380000054836, 0.2578000128269, 0.2504000067711, 0.2531000375748, 0.2480000108480,
        0.2587000429630, 0.2577999532223, 0.2656000256538, 0.2760000228882, 0.2625999450684
    ],
    [
        0.4451000094414, 0.5595000386238, 0.6034000515938, 0.5292999744415, 0.5012999176979,
        0.5023000240326, 0.4625000357628, 0.4645000100136, 0.4895999729633, 0.4793999791145
    ]
];

#[rustfmt::skip]
pub const FG_SUM_INV:[[f64; M];2] = [
    [
        4.2016806602478, 3.8789758682251, 3.9936101436615, 3.9510068893433, 4.0322580337524,
        3.8654806613922, 3.8789765834808, 3.7650599479675, 3.6231880187988, 3.8080739974976
    ],
    [
        2.2466859817505, 1.7873100042343, 1.6572753190994, 1.8892878293991, 1.9948137998581,
        1.9908419847488, 2.1621620655060, 2.1528525352478, 2.0424838066101, 2.0859408378601
    ]
];

pub const L_LIMIT: f64 = 0.005;
pub const M_LIMIT: f64 = 3.135;

#[allow(clippy::approx_constant)]
#[rustfmt::skip]
pub const LSPCB1: [[f64; 10]; 128] = [
    [ 0.1814, 0.2647, 0.4580, 1.1077, 1.4813, 1.7022, 2.1953, 2.3405, 2.5867, 2.6636 ],
    [ 0.2113, 0.3223, 0.4212, 0.5946, 0.7479, 0.9615, 1.9097, 2.1750, 2.4773, 2.6737 ],
    [ 0.1915, 0.2755, 0.3770, 0.5950, 1.3505, 1.6349, 2.2348, 2.3552, 2.5768, 2.6540 ],
    [ 0.2116, 0.3067, 0.4099, 0.5748, 0.8518, 1.2569, 2.0782, 2.1920, 2.3371, 2.4842 ],
    [ 0.2129, 0.2974, 0.4039, 1.0659, 1.2735, 1.4658, 1.9061, 2.0312, 2.6074, 2.6750 ],
    [ 0.2181, 0.2893, 0.4117, 0.5519, 0.8295, 1.5825, 2.1575, 2.3179, 2.5458, 2.6417 ],
    [ 0.1991, 0.2971, 0.4104, 0.7725, 1.3073, 1.4665, 1.6208, 1.6973, 2.3732, 2.5743 ],
    [ 0.1818, 0.2886, 0.4018, 0.7630, 1.1264, 1.2699, 1.6899, 1.8650, 2.1633, 2.6186 ],
    [ 0.2282, 0.3093, 0.4243, 0.5329, 1.1173, 1.7717, 1.9420, 2.0780, 2.5160, 2.6137 ],
    [ 0.2528, 0.3693, 0.5290, 0.7146, 0.9528, 1.1269, 1.2936, 1.9589, 2.4548, 2.6653 ],
    [ 0.2332, 0.3263, 0.4174, 0.5202, 1.3633, 1.8447, 2.0236, 2.1474, 2.3572, 2.4738 ],
    [ 0.1393, 0.2216, 0.3204, 0.5644, 0.7929, 1.1705, 1.7051, 2.0054, 2.3623, 2.5985 ],
    [ 0.2677, 0.3871, 0.5746, 0.7091, 1.3311, 1.5260, 1.7288, 1.9122, 2.5787, 2.6598 ],
    [ 0.1570, 0.2328, 0.3111, 0.4216, 1.1688, 1.4605, 1.9505, 2.1173, 2.4038, 2.7460 ],
    [ 0.2346, 0.3321, 0.5621, 0.8160, 1.4042, 1.5860, 1.7518, 1.8631, 2.0749, 2.5380 ],
    [ 0.2505, 0.3368, 0.4758, 0.6405, 0.8104, 1.2533, 1.9329, 2.0526, 2.2155, 2.6459 ],
    [ 0.2196, 0.3049, 0.6857, 1.3976, 1.6100, 1.7958, 2.0813, 2.2211, 2.4789, 2.5857 ],
    [ 0.1232, 0.2011, 0.3527, 0.6969, 1.1647, 1.5081, 1.8593, 2.2576, 2.5594, 2.6896 ],
    [ 0.3682, 0.4632, 0.6600, 0.9118, 1.5245, 1.7071, 1.8712, 1.9939, 2.4356, 2.5380 ],
    [ 0.2690, 0.3711, 0.4635, 0.6644, 1.4633, 1.6495, 1.8227, 1.9983, 2.1797, 2.2954 ],
    [ 0.3555, 0.5240, 0.9751, 1.1685, 1.4114, 1.6168, 1.7769, 2.0178, 2.4420, 2.5724 ],
    [ 0.3493, 0.4404, 0.7231, 0.8587, 1.1272, 1.4715, 1.6760, 2.2042, 2.4735, 2.5604 ],
    [ 0.3747, 0.5263, 0.7284, 0.8994, 1.4017, 1.5502, 1.7468, 1.9816, 2.2380, 2.3404 ],
    [ 0.2972, 0.4470, 0.5941, 0.7078, 1.2675, 1.4310, 1.5930, 1.9126, 2.3026, 2.4208 ],
    [ 0.2467, 0.3180, 0.4712, 1.1281, 1.6206, 1.7876, 1.9544, 2.0873, 2.3521, 2.4721 ],
    [ 0.2292, 0.3430, 0.4383, 0.5747, 1.3497, 1.5187, 1.9070, 2.0958, 2.2902, 2.4301 ],
    [ 0.2573, 0.3508, 0.4484, 0.7079, 1.6577, 1.7929, 1.9456, 2.0847, 2.3060, 2.4208 ],
    [ 0.1968, 0.2789, 0.3594, 0.4361, 1.0034, 1.7040, 1.9439, 2.1044, 2.2696, 2.4558 ],
    [ 0.2955, 0.3853, 0.7986, 1.2470, 1.4723, 1.6522, 1.8684, 2.0084, 2.2849, 2.4268 ],
    [ 0.2036, 0.3189, 0.4314, 0.6393, 1.2834, 1.4278, 1.5796, 2.0506, 2.2044, 2.3656 ],
    [ 0.2916, 0.3684, 0.5907, 1.1394, 1.3933, 1.5540, 1.8341, 1.9835, 2.1301, 2.2800 ],
    [ 0.2289, 0.3402, 0.5166, 0.7716, 1.0614, 1.2389, 1.4386, 2.0769, 2.2715, 2.4366 ],
    [ 0.0829, 0.1723, 0.5682, 0.9773, 1.3973, 1.6174, 1.9242, 2.2128, 2.4855, 2.6327 ],
    [ 0.2244, 0.3169, 0.4368, 0.5625, 0.6897, 1.3763, 1.7524, 1.9393, 2.5121, 2.6556 ],
    [ 0.1591, 0.2387, 0.2924, 0.4056, 1.4677, 1.6802, 1.9389, 2.2067, 2.4635, 2.5919 ],
    [ 0.1756, 0.2566, 0.3251, 0.4227, 1.0167, 1.2649, 1.6801, 2.1055, 2.4088, 2.7276 ],
    [ 0.1050, 0.2325, 0.7445, 0.9491, 1.1982, 1.4658, 1.8093, 2.0397, 2.4155, 2.5797 ],
    [ 0.2043, 0.3324, 0.4522, 0.7477, 0.9361, 1.1533, 1.6703, 1.7631, 2.5071, 2.6528 ],
    [ 0.1522, 0.2258, 0.3543, 0.5504, 0.8815, 1.5516, 1.8110, 1.9915, 2.3603, 2.7735 ],
    [ 0.1862, 0.2759, 0.4715, 0.6908, 0.8963, 1.4341, 1.6322, 1.7630, 2.2027, 2.6043 ],
    [ 0.1460, 0.2254, 0.3790, 0.8622, 1.3394, 1.5754, 1.8084, 2.0798, 2.4319, 2.7632 ],
    [ 0.2621, 0.3792, 0.5463, 0.7948, 1.0043, 1.1921, 1.3409, 1.4845, 2.3159, 2.6002 ],
    [ 0.1935, 0.2937, 0.3656, 0.4927, 1.4015, 1.6086, 1.7724, 1.8837, 2.4374, 2.5971 ],
    [ 0.2171, 0.3282, 0.4412, 0.5713, 1.1554, 1.3506, 1.5227, 1.9923, 2.4100, 2.5391 ],
    [ 0.2274, 0.3157, 0.4263, 0.8202, 1.4293, 1.5884, 1.7535, 1.9688, 2.3939, 2.4934 ],
    [ 0.1704, 0.2633, 0.3259, 0.4134, 1.2948, 1.4802, 1.6619, 2.0393, 2.3165, 2.6083 ],
    [ 0.1763, 0.2585, 0.4012, 0.7609, 1.1503, 1.5847, 1.8309, 1.9352, 2.0982, 2.6681 ],
    [ 0.2447, 0.3535, 0.4618, 0.5979, 0.7530, 0.8908, 1.5393, 2.0075, 2.3557, 2.6203 ],
    [ 0.1826, 0.3496, 0.7764, 0.9888, 1.3915, 1.7421, 1.9412, 2.1620, 2.4999, 2.6931 ],
    [ 0.3033, 0.3802, 0.6981, 0.8664, 1.0254, 1.5401, 1.7180, 1.8124, 2.5068, 2.6119 ],
    [ 0.2960, 0.4001, 0.6465, 0.7672, 1.3782, 1.5751, 1.9559, 2.1373, 2.3601, 2.4760 ],
    [ 0.3132, 0.4613, 0.6544, 0.8532, 1.0721, 1.2730, 1.7566, 1.9217, 2.1693, 2.6531 ],
    [ 0.3329, 0.4131, 0.8073, 1.1297, 1.2869, 1.4937, 1.7885, 1.9150, 2.4505, 2.5760 ],
    [ 0.2340, 0.3605, 0.7659, 0.9874, 1.1854, 1.3337, 1.5128, 2.0062, 2.4427, 2.5859 ],
    [ 0.4131, 0.5330, 0.6530, 0.9360, 1.3648, 1.5388, 1.6994, 1.8707, 2.4294, 2.5335 ],
    [ 0.3754, 0.5229, 0.7265, 0.9301, 1.1724, 1.3440, 1.5118, 1.7098, 2.5218, 2.6242 ],
    [ 0.2138, 0.2998, 0.6283, 1.2166, 1.4187, 1.6084, 1.7992, 2.0106, 2.5377, 2.6558 ],
    [ 0.1761, 0.2672, 0.4065, 0.8317, 1.0900, 1.4814, 1.7672, 1.8685, 2.3969, 2.5079 ],
    [ 0.2801, 0.3535, 0.4969, 0.9809, 1.4934, 1.6378, 1.8021, 2.1200, 2.3135, 2.4034 ],
    [ 0.2365, 0.3246, 0.5618, 0.8176, 1.1073, 1.5702, 1.7331, 1.8592, 1.9589, 2.3044 ],
    [ 0.2529, 0.3251, 0.5147, 1.1530, 1.3291, 1.5005, 1.7028, 1.8200, 2.3482, 2.4831 ],
    [ 0.2125, 0.3041, 0.4259, 0.9935, 1.1788, 1.3615, 1.6121, 1.7930, 2.5509, 2.6742 ],
    [ 0.2685, 0.3518, 0.5707, 1.0410, 1.2270, 1.3927, 1.7622, 1.8876, 2.0985, 2.5144 ],
    [ 0.2373, 0.3648, 0.5099, 0.7373, 0.9129, 1.0421, 1.7312, 1.8984, 2.1512, 2.6342 ],
    [ 0.2229, 0.3876, 0.8621, 1.1986, 1.5655, 1.8861, 2.2376, 2.4239, 2.6648, 2.7359 ],
    [ 0.3009, 0.3719, 0.5887, 0.7297, 0.9395, 1.8797, 2.0423, 2.1541, 2.5132, 2.6026 ],
    [ 0.3114, 0.4142, 0.6476, 0.8448, 1.2495, 1.7192, 2.2148, 2.3432, 2.5246, 2.6046 ],
    [ 0.3666, 0.4638, 0.6496, 0.7858, 0.9667, 1.4213, 1.9300, 2.0564, 2.2119, 2.3170 ],
    [ 0.4218, 0.5075, 0.8348, 1.0009, 1.2057, 1.5032, 1.9416, 2.0540, 2.4352, 2.5504 ],
    [ 0.3726, 0.4602, 0.5971, 0.7093, 0.8517, 1.2361, 1.8052, 1.9520, 2.4137, 2.5518 ],
    [ 0.4482, 0.5318, 0.7114, 0.8542, 1.0328, 1.4751, 1.7278, 1.8237, 2.3496, 2.4931 ],
    [ 0.3316, 0.4498, 0.6404, 0.8162, 1.0332, 1.2209, 1.5130, 1.7250, 1.9715, 2.4141 ],
    [ 0.2375, 0.3221, 0.5042, 0.9760, 1.7503, 1.9014, 2.0822, 2.2225, 2.4689, 2.5632 ],
    [ 0.2813, 0.3575, 0.5032, 0.5889, 0.6885, 1.6040, 1.9318, 2.0677, 2.4546, 2.5701 ],
    [ 0.2198, 0.3072, 0.4090, 0.6371, 1.6365, 1.9468, 2.1507, 2.2633, 2.5063, 2.5943 ],
    [ 0.1754, 0.2716, 0.3361, 0.5550, 1.1789, 1.3728, 1.8527, 1.9919, 2.1349, 2.3359 ],
    [ 0.2832, 0.3540, 0.6080, 0.8467, 1.0259, 1.6467, 1.8987, 1.9875, 2.4744, 2.5527 ],
    [ 0.2670, 0.3564, 0.5628, 0.7172, 0.9021, 1.5328, 1.7131, 2.0501, 2.5633, 2.6574 ],
    [ 0.2729, 0.3569, 0.6252, 0.7641, 0.9887, 1.6589, 1.8726, 1.9947, 2.1884, 2.4609 ],
    [ 0.2155, 0.3221, 0.4580, 0.6995, 0.9623, 1.2339, 1.6642, 1.8823, 2.0518, 2.2674 ],
    [ 0.4224, 0.7009, 1.1714, 1.4334, 1.7595, 1.9629, 2.2185, 2.3304, 2.5446, 2.6369 ],
    [ 0.4560, 0.5403, 0.7568, 0.8989, 1.1292, 1.7687, 1.9575, 2.0784, 2.4260, 2.5484 ],
    [ 0.4299, 0.5833, 0.8408, 1.0596, 1.5524, 1.7484, 1.9471, 2.2034, 2.4617, 2.5812 ],
    [ 0.2614, 0.3624, 0.8381, 0.9829, 1.2220, 1.6064, 1.8083, 1.9362, 2.1397, 2.2773 ],
    [ 0.5064, 0.7481, 1.1021, 1.3271, 1.5486, 1.7096, 1.9503, 2.1006, 2.3911, 2.5141 ],
    [ 0.5375, 0.6552, 0.8099, 1.0219, 1.2407, 1.4160, 1.8266, 1.9936, 2.1951, 2.2911 ],
    [ 0.4994, 0.6575, 0.8365, 1.0706, 1.4116, 1.6224, 1.9200, 2.0667, 2.3262, 2.4539 ],
    [ 0.3353, 0.4426, 0.6469, 0.9161, 1.2528, 1.3956, 1.6080, 1.8909, 2.0600, 2.1380 ],
    [ 0.2745, 0.4341, 1.0424, 1.2928, 1.5461, 1.7940, 2.0161, 2.1758, 2.4742, 2.5937 ],
    [ 0.1562, 0.2393, 0.4786, 0.9513, 1.2395, 1.8010, 2.0320, 2.2143, 2.5243, 2.6204 ],
    [ 0.2979, 0.4242, 0.8224, 1.0564, 1.4881, 1.7808, 2.0898, 2.1882, 2.3328, 2.4389 ],
    [ 0.2294, 0.3070, 0.5490, 0.9244, 1.2229, 1.8248, 1.9704, 2.0627, 2.2458, 2.3653 ],
    [ 0.3423, 0.4502, 0.9144, 1.2313, 1.3694, 1.5517, 1.9907, 2.1326, 2.4509, 2.5789 ],
    [ 0.2470, 0.3275, 0.4729, 1.0093, 1.2519, 1.4216, 1.8540, 2.0877, 2.3151, 2.4156 ],
    [ 0.3447, 0.4401, 0.7099, 1.0493, 1.2312, 1.4001, 2.0225, 2.1317, 2.2894, 2.4263 ],
    [ 0.3481, 0.4494, 0.6446, 0.9336, 1.1198, 1.2620, 1.8264, 1.9712, 2.1435, 2.2552 ],
    [ 0.1646, 0.3229, 0.7112, 1.0725, 1.2964, 1.5663, 1.9843, 2.2363, 2.5798, 2.7572 ],
    [ 0.2614, 0.3707, 0.5241, 0.7425, 0.9269, 1.2976, 2.0945, 2.2014, 2.6204, 2.6959 ],
    [ 0.1963, 0.2900, 0.4131, 0.8397, 1.2171, 1.3705, 2.0665, 2.1546, 2.4640, 2.5782 ],
    [ 0.3387, 0.4415, 0.6121, 0.8005, 0.9507, 1.0937, 2.0836, 2.2342, 2.3849, 2.5076 ],
    [ 0.2362, 0.5876, 0.7574, 0.8804, 1.0961, 1.4240, 1.9519, 2.1742, 2.4935, 2.6493 ],
    [ 0.2793, 0.4282, 0.6149, 0.8352, 1.0106, 1.1766, 1.8392, 2.0119, 2.6433, 2.7117 ],
    [ 0.3603, 0.4604, 0.5955, 0.9251, 1.1006, 1.2572, 1.7688, 1.8607, 2.4687, 2.5623 ],
    [ 0.3975, 0.5849, 0.8059, 0.9182, 1.0552, 1.1850, 1.6356, 1.9627, 2.3318, 2.4719 ],
    [ 0.2231, 0.3192, 0.4256, 0.7373, 1.4831, 1.6874, 1.9765, 2.1097, 2.6152, 2.6906 ],
    [ 0.1221, 0.2081, 0.3665, 0.7734, 1.0341, 1.2818, 1.8162, 2.0727, 2.4446, 2.7377 ],
    [ 0.2010, 0.2791, 0.3796, 0.8845, 1.4030, 1.5615, 2.0538, 2.1567, 2.3171, 2.4686 ],
    [ 0.2086, 0.3053, 0.4047, 0.8224, 1.0656, 1.2115, 1.9641, 2.0871, 2.2430, 2.4313 ],
    [ 0.3203, 0.4285, 0.5467, 0.6891, 1.2039, 1.3569, 1.8578, 2.2055, 2.3906, 2.4881 ],
    [ 0.3074, 0.4192, 0.5772, 0.7799, 0.9866, 1.1335, 1.6068, 2.2441, 2.4194, 2.5089 ],
    [ 0.2108, 0.2910, 0.4993, 0.7695, 0.9528, 1.5681, 1.7838, 2.1495, 2.3522, 2.4636 ],
    [ 0.3492, 0.4560, 0.5906, 0.7379, 0.8855, 1.0257, 1.7128, 1.9997, 2.2019, 2.3694 ],
    [ 0.5185, 0.7316, 0.9708, 1.1954, 1.5066, 1.7887, 2.1396, 2.2918, 2.5429, 2.6489 ],
    [ 0.4276, 0.4946, 0.6934, 0.8308, 0.9944, 1.4582, 2.0324, 2.1294, 2.4891, 2.6324 ],
    [ 0.3847, 0.5973, 0.7202, 0.8787, 1.3938, 1.5959, 1.8463, 2.1574, 2.5050, 2.6687 ],
    [ 0.4835, 0.5919, 0.7235, 0.8862, 1.0756, 1.2853, 1.9118, 2.0215, 2.2213, 2.4638 ],
    [ 0.5492, 0.8062, 0.9810, 1.1293, 1.3189, 1.5415, 1.9385, 2.1378, 2.4439, 2.5691 ],
    [ 0.5190, 0.6764, 0.8123, 1.0154, 1.2085, 1.4266, 1.8433, 2.0866, 2.5113, 2.6474 ],
    [ 0.4602, 0.6503, 0.9602, 1.1427, 1.3043, 1.4427, 1.6676, 1.8758, 2.2868, 2.4271 ],
    [ 0.3764, 0.4845, 0.7627, 0.9914, 1.1961, 1.3421, 1.5129, 1.6707, 2.1836, 2.3322 ],
    [ 0.3334, 0.5701, 0.8622, 1.1232, 1.3851, 1.6767, 2.0600, 2.2946, 2.5375, 2.7295 ],
    [ 0.1449, 0.2719, 0.5783, 0.8807, 1.1746, 1.5422, 1.8804, 2.1934, 2.4734, 2.8728 ],
    [ 0.2333, 0.3024, 0.4780, 1.2327, 1.4180, 1.5815, 1.9804, 2.0921, 2.3524, 2.5304 ],
    [ 0.2154, 0.3075, 0.4746, 0.8477, 1.1170, 1.5369, 1.9847, 2.0733, 2.1880, 2.2504 ],
    [ 0.1709, 0.4486, 0.8705, 1.0643, 1.3047, 1.5269, 1.9175, 2.1621, 2.4073, 2.5718 ],
    [ 0.2835, 0.3752, 0.5234, 0.9898, 1.1484, 1.2974, 1.9363, 2.0378, 2.4065, 2.6214 ],
    [ 0.3211, 0.4077, 0.5809, 1.0206, 1.2542, 1.3835, 1.5723, 2.1209, 2.3464, 2.4336 ],
    [ 0.2101, 0.3146, 0.6779, 0.8783, 1.0561, 1.3045, 1.8395, 2.0695, 2.2831, 2.4328 ],
];

#[rustfmt::skip]
pub const LSPCB2: [[f64; 10]; 32] = [
    [ -0.0532, -0.0995, -0.0906,  0.1261, -0.0633,  0.0711, -0.1467,  0.1012,  0.0106,  0.0470 ],
    [ -0.1017, -0.1088,  0.0566, -0.0010, -0.1528,  0.1771,  0.0089, -0.0282,  0.1055,  0.0808 ],
    [ -0.1247,  0.0283, -0.0374,  0.0393, -0.0269, -0.0200, -0.0643, -0.0921, -0.1994,  0.0327 ],
    [  0.0070, -0.0242, -0.0415, -0.0041, -0.1793,  0.0700,  0.0972, -0.0207, -0.0771,  0.0997 ],
    [  0.0209, -0.0428,  0.0359,  0.2027,  0.0554,  0.0634,  0.0356,  0.0195, -0.0782, -0.1583 ],
    [ -0.0856, -0.1028, -0.0071,  0.1160,  0.1089,  0.1892,  0.0874,  0.0644, -0.0872, -0.0236 ],
    [  0.0713,  0.0039, -0.0353,  0.0435, -0.0407, -0.0558,  0.0748, -0.0346, -0.1686, -0.0905 ],
    [ -0.0134, -0.0987,  0.0283,  0.0095, -0.0107, -0.0420,  0.1638,  0.1328, -0.0799, -0.0695 ],
    [ -0.1049,  0.1510,  0.0672,  0.1043,  0.0872, -0.0663, -0.2139, -0.0239, -0.0120, -0.0338 ],
    [ -0.1071, -0.1165, -0.1524, -0.0365,  0.0260, -0.0288, -0.0889,  0.1159,  0.1852,  0.1093 ],
    [ -0.0094,  0.0420, -0.0758,  0.0932,  0.0505,  0.0614, -0.0443, -0.1172, -0.0590,  0.1693 ],
    [ -0.0384, -0.0375, -0.0313, -0.1539, -0.0524,  0.0550, -0.0569, -0.0133,  0.1233,  0.2714 ],
    [  0.0869,  0.0847,  0.0637,  0.0794,  0.1594, -0.0035, -0.0462,  0.0909, -0.1227,  0.0294 ],
    [ -0.0137, -0.0332, -0.0611,  0.1156,  0.2116,  0.0332, -0.0019,  0.1110, -0.0317,  0.2061 ],
    [  0.0703, -0.0013, -0.0572, -0.0243,  0.1345, -0.1235,  0.0710, -0.0065, -0.0912,  0.1072 ],
    [  0.0178, -0.0349, -0.1563, -0.0487,  0.0044, -0.0609, -0.1682,  0.0023, -0.0542,  0.1811 ],
    [ -0.1384, -0.1020,  0.1649,  0.1568, -0.0116,  0.1240, -0.0271,  0.0541,  0.0455, -0.0433 ],
    [ -0.1782, -0.1511,  0.0509, -0.0261,  0.0570,  0.0817,  0.0805,  0.2003,  0.1138,  0.0653 ],
    [ -0.0019,  0.0081,  0.0572,  0.1245, -0.0914,  0.1691, -0.0223, -0.1108, -0.0881, -0.0320 ],
    [ -0.0413,  0.0181,  0.1764,  0.0092, -0.0928,  0.0695,  0.1523,  0.0412,  0.0508, -0.0148 ],
    [  0.0476,  0.0292,  0.1915,  0.1198,  0.0139,  0.0451, -0.1225, -0.0619, -0.0717, -0.1104 ],
    [ -0.0382, -0.0120,  0.1159,  0.0039,  0.1348,  0.0088, -0.0173,  0.1789,  0.0078, -0.0959 ],
    [  0.1376,  0.0713,  0.1020,  0.0339, -0.1415,  0.0254,  0.0368, -0.1077,  0.0143, -0.0494 ],
    [  0.0658, -0.0140,  0.1046, -0.0603,  0.0273, -0.1114,  0.0761, -0.0093,  0.0338, -0.0538 ],
    [  0.2683,  0.2853,  0.1549,  0.0819,  0.0372, -0.0327, -0.0642,  0.0172,  0.1077, -0.0170 ],
    [ -0.1949,  0.0672,  0.0978, -0.0557, -0.0069, -0.0851,  0.1057,  0.1294,  0.0505,  0.0545 ],
    [  0.1409,  0.0724, -0.0094,  0.1511, -0.0039,  0.0710, -0.1266, -0.1093,  0.0817,  0.0363 ],
    [  0.0485,  0.0682,  0.0248, -0.0974, -0.1122,  0.0004,  0.0845, -0.0357,  0.1282,  0.0955 ],
    [  0.0408,  0.1801,  0.0772, -0.0098,  0.0059, -0.1296, -0.0591,  0.0443, -0.0729, -0.1041 ],
    [ -0.0666, -0.0403, -0.0524, -0.0831,  0.1384, -0.1443, -0.0909,  0.1636,  0.0320,  0.0077 ],
    [  0.1612,  0.1010, -0.0486, -0.0704,  0.0417, -0.0945, -0.0590, -0.1523, -0.0086,  0.0120 ],
    [ -0.0199,  0.0823, -0.0014, -0.1082,  0.0649, -0.1374, -0.0324, -0.0296,  0.0885,  0.1141 ],
];

#[rustfmt::skip]
pub const INIT_FREQ_PREV: [f64; M] = [
    0.285599, 0.571199, 0.856798, 1.142397, 1.427997,
    1.713596, 1.999195, 2.284795, 2.570394, 2.855993,
];

#[rustfmt::skip]
pub const INIT_LSP_OLD: [f64; M] = [
    0.9595,   0.8413,  0.6549,  0.4154,  0.1423,
    -0.1423, -0.4154, -0.6549, -0.8413, -0.9595
];

// filter coefficients (fc = 100 Hz )
pub const B100: [f64; 3] = [0.93980581E+00, -0.18795834E+01, 0.93980581E+00];
pub const A100: [f64; 3] = [1.00000000E+00, 0.19330735E+01, -0.93589199E+00];
