push: | target/doc
	git add --all && git commit -m "update" && git push;

generate_docs: 
	cargo doc
