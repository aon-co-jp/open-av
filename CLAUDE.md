# open-av 開発メモ(CLAUDE.md)

開発方針・ルールの正本は [`open-raid-z/CLAUDE.md`](https://github.com/aon-co-jp/open-raid-z)。

## 役割
MP4/MKVの映像 + DSD音声 + イマーシブ配置メタデータを1つのMKVにまとめるオープンな形式のプロファイル(`SPEC.md`)と参照実装。MQA・Auro-CXは**復号も再実装もしない**(特許・非公開)。素通し(opaque)と、公開規格による代替(open-mqa/open-mqa-dsd)だけ。

## ユーザー指示(2026-09-25)
「mp4やmkvの動画DATA + DSD音声 + Auro-CXとMQAのオープンソースを融合した新しいビデオフォーマット」を`open-av`として新規リポジトリに。make-diskの最新インストーラー・最新READMEへのリンクをREADMEに貼る。

## HANDOFF
- **2026-09-25 初版**: `src/manifest.rs`(仕様・検証、単体5)、`src/pack.rs`(ffmpegでMKV添付として同梱/取り出し)、CLI、`tests/e2e.rs`(実ffmpegでMP4+DSF+互換音声→MKV→inspect→extract、DSDがビット一致、非対応プレーヤー相当のデコードOK)。`cargo test`は7件通過。
- 次: (1) open-barで`open-av` MKVを開き、DSDを主音声・映像を追従表示(音声マスタークロック、libmpv)。(2) make-diskから「動画+DSD音声」をopen-avで書き出し。(3) 外部ファイル形式(`open-bar`の`.obar.json` Combo)との統一。(4) イマーシブDSD(10ch等)の実データ確認。
- リンクの更新: READMEのmake-disk Windowsリンクはバージョン固定(v0.1.29)。新版が出たら更新するか`releases/latest`を案内。
- **2026-09-25 続き**: 音声だけの形式を**`open-audio`**と命名(ユーザー指示。先に`open-a`と指示されたが最終的に`open-audio`に変更)。`format: "open-audio"`、添付名`open-audio.json`、`.mka`。`pack`は形式で出力拡張子を切替。並列実行で一時名が衝突する実バグ(`open_av_inspect_<pid>`)を通し番号で修正。`cargo test`は単体7+E2E 3が通過。
