#!/usr/bin/env python3


with open("/home/pengu/test/rust-dev/gof-rs/presets/new.txt", "r") as file:
        cnt = 0
        content = ""
        for line in file.readlines():
            if line != '\n':
                content += line.rstrip().center(72, ".") + '\n'
            if line == '\n':
                with open(f"./pattern{cnt}.txt", "w") as pat:
                    pat.write(content)
                    content = ""
                    cnt += 1
    
