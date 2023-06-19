<?php
namespace FastFFI\Pinyin;

use FFI;
use RuntimeException;

class Pinyin
{
    /**
     * @var ?Pinyin
     */
    private static ?Pinyin $py = null;

    /**
     * @var FFI
     */
    private FFI $ffi;

    /**
     * 无音标模式
     */
    const Plain = 1;

    /**
     * 音标模式
     */
    const Tone = 2;

    /**
     * 音字母模式
     */
    const Letter = 3;

    /**
     * 音标数字模式
     */
    const ToneNum = 4;

    /**
     * 音标数字末尾模式
     */
    const ToneNumEnd = 5;

    /**
     * 默认分隔符
     */
    const Separator = ' ';

    /**
     * FFIPinyin constructor.
     */
    private function __construct()
    {
        if (!ini_get('ffi.enable')) {
            throw new RuntimeException("请设置php.ini中的ffi.enable参数");
        }
        $this->ffi = $this->makeFFI();
    }

    /**
     * @return static
     */
    public static function new(): Pinyin
    {
        if (self::$py == null) {
            self::$py = new static();
        }
        return self::$py;
    }

    /**
     * @return FFI
     */
    public function getFfi(): FFI
    {
        return $this->ffi;
    }

    /**
     * @return void
     */
    private function __clone()
    {

    }

    /**
     * 生成URL Slugs
     *
     * @param string $str
     * @param string $separator
     * @return string
     */
    public function slug(string $str, string $separator = '-'): string
    {
        if (empty($str)) {
            return "";
        }
        return $this->toPinyin($str, false, false, $separator, false, self::Plain, 1);
    }

    /**
     * 普通风格没有音调
     *
     * @param string $str
     * @param bool $isSkipUnknown 跳过未识别字符
     * @param bool $isMulti 是否多音字模式
     * @param string $separator 分隔符默认 '-'
     * @param bool $notSplitUnknownChar 不分隔未识别的字符
     * @return string
     */
    public function plain(string $str, bool $isSkipUnknown = true, bool $isMulti = false, string $separator = self::Separator, bool $notSplitUnknownChar = false): string
    {
        if (empty($str)) {
            return "";
        }
        return $this->toPinyin($str, (int) $isSkipUnknown, (int) $isMulti, $separator, (int) $notSplitUnknownChar, self::Plain);
    }

    /**
     * 音调
     *
     * @param string $str
     * @param bool $isSkipUnknown 跳过未识别字符
     * @param bool $isMulti 是否多音字模式
     * @param string $separator 分隔符默认 '-'
     * @param bool $notSplitUnknownChar 不分隔未识别的字符
     * @return string
     */
    public function tone(string $str, bool $isSkipUnknown = true, bool $isMulti = false, string $separator = self::Separator, bool $notSplitUnknownChar = false): string
    {
        if (empty($str)) {
            return "";
        }
        return $this->toPinyin($str, (int) $isSkipUnknown, (int) $isMulti, $separator, $notSplitUnknownChar, self::Tone);
    }

    /**
     * 音调为数字
     *
     * @param string $str
     * @param bool $isSkipUnknown 跳过未识别字符
     * @param bool $isMulti 是否多音字模式
     * @param string $separator 分隔符默认 '-'
     * @param bool $notSplitUnknownChar 不分隔未识别的字符
     * @return string
     */
    public function toneNum(string $str, bool $isSkipUnknown = true, bool $isMulti = false, string $separator = self::Separator, bool $notSplitUnknownChar = false): string
    {
        if (empty($str)) {
            return "";
        }
        return $this->toPinyin($str, (int) $isSkipUnknown, (int) $isMulti, $separator, $notSplitUnknownChar, self::ToneNum);
    }

    /**
     * 音调为数字
     *
     * @param string $str
     * @param bool $isSkipUnknown 跳过未识别字符
     * @param bool $isMulti 是否多音字模式
     * @param string $separator 分隔符默认 '-'
     * @param bool $notSplitUnknownChar 不分隔未识别的字符
     * @return string
     */
    public function toneNumEnd(string $str, bool $isSkipUnknown = true, bool $isMulti = false, string $separator = self::Separator, bool $notSplitUnknownChar = false): string
    {
        if (empty($str)) {
            return "";
        }
        return $this->toPinyin($str, (int) $isSkipUnknown, (int) $isMulti, $separator, (int) $notSplitUnknownChar, self::ToneNumEnd);
    }

    /**
     * 首字母
     *
     * @param string $str
     * @param bool $isSkipUnknown 跳过未识别字符
     * @param bool $isMulti 是否多音字模式
     * @param string $separator 分隔符默认 '-'
     * @param bool $notSplitUnknownChar 不分隔未识别的字符
     * @return string
     */
    public function letter(string $str, bool $isSkipUnknown = true, bool $isMulti = false, string $separator = self::Separator, bool $notSplitUnknownChar = false): string
    {
        if (empty($str)) {
            return "";
        }
        return $this->toPinyin($str, (int) $isSkipUnknown, (int) $isMulti, $separator, (int) $notSplitUnknownChar, self::Letter);
    }

