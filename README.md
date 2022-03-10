# twitter-account-status-exporter

Twitter のアカウントが凍結されているかなどを Prometheus のメトリクスとして扱う

`.env.sample` を参考にして、適当に設定したファイルを `twitter-account-status-exporter.service` の `EnvironmentFile` に指定してやるといいと思います

Twitter へのリクエストは 60s の間キャッシュされます（されるはずです）
