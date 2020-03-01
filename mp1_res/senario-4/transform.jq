#!/usr/bin/jq -f
map(
    .[0] |= (
        split(".")
        | .[0]
        | (strptime("%H:%M:%S") | .[0] |= 2020 | mktime)))
| group_by(.[0])
| map([.[0][0], (map(.[1] | tonumber) | add)])
| .[0][0] as $start
| map(.[0] |= . - $start)
