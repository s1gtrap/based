# based

Hacked together draft for counting unique integers in rising bases.

Whereas overflows on a conventional ALU roll back to zero, the based iterator 
increments the base instead in addition to skipping numbers already output. To
avoid revisiting the added value varies, producing streams such as
(`(bitwidth, start base)`):

```
          (3, 2)          (4, 2)          (2, 4)          (3, 3)
           0b000          0b0000            4r00           3r000
           0b001          0b0001            4r01           3r001
           0b010          0b0010            4r02           3r002
           0b011          0b0011            4r03           3r010
           0b100          0b0100            4r10           3r011
           0b101          0b0101            4r11           3r012
           0b110          0b0110            4r12           3r020
           0b111          0b0111            4r13           3r021
           3r002          0b1000            4r20           3r022
           3r012          0b1001            4r21           3r100
           3r020          0b1010            4r22           3r101
           3r021          0b1011            4r23           3r102
           3r022          0b1100            4r30           3r110
           3r102          0b1101            4r31           3r111
           3r112          0b1110            4r32           3r112
           3r120          0b1111            4r33           3r120
           3r121          3r0002            5r04           3r121
           3r122          3r0012            5r14           3r122
           3r200          3r0020            5r24           3r200
           3r201          3r0021            5r34           3r201
           3r202          3r0022            5r40           3r202
           3r210          3r0102            5r41           3r210
           3r211          3r0112            5r42           3r211
           3r212          3r0120            5r43           3r212
           3r220          3r0121            5r44           3r220
           3r221          3r0122            6r05           3r221
           3r222          3r0200            6r15           3r222
           4r003          3r0201            6r25           4r003
           4r013          3r0202            6r35           4r013
           4r023          3r0210            6r45           4r023
```
