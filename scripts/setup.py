import datetime
import os

if __name__ == '__main__':
    # Create the repository

    date = datetime.datetime.now()
    day = str(date.day + 1)

    if len(day) == 1:
        day = '0' + day

    os.system(f"cargo new day{day}")
    # copy the skeleton file
    os.system(f"cp skeleton/main.rs day{day}/src/main.rs")
    # copy the rustfmt file
    os.system(f"cp skeleton/rustfmt.toml day{day}")
    # copy the editorconfig
    os.system(f"cp skeleton/.editorconfig day{day}")

    with open(f"day{day}/Cargo.toml", 'r') as fp:
        lines = [line.strip() for line in fp.readlines()]

    # append packages
    lines.append('regex = "1.4.2"')
    lines.append('colored = "2"')
    lines.append('itertools = "0.9.0"')

    with open(f"day{day}/Cargo.toml", 'w') as fp:
        fp.write('\n'.join(lines))