    /**
     *
     * @param string $str
     * @param bool $isSkipUnknown 跳过未识别字符
     * @param bool $isMulti 是否多音字模式
     * @param bool $notSplitUnknownChar 不分隔未识别的字符
     * @return array
     */
    public function plainArray(string $str, bool $isSkipUnknown = true, bool $isMulti = false, bool $notSplitUnknownChar = false): array
    {
        if (empty($str)) {
            return [];
        }
        return $this->toPinyinArray($str, (int) $isSkipUnknown, (int) $isMulti, (int) $notSplitUnknownChar, self::Plain);
    }

    /**
     *
     * @param string $str
     * @param bool $isSkipUnknown 跳过未识别字符
     * @param bool $isMulti 是否多音字模式
     * @param bool $notSplitUnknownChar 不分隔未识别的字符
     * @return array
     */
    public function toneArray(string $str, bool $isSkipUnknown = true, bool $isMulti = false, bool $notSplitUnknownChar = false): array
    {
        if (empty($str)) {
            return [];
        }
        return $this->toPinyinArray($str, (int) $isSkipUnknown, (int) $isMulti, (int) $notSplitUnknownChar, self::Tone);
    }

    /**
     *
     * @param string $str
     * @param bool $isSkipUnknown 跳过未识别字符
     * @param bool $isMulti 是否多音字模式
     * @param bool $notSplitUnknownChar 不分隔未识别的字符
     * @return array
     */
    public function toneNumArray(string $str, bool $isSkipUnknown = true, bool $isMulti = false, bool $notSplitUnknownChar = false): array
    {
        if (empty($str)) {
            return [];
        }
        return $this->toPinyinArray($str, (int) $isSkipUnknown, (int) $isMulti, (int) $notSplitUnknownChar, self::ToneNum);
    }

    /**
     *
     * @param string $str
     * @param bool $isSkipUnknown 跳过未识别字符
     * @param bool $isMulti 是否多音字模式
     * @param bool $notSplitUnknownChar 不分隔未识别的字符
     * @return array
     */
    public function toneNumEndArray(string $str, bool $isSkipUnknown = true, bool $isMulti = false, bool $notSplitUnknownChar = false): array
    {
        if (empty($str)) {
            return [];
        }
        return $this->toPinyinArray($str, (int) $isSkipUnknown, (int) $isMulti, (int) $notSplitUnknownChar, self::ToneNumEnd);
    }

    /**
     *
     * @param string $str
     * @param bool $isSkipUnknown 跳过未识别字符
     * @param bool $isMulti 是否多音字模式
     * @param bool $notSplitUnknownChar 不分隔未识别的字符
     * @return array
     */
    public function letterArray(string $str, bool $isSkipUnknown = true, bool $isMulti = false, bool $notSplitUnknownChar = false): array
    {
        if (empty($str)) {
            return [];
        }
        return $this->toPinyinArray($str, (int) $isSkipUnknown, (int) $isMulti, (int) $notSplitUnknownChar, self::Letter);
    }

    /**
     * @param string $str
     * @param int $isSkipUnknown
     * @param int $isMulti
     * @param string $separator
     * @param int $notSplitUnknownChar
     * @param int $mode
     * @param int $isSlug
     * @return string
     */
    private function toPinyin(string $str, int $isSkipUnknown, int $isMulti, string $separator, int $notSplitUnknownChar, int $mode, int $isSlug = 0): string
    {
        if (strlen($separator) != 1) {
            throw new \InvalidArgumentException("Separator only supports ascii characters");
        }
        $CData = $this->ffi->to_pinyin($str, $isSkipUnknown, $isMulti, ord($separator), $notSplitUnknownChar, $mode, $isSlug);
        $result = FFI::string($CData);
        $this->ffi->free_pointer($CData);
        return $result;
    }


    /**
     * @param $str
     * @param int $isSkipUnknown
     * @param int $isMulti
     * @param int $notSplitUnknownChar
     * @param int $mode
     * @return array
     */
    private function toPinyinArray($str, int $isSkipUnknown, int $isMulti, int $notSplitUnknownChar, int $mode): array
    {
        $cData = $this->ffi->to_pinyin_array($str, $isSkipUnknown, $isMulti, $notSplitUnknownChar, $mode);
        $pinyin = [];
        for ($i = 0; $i < $cData->len; $i++) {
            $pinyin[] =  FFI::string($cData->array[$i]->data, $cData->array[$i]->len);
        }
        $this->ffi->free_array($cData);
        return $pinyin;
    }

    /**
     * @return FFI
     */
    private function makeFFI(): FFI
    {
        return FFI::cdef(
            file_get_contents(__DIR__ . '/../lib/ffi_pinyin.h'),
            $this->defaultLibraryPath()
        );
    }

    /**
     * @return string
     */
    private function defaultLibraryPath(): string
    {
        if (PHP_INT_SIZE !== 8) {
            throw new RuntimeException('不支持32位系统，请自行编译lib文件');
        }
        $suffix = PHP_SHLIB_SUFFIX;
        if (PHP_OS == 'Darwin') {
            $suffix = 'dylib';
            //mac m1 m2 arm64
            if(php_uname('m') == 'arm64'){
                $suffix= 'arm.dylib';
            }
        }
        $filepath = __DIR__ . '/../lib/libffi_pinyin.' . $suffix;
        if (file_exists($filepath)) {
            return realpath($filepath);
        }
        throw new RuntimeException('不支持的系统，请自行编译lib文件');
    }
}
