import string

number_string_map = {
    "one": "one1one",
    "two": "two2two",
    "three": "three3three",
    "four": "four4four",
    "five": "five5five",
    "six": "six6six",
    "seven": "seven7seven",
    "eight": "eight8eight",
    "nine": "nine9nine",
}

def number_words_to_ints(line: str) -> str:
    new_line = line
    for (key, val) in number_string_map.items():
        new_line = new_line.replace(key, str(val))
        
    return new_line
    

def get_numbers_from_line(line: str) -> int:
    first_num = -1
    last_num = -1

    for char in line:
        if char in string.digits:
            if first_num == -1:
                first_num = int(char)
                continue

            last_num = int(char)

    if last_num == -1:
        last_num = first_num

    return int(f"{first_num}{last_num}")


with open("real.txt", "r") as test_file:
    lines = test_file.read().splitlines()

    numbers = []
    for line in lines:
        numbers_intified = number_words_to_ints(line)
        numbers.append(get_numbers_from_line(numbers_intified))
    
    print(sum(numbers))
    # print(number_words_to_ints("one1two2"))