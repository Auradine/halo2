use halo2_gadgets::sha256::{BlockWord, BLOCK_SIZE};
use std::num::ParseIntError;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[test]
fn test_sha256_main() {
    let ch_sh = "010001050303af1c7045f3457951a330683e8ed7304504b3de9ab866a3a3b300f9b4d769311e2098a060686c2ba52707ef471dac9178891cb0c288698ead6984051050c2e34391000c00ff1301c02bc02f009e009c010000b00016000000170000000d00220020060305030403030302030806080b0805080a0804080906010501040103010201002b000302030400330047004500170041045f413967369ad03b0003fca54f79c018e6a726ad459c34aa5ef9e4061a09b4a0ce212c30bf14ad0f2b9f6a4837bf9308c9c042c43e3740bfb0285f0aaa285a11002d0003020100000b00020100000a000e000c001701000101010201030104000f000101001c00024001000900020100020000970303e618a9dfeb826fdd66757813149920579ca3b0cd566fd60ac4668268827f6c622098a060686c2ba52707ef471dac9178891cb0c288698ead6984051050c2e34391130100004f003300450017004104ecfe6e4489100f404beb822b6d065de083b0bf9474c4dcf4541b52da11685c1d98afca65695e94a06376f29daceff0e3221debbe0716ec94dbcb45ed51dfaac3002b00020304";
    let decrypted_serv_ext = "0800000200000b0009df000009db00058a308205863082050da003020102021005076f66d11b692256ccacd546ffec53300a06082a8648ce3d0403033056310b300906035504061302555331153013060355040a130c446967694365727420496e633130302e06035504031327446967694365727420544c53204879627269642045434320534841333834203230323020434131301e170d3231303131313030303030305a170d3232303131383233353935395a3072310b3009060355040613025553311330110603550408130a43616c69666f726e6961311630140603550407130d53616e204672616e636973636f31193017060355040a1310436c6f7564666c6172652c20496e632e311b301906035504031312636c6f7564666c6172652d646e732e636f6d3059301306072a8648ce3d020106082a8648ce3d0301070342000417ad1fe835af70d38d9c9e64fd471e5b970c0ad110a826321136664d1299c3e131bbf5216373dda5c1c1a0f06da4c45ee1c2dbdaf90d34801af7b9e03af2d574a382039f3082039b301f0603551d230418301680140abc0829178ca5396d7a0ece33c72eb3edfbc37a301d0603551d0e04160414e1b6fc06f9b98b05f4c1e2489b02b90bc1b53d793081a60603551d1104819e30819b8212636c6f7564666c6172652d646e732e636f6d82142a2e636c6f7564666c6172652d646e732e636f6d820f6f6e652e6f6e652e6f6e652e6f6e658704010101018704010000018704a29f24018704a29f2e01871026064700470000000000000000001111871026064700470000000000000000001001871026064700470000000000000000000064871026064700470000000000000000006400300e0603551d0f0101ff040403020780301d0603551d250416301406082b0601050507030106082b060105050703023081970603551d1f04818f30818c3044a042a040863e687474703a2f2f63726c332e64696769636572742e636f6d2f4469676943657274544c53487962726964454343534841333834323032304341312e63726c3044a042a040863e687474703a2f2f63726c342e64696769636572742e636f6d2f4469676943657274544c53487962726964454343534841333834323032304341312e63726c304b0603551d2004443042303606096086480186fd6c01013029302706082b06010505070201161b687474703a2f2f7777772e64696769636572742e636f6d2f4350533008060667810c01020230818306082b0601050507010104773075302406082b060105050730018618687474703a2f2f6f6373702e64696769636572742e636f6d304d06082b060105050730028641687474703a2f2f636163657274732e64696769636572742e636f6d2f4469676943657274544c53487962726964454343534841333834323032304341312e637274300c0603551d130101ff0402300030820104060a2b06010401d6790204020481f50481f200f00076002979bef09e393921f056739f63a577e5be577d9c600af8f94d5d265c255dc78400000176f2e812a80000040300473045022100d1b2f68cf853959de4d453063482028a0aea8aa7bc271efb561ed114641fae67022025b186dd1b2ae78f01c440f6c31678ab61bff63a34fc4788130765f460bb34420076002245450759552456963fa12ff1f76d86e0232663adc04b7f5dc6835c6ee20f0200000176f2e8130f000004030047304502210095dd1a674a2cecac9d6f8bfe3cfea4f53e8725658237379d66bde45d0f68245902207565fe30bb806bcce2b8a18896a8e802268ebecff821faad85a00d87a1d6f134300a06082a8648ce3d0403030367003064023024c2cf6cbdf6aed1c9d51f4a742e3c3dd1c03edcd71bd394715bfea5861626820122d30a6efc98b5d2e2b9e5076977960230457b6f82a67db662c33185d5b5355d4f4c8488ac1a003d0c8440dcb0a7ca1c1327151e37f946c3aed9fdf9b9238b7f2a0000000447308204433082032ba00302010202100a275fe704d6eecb23d5cd5b4b1a4e04300d06092a864886f70d01010c05003061310b300906035504061302555331153013060355040a130c446967694365727420496e6331193017060355040b13107777772e64696769636572742e636f6d3120301e06035504031317446967694365727420476c6f62616c20526f6f74204341301e170d3230303932333030303030305a170d3330303932323233353935395a3056310b300906035504061302555331153013060355040a130c446967694365727420496e633130302e06035504031327446967694365727420544c532048796272696420454343205348413338342032303230204341313076301006072a8648ce3d020106052b8104002203620004c11bc69a5b98d9a429a0e9d404b5dbeba6b26c55c0ffed98c6492f062751cbbf70c1057ac3b19d8789baadb41317c9a8b483c8b890d1cc7435363c8372b0b5d0f72269c8f180c47b408fcf6887265c3989f14d914dda898be403c343e5bf2f73a38201ae308201aa301d0603551d0e041604140abc0829178ca5396d7a0ece33c72eb3edfbc37a301f0603551d2304183016801403de503556d14cbb66f0a3e21b1bc397b23dd155300e0603551d0f0101ff040403020186301d0603551d250416301406082b0601050507030106082b0601050507030230120603551d130101ff040830060101ff020100307606082b06010505070101046a3068302406082b060105050730018618687474703a2f2f6f6373702e64696769636572742e636f6d304006082b060105050730028634687474703a2f2f636163657274732e64696769636572742e636f6d2f4469676943657274476c6f62616c526f6f7443412e637274307b0603551d1f047430723037a035a0338631687474703a2f2f63726c332e64696769636572742e636f6d2f4469676943657274476c6f62616c526f6f7443412e63726c3037a035a0338631687474703a2f2f63726c342e64696769636572742e636f6d2f4469676943657274476c6f62616c526f6f7443412e63726c30300603551d20042930273007060567810c01013008060667810c0102013008060667810c0102023008060667810c010203300d06092a864886f70d01010c05000382010100de3a971b85bd7b8c0a58e3e3f00b06007aaf44632a7fdd816d1118a6faf73860b1b03962bdb87b2ef008c9925b73df6590b7deb457acc02f1c99674d8b5ef201a1adf79653cdd5df5127370646f26f3c31892c4a16291182e7edb7421493e0d960fa3f1c64cb32c3311724d9af3e556fff0f806f1b0108caa40463c8da534cf21926c4f69ddf5cfa9f9f125ea25d1ff6d8178eb662f81d81b60a3b2b686cf0324e0af7998239b148de0b3a89c75f8d2f273c9c02bc21bd87c670be22bad6cdf44bbd536f7bc29a37d12d047ed3b8cc0d5b84a90c2986afa7d8d61ded0e31e19136fd3c0dface8689ceed1c230dbae6f66dea10a6616242fbf8fe7e067070fc0b00000f00004a04030046304402201ad109b284084dbd51699f4a965b3969ec7a972e14b1e5e79f9dc7a9ff876976022000ea18516c04962f72138d2c1babd86c58f508021e0b3bcd1ff869f7b45bec481400002088c84bf0dcc10547d0b4368d100c1b3e3383eeb5f16a7e91bda186179bbeda75";
    let h3 = "fb12b7580580d48ad0de4a06cd1b9ff9d40d4ff668cc38812487d01e447bd696";

    // println!("{:?}", ch_sh);
    // println!("{:?}", decrypted_serv_ext);
    // println!("{:?}", h3);

    let plaintext = format!("{}{}", ch_sh, decrypted_serv_ext);
    // println!("plaintext({}): {:?}", plaintext.len(), plaintext);

    let decoded = decode_hex(&plaintext).unwrap();
    println!("decoded({}): {:?}", decoded.len(), decoded);

    // pad decoded to multiples of 512

    // Test vector: "abc"
    let test_input = [
        BlockWord(Some(0b01100001011000100110001110000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000000000)),
        BlockWord(Some(0b00000000000000000000000000011000)),
    ];

    // println!("{:?}", &test_input);

    // // Create a message of length 31 blocks
    let mut input = Vec::with_capacity(31 * BLOCK_SIZE);
    for _ in 0..31 {
        input.extend_from_slice(&test_input);
    }

    // println!("{:?}", &input);
}
