#!/usr/bin/env bash

time=$(date +"%H:%M")

muted="$(osascript -e 'get volume settings' | sed -e 's/.*://')"
volume="mute"

uptime="$(uptime |tr " " "\n" | tail -n 3 | head -n 1)"

version="$(sw_vers -productVersion | cut -c 1-4)"

if [ "$muted" = "false" ]
then
    volume="$(osascript -e 'get volume settings' | sed -e 's/output volume://' | sed -e 's/,.*//')%"
fi

echo "{ \
  \"icon_top_left\": \"\", \
  \"text_top_left\": \"$time\", \
  \"icon_top_right\": \"\", \
  \"text_top_right\": \"$(printf %5s $volume)\", \
  \"icon_bottom_left\": \"\", \
  \"text_bottom_left\": \"$(printf %5s $uptime)\", \
  \"icon_bottom_right\": \"\", \
  \"text_bottom_right\": \"$(printf %5s $version)\" \
}" | cargo run --release