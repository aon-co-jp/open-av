//! open-av CLI: `validate` / `pack` / `inspect` / `extract`。

use open_av::{extract, inspect, make_preview, pack, Manifest, DEFAULT_FADE_SECS, DEFAULT_PREVIEW_SECS};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn usage() -> ExitCode {
    eprintln!(
        "使い方 / usage:\n  open-av validate <manifest.json>\n  open-av pack <video.mp4|mkv> <manifest.json> <out.mkv> [asset...]   映像+DSD等+マニフェストを1つのMKVへ\n  open-av inspect <file.mkv>                                          構成とマニフェストを表示\n  open-av extract <file.mkv> <out_dir>                                添付(DSD・マニフェスト)を取り出す\n  open-av preview <in> <out> [秒数(既定30)]                           試聴用の短縮版(フェードアウト付き)を作る"
    );
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("validate") if args.len() == 2 => match std::fs::read_to_string(&args[1]).map_err(|e| e.to_string()).and_then(|t| Manifest::from_json(&t).map_err(|e| e.to_string())) {
            Ok(m) => match m.validate() {
                Ok(()) => {
                    println!("OK: {} 音声トラック / {} audio track(s)", m.audio_tracks.len(), m.audio_tracks.len());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("NG: {e}");
                    ExitCode::FAILURE
                }
            },
            Err(e) => {
                eprintln!("読み込めません: {e}");
                ExitCode::FAILURE
            }
        },
        Some("pack") if args.len() >= 4 => {
            let m = match std::fs::read_to_string(&args[2]).map_err(|e| e.to_string()).and_then(|t| Manifest::from_json(&t).map_err(|e| e.to_string())) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("マニフェストを読めません: {e}");
                    return ExitCode::FAILURE;
                }
            };
            let assets: Vec<PathBuf> = args[4..].iter().map(PathBuf::from).collect();
            match pack(Path::new(&args[1]), &m, &assets, Path::new(&args[3])) {
                Ok(()) => {
                    println!("作成しました / created: {}", args[3]);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("{e}");
                    ExitCode::FAILURE
                }
            }
        }
        Some("inspect") if args.len() == 2 => match inspect(Path::new(&args[1])) {
            Ok(i) => {
                println!("映像 {} / 音声 {} / 添付 {}", i.video_streams, i.audio_streams, i.attachments.len());
                for a in &i.attachments {
                    println!("  添付: {} ({})", a.filename, a.mimetype);
                }
                match i.manifest {
                    Some(m) => println!("{}", m.to_json()),
                    None => println!("open-avマニフェストはありません(通常のMKVです)"),
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        },
        Some("extract") if args.len() == 3 => match extract(Path::new(&args[1]), Path::new(&args[2])) {
            Ok(v) => {
                for p in v {
                    println!("{}", p.display());
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        },
        Some("preview") if args.len() == 3 || args.len() == 4 => {
            let secs = args.get(3).and_then(|s| s.parse::<f64>().ok()).unwrap_or(DEFAULT_PREVIEW_SECS);
            match make_preview(Path::new(&args[1]), Path::new(&args[2]), secs, DEFAULT_FADE_SECS) {
                Ok(()) => {
                    println!("作成しました / created: {}", args[2]);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("{e}");
                    ExitCode::FAILURE
                }
            }
        }
        _ => usage(),
    }
}
