# Makefile

CURRENT_DIR := $(shell pwd)
DOCKER_VOLUME := $(CURRENT_DIR):/notebooks

go:
	docker run -it -p 8888:8888 -v $(DOCKER_VOLUME) gopherdata/gophernotes:latest-ds
