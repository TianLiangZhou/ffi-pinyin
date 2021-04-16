<?php

include __DIR__ . '/../src/Pinyin.php';


$py = FastFFI\Pinyin\Pinyin::new();

echo "无音标: ", $py->plain("中国人..."), "\n";
echo "首字母: ", $py->letter("中国人"), "\n";
echo "音标: ", $py->tone("中国人"), "\n";
echo "多音音标:", $py->tone_multi("中国人"), "\n";
