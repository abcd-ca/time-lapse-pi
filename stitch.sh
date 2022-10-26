ffmpeg -r 5 -pattern_type glob -i "output/*.jpg" -s 1280x720 -vcodec libx264 output.mp4
