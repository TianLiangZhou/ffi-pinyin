<?php


use Overtrue\Pinyin\Pinyin;

class Bench
{
    public function benchFFI()
    {
        $pinyin = \FastFFI\Pinyin\Pinyin::new();
        $test = <<<EOF
关关雎鸠，在河之洲。窈窕淑女，君子好逑。
参差荇菜，左右流之。窈窕淑女，寤寐求之。
求之不得，寤寐思服。悠哉悠哉，辗转反侧。
参差荇菜，左右采之。窈窕淑女，琴瑟友之。
参差荇菜，左右芼之。窈窕淑女，钟鼓乐之。
EOF;
        for ($i = 0 ; $i < 100; $i++) {
            $pinyin->tone($test);
        }
    }

    public function benchNative()
    {
        $pinyin = new Pinyin();
        $test = <<<EOF
关关雎鸠，在河之洲。窈窕淑女，君子好逑。
参差荇菜，左右流之。窈窕淑女，寤寐求之。
求之不得，寤寐思服。悠哉悠哉，辗转反侧。
参差荇菜，左右采之。窈窕淑女，琴瑟友之。
参差荇菜，左右芼之。窈窕淑女，钟鼓乐之。
EOF;
        for ($i = 0 ; $i < 100; $i++) {
            $pinyin->permalink($test);
        }
    }

    public function benchNativeMemory()
    {
        $pinyin = new Pinyin('\\Overtrue\\Pinyin\\MemoryFileDictLoader');

        $test = <<<EOF
关关雎鸠，在河之洲。窈窕淑女，君子好逑。
参差荇菜，左右流之。窈窕淑女，寤寐求之。
求之不得，寤寐思服。悠哉悠哉，辗转反侧。
参差荇菜，左右采之。窈窕淑女，琴瑟友之。
参差荇菜，左右芼之。窈窕淑女，钟鼓乐之。
EOF;

        for ($i = 0 ; $i < 100; $i++) {
            $pinyin->permalink($test);
        }
    }

    public function benchNativeGenerator()
    {
        $pinyin = new Pinyin('\\Overtrue\\Pinyin\\GeneratorFileDictLoader');

        $test = <<<EOF
关关雎鸠，在河之洲。窈窕淑女，君子好逑。
参差荇菜，左右流之。窈窕淑女，寤寐求之。
求之不得，寤寐思服。悠哉悠哉，辗转反侧。
参差荇菜，左右采之。窈窕淑女，琴瑟友之。
参差荇菜，左右芼之。窈窕淑女，钟鼓乐之。
EOF;

        for ($i = 0 ; $i < 100; $i++) {
            $pinyin->permalink($test);
        }
    }
}
