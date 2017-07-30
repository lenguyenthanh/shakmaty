// Fixed shift magics found by Volker Annuss.
// From: http://talkchess.com/forum/viewtopic.php?p=670709#670709

pub struct Magic {
    pub mask: u64,
    pub factor: u64,
    pub offset: usize,
}

pub const BISHOP_MAGICS: [Magic; 64] = [
    Magic { mask: 0x0040201008040200, factor: 0x0000404040404040, offset: 33104 },
    Magic { mask: 0x0000402010080400, factor: 0x0000a060401007fc, offset:  4094 },
    Magic { mask: 0x0000004020100a00, factor: 0x0000401020200000, offset: 24764 },
    Magic { mask: 0x0000000040221400, factor: 0x0000806004000000, offset: 13882 },
    Magic { mask: 0x0000000002442800, factor: 0x0000440200000000, offset: 23090 },
    Magic { mask: 0x0000000204085000, factor: 0x0000080100800000, offset: 32640 },
    Magic { mask: 0x0000020408102000, factor: 0x0000104104004000, offset: 11558 },
    Magic { mask: 0x0002040810204000, factor: 0x0000020020820080, offset: 32912 },
    Magic { mask: 0x0020100804020000, factor: 0x0000040100202004, offset: 13674 },
    Magic { mask: 0x0040201008040000, factor: 0x0000020080200802, offset:  6109 },
    Magic { mask: 0x00004020100a0000, factor: 0x0000010040080200, offset: 26494 },
    Magic { mask: 0x0000004022140000, factor: 0x0000008060040000, offset: 17919 },
    Magic { mask: 0x0000000244280000, factor: 0x0000004402000000, offset: 25757 },
    Magic { mask: 0x0000020408500000, factor: 0x00000021c100b200, offset: 17338 },
    Magic { mask: 0x0002040810200000, factor: 0x0000000400410080, offset: 16983 },
    Magic { mask: 0x0004081020400000, factor: 0x000003f7f05fffc0, offset: 16659 },
    Magic { mask: 0x0010080402000200, factor: 0x0004228040808010, offset: 13610 },
    Magic { mask: 0x0020100804000400, factor: 0x0000200040404040, offset:  2224 },
    Magic { mask: 0x004020100a000a00, factor: 0x0000400080808080, offset: 60405 },
    Magic { mask: 0x0000402214001400, factor: 0x0000200200801000, offset:  7983 },
    Magic { mask: 0x0000024428002800, factor: 0x0000240080840000, offset:    17 },
    Magic { mask: 0x0002040850005000, factor: 0x000018000c03fff8, offset: 34321 },
    Magic { mask: 0x0004081020002000, factor: 0x00000a5840208020, offset: 33216 },
    Magic { mask: 0x0008102040004000, factor: 0x0000058408404010, offset: 17127 },
    Magic { mask: 0x0008040200020400, factor: 0x0002022000408020, offset:  6397 },
    Magic { mask: 0x0010080400040800, factor: 0x0000402000408080, offset: 22169 },
    Magic { mask: 0x0020100a000a1000, factor: 0x0000804000810100, offset: 42727 },
    Magic { mask: 0x0040221400142200, factor: 0x000100403c0403ff, offset:   155 },
    Magic { mask: 0x0002442800284400, factor: 0x00078402a8802000, offset:  8601 },
    Magic { mask: 0x0004085000500800, factor: 0x0000101000804400, offset: 21101 },
    Magic { mask: 0x0008102000201000, factor: 0x0000080800104100, offset: 29885 },
    Magic { mask: 0x0010204000402000, factor: 0x0000400480101008, offset: 29340 },
    Magic { mask: 0x0004020002040800, factor: 0x0001010102004040, offset: 19785 },
    Magic { mask: 0x0008040004081000, factor: 0x0000808090402020, offset: 12258 },
    Magic { mask: 0x00100a000a102000, factor: 0x0007fefe08810010, offset: 50451 },
    Magic { mask: 0x0022140014224000, factor: 0x0003ff0f833fc080, offset:  1712 },
    Magic { mask: 0x0044280028440200, factor: 0x007fe08019003042, offset: 78475 },
    Magic { mask: 0x0008500050080400, factor: 0x0000202040008040, offset:  7855 },
    Magic { mask: 0x0010200020100800, factor: 0x0001004008381008, offset: 13642 },
    Magic { mask: 0x0020400040201000, factor: 0x0000802003700808, offset:  8156 },
    Magic { mask: 0x0002000204081000, factor: 0x0000208200400080, offset:  4348 },
    Magic { mask: 0x0004000408102000, factor: 0x0000104100200040, offset: 28794 },
    Magic { mask: 0x000a000a10204000, factor: 0x0003ffdf7f833fc0, offset: 22578 },
    Magic { mask: 0x0014001422400000, factor: 0x0000008840450020, offset: 50315 },
    Magic { mask: 0x0028002844020000, factor: 0x0000020040100100, offset: 85452 },
    Magic { mask: 0x0050005008040200, factor: 0x007fffdd80140028, offset: 32816 },
    Magic { mask: 0x0020002010080400, factor: 0x0000202020200040, offset: 13930 },
    Magic { mask: 0x0040004020100800, factor: 0x0001004010039004, offset: 17967 },
    Magic { mask: 0x0000020408102000, factor: 0x0000040041008000, offset: 33200 },
    Magic { mask: 0x0000040810204000, factor: 0x0003ffefe0c02200, offset: 32456 },
    Magic { mask: 0x00000a1020400000, factor: 0x0000001010806000, offset:  7762 },
    Magic { mask: 0x0000142240000000, factor: 0x0000000008403000, offset:  7794 },
    Magic { mask: 0x0000284402000000, factor: 0x0000000100202000, offset: 22761 },
    Magic { mask: 0x0000500804020000, factor: 0x0000040100200800, offset: 14918 },
    Magic { mask: 0x0000201008040200, factor: 0x0000404040404000, offset: 11620 },
    Magic { mask: 0x0000402010080400, factor: 0x00006020601803f4, offset: 15925 },
    Magic { mask: 0x0002040810204000, factor: 0x0003ffdfdfc28048, offset: 32528 },
    Magic { mask: 0x0004081020400000, factor: 0x0000000820820020, offset: 12196 },
    Magic { mask: 0x000a102040000000, factor: 0x0000000010108060, offset: 32720 },
    Magic { mask: 0x0014224000000000, factor: 0x0000000000084030, offset: 26781 },
    Magic { mask: 0x0028440200000000, factor: 0x0000000001002020, offset: 19817 },
    Magic { mask: 0x0050080402000000, factor: 0x0000000040408020, offset: 24732 },
    Magic { mask: 0x0020100804020000, factor: 0x0000004040404040, offset: 25468 },
    Magic { mask: 0x0040201008040200, factor: 0x0000404040404040, offset: 10186 },
];

