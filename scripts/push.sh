function push() {
    cargo doc;
    git add --all && git commit -m "update" && git push;
}
