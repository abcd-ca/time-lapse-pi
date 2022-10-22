ffmpeg -r 5 -pattern_type glob -i "output/*.jpg" -s 1920x1440 -vcodec libx264 output.mp4
