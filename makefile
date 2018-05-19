curdir := $(CURDIR)

build:
	docker run --rm --name amazonlinux-neon -d -it -v $(curdir):/work bokuweb/amazonlinux-neon
	docker exec amazonlinux-neon neon clean && neon build
	-docker exec amazonlinux-neon rm deployment.zip
	docker exec amazonlinux-neon zip -r deployment.zip * -x "*.zip" "*.json" "*.log" "makefile" "node_modules/*" "native/target/release/build/*" "native/target/release/deps/*" "target/*"
	docker stop amazonlinux-neon