student = {
    "李四": 78,
    "如一": 65,
    "赵敏": 78,
    "张旭宁": 99,
    "张三": 45,
    "司音": 90,
    "思琪": 87,
    "沙思思": 96,
    "柏龙": 60,
    "徐来": 40,
}
print("成绩分别为：")
for name, score in student.items():
    print(name, score)
print(
    "全班共有",
    len(student.keys()),
    "人，平均成绩为：",
    sum(student.values()) / len(student.values()),
    "分",
)
