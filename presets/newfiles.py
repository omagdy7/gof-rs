#!/usr/bin/env python3

with open("/home/pengu/test/rust-dev/gof-rs/presets/patterns-striped.txt", "r") as file:
        cnt = 0
        content = ""
        for line in file.readlines():
            print(line)
            print("ehll")
            if line != '\n':
                content += line.rstrip().center(72, ".") + '\n'
                print("Hello" + content)
            if line == '\n':
                with open(f"./pattern{cnt}.txt", "w") as pat:
                    print(content)
                    pat.write(content)
                    content = ""
                    cnt += 1
    
