# tui-cat

本アプリケーションでは直感的な操作でファイルの中身を確認することができます。

![demo](/img/demo.gif)

## 動作環境(確認済み)

-   windows
-   Ubuntu(20.04)

## 使い方

```shell
git clone https://github.com/fedyya/tui-cat
cd tui-cat
cargo install --path .

tui-cat
```

## 操作方法

| key                         | 動作                                         |
| --------------------------- | -------------------------------------------- |
| <kdb>w</kdb> , <kdb>↑</kdb> | 上に移動                                     |
| <kdb>s</kdb> , <kdb>↓</kdb> | 下に移動                                     |
| <kdb>e</kdb>                | 詳細モードに移行<br>（ファイルの中身を確認） |
| <kdb>p</kdb>                | プロパティモードに移行                       |
| <kdb>l</kdb>                | ラインモードに移行                           |
| <kdb>g</kdb>                | 先頭に移動                                   |
| <kdb>G</kdb>                | 最終行に移動                                 |
| <kdb>q</kdb>                | 終了<br>quit                                 |
