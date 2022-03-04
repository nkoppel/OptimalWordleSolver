convert -size 280x280 xc:#e64239 \
    -strokewidth 24 -stroke white \
    -draw "line 0,0 280,280" \
    -draw "line 0,280 280,0" \
    -stroke none \
    clear.png

convert -size 280x280 xc:white -alpha transparent \
    -fill '#e64239' \
    -draw "polygon 40,140 240,240 240,40" \
    delete.png

