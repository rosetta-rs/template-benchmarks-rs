cargo bench | ggrep -P 'time:[[:space:]]*\[[^\]]*\]' > ./results/results.txt
cp ./target/criterion/Teams/report/violin.svg ./results/teams.svg
cp ./target/criterion/Big\ table/report/violin.svg ./results/big-table.svg
