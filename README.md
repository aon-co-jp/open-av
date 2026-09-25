# open-av

[English](#english) / 日本語

**MP4/MKVの映像 + DSD音声 + イマーシブ(高さ方向)配置メタデータ**を、1つのMKVにまとめる**オープンなビデオ形式のプロファイル**と、その参照実装(Rust)です。「動画は普通に見られて、対応プレーヤーなら音声がDSD」という後方互換の形を目指しています。

## ダウンロード・関連リンク

| | リンク |
|---|---|
| **make-disk 最新インストーラー**(Windows/macOS/Linux/Android、常に最新版) | https://github.com/aon-co-jp/make-disk/releases/latest |
| make-disk 最新版のWindowsインストーラー(v0.1.29) | https://github.com/aon-co-jp/make-disk/releases/download/v0.1.29/make-disk_0.1.29_x64-setup.exe |
| **make-disk 最新README**(機能・使い方) | https://github.com/aon-co-jp/make-disk#readme |
| open-bar(再生ソフト、DSD/DoP/排他モード) | https://github.com/aon-co-jp/open-bar |
| open-mqa / open-mqa-dsd(WAV・DoP・DSDの部品) | https://github.com/aon-co-jp/open-mqa / https://github.com/aon-co-jp/open-mqa-dsd |

> 「常に最新版」は`releases/latest`のリンクです。バージョン固定のWindowsリンク(v0.1.29)は、新しい版が出たら古くなるので、まず`releases/latest`を開いてください。

## 形式の概要(詳細は[SPEC.md](SPEC.md))

- **1つのMKV**に、①映像(MP4/MKVの映像ストリームをそのままコピー)、②互換用の通常音声(AAC/FLAC/Opus/PCM)、③**DSD音声(DSF)を添付ファイルとして同梱**、④マニフェスト`open-av.json`(トラック一覧・チャンネル配置・同期)を入れる。
- open-av非対応のプレーヤーは、③④を無視して**映像+互換音声として普通に再生**できる(実ffmpegで確認)。対応プレーヤー(`open-bar`を予定)は、DSDを主音声として鳴らす。
- MP4/MKVにはDSDの入れ場所(標準のコーデック)が無いため、Matroskaの**添付ファイル**(任意のMIMEタイプ)を使う。DSDファイルは**ビット単位で一致して**取り出せる(テスト済み)。
- **イマーシブ配置**: チャンネル配置をスピーカー位置名(FL/FR/FC/LFE/SL/SR/TFL/TFR/TSL/TSR/TC…)で表す。`9.1-height`(5.1+高さ4本)、`11.1-height`などのプリセットあり。特定社の商標名や独自符号化には依存しない。

## MQAとAuro-CXについて(正直な開示)

**MQAもAuro-CXも、特許・営業秘密で保護された非公開技術で、オープンソース版は存在せず、open-avも復号・再実装しません。** 「MQAとAuro-CXのオープンソースを融合」という構想は、次のように解釈して実現しています。

- **MQA**: 公開規格(WAV/FLAC/DSD/DoP)で、同じ目的(高解像度を配信帯域に収める)を狙う`open-mqa`・`open-mqa-dsd`を部品として使う。すでにMQAでエンコードされた音声は、`kind = "opaque"`・`decode = "none"`として**中身に触れずに運ぶ**だけ(MQA対応DACへビットパーフェクトで素通し)。
- **Auro-CX**: イマーシブ(高さ方向)の**チャンネル配置**を、公開のスピーカー位置名で表現できるようにした。Auro-CXでエンコード済みのストリームは、同様に素通しとして運べるが、復号は認定デコーダに任せる。**Auro-3D/Auro-CXとの互換性は主張しない**。

## 現状(2026-09-25、初版 v0.1)

| 機能 | 状態 |
|---|---|
| マニフェストの仕様と検証(`open-av validate`) | ✅ 単体テスト5件 |
| 映像+DSD+マニフェストを1つのMKVへ(`open-av pack`) | ✅ 実ffmpegで確認 |
| 構成の確認・添付の取り出し(`inspect` / `extract`) | ✅ DSDがビット一致で往復 |
| 非対応プレーヤーでの通常再生 | ✅ ffmpegで映像+音声をデコードできることを確認 |
| 対応プレーヤー(open-barでDSD主音声を鳴らす) | ❌ 未実装(次) |
| make-diskからの書き出し | ❌ 未実装(次) |
| イマーシブDSDの実データでの再生確認 | ❌ 未確認(配置メタデータの定義のみ) |

## 使い方

```
open-av validate manifest.json
open-av pack video.mp4 manifest.json out.mkv track.dsf     映像+DSD+マニフェストを1つのMKVへ
open-av inspect out.mkv                                    構成とマニフェストを表示
open-av extract out.mkv out_dir                            添付(DSD・マニフェスト)を取り出す
```

ffmpeg/ffprobeが必要です(環境変数`OPEN_AV_FFMPEG`/`OPEN_AV_FFPROBE`で場所を指定可)。

<a id="english"></a>
## English

**An open video-format profile plus a Rust reference implementation** that bundles **MP4/MKV video + DSD audio + immersive (height) layout metadata** into a single MKV: the video plays normally everywhere, and a capable player plays the DSD audio.

**Links:** [make-disk latest installers](https://github.com/aon-co-jp/make-disk/releases/latest) (all platforms; always the newest version) · [Windows installer v0.1.29](https://github.com/aon-co-jp/make-disk/releases/download/v0.1.29/make-disk_0.1.29_x64-setup.exe) · [make-disk latest README](https://github.com/aon-co-jp/make-disk#readme) · [open-bar player](https://github.com/aon-co-jp/open-bar) · [open-mqa](https://github.com/aon-co-jp/open-mqa) / [open-mqa-dsd](https://github.com/aon-co-jp/open-mqa-dsd). The pinned Windows link goes stale when a new release ships; open `releases/latest` first.

**How it works:** the MKV carries the untouched video stream, a normal fallback audio track, the DSD file(s) as **Matroska attachments** (MP4/MKV have no standard DSD codec), and a manifest `open-av.json` (tracks, channel layouts, sync). Players without open-av support ignore the attachments and play video + fallback audio (verified with ffmpeg); a capable player plays DSD as the main audio. DSD files extract **bit-exactly** (tested). Channel layouts are expressed as speaker positions (`9.1-height` = 5.1 + 4 height channels, `11.1-height`, …) without depending on any vendor trademark or codec.

**MQA and Auro-CX (honest disclosure):** both are patented, proprietary technologies with no open-source implementation; open-av neither decodes nor reimplements them. MQA-encoded audio can be carried untouched as `kind: "opaque"`, `decode: "none"` (pass it bit-perfectly to an MQA-capable DAC); the open parts of the idea come from `open-mqa`/`open-mqa-dsd` (WAV/FLAC/DSD/DoP). Auro-CX streams can likewise be carried as opaque passthrough for a licensed decoder; **no Auro-3D/Auro-CX compatibility is claimed**.

**Status (2026-09-25, v0.1):** manifest spec + validation (5 unit tests), `pack` / `inspect` / `extract` verified with real ffmpeg (DSD round-trips bit-exactly, fallback video+audio still decodes). Not yet: playback of the DSD main track in open-bar, export from make-disk, real immersive-DSD playback tests.
