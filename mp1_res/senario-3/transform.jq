#!/usr/bin/jq -f
map(.[0] |= (tonumber | floor))
| group_by(.[0])
| map([.[0][0], (map(.[1] | tonumber) | add)])
| .[0][0] as $start
| map(.[0] |= . - $start)
