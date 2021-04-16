## ffi-pinyin

该项目是将中文汉字转换成拼音，使用`rust`构建动态链接库来给`php`调用。
该库主要是为了提高`php`转换中文拼音的性能而构建（__特别是长文章__）。

### 环境

需要`php >= 7.4` 以上的版本并且开启了`FFI`扩展。如果你需要自己编译库还需要装`rust` 工具链。

还需要设置`php.ini` 中的`ffi.enable`为`On`。

### Usage 

该库提供四种基础用法：不带音标，带音标，首字母，多音字带音标。

```php
<?php

include __DIR__ . '/../src/Pinyin.php';

$py = FastFFI\Pinyin\Pinyin::new();

echo "无音标: ", $py->plain("中国人..."), "\n";
echo "首字母: ", $py->letter("中国人"), "\n";
echo "音标: ", $py->tone("中国人"), "\n";
echo "多音音标:", $py->tone_multi("中国人"), "\n";

```

以上程序执行后的结果: 

> 无音标: zhong guo ren - - -
> 
> 首字母: z g r
> 
> 音标: zhōng guó rén
> 
> 多音音标:zhōng:zhòng guó rén

转换后的多个拼音都是以`" "`空格隔开，不能识别的字符都是以`-`来代替，多音字是以`:`来连接的。

### FAQ

- 在`centos`上执行失败?
  
  确定是不是`glibc`版本过低。可以使用`ldd lib/libffi_pinyin.so` 来查看库信息。
如果出现`/lib64/libc.so.6: version 'glibc_2.18' not found`就说明你服务的`glibc`版本过低。 
  下载glibc编译升级，下载地址: `wget http://mirrors.ustc.edu.cn/gnu/libc/glibc-2.18.tar.gz` 
  
