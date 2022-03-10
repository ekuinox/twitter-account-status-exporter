# twitter-account-status-exporter

Twitter のアカウントが凍結されているかなどを Prometheus のメトリクスとして扱う

## 導入

事前に Twitter の API_KEY と API_SECRET を取得しておく必要がある

1. `$ cargo build --release` したものを `/usr/bin` など適当なディレクトリにインストール
2. `.env.sample` を参考にして、適当に設定したファイルを `twitter-account-status-exporter.service` の `EnvironmentFile` に指定する
3. 適宜 `twitter-account-status-exporter.service` は合わせて変更
4. `$ systemctl daemon-reload` とかする
5. `prometheus.yml` の `scrape_configs` > `static_configs` > `targets` に追加する

## メトリックについて

- `GET /metrics` にリクエストするとメトリックが返される
- 取得した Twitter アカウント情報は 60s 間キャッシュされる（はず）
- メトリック名は `twitter` で、 `account` ラベルにユーザ名が入る
- アカウントの状態は以下の値で示される
  * 健全な状態 ... `0`
  * 凍結(Forbidden) ... `1`
  * 見つからない ... `2`
  * それ以外(取得エラーとか) ... `3`
