video:
	cargo run
	ffmpeg -r 50 -i video/cgol-%d.ppm out.mp4