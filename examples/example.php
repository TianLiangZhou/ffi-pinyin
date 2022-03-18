<?php

include __DIR__ . '/../src/Pinyin.php';


$py = FastFFI\Pinyin\Pinyin::new();

echo "无音标: ", $py->plain("中国人...😄😄👩", false, false), "\n";
echo "音标: ", $py->tone("中国人", false, false), "\n";
echo "音标数字: ", $py->toneNum("中国人", false, false), "\n";
echo "音标数字结尾: ", $py->toneNumEnd("中国人", false, false), "\n";
echo "首字母: ", $py->letter("中国人", false, false), "\n";
echo "音标转换模式: ", $py->tone("中国人😄😄", true, false), "\n";
echo "音标多音词模式: ", $py->tone("中国人", false, true), "\n";

echo "音标未识别跳过: ", $py->plain("PHP永远滴神，rust永远的神", true, false, '-'), "\n";
echo "音标未识别不分隔: ", $py->plain("PHP永远滴神，rust永远的神", false, false, '-', true), "\n";

var_export($py->plainArray("PHP永远滴神，rust永远的神", false, false, true));
echo "\n";
var_export($py->toneArray("我的中国心，永恒之❤️", true, false));
echo "\n";
var_export($py->toneNumArray("我的中国心，永恒之❤️", false, false));
echo "\n";
var_export($py->toneNumEndArray("我的中国心，永恒之❤️", true, false, true));
echo "\n";
var_export($py->letterArray("我的中国心，永恒之❤️", false, false));
echo "\n";

$test = <<<EOF
关关雎鸠，在河之洲。窈窕淑女，君子好逑。
参差荇菜，左右流之。窈窕淑女，寤寐求之。
求之不得，寤寐思服。悠哉悠哉，辗转反侧。
参差荇菜，左右采之。窈窕淑女，琴瑟友之。
参差荇菜，左右芼之。窈窕淑女，钟鼓乐之。
EOF;
for ($i = 0 ; $i < 100; $i++) {
    echo $i, " = ", $py->tone($test), "\n";
}
