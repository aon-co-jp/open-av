# open-av 仕様 v0.1 / Specification v0.1

## 1. パッケージ / Package

open-avの単一ファイル形式は**Matroska(.mkv)**で、次を含む。 / The single-file form is **Matroska (.mkv)** containing:

1. **映像ストリーム** — MP4/MKVの映像をそのままコピー(再エンコードしない)。 / the untouched video stream.
2. **互換音声**(推奨、`role: "fallback"`)— 通常のAAC/FLAC/Opus/PCMの音声ストリーム。open-av非対応のプレーヤーが鳴らす。 / a normal audio stream for players without open-av support.
3. **添付ファイル** — マニフェストが参照するDSD(DSF/DSDIFF)などのファイル。MIMEタイプ: `audio/x-dsf`、`audio/x-dsdiff`、`audio/flac`、`audio/wav`。 / attachments referenced by the manifest.
4. **マニフェスト** — 添付ファイル名`open-av.json`、MIME`application/json`。 / attachment named `open-av.json`.

外部ファイル形式(MKVに同梱せず、`open-av.json`と同じフォルダに映像・音声を並べる)も許す。`file`は相対パス。 / An external-files form (manifest next to the media files, `file` = relative path) is also allowed.

## 2. マニフェスト / Manifest

```json
{
  "format": "open-av", "version": "0.1", "title": "…",
  "video": { "file": "movie.mp4", "stream_index": 0 },          // 外部ファイル形式のとき / external form only
  "audio_tracks": [ { …AudioTrack… } ],
  "sync": { "audio_offset_ms": 0 }
}
```

### AudioTrack

| キー | 意味 |
|---|---|
| `id` | 一意なID |
| `kind` | `dsd` / `pcm` / `opaque`(素通し) |
| `role` | `main`(**ちょうど1本**)/ `fallback` / `commentary` / `alternate` |
| `rate_hz` | DSDならビットレート(2,822,400または3,072,000の整数倍)、PCMならサンプルレート |
| `channels` | チャンネル数 |
| `layout` | プリセット名(`mono` `stereo` `5.1` `7.1` `9.1-height` `11.1-height`)またはスピーカー位置名の配列。**数は`channels`と一致**、重複不可 |
| `file` | `dsd`は必須(添付名/相対パス)。`opaque`は`file`か`stream_index` |
| `stream_index` | コンテナ内の音声ストリーム番号(`pcm`) |
| `codec` / `decode` | `opaque`のみ。`decode`は**`"none"`必須**、`codec`は`mqa-flac`・`auro-cx`など |
| `language` `title` `note` | 任意 |

### スピーカー位置名 / Speaker names

FL/FR(前方左右)、FC(中央)、LFE、SL/SR(側方)、BL/BR(後方)、TFL/TFR(前方の高さ)、TSL/TSR(側方の高さ)、TBL/TBR(後方の高さ)、TC(頭上中央)。ITU-R BS.2051・ffmpegの一般的な略称に沿う。`9.1-height` = FL FR FC LFE SL SR TFL TFR TSL TSR(10ch)、`11.1-height` = それ+TC(11ch)。

## 3. 再生側の規則 / Player rules

- `role: "main"`のトラックを最優先で再生する。ハードウェアがDSD(ネイティブ/DoP)に対応していればそのまま、非対応ならPCMへ変換して再生する(open-barの再生計画に従う)。
- `kind: "opaque"`は**解釈しない**。ビットパーフェクトな経路で対応機器へ渡せるときだけ再生し、渡せなければ再生しない/通常の互換音声へ落とす。
- 音声を時間の基準(マスタークロック)にして映像を追従させ、`sync.audio_offset_ms`(音声の遅れ)を適用する。

## 4. 非目標 / Non-goals

MQA・Auro-CX・Auro-3D・Dolby Atmos等の**復号・再エンコード・互換性の主張**はしない。 / No decoding, re-encoding or compatibility claims for MQA, Auro-CX, Auro-3D, Dolby Atmos or similar proprietary formats.
