/// See https://github.com/unicode-org/icu/blob/d1dcb6931884dcf4b8b9a88fa17d19159a95a04c/icu4c/source/common/patternprops.cpp#L119
pub fn is_pattern_syntax(c: char) -> bool {
  PATTERN_SYNTAX_CODE_POINTS.binary_search(&(c as u32)).is_ok()
}

/// See https://github.com/node-unicode/unicode-13.0.0/blob/dbbbf9d0b97b5181cdad6c928dec838df387b794/Binary_Property/Pattern_Syntax/code-points.js
const PATTERN_SYNTAX_CODE_POINTS: &[u32] = &[
  33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58, 59, 60, 61,
  62, 63, 64, 91, 92, 93, 94, 96, 123, 124, 125, 126, 161, 162, 163, 164, 165,
  166, 167, 169, 171, 172, 174, 176, 177, 182, 187, 191, 215, 247, 8208, 8209,
  8210, 8211, 8212, 8213, 8214, 8215, 8216, 8217, 8218, 8219, 8220, 8221, 8222,
  8223, 8224, 8225, 8226, 8227, 8228, 8229, 8230, 8231, 8240, 8241, 8242, 8243,
  8244, 8245, 8246, 8247, 8248, 8249, 8250, 8251, 8252, 8253, 8254, 8257, 8258,
  8259, 8260, 8261, 8262, 8263, 8264, 8265, 8266, 8267, 8268, 8269, 8270, 8271,
  8272, 8273, 8274, 8275, 8277, 8278, 8279, 8280, 8281, 8282, 8283, 8284, 8285,
  8286, 8592, 8593, 8594, 8595, 8596, 8597, 8598, 8599, 8600, 8601, 8602, 8603,
  8604, 8605, 8606, 8607, 8608, 8609, 8610, 8611, 8612, 8613, 8614, 8615, 8616,
  8617, 8618, 8619, 8620, 8621, 8622, 8623, 8624, 8625, 8626, 8627, 8628, 8629,
  8630, 8631, 8632, 8633, 8634, 8635, 8636, 8637, 8638, 8639, 8640, 8641, 8642,
  8643, 8644, 8645, 8646, 8647, 8648, 8649, 8650, 8651, 8652, 8653, 8654, 8655,
  8656, 8657, 8658, 8659, 8660, 8661, 8662, 8663, 8664, 8665, 8666, 8667, 8668,
  8669, 8670, 8671, 8672, 8673, 8674, 8675, 8676, 8677, 8678, 8679, 8680, 8681,
  8682, 8683, 8684, 8685, 8686, 8687, 8688, 8689, 8690, 8691, 8692, 8693, 8694,
  8695, 8696, 8697, 8698, 8699, 8700, 8701, 8702, 8703, 8704, 8705, 8706, 8707,
  8708, 8709, 8710, 8711, 8712, 8713, 8714, 8715, 8716, 8717, 8718, 8719, 8720,
  8721, 8722, 8723, 8724, 8725, 8726, 8727, 8728, 8729, 8730, 8731, 8732, 8733,
  8734, 8735, 8736, 8737, 8738, 8739, 8740, 8741, 8742, 8743, 8744, 8745, 8746,
  8747, 8748, 8749, 8750, 8751, 8752, 8753, 8754, 8755, 8756, 8757, 8758, 8759,
  8760, 8761, 8762, 8763, 8764, 8765, 8766, 8767, 8768, 8769, 8770, 8771, 8772,
  8773, 8774, 8775, 8776, 8777, 8778, 8779, 8780, 8781, 8782, 8783, 8784, 8785,
  8786, 8787, 8788, 8789, 8790, 8791, 8792, 8793, 8794, 8795, 8796, 8797, 8798,
  8799, 8800, 8801, 8802, 8803, 8804, 8805, 8806, 8807, 8808, 8809, 8810, 8811,
  8812, 8813, 8814, 8815, 8816, 8817, 8818, 8819, 8820, 8821, 8822, 8823, 8824,
  8825, 8826, 8827, 8828, 8829, 8830, 8831, 8832, 8833, 8834, 8835, 8836, 8837,
  8838, 8839, 8840, 8841, 8842, 8843, 8844, 8845, 8846, 8847, 8848, 8849, 8850,
  8851, 8852, 8853, 8854, 8855, 8856, 8857, 8858, 8859, 8860, 8861, 8862, 8863,
  8864, 8865, 8866, 8867, 8868, 8869, 8870, 8871, 8872, 8873, 8874, 8875, 8876,
  8877, 8878, 8879, 8880, 8881, 8882, 8883, 8884, 8885, 8886, 8887, 8888, 8889,
  8890, 8891, 8892, 8893, 8894, 8895, 8896, 8897, 8898, 8899, 8900, 8901, 8902,
  8903, 8904, 8905, 8906, 8907, 8908, 8909, 8910, 8911, 8912, 8913, 8914, 8915,
  8916, 8917, 8918, 8919, 8920, 8921, 8922, 8923, 8924, 8925, 8926, 8927, 8928,
  8929, 8930, 8931, 8932, 8933, 8934, 8935, 8936, 8937, 8938, 8939, 8940, 8941,
  8942, 8943, 8944, 8945, 8946, 8947, 8948, 8949, 8950, 8951, 8952, 8953, 8954,
  8955, 8956, 8957, 8958, 8959, 8960, 8961, 8962, 8963, 8964, 8965, 8966, 8967,
  8968, 8969, 8970, 8971, 8972, 8973, 8974, 8975, 8976, 8977, 8978, 8979, 8980,
  8981, 8982, 8983, 8984, 8985, 8986, 8987, 8988, 8989, 8990, 8991, 8992, 8993,
  8994, 8995, 8996, 8997, 8998, 8999, 9000, 9001, 9002, 9003, 9004, 9005, 9006,
  9007, 9008, 9009, 9010, 9011, 9012, 9013, 9014, 9015, 9016, 9017, 9018, 9019,
  9020, 9021, 9022, 9023, 9024, 9025, 9026, 9027, 9028, 9029, 9030, 9031, 9032,
  9033, 9034, 9035, 9036, 9037, 9038, 9039, 9040, 9041, 9042, 9043, 9044, 9045,
  9046, 9047, 9048, 9049, 9050, 9051, 9052, 9053, 9054, 9055, 9056, 9057, 9058,
  9059, 9060, 9061, 9062, 9063, 9064, 9065, 9066, 9067, 9068, 9069, 9070, 9071,
  9072, 9073, 9074, 9075, 9076, 9077, 9078, 9079, 9080, 9081, 9082, 9083, 9084,
  9085, 9086, 9087, 9088, 9089, 9090, 9091, 9092, 9093, 9094, 9095, 9096, 9097,
  9098, 9099, 9100, 9101, 9102, 9103, 9104, 9105, 9106, 9107, 9108, 9109, 9110,
  9111, 9112, 9113, 9114, 9115, 9116, 9117, 9118, 9119, 9120, 9121, 9122, 9123,
  9124, 9125, 9126, 9127, 9128, 9129, 9130, 9131, 9132, 9133, 9134, 9135, 9136,
  9137, 9138, 9139, 9140, 9141, 9142, 9143, 9144, 9145, 9146, 9147, 9148, 9149,
  9150, 9151, 9152, 9153, 9154, 9155, 9156, 9157, 9158, 9159, 9160, 9161, 9162,
  9163, 9164, 9165, 9166, 9167, 9168, 9169, 9170, 9171, 9172, 9173, 9174, 9175,
  9176, 9177, 9178, 9179, 9180, 9181, 9182, 9183, 9184, 9185, 9186, 9187, 9188,
  9189, 9190, 9191, 9192, 9193, 9194, 9195, 9196, 9197, 9198, 9199, 9200, 9201,
  9202, 9203, 9204, 9205, 9206, 9207, 9208, 9209, 9210, 9211, 9212, 9213, 9214,
  9215, 9216, 9217, 9218, 9219, 9220, 9221, 9222, 9223, 9224, 9225, 9226, 9227,
  9228, 9229, 9230, 9231, 9232, 9233, 9234, 9235, 9236, 9237, 9238, 9239, 9240,
  9241, 9242, 9243, 9244, 9245, 9246, 9247, 9248, 9249, 9250, 9251, 9252, 9253,
  9254, 9255, 9256, 9257, 9258, 9259, 9260, 9261, 9262, 9263, 9264, 9265, 9266,
  9267, 9268, 9269, 9270, 9271, 9272, 9273, 9274, 9275, 9276, 9277, 9278, 9279,
  9280, 9281, 9282, 9283, 9284, 9285, 9286, 9287, 9288, 9289, 9290, 9291, 9292,
  9293, 9294, 9295, 9296, 9297, 9298, 9299, 9300, 9301, 9302, 9303, 9304, 9305,
  9306, 9307, 9308, 9309, 9310, 9311, 9472, 9473, 9474, 9475, 9476, 9477, 9478,
  9479, 9480, 9481, 9482, 9483, 9484, 9485, 9486, 9487, 9488, 9489, 9490, 9491,
  9492, 9493, 9494, 9495, 9496, 9497, 9498, 9499, 9500, 9501, 9502, 9503, 9504,
  9505, 9506, 9507, 9508, 9509, 9510, 9511, 9512, 9513, 9514, 9515, 9516, 9517,
  9518, 9519, 9520, 9521, 9522, 9523, 9524, 9525, 9526, 9527, 9528, 9529, 9530,
  9531, 9532, 9533, 9534, 9535, 9536, 9537, 9538, 9539, 9540, 9541, 9542, 9543,
  9544, 9545, 9546, 9547, 9548, 9549, 9550, 9551, 9552, 9553, 9554, 9555, 9556,
  9557, 9558, 9559, 9560, 9561, 9562, 9563, 9564, 9565, 9566, 9567, 9568, 9569,
  9570, 9571, 9572, 9573, 9574, 9575, 9576, 9577, 9578, 9579, 9580, 9581, 9582,
  9583, 9584, 9585, 9586, 9587, 9588, 9589, 9590, 9591, 9592, 9593, 9594, 9595,
  9596, 9597, 9598, 9599, 9600, 9601, 9602, 9603, 9604, 9605, 9606, 9607, 9608,
  9609, 9610, 9611, 9612, 9613, 9614, 9615, 9616, 9617, 9618, 9619, 9620, 9621,
  9622, 9623, 9624, 9625, 9626, 9627, 9628, 9629, 9630, 9631, 9632, 9633, 9634,
  9635, 9636, 9637, 9638, 9639, 9640, 9641, 9642, 9643, 9644, 9645, 9646, 9647,
  9648, 9649, 9650, 9651, 9652, 9653, 9654, 9655, 9656, 9657, 9658, 9659, 9660,
  9661, 9662, 9663, 9664, 9665, 9666, 9667, 9668, 9669, 9670, 9671, 9672, 9673,
  9674, 9675, 9676, 9677, 9678, 9679, 9680, 9681, 9682, 9683, 9684, 9685, 9686,
  9687, 9688, 9689, 9690, 9691, 9692, 9693, 9694, 9695, 9696, 9697, 9698, 9699,
  9700, 9701, 9702, 9703, 9704, 9705, 9706, 9707, 9708, 9709, 9710, 9711, 9712,
  9713, 9714, 9715, 9716, 9717, 9718, 9719, 9720, 9721, 9722, 9723, 9724, 9725,
  9726, 9727, 9728, 9729, 9730, 9731, 9732, 9733, 9734, 9735, 9736, 9737, 9738,
  9739, 9740, 9741, 9742, 9743, 9744, 9745, 9746, 9747, 9748, 9749, 9750, 9751,
  9752, 9753, 9754, 9755, 9756, 9757, 9758, 9759, 9760, 9761, 9762, 9763, 9764,
  9765, 9766, 9767, 9768, 9769, 9770, 9771, 9772, 9773, 9774, 9775, 9776, 9777,
  9778, 9779, 9780, 9781, 9782, 9783, 9784, 9785, 9786, 9787, 9788, 9789, 9790,
  9791, 9792, 9793, 9794, 9795, 9796, 9797, 9798, 9799, 9800, 9801, 9802, 9803,
  9804, 9805, 9806, 9807, 9808, 9809, 9810, 9811, 9812, 9813, 9814, 9815, 9816,
  9817, 9818, 9819, 9820, 9821, 9822, 9823, 9824, 9825, 9826, 9827, 9828, 9829,
  9830, 9831, 9832, 9833, 9834, 9835, 9836, 9837, 9838, 9839, 9840, 9841, 9842,
  9843, 9844, 9845, 9846, 9847, 9848, 9849, 9850, 9851, 9852, 9853, 9854, 9855,
  9856, 9857, 9858, 9859, 9860, 9861, 9862, 9863, 9864, 9865, 9866, 9867, 9868,
  9869, 9870, 9871, 9872, 9873, 9874, 9875, 9876, 9877, 9878, 9879, 9880, 9881,
  9882, 9883, 9884, 9885, 9886, 9887, 9888, 9889, 9890, 9891, 9892, 9893, 9894,
  9895, 9896, 9897, 9898, 9899, 9900, 9901, 9902, 9903, 9904, 9905, 9906, 9907,
  9908, 9909, 9910, 9911, 9912, 9913, 9914, 9915, 9916, 9917, 9918, 9919, 9920,
  9921, 9922, 9923, 9924, 9925, 9926, 9927, 9928, 9929, 9930, 9931, 9932, 9933,
  9934, 9935, 9936, 9937, 9938, 9939, 9940, 9941, 9942, 9943, 9944, 9945, 9946,
  9947, 9948, 9949, 9950, 9951, 9952, 9953, 9954, 9955, 9956, 9957, 9958, 9959,
  9960, 9961, 9962, 9963, 9964, 9965, 9966, 9967, 9968, 9969, 9970, 9971, 9972,
  9973, 9974, 9975, 9976, 9977, 9978, 9979, 9980, 9981, 9982, 9983, 9984, 9985,
  9986, 9987, 9988, 9989, 9990, 9991, 9992, 9993, 9994, 9995, 9996, 9997, 9998,
  9999, 10000, 10001, 10002, 10003, 10004, 10005, 10006, 10007, 10008, 10009,
  10010, 10011, 10012, 10013, 10014, 10015, 10016, 10017, 10018, 10019, 10020,
  10021, 10022, 10023, 10024, 10025, 10026, 10027, 10028, 10029, 10030, 10031,
  10032, 10033, 10034, 10035, 10036, 10037, 10038, 10039, 10040, 10041, 10042,
  10043, 10044, 10045, 10046, 10047, 10048, 10049, 10050, 10051, 10052, 10053,
  10054, 10055, 10056, 10057, 10058, 10059, 10060, 10061, 10062, 10063, 10064,
  10065, 10066, 10067, 10068, 10069, 10070, 10071, 10072, 10073, 10074, 10075,
  10076, 10077, 10078, 10079, 10080, 10081, 10082, 10083, 10084, 10085, 10086,
  10087, 10088, 10089, 10090, 10091, 10092, 10093, 10094, 10095, 10096, 10097,
  10098, 10099, 10100, 10101, 10132, 10133, 10134, 10135, 10136, 10137, 10138,
  10139, 10140, 10141, 10142, 10143, 10144, 10145, 10146, 10147, 10148, 10149,
  10150, 10151, 10152, 10153, 10154, 10155, 10156, 10157, 10158, 10159, 10160,
  10161, 10162, 10163, 10164, 10165, 10166, 10167, 10168, 10169, 10170, 10171,
  10172, 10173, 10174, 10175, 10176, 10177, 10178, 10179, 10180, 10181, 10182,
  10183, 10184, 10185, 10186, 10187, 10188, 10189, 10190, 10191, 10192, 10193,
  10194, 10195, 10196, 10197, 10198, 10199, 10200, 10201, 10202, 10203, 10204,
  10205, 10206, 10207, 10208, 10209, 10210, 10211, 10212, 10213, 10214, 10215,
  10216, 10217, 10218, 10219, 10220, 10221, 10222, 10223, 10224, 10225, 10226,
  10227, 10228, 10229, 10230, 10231, 10232, 10233, 10234, 10235, 10236, 10237,
  10238, 10239, 10240, 10241, 10242, 10243, 10244, 10245, 10246, 10247, 10248,
  10249, 10250, 10251, 10252, 10253, 10254, 10255, 10256, 10257, 10258, 10259,
  10260, 10261, 10262, 10263, 10264, 10265, 10266, 10267, 10268, 10269, 10270,
  10271, 10272, 10273, 10274, 10275, 10276, 10277, 10278, 10279, 10280, 10281,
  10282, 10283, 10284, 10285, 10286, 10287, 10288, 10289, 10290, 10291, 10292,
  10293, 10294, 10295, 10296, 10297, 10298, 10299, 10300, 10301, 10302, 10303,
  10304, 10305, 10306, 10307, 10308, 10309, 10310, 10311, 10312, 10313, 10314,
  10315, 10316, 10317, 10318, 10319, 10320, 10321, 10322, 10323, 10324, 10325,
  10326, 10327, 10328, 10329, 10330, 10331, 10332, 10333, 10334, 10335, 10336,
  10337, 10338, 10339, 10340, 10341, 10342, 10343, 10344, 10345, 10346, 10347,
  10348, 10349, 10350, 10351, 10352, 10353, 10354, 10355, 10356, 10357, 10358,
  10359, 10360, 10361, 10362, 10363, 10364, 10365, 10366, 10367, 10368, 10369,
  10370, 10371, 10372, 10373, 10374, 10375, 10376, 10377, 10378, 10379, 10380,
  10381, 10382, 10383, 10384, 10385, 10386, 10387, 10388, 10389, 10390, 10391,
  10392, 10393, 10394, 10395, 10396, 10397, 10398, 10399, 10400, 10401, 10402,
  10403, 10404, 10405, 10406, 10407, 10408, 10409, 10410, 10411, 10412, 10413,
  10414, 10415, 10416, 10417, 10418, 10419, 10420, 10421, 10422, 10423, 10424,
  10425, 10426, 10427, 10428, 10429, 10430, 10431, 10432, 10433, 10434, 10435,
  10436, 10437, 10438, 10439, 10440, 10441, 10442, 10443, 10444, 10445, 10446,
  10447, 10448, 10449, 10450, 10451, 10452, 10453, 10454, 10455, 10456, 10457,
  10458, 10459, 10460, 10461, 10462, 10463, 10464, 10465, 10466, 10467, 10468,
  10469, 10470, 10471, 10472, 10473, 10474, 10475, 10476, 10477, 10478, 10479,
  10480, 10481, 10482, 10483, 10484, 10485, 10486, 10487, 10488, 10489, 10490,
  10491, 10492, 10493, 10494, 10495, 10496, 10497, 10498, 10499, 10500, 10501,
  10502, 10503, 10504, 10505, 10506, 10507, 10508, 10509, 10510, 10511, 10512,
  10513, 10514, 10515, 10516, 10517, 10518, 10519, 10520, 10521, 10522, 10523,
  10524, 10525, 10526, 10527, 10528, 10529, 10530, 10531, 10532, 10533, 10534,
  10535, 10536, 10537, 10538, 10539, 10540, 10541, 10542, 10543, 10544, 10545,
  10546, 10547, 10548, 10549, 10550, 10551, 10552, 10553, 10554, 10555, 10556,
  10557, 10558, 10559, 10560, 10561, 10562, 10563, 10564, 10565, 10566, 10567,
  10568, 10569, 10570, 10571, 10572, 10573, 10574, 10575, 10576, 10577, 10578,
  10579, 10580, 10581, 10582, 10583, 10584, 10585, 10586, 10587, 10588, 10589,
  10590, 10591, 10592, 10593, 10594, 10595, 10596, 10597, 10598, 10599, 10600,
  10601, 10602, 10603, 10604, 10605, 10606, 10607, 10608, 10609, 10610, 10611,
  10612, 10613, 10614, 10615, 10616, 10617, 10618, 10619, 10620, 10621, 10622,
  10623, 10624, 10625, 10626, 10627, 10628, 10629, 10630, 10631, 10632, 10633,
  10634, 10635, 10636, 10637, 10638, 10639, 10640, 10641, 10642, 10643, 10644,
  10645, 10646, 10647, 10648, 10649, 10650, 10651, 10652, 10653, 10654, 10655,
  10656, 10657, 10658, 10659, 10660, 10661, 10662, 10663, 10664, 10665, 10666,
  10667, 10668, 10669, 10670, 10671, 10672, 10673, 10674, 10675, 10676, 10677,
  10678, 10679, 10680, 10681, 10682, 10683, 10684, 10685, 10686, 10687, 10688,
  10689, 10690, 10691, 10692, 10693, 10694, 10695, 10696, 10697, 10698, 10699,
  10700, 10701, 10702, 10703, 10704, 10705, 10706, 10707, 10708, 10709, 10710,
  10711, 10712, 10713, 10714, 10715, 10716, 10717, 10718, 10719, 10720, 10721,
  10722, 10723, 10724, 10725, 10726, 10727, 10728, 10729, 10730, 10731, 10732,
  10733, 10734, 10735, 10736, 10737, 10738, 10739, 10740, 10741, 10742, 10743,
  10744, 10745, 10746, 10747, 10748, 10749, 10750, 10751, 10752, 10753, 10754,
  10755, 10756, 10757, 10758, 10759, 10760, 10761, 10762, 10763, 10764, 10765,
  10766, 10767, 10768, 10769, 10770, 10771, 10772, 10773, 10774, 10775, 10776,
  10777, 10778, 10779, 10780, 10781, 10782, 10783, 10784, 10785, 10786, 10787,
  10788, 10789, 10790, 10791, 10792, 10793, 10794, 10795, 10796, 10797, 10798,
  10799, 10800, 10801, 10802, 10803, 10804, 10805, 10806, 10807, 10808, 10809,
  10810, 10811, 10812, 10813, 10814, 10815, 10816, 10817, 10818, 10819, 10820,
  10821, 10822, 10823, 10824, 10825, 10826, 10827, 10828, 10829, 10830, 10831,
  10832, 10833, 10834, 10835, 10836, 10837, 10838, 10839, 10840, 10841, 10842,
  10843, 10844, 10845, 10846, 10847, 10848, 10849, 10850, 10851, 10852, 10853,
  10854, 10855, 10856, 10857, 10858, 10859, 10860, 10861, 10862, 10863, 10864,
  10865, 10866, 10867, 10868, 10869, 10870, 10871, 10872, 10873, 10874, 10875,
  10876, 10877, 10878, 10879, 10880, 10881, 10882, 10883, 10884, 10885, 10886,
  10887, 10888, 10889, 10890, 10891, 10892, 10893, 10894, 10895, 10896, 10897,
  10898, 10899, 10900, 10901, 10902, 10903, 10904, 10905, 10906, 10907, 10908,
  10909, 10910, 10911, 10912, 10913, 10914, 10915, 10916, 10917, 10918, 10919,
  10920, 10921, 10922, 10923, 10924, 10925, 10926, 10927, 10928, 10929, 10930,
  10931, 10932, 10933, 10934, 10935, 10936, 10937, 10938, 10939, 10940, 10941,
  10942, 10943, 10944, 10945, 10946, 10947, 10948, 10949, 10950, 10951, 10952,
  10953, 10954, 10955, 10956, 10957, 10958, 10959, 10960, 10961, 10962, 10963,
  10964, 10965, 10966, 10967, 10968, 10969, 10970, 10971, 10972, 10973, 10974,
  10975, 10976, 10977, 10978, 10979, 10980, 10981, 10982, 10983, 10984, 10985,
  10986, 10987, 10988, 10989, 10990, 10991, 10992, 10993, 10994, 10995, 10996,
  10997, 10998, 10999, 11000, 11001, 11002, 11003, 11004, 11005, 11006, 11007,
  11008, 11009, 11010, 11011, 11012, 11013, 11014, 11015, 11016, 11017, 11018,
  11019, 11020, 11021, 11022, 11023, 11024, 11025, 11026, 11027, 11028, 11029,
  11030, 11031, 11032, 11033, 11034, 11035, 11036, 11037, 11038, 11039, 11040,
  11041, 11042, 11043, 11044, 11045, 11046, 11047, 11048, 11049, 11050, 11051,
  11052, 11053, 11054, 11055, 11056, 11057, 11058, 11059, 11060, 11061, 11062,
  11063, 11064, 11065, 11066, 11067, 11068, 11069, 11070, 11071, 11072, 11073,
  11074, 11075, 11076, 11077, 11078, 11079, 11080, 11081, 11082, 11083, 11084,
  11085, 11086, 11087, 11088, 11089, 11090, 11091, 11092, 11093, 11094, 11095,
  11096, 11097, 11098, 11099, 11100, 11101, 11102, 11103, 11104, 11105, 11106,
  11107, 11108, 11109, 11110, 11111, 11112, 11113, 11114, 11115, 11116, 11117,
  11118, 11119, 11120, 11121, 11122, 11123, 11124, 11125, 11126, 11127, 11128,
  11129, 11130, 11131, 11132, 11133, 11134, 11135, 11136, 11137, 11138, 11139,
  11140, 11141, 11142, 11143, 11144, 11145, 11146, 11147, 11148, 11149, 11150,
  11151, 11152, 11153, 11154, 11155, 11156, 11157, 11158, 11159, 11160, 11161,
  11162, 11163, 11164, 11165, 11166, 11167, 11168, 11169, 11170, 11171, 11172,
  11173, 11174, 11175, 11176, 11177, 11178, 11179, 11180, 11181, 11182, 11183,
  11184, 11185, 11186, 11187, 11188, 11189, 11190, 11191, 11192, 11193, 11194,
  11195, 11196, 11197, 11198, 11199, 11200, 11201, 11202, 11203, 11204, 11205,
  11206, 11207, 11208, 11209, 11210, 11211, 11212, 11213, 11214, 11215, 11216,
  11217, 11218, 11219, 11220, 11221, 11222, 11223, 11224, 11225, 11226, 11227,
  11228, 11229, 11230, 11231, 11232, 11233, 11234, 11235, 11236, 11237, 11238,
  11239, 11240, 11241, 11242, 11243, 11244, 11245, 11246, 11247, 11248, 11249,
  11250, 11251, 11252, 11253, 11254, 11255, 11256, 11257, 11258, 11259, 11260,
  11261, 11262, 11263, 11776, 11777, 11778, 11779, 11780, 11781, 11782, 11783,
  11784, 11785, 11786, 11787, 11788, 11789, 11790, 11791, 11792, 11793, 11794,
  11795, 11796, 11797, 11798, 11799, 11800, 11801, 11802, 11803, 11804, 11805,
  11806, 11807, 11808, 11809, 11810, 11811, 11812, 11813, 11814, 11815, 11816,
  11817, 11818, 11819, 11820, 11821, 11822, 11823, 11824, 11825, 11826, 11827,
  11828, 11829, 11830, 11831, 11832, 11833, 11834, 11835, 11836, 11837, 11838,
  11839, 11840, 11841, 11842, 11843, 11844, 11845, 11846, 11847, 11848, 11849,
  11850, 11851, 11852, 11853, 11854, 11855, 11856, 11857, 11858, 11859, 11860,
  11861, 11862, 11863, 11864, 11865, 11866, 11867, 11868, 11869, 11870, 11871,
  11872, 11873, 11874, 11875, 11876, 11877, 11878, 11879, 11880, 11881, 11882,
  11883, 11884, 11885, 11886, 11887, 11888, 11889, 11890, 11891, 11892, 11893,
  11894, 11895, 11896, 11897, 11898, 11899, 11900, 11901, 11902, 11903, 12289,
  12290, 12291, 12296, 12297, 12298, 12299, 12300, 12301, 12302, 12303, 12304,
  12305, 12306, 12307, 12308, 12309, 12310, 12311, 12312, 12313, 12314, 12315,
  12316, 12317, 12318, 12319, 12320, 12336, 64830, 64831, 65093, 65094,
];
