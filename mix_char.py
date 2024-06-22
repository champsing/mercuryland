import chinese_converter
s_in = "絕無思想審查制度。您可以在我們的伺服器復刻世界上的有名建築，或單單只是在這個世界蓋出專屬於您的根據地。"

def mix_chars(s: str) -> str:
    s2 = ''
    for i, c in enumerate(s):
        if i % 2 == 0:
            s2 += chinese_converter.to_traditional(c)
        else:
            s2 += chinese_converter.to_simplified(c)
    return s2

print(mix_chars(s_in))