//! open-av: MP4/MKVの映像 + DSD音声 + イマーシブ配置メタデータ(+ MQA/Auro-CXの素通し)をまとめる、オープンなビデオ形式の
//! プロファイルと参照実装。詳細は`SPEC.md`。**MQAとAuro-CXは復号も再実装もしない**(特許・非公開技術)。
//!
//! **2026-09-26追記**: マニフェスト・パッケージング(`pack`/`inspect`/`extract`)・プレビュー生成の実体は
//! `open-mqa-dsd`クレートの`container`モジュールへ移設した(音声専用の「open-mqa-dsd形式」〈旧称
//! open-audio〉と実装を共有しているため)。ここでは互換のため同じ名前で再エクスポートするだけで、
//! open-av自身は映像プロファイル向けのCLI・ドキュメントを提供する薄いクレートになった。
pub use open_mqa_dsd::container::{
    extract, inspect, make_preview, pack, preset_speakers, AudioTrack, Attachment, Layout, Licensing, Manifest, ManifestError, PackError, PackageInfo, PreviewError, Role, Sync, TrackKind, Video,
    DEFAULT_FADE_SECS, DEFAULT_PREVIEW_SECS, FORMAT_AUDIO, FORMAT_NAME, VERSION,
};

/// `open_av::manifest::{...}`という古い参照パスも当面残す(内部で使っていたコード・ドキュメント向け)。
pub mod manifest {
    pub use open_mqa_dsd::container::{preset_speakers, AudioTrack, Layout, Licensing, Manifest, ManifestError, Role, Sync, TrackKind, Video, FORMAT_AUDIO, FORMAT_NAME, VERSION};
}