pub const ROOK_MAGICS: [Magic; 64] = [
    Magic { mask: 0x000101010101017e, factor: 0x00280077ffebfffe, offset: 41305 },
    Magic { mask: 0x000202020202027c, factor: 0x2004010201097fff, offset: 14326 },
    Magic { mask: 0x000404040404047a, factor: 0x0010020010053fff, offset: 24477 },
    Magic { mask: 0x0008080808080876, factor: 0x0030002ff71ffffa, offset:  8223 },
    Magic { mask: 0x001010101010106e, factor: 0x7fd00441ffffd003, offset: 49795 },
    Magic { mask: 0x002020202020205e, factor: 0x004001d9e03ffff7, offset: 60546 },
    Magic { mask: 0x004040404040403e, factor: 0x004000888847ffff, offset: 28543 },
    Magic { mask: 0x008080808080807e, factor: 0x006800fbff75fffd, offset: 79282 },
    Magic { mask: 0x0001010101017e00, factor: 0x000028010113ffff, offset:  6457 },
    Magic { mask: 0x0002020202027c00, factor: 0x0020040201fcffff, offset:  4125 },
    Magic { mask: 0x0004040404047a00, factor: 0x007fe80042ffffe8, offset: 81021 },
    Magic { mask: 0x0008080808087600, factor: 0x00001800217fffe8, offset: 42341 },
    Magic { mask: 0x0010101010106e00, factor: 0x00001800073fffe8, offset: 14139 },
    Magic { mask: 0x0020202020205e00, factor: 0x007fe8009effffe8, offset: 19465 },
    Magic { mask: 0x0040404040403e00, factor: 0x00001800602fffe8, offset:  9514 },
    Magic { mask: 0x0080808080807e00, factor: 0x000030002fffffa0, offset: 71090 },
    Magic { mask: 0x00010101017e0100, factor: 0x00300018010bffff, offset: 75419 },
    Magic { mask: 0x00020202027c0200, factor: 0x0003000c0085fffb, offset: 33476 },
    Magic { mask: 0x00040404047a0400, factor: 0x0004000802010008, offset: 27117 },
    Magic { mask: 0x0008080808760800, factor: 0x0002002004002002, offset: 85964 },
    Magic { mask: 0x00101010106e1000, factor: 0x0002002020010002, offset: 54915 },
    Magic { mask: 0x00202020205e2000, factor: 0x0001002020008001, offset: 36544 },
    Magic { mask: 0x00404040403e4000, factor: 0x0000004040008001, offset: 71854 },
    Magic { mask: 0x00808080807e8000, factor: 0x0000802000200040, offset: 37996 },
    Magic { mask: 0x000101017e010100, factor: 0x0040200010080010, offset: 30398 },
    Magic { mask: 0x000202027c020200, factor: 0x0000080010040010, offset: 55939 },
    Magic { mask: 0x000404047a040400, factor: 0x0004010008020008, offset: 53891 },
    Magic { mask: 0x0008080876080800, factor: 0x0000040020200200, offset: 56963 },
    Magic { mask: 0x001010106e101000, factor: 0x0000010020020020, offset: 77451 },
    Magic { mask: 0x002020205e202000, factor: 0x0000010020200080, offset: 12319 },
    Magic { mask: 0x004040403e404000, factor: 0x0000008020200040, offset: 88500 },
    Magic { mask: 0x008080807e808000, factor: 0x0000200020004081, offset: 51405 },
    Magic { mask: 0x0001017e01010100, factor: 0x00fffd1800300030, offset: 72878 },
    Magic { mask: 0x0002027c02020200, factor: 0x007fff7fbfd40020, offset:   676 },
    Magic { mask: 0x0004047a04040400, factor: 0x003fffbd00180018, offset: 83122 },
    Magic { mask: 0x0008087608080800, factor: 0x001fffde80180018, offset: 22206 },
    Magic { mask: 0x0010106e10101000, factor: 0x000fffe0bfe80018, offset: 75186 },
    Magic { mask: 0x0020205e20202000, factor: 0x0001000080202001, offset:   681 },
    Magic { mask: 0x0040403e40404000, factor: 0x0003fffbff980180, offset: 36453 },
    Magic { mask: 0x0080807e80808000, factor: 0x0001fffdff9000e0, offset: 20369 },
    Magic { mask: 0x00017e0101010100, factor: 0x00fffeebfeffd800, offset:  1981 },
    Magic { mask: 0x00027c0202020200, factor: 0x007ffff7ffc01400, offset: 13343 },
    Magic { mask: 0x00047a0404040400, factor: 0x0000408104200204, offset: 10650 },
    Magic { mask: 0x0008760808080800, factor: 0x001ffff01fc03000, offset: 57987 },
    Magic { mask: 0x00106e1010101000, factor: 0x000fffe7f8bfe800, offset: 26302 },
    Magic { mask: 0x00205e2020202000, factor: 0x0000008001002020, offset: 58357 },
    Magic { mask: 0x00403e4040404000, factor: 0x0003fff85fffa804, offset: 40546 },
    Magic { mask: 0x00807e8080808000, factor: 0x0001fffd75ffa802, offset:     0 },
    Magic { mask: 0x007e010101010100, factor: 0x00ffffec00280028, offset: 14967 },
    Magic { mask: 0x007c020202020200, factor: 0x007fff75ff7fbfd8, offset: 80361 },
    Magic { mask: 0x007a040404040400, factor: 0x003fff863fbf7fd8, offset: 40905 },
    Magic { mask: 0x0076080808080800, factor: 0x001fffbfdfd7ffd8, offset: 58347 },
    Magic { mask: 0x006e101010101000, factor: 0x000ffff810280028, offset: 20381 },
    Magic { mask: 0x005e202020202000, factor: 0x0007ffd7f7feffd8, offset: 81868 },
    Magic { mask: 0x003e404040404000, factor: 0x0003fffc0c480048, offset: 59381 },
    Magic { mask: 0x007e808080808000, factor: 0x0001ffffafd7ffd8, offset: 84404 },
    Magic { mask: 0x7e01010101010100, factor: 0x00ffffe4ffdfa3ba, offset: 45811 },
    Magic { mask: 0x7c02020202020200, factor: 0x007fffef7ff3d3da, offset: 62898 },
    Magic { mask: 0x7a04040404040400, factor: 0x003fffbfdfeff7fa, offset: 45796 },
    Magic { mask: 0x7608080808080800, factor: 0x001fffeff7fbfc22, offset: 66994 },
    Magic { mask: 0x6e10101010101000, factor: 0x0000020408001001, offset: 67204 },
    Magic { mask: 0x5e20202020202000, factor: 0x0007fffeffff77fd, offset: 32448 },
    Magic { mask: 0x3e40404040404000, factor: 0x0003ffffbf7dfeec, offset: 62946 },
    Magic { mask: 0x7e80808080808000, factor: 0x0001ffff9dffa333, offset: 17005 },
];
