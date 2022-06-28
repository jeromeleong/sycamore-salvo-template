cargo-watch \
    -s "cd frontend" \
    -s "trunk build --filehash false --dist ../app" \
    -s "cd ../app" \
    -s "sed -i '' 's/\/frontend/\/static\/frontend/g' index.html" \
    -s "mkdir static" \
    -s "mv frontend.js ./static/frontend.js" \
    -s "mv frontend_bg.wasm ./static/frontend_bg.wasm" \
    -s "cd .." \
    -s "mkdir -p storage" \
    -s "cargo run -p backend" 