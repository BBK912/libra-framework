///
import { readFileSync, writeFileSync } from 'fs';

const str = readFileSync('./sample_export_recovery.json').toString()
let all = JSON.parse(str)
console.log(all.length)

let duplicates = [
"dd62f3295e25ec68dec66616ef9734a4be35ad8127f57849e97a3ffb81e5854d",
"be35ad8127f57849e97a3ffb81e5854d",
"9a591ef6e13218b17cf8f0a6064b7387d502efc921b380922539c24649679339",
"d502efc921b380922539c24649679339",
"f173109c285487af8009658ef431e845e8a1bf5e60fa00c2b792098b12fde964",
"e8a1bf5e60fa00c2b792098b12fde964",
"57d3919ed378a98e4b4768bcba49a7b4da71b9b90ca4a6d59da029b027faaeea",
"da71b9b90ca4a6d59da029b027faaeea",
"52a76e6fd71cc17dec57d3d7a278e223402281b3e33b3c135b6616fcf8059756",
"402281b3e33b3c135b6616fcf8059756",
"9880f804f4f7fd16096d0bf22b29e6a6eedeb8a9de35ddae4469a6d6dfef0747",
"eedeb8a9de35ddae4469a6d6dfef0747",
"5412770eddd3236e8c41f8646030d636d493f481d4c84de18ab4a2d5080d181a",
"d493f481d4c84de18ab4a2d5080d181a",
"36ad103acecd5f254678cda4e97311afb2c091a8b0514e64ea42d80328db2729",
"b2c091a8b0514e64ea42d80328db2729",
"f50f44e2b2f0d44ea83457ad6c4eca7964d54a14ba2f83c14de003fac6e8f6ad",
"64d54a14ba2f83c14de003fac6e8f6ad",
"be11f648039274967ba1d75dbd6189c10fae13eac07733bb150bb135f5739f3a",
"fae13eac07733bb150bb135f5739f3a",
"8f3a3c82cbfa4687a59ef3d33d3e04db31041d093f75532b8c37e0c2ef9788f0",
"31041d093f75532b8c37e0c2ef9788f0",
"1dda0a12881542b13b8417de792d719d24b04714269229275bf20fb31c9463b8",
"24b04714269229275bf20fb31c9463b8",
"fb9ca03d984a0221ca09c42ea93872d2320084c8a5f308cdb9471589c05f2d97",
"320084c8a5f308cdb9471589c05f2d97",
"9ef44a224f9025a10d927e680cea39d9e32e8b983f2c70ceb1f765d3b3964985",
"e32e8b983f2c70ceb1f765d3b3964985",
"f12b02cb7bece145dc7fee2dedfe39d4a88b19e3c56c425408b3caea5ac93cca",
"a88b19e3c56c425408b3caea5ac93cca",
"b1d4a43d4067ba9cd97e1d14bf2f67148a6c5b3876f42f368bde454c1d11ef60",
"8a6c5b3876f42f368bde454c1d11ef60",
"98f4293fd1c2ec031cdae54c33eaa3d85d4d1d01f7badb83f4cb7b6bec816eef",
"5d4d1d01f7badb83f4cb7b6bec816eef",
"77aa9e8d32088aaaa88d885bc341d04bc44a50622c2d465e6c15982b8e127349",
"c44a50622c2d465e6c15982b8e127349",
"3d89b9dabc828735e1215c0714c851f8647f14a85feb267ad78368c10e203322",
"647f14a85feb267ad78368c10e203322"]


let b = all.filter((e: string) => e.account.includes("0000000000000000000000000000000000000000000000000000000000000003")
  // && !e.account.includes("38c091fb1fd44d7913ae3796fb2c98b5")
  // && !e.account.includes("b84c2c84e241733dc685dde2c55338a")
  // && !e.account.includes("d49e2f74b3f10c944f2fa4063f1ab4e3")
  // && !e.account.includes("8e73e907027707677facc7f272c0c24b")
  // && !e.account.includes("385a69bfb47273ad316d48d2a2bcafcf")
  // && !e.account.includes("44f53d9235410335f10ca433d544ab89")
  // && !e.account.includes("ff5d10d60a1ee396153e8eafb23959b4")
  // && !e.account.includes("95d9a5635b751dc137bc22ff265e5d9c7f")
  // && !e.account.includes("95e42cd20d430f5ab50e4693f8465d48")
  // && !e.account.includes("2624f05af4b4f23ef888fb3e119751c1")

)

// console.log(b.length)",
console.log(b)
writeFileSync('./test.json', JSON.stringify(b))