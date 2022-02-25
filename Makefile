doc_dir:=./target/doc

push: | $(doc_dir)
	git add --all && git commit -m "update" && git push;

$(doc_dir): 
	cargo doc
