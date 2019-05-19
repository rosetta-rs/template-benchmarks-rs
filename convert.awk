{
    time = $4;
    sub(/[a-z]+$/, "", time);
    unit = $4;
    sub(/^[^a-z]+/, "", unit);

    # convert to nanoseconds
    if (unit == "us") {
        time *= 1000;
    } else if (unit == "ms") {
        time *= 1000000;
    } else if (unit == "s") {
        time *= 1000000000;
    }

    printf "%-24s%s %s %s %sns\n", $1, $3, $4, $5, time
	}
