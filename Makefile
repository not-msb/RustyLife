clean:
	rm -rf video

video:
	cargo run --release
	ffmpeg -y -r 10 -i video/cgol-%d.ppm out.mp4 -loglevel quiet