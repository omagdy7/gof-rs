#!/usr/bin/env python3

import os

path = "/home/pengu/test/rust-dev/gof-rs/presets/dir/patterns"
newpath = "/home/pengu/test/rust-dev/gof-rs/presets/dir/newpatterns"

os.chdir(path)

def get_file_lines(file):
    cnt = 0
    with open(f"{path}/{file}", "r") as file:
        for _ in file.readlines():
            cnt += 1
    return cnt

cnt = 0
for file in os.listdir():
    cnt += 1
    lines = 42 - get_file_lines(file)
    upper = 0
    lower = 0
    if lines % 2 == 0:
        upper = lines // 2
        lower = lines // 2
    else:
        upper = lines // 2
        lower = upper + 1
    with open(f"{path}/{file}", "r") as r:
        with open(f"{newpath}/pattern{cnt}.txt", "w") as w:
            for _ in range(upper):
                w.write("........................................................................\n")
            for line in r.readlines():
                w.write(line)
            for _ in range(lower):
                w.write("........................................................................\n")






