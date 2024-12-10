import re
from pathlib import Path

def exercise_1(input_text: str):
    pattern = re.compile(r"mul\((\d{1,3}),(\d{1,3})\)")

    matches = re.findall(pattern, input_text)

    s = sum([int(match[0]) * int(match[1]) for match in matches])

    print(s)

def exercise_2(input_text: str):
    pattern = re.compile(r"((do\(\)|don't\(\)).+?)?mul\((\d{1,3}),(\d{1,3})\)")

    matches = re.findall(pattern, input_text)

    operands = [(int(match[2]), int(match[3])) for match in matches if match[1] != "don't()"]
    s = sum([operand[0] * operand[1] for operand in operands])

    print(s)


if __name__ == "__main__":
    input_text: str = Path("../input.txt").read_text()

    exercise_1(input_text)
    exercise_2(input_text)