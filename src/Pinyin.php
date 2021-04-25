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
     * @return string
     */
    public function plain(string $str): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->plain($str);
        return $this->convert($char);
    }

    /**
     * 音调
     *
     * @param string $str
     * @return string
     */
    public function tone(string $str): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->tone($str);
        return $this->convert($char);
    }

    /**
     * 多音字音调
     *
     * @param string $str
     * @return string
     */
    public function tone_multi(string $str): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->tone_multi($str);
        return $this->convert($char);
    }

    /**
     * 首字母
     *
     * @param string $str
     * @return string
     */
    public function letter(string $str): string
    {
        if (empty($str)) {
            return "";
        }
        $char = $this->ffi->letter($str);
        return $this->convert($char);
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
