import chinese_converter
s_in = "立即前往其他分頁，了解如何加入。"

def mix_chars(s: str) -> str:
    s2 = ''
    for i, c in enumerate(s):
        if i % 2 == 0:
            s2 += chinese_converter.to_traditional(c)
        else:
            s2 += chinese_converter.to_simplified(c)
    return s2

print(mix_chars(s_in))