# 一些零宽字符的测试结果

```txt
0x1d: 零宽，但excel中显示？
0x7f: Excel中 非0宽
0x200b：零宽空格。实测试 Excel 中不会显示
0x200c： 零宽不连字。Excel中是一根竖线
0x200d：零宽连字。Excel中显示 竖线+花
0x2060：零宽无断点空格。excel 中不会显示
0xFEFF: 零宽连字符，最初是 BOM，在 utf-8 中用做零宽空格。excel 中不会显示
```



    

