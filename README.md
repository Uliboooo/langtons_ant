# ラングトンのアリ

## 使用

.envファイルを実行ファイルと同じディレクトリに設置して以下のように値を設定します。設定によって.envファイルは隠しファイルとして表示されていない可能性があるので表示させるかターミナルなどから編集してください。

```env: .env
# .envファイル
# 👇試行回数
loop_count=200
# 👇マスの数(正方形で一辺の数)
len=11
```

### 実行

実行ファイルを実行してください。
もしくは以下コードで実行

```bash: run
# if you use mac or unix or linux
./langton_ant
# if you use windows
./langton_ant.exe
```

## 注意

間違ってるかも。あまり厳密には確かめてません。

## 参考

- [wikipedia: ラングトンのアリ](https://ja.wikipedia.org/wiki/ラングトンのアリ)
