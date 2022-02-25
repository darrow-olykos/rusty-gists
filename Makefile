.PHONY: push
push:
	git add --all && git commit -m "update" && git push;
	
.PHONY: build-docs
build-docs: 
	cargo doc
