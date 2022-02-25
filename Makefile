push: | "target/doc"
	git add --all && git commit -m "update" && git push;

"target/doc": 
	cargo doc
