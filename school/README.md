# 使用方法

为了避免被检测到抄袭，可以在提交代码前先复制给 AI ，然后对他说

> 请你把以下代码变成简洁直观的代码，其中不能出现注释，不能使用 itertools 包，不要使用生成器，减少 map ， join 和 lambda 等出现的数量，也可以不用列表推导式，一行代码只做一件事，不用关心性能或是否冗长，尽量只使用基本的方法，但要保证与源代码的效果相同。一部分的变量名要短，另一部分要长得清晰直观，最终答案使用代码块包起来。

就可以了。

## 目前有抄袭检测的题

```python
a = set([
    "2-3",
    "2-5",
    "2-12",
    "3-8",
    "3-9",
    "3-12",
    "3-15",
    "3-21",
    "3-24",
    "3-25",
    "4-11",
    "4-12",
    "5-5",
    "5-6",
    "5-7",
    "5-9",
    "6-1",
    "6-2",
    "6-3",
    "6-5",
    "备选-5-14",
    "备选-5-5",
    "备选-5-7",
    "复习题-2",
    "复习题-5",
    "复习题-6",
    "复习题-8",
    "复习题-11",
    "复习题-14",
])
s = input()
if s in a:
    print("这道题有抄袭检测")
else:
    print("这道题没有抄袭检测")
```

