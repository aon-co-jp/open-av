//! open-av: MP4/MKVの映像 + DSD音声 + イマーシブ配置メタデータ(+ MQA/Auro-CXの素通し)をまとめる、オープンなビデオ形式の
//! プロファイルと参照実装。詳細は`SPEC.md`。**MQAとAuro-CXは復号も再実装もしない**(特許・非公開技術)。
pub mod manifest;
pub mod pack;

pub use manifest::{AudioTrack, Layout, Manifest, ManifestError, Role, TrackKind};
pub use pack::{extract, inspect, pack, PackError, PackageInfo};
