import requests

url = "http://vm.cau.edu.cn/~s03124/book/honglou.txt"
is_chinese = lambda uchar: uchar >= "\u4e00" and uchar <= "\u9fa5"
response = requests.get(url)
alldata = response.text
print(len(set(filter(is_chinese, alldata))))
print(alldata.count(input()))
