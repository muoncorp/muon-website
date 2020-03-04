#!/bin/sh

(cd frontend; npm run build)

systemfd --no-pid -s http::3000 -- cargo watch -x run