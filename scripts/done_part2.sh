#!/bin/bash
YEAR=2024
DAY=$1
DAY_WITH_ZERO=$(printf "%02d" $DAY)
# comment dayXX in main.rs
sed -i 's/mod day'${DAY_WITH_ZERO}'_part2;/\/\/ mod day'${DAY_WITH_ZERO}'_part2;/' ./src/main.rs
sed -i 's/day'${DAY_WITH_ZERO}'_part2::main()/\/\/ day'${DAY_WITH_ZERO}'_part2::main()/' ./src/main.rs
# remember to copy enonce
echo "todo" > ./enonces/day${DAY_WITH_ZERO}.txt
