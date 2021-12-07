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
    protected FFI $ffi;

    /**
     * FFIPinyin constructor.
     */
    private function __construct()
    {
        if (ini_get('ffi.enable') == false) {
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

    private function __clone()
    {

    }

    /**
     * 普通风格没有音调
     *
     * @param string $str
     * @param bool $isConvert 是否将无法识别的字转成 "-"
     * @param bool $isMulti 是否多音字模式
     * @return string
     */
    public function plain(string $str, bool $isConvert = true, bool $isMulti = false): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->plain($str, (int) $isConvert, (int) $isMulti);
        return $this->convert($char);
    }

    /**
     * 音调
     *
     * @param string $str
     * @param bool $isConvert 是否将无法识别的字转成 "-"
     * @param bool $isMulti 是否多音字模式
     * @return string
     */
    public function tone(string $str, bool $isConvert = true, bool $isMulti = false): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->tone($str, (int) $isConvert, (int) $isMulti);
        return $this->convert($char);
    }

    /**
     * 音调为数字
     *
     * @param string $str
     * @param bool $isConvert 是否将无法识别的字转成 "-"
     * @param bool $isMulti 是否多音字模式
     * @return string
     */
    public function toneNum(string $str, bool $isConvert = true, bool $isMulti = false): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->tone_num($str, (int) $isConvert, (int) $isMulti);
        return $this->convert($char);
    }

    /**
     * 音调为数字
     *
     * @param string $str
     * @param bool $isConvert 是否将无法识别的字转成 "-"
     * @param bool $isMulti 是否多音字模式
     * @return string
     */
    public function toneNumEnd(string $str, bool $isConvert = true, bool $isMulti = false): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->tone_num_end($str, (int) $isConvert, (int) $isMulti);
        return $this->convert($char);
    }

    /**
     * 首字母
     *
     * @param string $str
     * @param bool $isConvert 是否将无法识别的字转成 "-"
     * @param bool $isMulti 是否多音字模式
     * @return string
     */
    public function letter(string $str, bool $isConvert = true, bool $isMulti = false): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->letter($str, (int) $isConvert, (int) $isMulti);
        return $this->convert($char);
    }

    /**
     *
     * @param string $str
     * @param bool $isConvert 是否将无法识别的字转成 "-"
     * @param bool $isMulti 是否多音字模式
     * @return array
     */
    public function plainArray(string $str, bool $isConvert = true, bool $isMulti = false): array
    {
        if (empty($str)) {
            return [];
        }
        $plainArray = $this->ffi->plain_array($str, (int) $isConvert, (int) $isMulti);
        return $this->convertArray($plainArray);
    }

    /**
     *
     * @param string $str
     * @param bool $isConvert 是否将无法识别的字转成 "-"
     * @param bool $isMulti 是否多音字模式
     * @return array
     */
    public function toneArray(string $str, bool $isConvert = true, bool $isMulti = false): array
    {
        if (empty($str)) {
            return [];
        }
        $toneArray = $this->ffi->tone_array($str, (int) $isConvert, (int) $isMulti);
        return $this->convertArray($toneArray);
    }

    /**
     *
     * @param string $str
     * @param bool $isConvert 是否将无法识别的字转成 "-"
     * @param bool $isMulti 是否多音字模式
     * @return array
     */
    public function toneNumArray(string $str, bool $isConvert = true, bool $isMulti = false): array
    {
        if (empty($str)) {
            return [];
        }
        $toneNumArray = $this->ffi->tone_array($str, (int) $isConvert, (int) $isMulti);
        return $this->convertArray($toneNumArray);
    }

    /**
     *
     * @param string $str
     * @param bool $isConvert 是否将无法识别的字转成 "-"
     * @param bool $isMulti 是否多音字模式
     * @return array
     */
    public function toneNumEndArray(string $str, bool $isConvert = true, bool $isMulti = false): array
    {
        if (empty($str)) {
            return [];
        }
        $toneNumEndArray = $this->ffi->tone_array($str, (int) $isConvert, (int) $isMulti);
        return $this->convertArray($toneNumEndArray);
    }

    /**
     *
     * @param string $str
     * @param bool $isConvert 是否将无法识别的字转成 "-"
     * @param bool $isMulti 是否多音字模式
     * @return array
     */
    public function letterArray(string $str, bool $isConvert = true, bool $isMulti = false): array
    {
        if (empty($str)) {
            return [];
        }
        $letterArray = $this->ffi->tone_array($str, (int) $isConvert, (int) $isMulti);
        return $this->convertArray($letterArray);
    }

    /**
     * @param FFI\CData $CData
     * @return string
     */
    private function convert(FFI\CData $CData)
    {
        $result = FFI::string($CData);
        $this->ffi->free_pointer($CData);
        return $result;
    }


    /**
     * @param FFI\CData $cData
     * @return array
     */
    private function convertArray(FFI\CData $cData)
    {
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
        }
        $filepath = __DIR__ . '/../lib/libffi_pinyin.' . $suffix;
        if (file_exists($filepath)) {
            return realpath($filepath);
        }
        throw new RuntimeException('不支持的系统，请自行编译lib文件');
    }
}
