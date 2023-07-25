#!/usr/bin/env bash

# Prerequisites:
#   sudo apt update && sudo apt install -y imagemagick

dir=$(dirname "$0")
cd "$dir"
convert background.jpg -resize 66.67% -quality 75 background.optimized.jpg
# convert logo_center.png -resize 21.7% logo_center.optimized.png
convert logo_center.png -crop 916x350+40+190 -resize 37.14% logo_center.optimized.png
# convert logo_center.png -crop 1040x420+100+50 -resize 31% logo_center.optimized.png
# convert logo_right_or_left.png -crop 1040x1040+80+80 -resize 12.5% logo_left.optimized.png
# 

# convert logo_right.png -resize 20% logo_right.optimized.png

# convert logo_вправо_SOC.png -crop 500x270+0+100 -resize 50% logo_вправо_SOC.optimized.png
# convert logo_влево_SOC.png -resize 25% logo_влево_SOC.optimized.png

# convert background.png -crop 2482x1508+0+1000  -resize 25%  background.optimized.png

# convert background.jpg -alpha set -channel Alpha -evaluate set 30% png32:background.optimized.png
# convert background.jpg -fill white -colorize 50% background.optimized.jpg

# convert logo_left.png -crop 1600x1600+480+0 -resize 6.25% logo_left.optimized.png

# convert title.png -crop 1070x440+70+0  title.optimized.png
# convert title.png -crop 1070x440+70+0 -resize 33% title.optimized.png

# convert logo_left.png -crop 1600x1600+480+0 logo_left.optimized.png
# convert hotel.png -resize 15%  hotel.optimized.png
# convert logo.png -resize 5%  logo.optimized.png
# convert logo_left.jpg -crop 1620x250+150+750 -resize 25%  logo_left.optimized.jpg
# convert logo_left.jpg -crop 410x95+10+15 -resize 80%  logo_left.optimized.jpg

# convert title.png -crop 1200x600+370+830 -resize 25% title.optimized.png

# convert title.png -crop 1500x1720+215+375 -resize 10%  logo_right.optimized.png
# convert title.png -crop 1500x1720+215+375 -resize 10%  logo_left.optimized.png

# convert logo_center.png -resize 30% logo_center.optimized.png
# convert logo_right.png -resize 10% logo_right.optimized.png
# convert logo.png -crop 730x1250+180+420 -resize 10% logo.optimized.png
# convert ../hotels/Crocus-Expo-logo-white.png -crop 321x132+102+76 Crocus-Expo-logo-white.optimized.png

