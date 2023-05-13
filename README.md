# pngme

A program to encode and decode messages in PNG files following [this challenge](https://picklenerd.github.io/pngme_book/introduction.html)

The format of png files can be found [here](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html). The program conceals messages in images using private chunks.

The following commands can be used:
```
pngme encode <image> <chunk> <message>
pngme decode <image> <chunk>
pngme remove <image> <chunk>
pngme print <image>
```