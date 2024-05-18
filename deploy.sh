#!/bin/bash

cd /home/ubuntu/rust-app-completo-render-server-side-deploy/
git pull --force
cargo build --release

cd /home/ubuntu/app

cp /home/ubuntu/rust-app-completo-render-server-side-deploy/target/release/console .
cp -R /home/ubuntu/rust-app-completo-render-server-side-deploy/templates .
cp -R /home/ubuntu/rust-app-completo-render-server-side-deploy/static .


# Porta a ser verificada
PORT=8000

# Verifica se a porta está em uso
PID=$(lsof -ti :$PORT)

# Se a porta estiver em uso, mata o processo
if [ -n "$PID" ]; then
    echo "A porta $PORT está em uso pelo processo $PID. Matando o processo..."
    kill -9 $PID
    echo "Processo $PID foi morto."
else
    echo "A porta $PORT não está em uso."
fi


/home/ubuntu/start.sh