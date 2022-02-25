.PHONY: push
.PHONY: build-docs

push:
	git add --all && git commit -m "update" && git push;

build-docs: 
	cargo doc
