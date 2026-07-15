cd "/home/$USER"

mkdir -p .refract/core
cd "/home/$USER/.refract/core"

git clone https://github.com/playerhazu/Refract.git

cd Refract

cargo build --release
sudo ln -sf "/home/$USER/.refract/core/Refract/target/release/refract" /usr/local/bin/refract
