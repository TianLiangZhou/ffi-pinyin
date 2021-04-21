<?php

include __DIR__ . '/../src/Pinyin.php';


$py = FastFFI\Pinyin\Pinyin::new();

echo "无音标: ", $py->plain("中国人..."), "\n";
echo "首字母: ", $py->letter("中国人"), "\n";
echo "音标: ", $py->tone("中国人"), "\n";
echo "多音音标:", $py->tone_multi("中国人"), "\n";

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
