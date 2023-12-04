# [Day 3: Gear Ratios](https://adventofcode.com/2023/day/3)
You and the Elf eventually reach a [gondola lift](https://en.wikipedia.org/wiki/Gondola_lift) station; he says the gondola lift will take you up to the **water source**, but this is as far as he can bring you. You go inside.

It doesn&#x27;t take long to find the gondolas, but there seems to be a problem: they&#x27;re not moving.

&quot;Aaah!&quot;

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. &quot;Sorry, I wasn&#x27;t expecting anyone! The gondola lift isn&#x27;t working right now; it&#x27;ll still be a while before I can fix it.&quot; You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can **add up all the part numbers** in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don&#x27;t really understand, but apparently **any number adjacent to a symbol**, even diagonally, is a &quot;part number&quot; and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

```467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```
In this schematic, two numbers are **not** part numbers because they are not adjacent to a symbol: `114` (top right) and `58` (middle right). Every other number is adjacent to a symbol and so **is** a part number; their sum is **`4361`**.

Of course, the actual engine schematic is much larger. **What is the sum of all of the part numbers in the engine schematic?**


## Part Two
The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don&#x27;t seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled &quot;help&quot;, so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You&#x27;re going so slowly that you haven&#x27;t even left the station. You exit the gondola.

The missing part wasn&#x27;t the only issue - one of the gears in the engine is wrong. A **gear** is any `*` symbol that is adjacent to **exactly two part numbers**. Its **gear ratio** is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

```467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```
In this schematic, there are **two** gears. The first is in the top left; it has part numbers `467` and `35`, so its gear ratio is `16345`. The second gear is in the lower right; its gear ratio is `451490`. (The `*` adjacent to `617` is **not** a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces **`467835`**.

**What is the sum of all of the gear ratios in your engine schematic?**