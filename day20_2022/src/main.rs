#![feature(linked_list_remove)]
use std::collections::LinkedList;

fn decode_input(input: &str) -> Vec<i32> {
    input
        .split_terminator('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

fn display_list(list: &LinkedList<usize>, input: &[i32]) {
    list.iter().map(|e| input[*e]).take(10).for_each(|e| {
        print!("{}, ", e);
    });
    if list.len() > 10 {
        print!("...");
    }
    println!();
}

fn part1(input: &[i32]) -> i32 {
    let mut list = LinkedList::from_iter(0_usize..input.len());
    display_list(&list, input);
    input.iter().enumerate().for_each(|(index, code)| {
        if *code != 0 {
            let pos = list.iter().position(|v| *v == index).unwrap();
            let new_pos = if *code < 0 {
                let new_pos = (pos as i32 + *code) % input.len() as i32;
                if new_pos < 0 {
                    ((input.len() as i32 - 1) + new_pos) as usize
                } else {
                    new_pos as usize
                }
            } else {
                ((pos as i32 + *code) % input.len() as i32) as usize
            };
            if new_pos != pos {
                list.remove(pos);
                if new_pos == 0 {
                    if *code < 0 {
                        list.push_back(index);
                    } else {
                        list.push_front(index);
                    }
                } else if new_pos == input.len() - 1 {
                    if *code < 0 {
                        list.push_back(index);
                    } else {
                        list.push_front(index);
                    }
                } else {
                    let mut split = if new_pos < pos {
                        list.split_off(new_pos + 1)
                    } else {
                        list.split_off(new_pos)
                    };
                    split.push_front(index);
                    list.append(&mut split);
                }
            }
            display_list(&list, input);
        }
    });
    let index = input.iter().position(|v| *v == 0).unwrap();
    let pos = list.iter().position(|v| *v == index).unwrap();
    dbg!(input[*list.iter().nth((pos + 1000) % input.len()).unwrap()])
        + dbg!(input[*list.iter().nth((pos + 2000) % input.len()).unwrap()])
        + dbg!(input[*list.iter().nth((pos + 3000) % input.len()).unwrap()])
}

fn main() {
    let input = decode_input(INPUT);
    println!("{}", part1(&input));
}

#[cfg(test)]
mod test {
    use crate::{decode_input, part1};

    #[test]
    fn test_part() {
        assert_eq!(3, part1(&decode_input(TEST)));
    }

    const TEST: &str = r#"
1
2
-3
3
-2
0
4
"#;
}

const INPUT: &str = r#"
-6497
-7893
-4653
-1149
294
4741
6402
9406
-5250
-7657
2672
8685
-438
1324
3823
-2051
1249
-6995
920
8265
9991
-6554
6487
-9791
-5297
9288
9618
-69
-923
-4599
-9721
-6216
3409
4051
-8769
-4304
-4438
2573
-9768
-9801
4152
5197
4855
-4308
6656
7241
-9104
5614
901
-3764
-2651
-5809
-5098
7278
-8759
1410
2548
-8075
-7308
4338
5922
-184
1851
-1438
5076
-7021
-7701
7036
-5549
5116
6195
275
3720
-543
-214
-5372
3981
-5610
-124
1985
-5554
-4885
-2764
-9776
7240
9584
-1297
3230
868
6005
6400
-3409
-3585
1416
5969
9725
5290
8062
-1206
-2444
1914
-2721
1019
-8684
-5348
3366
-2126
5552
-1047
-3807
8119
-7877
-1302
1078
-5444
5450
-9589
8723
-9606
-5823
1920
-3285
-1107
5940
-8265
-4250
9812
-8094
-5091
2977
3893
9041
5466
1903
-5212
2196
9750
-6010
-1757
2222
3889
7734
5830
2051
2428
4658
-6887
-2068
879
3279
-6368
-9829
-4804
-4271
1428
-9164
-2276
8932
-2041
5717
-1214
9682
-1393
-3691
-1302
-9032
3098
7497
-9237
9127
2709
-1458
5412
-261
4517
1694
5202
-9225
-9793
-6903
-6755
4935
-3299
-681
9102
-3997
5161
7725
1608
8062
7900
9484
-5823
-536
-5853
-8277
9638
-3471
-8097
3098
-63
7548
7973
8765
-2235
-9799
-5197
-7256
-1320
9348
6712
-5685
-1071
-387
8918
7663
-1304
-3377
491
1078
-2906
4175
5190
-9200
5031
3161
-2424
247
369
-3489
3754
-924
5705
-1897
-3594
6499
7197
-8002
9812
-9106
-6721
1137
-4903
-4362
-8666
5343
1486
-1006
4740
8477
1771
-3334
1402
6554
7184
1954
-3416
3011
5301
8530
-7498
-8734
335
-4148
-5623
-5070
-3733
2819
-5526
382
-7545
-4227
-6504
557
9162
-7133
9584
2279
1316
-2524
6561
9830
-2075
8960
420
2803
6512
-2424
-4437
-9185
-7890
462
-8411
7370
5901
1942
8512
5907
5079
-8935
-9603
-7609
4194
-9343
8955
8406
1402
5543
454
8512
-71
746
-7192
5561
-9763
7272
-8430
8845
2152
923
4567
-2462
-3062
-3006
-8583
5070
-7751
2975
5398
-5316
6315
2739
-8872
5272
7641
-8876
3173
-7832
-9064
8676
7492
6882
-2890
-5479
-3351
7946
-2535
9045
157
8946
378
-4740
-4008
-6332
8709
-4938
-3982
-5970
4064
-382
-4635
-973
-2208
-1232
9892
-6927
868
-5033
-5532
82
5338
-6591
1440
9129
-1993
-6181
4437
8637
1072
1332
7698
-3733
-2707
-5491
-5384
571
8033
-4517
8434
1844
2061
-1918
6772
6018
-214
-161
6248
4328
-2658
6494
3366
-9948
382
3135
-1846
-2685
-6741
-1078
1729
9452
2900
-4669
-9897
-812
8213
4357
-5460
4160
9001
-8864
9408
7903
4405
-54
-1102
-4960
2091
7946
-1412
581
7335
-282
4822
-3165
9991
9832
2187
6378
2351
-1374
6713
-3088
-6649
-3584
-9857
637
-5315
1394
-4106
1343
4661
9911
-3356
-4300
-7966
-4999
-53
-133
-7289
1476
8801
8040
-595
-4407
-4357
-7872
-3320
-5674
-5081
1932
9787
-6069
-2321
4597
7449
8882
-7546
-9477
-6664
5491
4494
-2135
-5866
-2105
-1513
-8023
4404
6410
-8446
-1297
4917
1286
9073
612
100
1699
-4102
5712
6683
-5867
5373
-9489
3569
-7496
3848
7011
-2000
5900
390
-8147
2395
713
3228
-9425
-6163
8007
5020
7531
-381
564
-8322
833
2579
-7112
5188
7414
423
4271
3360
-9618
-9600
-7802
1985
-9110
-8080
6324
4305
-1648
7216
-9666
-1094
9891
6318
-5502
-747
-2935
3969
-9258
-8376
-4538
-8870
-6402
3009
9187
-350
-6555
-894
2190
7645
-5157
1126
-1458
2881
-1745
-6927
-7312
1371
1722
-3382
-3525
5731
8776
-9829
-8759
3877
896
1730
3890
-1880
-7574
-9088
-8239
7426
1982
6225
4970
4838
8966
-8080
4058
238
1611
-5391
-7021
-1304
4341
2624
-3223
-9178
1487
1382
-3332
-6240
-4506
-2827
-2650
4019
2845
5448
-4620
4751
-9652
-1477
-9503
8942
-8889
-793
1505
-7426
5659
740
485
8198
-6026
1900
-4842
9041
-5375
-5019
-3709
6217
4169
3521
-9559
2438
9262
-6124
8230
-6039
-7278
7328
3649
7403
-2517
6471
7819
9004
-8990
-3985
2889
2239
3306
-1915
8418
6255
-3627
-3673
-4285
5895
-3914
-5369
-3128
309
9179
-1201
8594
-5214
-4667
7629
3145
-4050
-2293
5052
-5149
-3997
-225
5680
6452
3056
8467
-4423
-991
8280
-2421
8092
-7991
-3275
-3584
7292
2412
1985
826
-1257
8523
5972
-4853
-9190
-6788
-8994
1571
-1880
478
-7695
7545
9018
7216
-9215
1438
7531
-2104
4174
-131
4984
-8430
-2216
8538
3030
-1508
3165
8667
-3467
5966
-8732
-931
2743
-2331
-1017
-4360
1296
8986
2472
-6075
2080
-8430
1443
-6026
-2299
9959
9867
9611
8767
-1797
4971
5151
2329
8676
4926
6649
-3843
4077
-3709
3410
-8027
9530
-2222
5207
7897
3420
-3204
-8790
6368
6269
-6219
1635
2208
2422
8776
-9857
3541
-4384
7961
-4553
6855
-702
-9042
-3709
4412
529
8093
9618
5208
6628
-1854
-9147
4861
-7127
-708
7153
9707
-2364
-8478
-2501
-6975
-6421
3546
2961
7476
-9439
2882
3522
7854
1227
-8566
-4505
-7335
5871
-159
1681
-6712
-1989
-4272
8289
-8870
9492
6422
-7341
-7485
3452
693
4843
-5013
-5269
-1129
4617
8469
-8865
-3413
9597
-1803
-5259
7538
6502
-1876
-2527
7670
-9035
9031
9675
-7746
6555
-8639
4441
-5340
5542
7672
-2377
7229
5761
5376
-4931
4896
-921
-5687
-1513
-595
-921
343
4307
8776
1980
720
246
-6527
-2721
-8235
3673
-71
-3182
-6358
5544
-5846
8424
-5554
1348
9826
-1666
5950
-5637
4649
-6679
-9998
9030
1810
-8176
-2254
1819
-2695
6348
4481
5329
-4273
3569
-4854
6722
1304
-5274
-2884
-9803
-938
9214
-8759
-2380
-9041
5389
-92
-1114
7892
8523
7800
-3577
2803
7132
2410
-7272
9632
5563
-5026
456
-861
9498
9616
-7660
9908
3848
4979
-8035
7713
-3457
256
-992
-3408
-8094
-1459
1681
-6788
-7180
-1852
1136
1756
5786
-6699
-8367
8126
-2517
6502
9537
4289
5410
3610
447
9742
-1006
-7026
-5197
3435
-2123
5886
9878
1376
8485
6570
-2932
-595
-9621
-6564
-9595
2122
5814
3009
7034
9063
-3742
4262
4795
-3300
-8857
52
-5453
-5968
6823
-4057
7542
3403
6714
-6620
-6085
-2188
-1182
4480
1080
6929
-8002
-1732
9406
-3130
9034
-6071
-9979
6393
-7373
8289
-7953
-2820
5832
-5727
-7358
7972
1944
-4489
1937
8388
-9143
7249
5803
3311
2602
-4988
4255
-2427
4036
7314
-9147
631
6800
5076
1621
-429
-5320
-4568
-1867
-9474
2367
-5279
-6708
3411
-5461
6161
3132
-7520
-6718
-4539
1811
-5962
714
1137
-3360
7105
-9489
-8649
-511
5705
3754
-2860
-7026
-579
3417
-511
-8199
-5031
7676
2428
5894
-8033
8551
-371
-1175
-6008
-3717
-7031
-8573
505
4828
5971
9008
1525
-8044
903
-3834
7141
-8459
-4404
6787
-1597
2481
3806
5965
-9025
-4188
3546
7612
-6372
-4068
8617
-2427
7349
9964
-6467
-8598
5351
5832
9080
6764
1277
5054
-312
9240
-8561
423
2162
2874
-4193
-3026
3710
835
-8743
-7690
-8382
7587
3435
-2072
8343
356
1420
-118
3526
8928
2721
49
5387
-4488
9206
-7857
-5659
-3193
2649
-5005
8962
1721
-3177
9314
-8994
2252
-7112
-4439
9647
-3382
8799
-7478
2829
2066
2669
4321
-2152
7333
6217
2080
5227
4657
-5826
4708
-3076
-1442
5155
-8350
-9032
-350
6338
6437
-1190
-4985
8954
-8173
-9838
3616
-8931
2906
-2855
7377
2977
9912
7079
-690
-9520
9554
-7209
-1908
2706
6424
-7695
2159
-8453
-9266
-6359
-9731
8862
6414
9967
-8229
-5320
7011
-9335
-4328
9035
-3196
-9688
-8435
6979
-427
-3885
-6463
-3990
-593
5705
142
1315
6085
-8229
3398
-7395
9324
-68
-7223
5365
-2937
4890
5087
-9607
-1158
-7966
-7928
1026
-3447
8520
7563
9914
741
6618
-6963
-9147
9281
-8859
1599
1360
9395
-8732
5697
-1244
7276
4398
-4571
714
2332
-5502
-9427
-3837
9989
8506
-6575
-3214
-4108
7629
-4353
8234
-1336
3266
-814
1501
-8992
-8752
7137
-5804
-6887
-9470
3490
5087
-3577
-1081
9197
-9686
6830
-6691
-6779
1006
-4160
-8949
-3973
-8793
-6277
2325
6346
9538
5619
-9883
7191
-7498
6300
491
-4586
2573
-4194
307
-2038
-8949
4367
-2164
-8860
-4511
-5550
-7748
-4599
-4709
3116
9883
-728
-9246
-9115
1722
-5828
8720
9172
-9263
-393
1408
-2299
-7452
-7640
2336
3107
-8473
-950
-9647
8421
-2721
6530
2900
-1697
4472
-7368
3243
-304
-8630
-840
2268
-8494
99
631
-2721
-3409
-5466
-8138
-3468
6142
-7998
-259
1988
8654
8855
4098
-7327
-6271
-3187
9398
-6782
6847
762
-9323
8742
1137
-8213
7692
-5554
-282
-1151
-1405
-2716
9550
4122
-9995
5057
4545
-6806
-4497
-3006
2573
3284
-5214
3132
7088
-4403
-2016
-8097
-6793
-1861
-8282
7109
1522
-4402
-3825
1678
-2059
-8055
-7045
-7794
6784
-7698
-8925
284
384
5648
2961
8617
-8423
8406
-768
-2673
3623
-9620
9709
1060
9657
879
761
-1952
8422
-2108
1959
2426
8440
-1637
9239
-6371
-6927
6217
4728
-5210
-6781
-4915
-9921
-1373
-5216
-6979
3635
2073
-5766
-7972
-9973
-2306
-8023
-1880
7317
1096
-8107
-1458
4659
-1112
-5840
7007
1823
1261
-464
148
-8913
-1382
-6126
7594
-7478
8152
-8519
-1775
-4009
-1044
-4920
3058
-5383
7243
6235
-695
4058
626
791
-8872
3698
9991
2673
1056
9896
3031
4365
5978
-2605
-4459
2073
-1387
3306
6449
-6191
9414
-3795
-2455
-5340
7800
-8665
5648
-9343
-5026
-9256
-5550
-2716
-3051
-3384
8431
2323
-962
-1171
-9110
5896
-9925
6693
7597
5062
4186
3203
-9263
-8918
-7693
-5133
-5098
3439
-7996
4597
-2140
1976
8259
4538
5543
-7026
5528
8585
7794
3009
-1676
7332
-169
-7265
-5291
-5856
630
-5693
-3377
9586
5092
1427
9347
4305
-2059
7637
2597
-3432
-9255
-6704
5076
-1894
4781
4255
1856
5623
2334
-6786
9202
1811
5662
5603
-3238
-6181
-7841
-2665
7924
-654
5207
-2041
5919
577
8069
-4201
-3562
-7424
4618
4619
3165
-7975
9221
-6459
-6786
-4627
541
-3036
-5133
-8106
2098
7327
-489
9826
-9666
9677
2070
-6461
-1044
6801
9771
-5327
-7089
-9375
-4692
5900
-3547
-2816
-3870
-7734
-698
1438
-812
-2024
-2304
-8332
7396
-4608
-3213
7635
-5996
6332
6568
-4538
-6163
2867
2070
-2212
2605
-8489
-3948
2015
6292
9329
2541
-3585
-1422
1936
7899
5803
-4669
-5943
-2614
3642
283
70
8696
715
1432
9007
-8142
2590
639
-4960
-4418
9929
-9088
1505
-8994
731
-9768
-927
6850
4999
1723
5536
-6404
-6626
-7923
-9214
-9799
-4635
6813
2642
-7712
-8976
-1622
1349
369
-8666
-3900
-3128
991
8018
-2785
-3313
-5931
7366
-1645
1135
-4656
7199
-4999
-1135
-4636
253
7390
2038
4204
8729
-2455
3782
7296
6696
-4402
-6967
-6587
-5979
-9559
8636
-5242
2329
-8941
-8126
7036
-5216
-2331
-8857
4338
5005
6457
9307
4561
2996
374
-9177
2977
-6530
628
9184
-9573
9797
2199
9726
-4236
-5409
-8489
9035
-9952
4624
-2540
-7021
7721
4907
-1024
-9197
-6421
-8333
-6463
2149
8104
-7734
1494
4557
6452
-876
1878
9994
1188
5448
6924
8729
9914
5748
-4131
3145
-9684
-7111
5001
7120
8711
-238
8625
8911
-5853
-1002
7856
3089
9832
-3653
-8813
2660
-6596
-5962
-623
-5157
-4230
3233
161
1982
-9510
5947
9640
1094
5851
1263
-4410
-4477
4706
-8843
-5065
-8804
-7122
-142
1920
-1306
5091
-6364
55
-6287
2502
8118
9590
2925
7390
6722
-6927
-6649
9511
-2972
-5257
5035
799
1428
9677
2073
-6871
-4116
-3051
-4699
-3795
-9919
-3833
-4308
5042
-2126
803
-7162
-6658
-3781
-3585
-9589
6970
5141
-5379
3610
2998
-9115
2077
6005
-5070
2391
-6884
4858
-8518
8118
-4262
-4273
-8793
6924
-1477
-5849
-4522
-7127
-6067
-8758
-1835
9832
5524
-4588
-7950
4280
8225
-8751
6835
8695
-9658
-7389
-3970
2474
637
2660
-387
-2812
-2595
1789
-1513
-9765
-2075
-2598
1271
-3432
5662
9580
7251
-7774
6430
6627
4544
-9881
6296
4550
4563
4657
6281
7997
-1761
-7408
-3653
-3689
-5285
-927
-8986
-1504
-7610
-3364
-802
-6895
-8839
-3560
-7261
7372
-5844
5144
-9581
8918
-6722
1658
-8113
5521
2866
-5964
-5224
-2080
-3597
5127
-4830
-6909
-5623
-664
1780
7021
-5142
-5133
-1368
-2780
7868
8905
-4589
-1691
-8618
8397
5083
-2672
-1337
2094
1525
6296
-4106
5057
9638
-6764
-9479
6704
8776
-3101
239
-2996
-4062
-904
1985
5761
-9534
-2733
413
5822
-6008
-9351
8919
-1197
-1136
-9656
-6543
-8110
1603
6907
-5610
8767
-9608
6201
4887
3607
282
-6643
-1649
3589
1042
8169
-3216
8296
-6642
-3213
1803
-5986
422
-4991
-378
-8876
4845
9911
1927
4601
995
-9447
3289
-7568
-9912
9812
-8118
-8857
-720
2665
9050
8689
-7923
-4920
-9787
8955
6924
-7906
9184
-5622
-6138
9908
9332
-5942
9913
300
-1044
-2014
-4817
-2130
-9190
4030
359
5216
9839
-1303
9959
-4994
-3382
6360
-4636
9014
1098
5534
1823
6398
-1659
4780
-5872
3879
-8411
5598
-3888
-3645
2181
-3718
-2905
2040
-3590
-7610
8039
-5612
4855
-4394
-9537
-9642
6842
8004
4624
2610
1848
-2256
-6882
-5554
-6183
2046
-4206
-3421
-9742
3309
-24
-2494
8444
1479
1522
-1871
-4309
-7195
-3501
8772
-7331
-6030
3311
6433
-4160
2170
8500
-4729
9124
-3180
5839
3325
-4947
5252
9743
-2349
3538
3806
-8559
-5703
-3066
8246
7267
9845
-761
7822
-4145
-2811
4238
-4188
1473
6591
-8791
-9046
4426
-5622
1551
-1501
-9651
-4317
6967
-467
-1523
-927
-5157
9675
2080
-2672
908
6373
5161
9314
-1367
-4635
-724
5567
99
4112
-9485
-5687
-3270
-2059
6618
-2240
8258
-4425
9309
-9327
-4714
-3167
624
-231
6714
1237
-5372
7132
2541
2447
6618
-6383
-9520
-9056
5005
-4273
-1523
-6854
3748
8169
-3023
3826
884
-8281
-2791
8436
-5009
-5964
-1669
8628
-1990
-7973
-1927
-9513
-4817
-4250
-1002
-1043
-6857
8332
4795
2233
-8282
729
5909
-4358
-7346
3665
-2530
46
3135
-2368
-7951
7696
-9838
-1851
6634
-8802
-4228
-8111
-7590
-3280
8903
8025
5720
901
3457
-88
280
9342
5709
-4373
-1768
-3994
-6563
1050
2911
-7972
8760
-5627
-6662
-6133
-5493
-4705
4540
-4680
3735
-476
-9516
-2023
-1648
166
-7642
3212
-7199
4502
5909
4778
6583
8048
2644
544
-4119
-2840
1465
9106
-9812
-6824
3720
7602
-3566
7680
-6046
-3223
899
-780
773
-5756
6180
-9803
5752
-8984
9135
8647
-6530
-1135
-1681
3056
7786
-449
0
-304
-3274
-286
-2709
-5458
9090
-9087
9309
4040
-8199
-5548
2357
4312
-1901
8855
-6882
-924
-6271
-7247
-8097
-6090
3145
-6243
-6277
9470
-7813
2179
-955
4387
-2374
3897
-1047
-5840
-4274
-3432
2294
-1142
7072
8366
-7838
-1928
9956
7390
7743
4667
-5363
10000
6743
-8732
-3130
2245
-1797
5992
9102
-8138
-6415
9278
4443
-9115
5697
4710
5900
7524
4778
2162
6686
5972
-8023
-6126
-7781
1100
280
-3853
5092
9932
-7156
7215
1248
-3967
8444
-5715
-7498
-4904
4384
333
5448
5628
857
-5754
2094
-1152
466
4024
-3651
-3468
1357
8097
-1664
8402
-7026
-68
1754
-3365
-1200
5263
4855
-2230
-6846
-5696
-8916
-3583
-4778
9413
-4755
6693
-9376
4094
-9343
9846
5779
8958
-9665
-3801
9858
-1824
6961
-4752
1988
3810
9213
-3501
9091
-4095
9493
-3222
2120
-1880
-1576
-8816
2516
5207
-3390
-5377
5410
1611
9847
4016
-3047
-6735
-6587
-9785
-2611
-851
-7809
1982
8720
9186
4690
-7737
-9997
-8607
821
921
786
2502
-1878
-3497
-7571
7267
-6494
6414
8025
-4835
-2827
-5329
-5841
-2176
1050
-7948
6411
-1040
7276
-481
3672
1643
-595
-1397
8523
-5895
3515
8418
-5903
5648
-955
-2112
1376
-7400
4443
9846
-9280
7038
492
3592
-8464
-4481
7094
8629
-8487
-2095
1369
7167
8870
-7277
-3178
215
5804
9356
-5554
6620
-3437
-8659
7241
-2884
-6403
2968
-7553
8226
-1651
-7519
1730
-4656
-233
2178
-7343
8736
4758
8398
-7808
7038
-9807
6471
6417
8258
-5372
-9599
-1565
-7104
71
-3424
-302
-9647
-7647
6018
2230
877
-2475
-1417
9324
-5048
-1040
162
203
2004
-1851
-4031
-3627
-8002
8294
8899
4736
3953
-2152
4494
9638
8988
1237
754
-8137
1614
-9466
-4539
2573
-2140
5507
1844
456
-2537
7553
5144
-2752
531
8628
-8367
-8858
4291
8696
1588
6803
-3317
2993
1789
-6663
-3285
-1766
-9868
6961
-8598
9704
-6308
-2410
-20
6648
5444
-8197
6813
-2240
5444
1846
-3850
4541
-6046
1873
-1882
-3610
8035
-5023
-6198
-1170
9652
-2791
-5193
-9974
-8273
2139
9913
-2841
202
1129
467
-1076
-5708
2786
-4890
2129
-5091
-921
-9785
-8876
-225
7492
9334
1962
9985
976
4091
-8938
8772
-3997
-7770
4236
2644
7292
-8627
6453
8753
6827
5966
7449
-3881
756
4397
-6211
-3332
-8136
-2640
-8318
-2280
-4493
9711
-4838
4909
-9868
6063
-9380
3414
4445
-3196
5208
9743
674
-2378
-856
1959
8766
6570
4541
903
-9300
9903
9642
-4362
-6863
-3529
8201
7542
8990
-5259
-9263
-946
-869
9492
6133
-2724
-7667
-8288
-2960
7065
9936
-6354
-2598
-2168
-2561
-4868
6190
2826
-8318
9956
-4840
660
-6802
2819
4679
-4108
-1713
8766
4354
-2616
5709
-4869
-5493
3031
-4282
1278
-118
8557
1683
-401
-252
2903
1551
5144
6472
-7954
5772
-1387
-7051
-8491
7341
7578
-1324
-708
1423
633
-8934
-4462
5324
7670
5507
4464
1658
5822
5359
-148
-2061
-3409
685
3290
-8224
2309
5127
-1670
-6786
1383
2826
3848
159
9828
-5455
46
4218
1611
-2923
-1819
-595
8750
-2170
-2028
8694
-3594
-4539
6502
-8846
-866
-2140
-1918
3984
-9118
1762
-1367
435
2486
7842
-4019
7542
-7433
579
-9110
-1790
-121
8743
133
7359
-339
158
4979
4244
1954
6195
-1211
-88
677
-7382
-4517
7701
-5377
8640
-4937
-8773
6101
2311
8370
5434
-55
-1796
-1439
5930
-8596
162
14
-916
2797
-9535
4921
8731
93
6612
5739
5060
5802
3370
-5024
7806
9927
6142
-5984
-8013
-4903
-8523
5549
2177
2742
5900
9245
-9772
4945
6398
8111
46
6588
-1955
9253
-2455
8031
-7272
639
-4102
6201
-6198
7458
-7263
-3994
-1387
-8331
2371
8425
3913
7709
-8491
-2860
-4593
-1246
1
-3742
391
-285
-5466
6180
-5542
-2154
8389
-6181
6716
-382
-3213
7011
9779
5187
281
4517
6231
5157
8041
5305
-9028
2897
4843
2968
-6064
7005
-5348
-6791
6115
6283
5240
-3594
8432
-4159
-5667
6401
4531
1878
-9705
-9670
-9520
-1006
-6731
4394
7492
4156
-4220
3840
-9874
2667
6036
-9559
-2061
-5173
8709
-6764
8512
-6925
-1497
-4329
4122
2367
-7469
1610
-1666
-3994
9936
-8122
-4956
9917
-5828
6550
-6115
1800
-5821
8933
-9471
-946
4128
5748
8932
8916
1094
5804
-2722
-4511
-3647
6914
9430
-3766
-6654
6402
-8834
-2216
2434
312
-9470
4739
1505
5324
-5701
3799
-4516
-6013
-6340
-2743
-2599
-6555
-2162
9883
7165
-2515
7637
-6286
4115
9787
8832
-6027
-2793
-3406
-7346
9346
4122
4328
-5827
9457
-11
6506
-6738
-7162
-7978
-6861
6857
5387
2845
-2403
7892
8179
6756
7292
-6281
5920
7070
7676
-7292
-7438
-3248
9832
3840
29
5966
-4600
-6554
-6097
-6995
9878
-5224
139
8698
-3987
6583
7341
-3517
-6563
6360
-9656
-216
5720
-2475
749
-3213
-5214
-1501
3657
4077
912
2425
-4769
2745
2889
2138
2110
2624
-9427
6789
9657
-1602
-5453
6278
5720
-6248
-667
-7929
-5687
3260
-2900
4174
-9559
6018
-6927
-4228
-3772
-8160
6660
-2095
5543
-8513
6783
9345
3254
7458
-4586
-7353
-4986
-327
8382
8694
7445
4003
7676
3755
-1140
9827
-2524
-7948
2155
-9795
-996
6966
-1631
-3559
-350
5102
-451
488
4194
7008
5391
-6581
6906
9959
6646
877
-9719
-279
9825
164
5881
1318
-3647
2162
-6336
-1989
-7832
-9000
-7744
-6146
3599
-5260
9512
1286
-9478
-5744
378
-1915
4140
8685
-916
1276
2098
8682
-2650
-1658
-305
6294
-2118
6338
6943
7333
-6918
-2865
7903
2577
1017
-6344
-5990
7856
-8768
4278
478
3183
3257
-3850
3013
6006
-9042
-8094
-9623
-4753
-4159
8448
5020
7822
8721
9752
4430
-7984
-1491
-1090
9710
-1305
-9107
7029
5878
-7308
5568
3699
5896
-1401
-1867
6674
-4915
4448
-6467
1138
-4587
-8103
-1151
-6409
-6734
1404
-91
-5830
-4127
-6288
-3482
-5808
-2831
8398
-2653
-9672
-9591
-291
-4409
1952
-3187
2529
-1071
-6429
5578
2493
-6079
1759
9649
9504
-344
2126
-1926
437
6674
-8803
2520
6940
3501
3150
9655
6031
9665
-6354
-5632
9578
4820
-7344
-4971
-9351
-2598
-1220
2422
-5078
2942
-8199
377
9253
5901
7522
28
-8213
-1955
882
148
-5943
-3805
-1023
-4004
-401
6629
-9375
6373
-6102
4298
799
5295
3893
9085
-9535
-426
6345
-1745
-9275
-803
-412
-9595
9139
4338
2424
2945
-9042
543
-6752
579
5520
1584
5823
-728
9739
6449
-8846
-2910
9334
-3139
-2140
2204
-4600
-5792
-2141
28
-4920
7141
-7180
1145
-3588
8069
-4830
-1071
-5872
1027
-49
5324
-2687
2071
9056
-6630
6410
8248
-7408
8552
1215
-8836
1243
3435
-4668
-3638
618
-6848
1090
-4020
545
6452
9245
-9409
-5208
491
-2135
-7701
-135
6283
3649
-429
5868
1352
-2742
1319
9584
-6832
7299
-8430
4255
498
-9272
-5951
-5026
-4484
8524
8893
-9995
-4773
-247
2876
1522
1920
5338
3624
-3080
-304
3752
-121
-1196
1920
-5414
1316
-8122
-1102
4046
5079
-9042
-7148
-3417
-821
-1732
6341
8297
-4719
5501
-5372
8163
-4397
5871
-2164
-2720
-3423
8051
-1661
8163
8637
8738
-3525
3649
-8934
-2819
6850
3652
9345
132
-4131
9512
-3834
-6554
9408
4627
3952
5434
-213
8244
7372
-9277
259
-6090
-2728
-828
7804
-1384
3920
-7420
-6508
3317
4439
243
447
4241
4758
-3440
7981
5678
-903
-7783
-6649
1771
4492
-1820
-8683
-1298
1097
-9753
-6757
-1040
-5578
31
7119
-71
-5951
-5395
-5081
6247
-7271
-8484
-1525
-728
-2843
1427
-9025
-9409
4695
1033
-4774
1398
6402
610
342
5952
7408
-6323
7861
9932
-5672
-5348
2646
-3492
7328
3546
-1086
3876
-2742
-449
-8232
-7500
1538
1117
7277
-4793
6036
2416
-798
-2742
4426
-4205
-5251
8839
-4699
-840
7008
-5578
-2358
-9854
7325
-5599
6453
-7026
7273
-142
-4885
7273
-7180
70
6704
-9506
9529
5756
845
7430
-8002
4915
2239
2940
3242
-6429
1356
6452
-8918
8759
-4821
-4596
2758
8506
7274
-3033
5894
-3609
1960
-2253
9018
6720
3312
-7119
1900
-4774
-1303
-942
-1199
7492
-6029
6205
-8154
9080
-6942
2263
7497
-3069
-91
-7073
-7882
751
-5476
-2477
-851
-9160
-7613
-381
5574
3596
-1659
-2991
9078
5076
-4528
5966
1876
-5743
8797
-6873
3501
8512
-7953
-2358
9642
-5274
-7753
9498
-2558
-5986
8301
5828
-8771
8617
5079
-1393
4206
-9263
4132
8104
6166
8592
7393
-383
-425
5127
-6219
7817
-4937
-3129
-8610
306
4448
-7213
9512
-8000
-5970
-8098
2378
-2205
1950
-7005
1416
3541
7612
3621
7618
5684
4736
1383
2799
6486
-6680
-3786
-3174
1460
8404
4665
-6163
9511
8964
6823
3635
3110
7374
4657
6195
9845
1383
-1182
1088
5410
-5943
-7337
6475
8637
4271
5246
6079
-7773
6276
5269
4472
-8935
3514
-6433
-2349
-5591
1525
-7334
1088
-7972
-7402
9169
-804
-702
1393
-6161
-2620
-595
2803
-912
-1345
-126
5570
7011
-8989
1023
2699
-3066
5263
-5406
-6030
3132
1589
-8892
4003
-5994
9761
7444
4072
8729
-7137
6956
4196
-2733
-9513
-2112
-5377
-9737
8061
35
-4669
-9942
-2381
3545
8487
-8865
-2289
8462
6006
1982
-6632
-1495
-2423
-9705
-5075
2390
9917
6018
2486
-3057
7415
4967
-5292
-9705
3823
7197
4301
3929
-9439
4978
2204
-9983
9343
-719
-8067
-1190
4183
-1837
-7348
-9018
505
-3225
-6737
-6248
8285
-4854
1848
-5532
530
-4737
-2007
7199
6207
7328
6392
-2910
3195
3637
7868
-7407
-8519
2738
8168
-15
-3213
-4419
1985
-4581
-9407
3009
-5699
-1272
-9876
-6864
3484
-8197
-4134
-9239
5835
5578
-8782
-5890
4201
7497
9067
5472
-8732
7426
-4445
-6044
7458
-6749
-3332
-568
148
-1185
-7501
8521
-6640
923
-6680
7458
-5216
-7811
-5339
-6537
3135
-6901
1952
-1508
9436
9240
-236
2017
3583
-5823
9064
6612
-2006
4376
-9032
-2687
8034
162
3609
-8928
1633
6703
2450
-3900
-496
5184
-6085
-9439
-8961
9091
-6146
-4165
9649
7325
-8281
-8877
-8199
3876
4058
8170
-8080
9899
-6040
4037
-9729
9228
8685
-9334
-2048
-1402
-5876
8989
6710
7088
-7398
7843
4140
998
-724
-5453
-2232
-9025
-1190
-921
9342
-8311
-8918
-525
-5931
2082
-793
-3671
-6371
-312
4920
4147
-7574
1479
762
488
-5328
-7975
7441
9952
8801
-7660
476
-6251
2234
4798
-2792
7751
-7613
6830
-6248
-2306
9100
2098
-6612
-4462
-2743
-4116
-2544
-3611
9262
1725
-5771
9097
6623
-3850
7802
4946
-7484
8159
-756
-636
6954
-5163
9012
2833
-4757
-2527
823
-9631
-7951
-7308
-5766
-931
908
-8613
3752
-8378
5521
-2884
5766
-8992
-8466
2098
8120
-2924
2395
4550
5128
6593
-2949
-3744
-344
-6211
2491
-2145
-9088
-1462
3311
-121
-4628
6891
-7948
-1101
6536
-5826
-9670
-3594
4394
-5639
-541
-9091
4028
3646
-3437
-6287
-8022
7578
9063
1048
142
-4102
-5708
9311
-6389
7359
-7245
-1650
3215
1851
-7751
-6849
7070
6437
-4669
8421
8104
-931
8265
6967
-8400
-6757
9877
-5903
-5923
4676
5343
6965
6005
4522
-1689
4119
-4581
4072
7172
3491
-4981
-6526
-6684
-9793
7524
6274
9162
-802
9632
-8266
-3391
-6703
3521
-1824
2240
-5679
-6744
-9624
-5225
6722
8506
9534
-6075
-2080
-9705
5363
5109
-7195
5987
-927
-9854
5339
-2506
4357
534
-5455
-384
3679
-8010
3946
-728
6009
-5765
-5830
-5011
-9237
-3538
890
5031
-4250
2703
-3848
4894
-5646
4107
-8467
8332
5139
-6248
-1658
-9870
2258
1249
5709
369
7403
7915
-2054
6704
-1267
-3885
-6499
-6768
-1198
6019
-7991
8424
-1372
-6364
5714
3116
2761
-2685
26
8628
-3536
-8256
3047
-7021
4604
-8151
9165
1350
-3279
7785
-9509
-3290
-9551
-4504
-9592
4492
6634
1801
806
4545
-6124
1511
7967
3592
-3997
8113
4169
3876
-5878
3161
-7065
-8256
5951
8544
933
-9499
6089
4426
6341
826
8888
-9872
-1259
-4301
6714
-2016
-3265
-246
8916
-3133
-7066
9030
8919
-6534
-5099
474
-4960
9844
-6980
-7633
447
-5609
-2751
-2068
817
-638
-5337
424
2290
-3913
-1334
-1047
-1931
-3248
-6806
-6806
1519
7164
-7794
-1119
5678
9476
2761
146
-29
7195
4667
9251
-7855
-8179
-5715
3226
-1195
4206
-4237
6517
-9230
-9582
-1290
2162
8051
2428
3015
-5699
8897
3608
-6067
3808
-7323
-1913
6338
-7685
5990
4902
-5068
-169
8557
-384
-4298
-4198
-1915
7725
-2767
4915
-9474
-2222
5599
-8473
-1866
-2212
6402
1233
4236
2670
8033
4328
5511
2518
1140
-7984
-52
-1938
7389
-7500
1440
-7621
-8518
4269
-7624
-803
2367
6845
7387
7892
8649
-2855
4606
3403
5305
-4399
865
-3624
2202
-9800
1126
-7977
2124
8402
-6643
2475
9845
-1368
31
5823
-2200
1763
259
-3313
-6565
2827
3756
2266
-8169
751
-8126
-5637
-2358
3084
-2550
8723
-2816
-937
9854
2239
-1382
-6956
369
-5331
3050
7868
9677
4842
7520
-2313
-4298
-3547
1161
8324
2303
3151
-2627
5300
1851
1642
-4581
4726
-6800
876
7896
450
3726
-7613
-5989
-1906
6346
-4188
-7426
-8539
4206
-1501
-6219
5891
-6859
6474
8778
7757
8887
3866
-1297
-2410
4733
-2665
3759
8974
541
-6281
-5023
-5950
1097
5978
-9966
-1764
-8318
1735
-8771
-931
-1525
-8101
8823
-792
-1851
2226
-1322
-9624
1311
-803
1319
8259
2311
2438
-1816
7068
-8870
9638
-8430
-3101
-1586
5544
-3645
3111
1959
8841
-4842
2792
-4434
5426
-8367
1113
-5402
5900
2357
3151
4727
-7205
7327
8841
-4236
2705
5814
-5618
-6569
-9153
-9768
-2222
9554
1293
-9702
5860
720
-7115
2963
-9422
-667
-9427
884
-4687
-9934
7913
-954
-6649
-6748
146
-8036
-9256
-568
8345
3056
8865
4734
-8686
-83
2542
-2830
4741
4426
-7993
9913
8741
-4392
-579
-7948
-2372
-2981
-4152
-4281
-3854
-4518
3904
4193
6267
998
-5629
-6832
-1182
-1666
1418
5978
-331
9197
-2426
1931
2491
-5762
2427
-6796
-3205
-6595
2700
1654
-6911
-232
7364
-7384
-5657
-2903
7296
6381
1601
-4817
-3201
7328
-3206
2579
-783
-2477
-6591
626
-8010
-4184
-6073
3616
-2743
-700
590
-9524
6195
-2873
1843
-7078
4319
-8235
8459
-2993
8903
-954
-7648
-6712
-420
-5210
-6618
-6537
-8072
9667
597
6703
9457
2440
-4532
-215
-5964
-9435
-9948
-7706
7099
-5348
-4401
6357
2745
8000
-6887
-4903
-3025
-2135
-9331
-489
-9897
-3817
6882
4204
-2002
4259
5437
5026
7636
4036
6874
5196
8225
3583
-5842
3203
7466
-9380
8892
9631
8271
-9119
-8013
8709
9160
2673
-7457
8230
5742
-2060
-6340
-7112
8297
4879
8095
-9530
-1393
-1422
-8435
1767
-4445
3634
-4725
4776
-3901
-9854
-6015
-3885
2431
8350
879
4820
-9376
-5552
9334
2606
6943
-819
-131
-8864
2129
7569
3414
1579
294
-8185
5068
-6389
-3713
-6942
5521
5420
-6717
-3025
-4391
-2648
-9027
-2865
-7734
-7596
9007
-9107
4311
8893
4241
6612
5900
3546
-5867
4920
2493
-6526
1280
8767
7810
3934
346
637
-9625
6745
8677
4658
4564
-6604
-7864
9755
1352
-8665
8958
1352
8160
-615
8172
-4735
-9798
1394
-513
-3006
-555
-4502
5649
6142
-1337
4357
-6424
1136
-9515
6924
-159
-2492
-918
5269
259
280
2889
-7520
2207
1789
2686
2706
8135
243
9911
-6463
-4469
-3457
-8791
7678
466
-8484
-4013
-6219
-4998
-7302
-2328
6236
8285
2138
791
-9811
6507
-2861
4822
3153
-5420
-8732
-1830
7204
-6758
-8273
-1658
8397
-654
-7040
7191
3043
-7550
-5458
2845
-8385
4287
-6612
4873
-191
-8623
-6859
-1081
-1925
-7864
-4026
-7168
1981
2238
5399
9240
4815
-8813
-6969
-6938
8426
2945
8119
-148
-3877
-2665
-5542
-2890
6745
-5632
-8736
5703
2602
3608
-5157
2709
-1276
3024
-4835
6660
-7205
8765
-5402
8406
6281
1207
-9331
-5401
454
-5330
-7191
1196
8738
-306
-8002
-381
9139
5496
-8103
1961
7247
-6291
6674
9354
5929
-2596
4281
6940
-6894
-5694
1383
7913
-9620
-4227
830
3710
-4526
70
-1824
-7245
-8158
9161
3298
7597
-5900
-8458
1682
668
-451
-8546
1923
9837
6828
4394
-7279
2579
7949
-4897
3047
1583
7091
-4712
-1059
6513
6612
-9456
-6956
9061
-1198
-1170
8485
-4228
-6064
-5209
-947
-2023
-4596
-3590
902
-2232
-7256
8285
-1786
-8572
-2424
-8923
-8959
1304
2891
-9435
4436
300
1382
-817
-9852
-1232
-5969
4562
-9947
-5238
-4986
1812
-1830
3744
-7195
2117
7588
5216
-7953
-4281
-1539
-6158
-1340
-8331
7350
3610
-5202
8406
3130
3107
6671
3948
8173
-9164
-9779
6714
9707
1422
1744
-2752
-6758
-6493
-8148
-6100
5734
6624
-7442
806
-3633
-7442
-9137
-6137
-1155
3621
-8266
1134
9578
2761
-9848
-3566
-3248
-8989
6766
8143
-6257
9793
-719
3384
8321
-4858
-5328
-1151
-5018
7351
7163
7370
-3394
-6433
-9813
1942
3634
-3730
3456
9117
340
9222
3013
-1375
3011
-9356
-4244
-1915
"#;
