#!/usr/bin/env python3


with open("/home/omar/programming/rust-dev/gof-rs/presets/patterns-striped.txt", "r") as file:
    with open("./new.txt", "w") as newfile:
        cnt = 0
        for line in file.readlines():
                if line != '\n':
                    if cnt > 0:
                        newfile.write("\n")
                    if len(line) < 73 and line != '\n':
                        txt = line.rstrip().center(72, ".") + '\n'
                        newfile.write(txt)
                    cnt = 0
                if line == '\n':
                    cnt += 1
        
