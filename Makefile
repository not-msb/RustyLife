video:
	cargo run
	ffmpeg -r 10 -i video/cgol-%d.ppm out.mp4