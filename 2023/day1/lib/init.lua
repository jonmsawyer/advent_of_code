--[[
    Advent of Codes's Lua Library
    By Jonathan Sawyer <https://github.com/jonmsawyer>
]]

--- Returns the contents of the input filename as a string.
---
--- @param filename string
--- @return string
function get_input(filename)
    local f = io.open(filename, "r")
    if f == nil then
        return ""
    end
    local content = f:read("*all")
    f:close()
    return content
end

--- Returns a 3-tuple of strings representing the index, first word in the input string, and the
--- string to replace the given wored. The return values will be in the form of
--- index, "word", "numeral". E.g., 12, "one", "1".
---
--- @param content string
--- @return number, string, string
function get_word_index(content)
    local zero_idx, _ = content:find("zero")
    local one_idx, _ = content:find("one")
    local two_idx, _ = content:find("two")
    local three_idx, _ = content:find("three")
    local four_idx, _ = content:find("four")
    local five_idx, _ = content:find("five")
    local six_idx, _ = content:find("six")
    local seven_idx, _ = content:find("seven")
    local eight_idx, _ = content:find("eight")
    local nine_idx, _ = content:find("nine")
    local min = math.min(
        zero_idx or 99,
        one_idx or 99,
        two_idx or 99,
        three_idx or 99,
        four_idx or 99,
        five_idx or 99,
        six_idx or 99,
        seven_idx or 99,
        eight_idx or 99,
        nine_idx or 99
    )
    print("Content:", content, "Min of content is:", min)
    if min == zero_idx then
        return min, "zero", "0"
    elseif min == one_idx then
        return min, "one", "1"
    elseif min == two_idx then
        return min, "two", "2"
    elseif min == three_idx then
        return min, "three", "3"
    elseif min == four_idx then
        return min, "four", "4"
    elseif min == five_idx then
        return min, "five", "5"
    elseif min == six_idx then
        return min, "six", "6"
    elseif min == seven_idx then
        return min, "seven", "7"
    elseif min == eight_idx then
        return min, "eight", "8"
    elseif min == nine_idx then
        return min, "nine", "9"
    end
    return min, "", ""
end

--- Return a string representing a conversion of numbers as words to numbers as numbers.
---
--- @param content string
--- @return string
function replace_word_numbers(content)
    local idx, word, numeral = get_word_index(content)
    while idx ~= 99 do
        print("Index:", idx, "Word is:", word, "Numeral is:", numeral)
        content = content:gsub(word, numeral, 1)
        idx, word, numeral = get_word_index(content)
    end
    return content
end

--- Returns the input string replacing the words 'one' through 'nine' with their numeral
--- representations.
---
--- @param content string
--- @param f function @return number
--- @return number
function read_lines(content, f)
    local total = 0
    for line in string.gmatch(content, "[a-zA-Z0-9]+[^\n]") do
        total = total + f(line)
    end
    return total
end

--- Given an input string, return a number that is the concatenated numerical strings of the first
--- and last digit of the input string. For example, input such as "abc123z9a" will return "19".
--- Input such as "asdf1asdf" will return "11".
---
--- @param line string
--- @return number?
function process_line_digit(line)
    local first_num, last_num, combined = nil, nil, nil
    for num in line.gmatch(line, '%d') do
        if first_num == nil then
            first_num = num
            last_num = num
        else
            last_num = num
        end
    end
    combined = first_num .. last_num
    print('', "First number:", first_num)
    print('', "Last number:", last_num)
    print('', "Combined:", combined)
    return tonumber(combined)
end

--- Given an input string, convert a string such as "onetwothree456seven" into "1234567" and
--- process that line according to the rules of the puzzle.
---
--- @param line string
--- @return number?
function process_line_word(line)
    print("Line pre-processed:", line)
    line = replace_word_numbers(line)
    print("Line post-processed:", line)
    return process_line_digit(line)
end
