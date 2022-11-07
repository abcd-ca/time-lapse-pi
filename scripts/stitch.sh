# this is just here for reference, it will be built into the time-lapse-pi CLI app as a subcommand
ffmpeg -r 5 -pattern_type glob -i "output/*.jpg" -s 1280x720 -vcodec libx264 output.mp4
