1.Rustのインストール: ターミナル（WindowsならPowerShell）で以下のコマンドを実行

winget install Rustlang.Rustup


2.Rustlingsのインストール: これは「意図的に壊れたコード」を修正しながら進む公式教材です。

・Rustインストール後にターミナルを再起動してから実行 <= VScodeならVScode自体を一度閉じる>
以下をターミナルで実行
cargo install rustlings
rustlings init
cd rustlings
rustlings

・毎回の起動
cd ./rustlings
rustlings