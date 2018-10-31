all:
	cargo xbuild --release --target x86_64-blog_os.json

objdump:
	cargo objdump --target x86_64-blog_os.json -- -disassemble -print-imm-hex target/x86_64-blog_os/release/panic

readelf:
	readelf -x .rodata target/x86_64-blog_os/release/panic
