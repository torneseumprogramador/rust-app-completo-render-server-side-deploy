#!/bin/bash

cd /home/ubuntu/app
./console > app.log 2>&1 &

echo "A aplicação foi iniciada na porta 8000"