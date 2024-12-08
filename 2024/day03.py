import re


def task1(string: str) -> int:
    return sum(int(l) * int(r) for l, r in re.findall(r"mul\((\d{1,3}),(\d{1,3})\)", string))
        
    
def task2(string: str) -> int:
    sub_strings = [s.split("don't")[0] for s in string.split("do()")]
    return sum(task1(s) for s in sub_strings)


def main():
    with open("input.txt") as f:
        string = f.read()
        print(task1(string))
        print(task2(string))


if __name__ == "__main__":
    main()