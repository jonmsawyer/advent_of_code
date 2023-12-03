--[[
    Advent of Code 2023 - Day 1
    by Jonathan Sawyer <https://github.com/jonmsawyer>
]]

require("lib")

function main(input_filename)
    local input = get_input(input_filename)
    local part1 = read_lines(input, process_line_digit)
    local part2 = read_lines(input, process_line_word)
    print("Part 1 sum of calibration values is:", part1)
    print("Part 2 sum of calibration values is:", part2)
end

main("input.txt")
