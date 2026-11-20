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

## 音声フォーマット(open-mqa-dsdへ移設)

音声だけの単独フォーマット(旧称open-audio)は[`open-mqa-dsd`](https://github.com/aon-co-jp/open-mqa-dsd)の`container`モジュールへ2026-09-26に移設し、`"format": "open-mqa-dsd"`へ改称した。ここ(open-av)は映像プロファイル(`.mkv`)専用として残る。

- 構成はopen-avと同じ: 互換用の通常音声(FLAC等)+ **DSD(DSF)を添付ファイルとして同梱** + マニフェスト`open-mqa-dsd.json`(`"format": "open-mqa-dsd"`)。映像(`video`)は持てない。
- 非対応のプレーヤーは、互換のFLAC等として**普通に再生**できる。対応プレーヤー(`open-bar`で実装済み)は、DSDを主音声として鳴らす。
- 位置づけ: **open-av = 映像 + (open-mqa-dsdの音声専用プロファイル)**。トラック(`dsd`/`pcm`/`opaque`)・チャンネル配置(`stereo`〜`9.1-height`/`11.1-height`)・MQA/Auro-CXの素通し規則・`pack`/`inspect`/`extract`/`make_preview`の実装は共通(`open_mqa_dsd::container`)。
- 例: [`open-mqa-dsd`の`examples/example.open-mqa-dsd.json`](https://github.com/aon-co-jp/open-mqa-dsd/blob/main/examples/example.open-mqa-dsd.json)。`open-av pack fallback.flac manifest.json out.mka track.dsf`で作れる(実ffmpegで往復・DSDのビット一致を確認済み)。

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
| 音声だけの形式(open-mqa-dsdの`container`モジュール、`.mka`)のpack/inspect/extract | ✅ 実ffmpegで確認(DSDビット一致) |
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

**Audio-only format (moved to open-mqa-dsd).** The audio-only profile (formerly called "open-audio", no video, extension `.mka`) was moved into [`open-mqa-dsd`](https://github.com/aon-co-jp/open-mqa-dsd)'s `container` module on 2026-09-26 and renamed `"format": "open-mqa-dsd"`. This crate (open-av) now only covers the video profile, but shares the exact same pack/inspect/extract/preview implementation (`open_mqa_dsd::container`, re-exported here as `open_av::*` for source compatibility).

**MQA and Auro-CX (honest disclosure):** both are patented, proprietary technologies with no open-source implementation; open-av neither decodes nor reimplements them. MQA-encoded audio can be carried untouched as `kind: "opaque"`, `decode: "none"` (pass it bit-perfectly to an MQA-capable DAC); the open parts of the idea come from `open-mqa`/`open-mqa-dsd` (WAV/FLAC/DSD/DoP). Auro-CX streams can likewise be carried as opaque passthrough for a licensed decoder; **no Auro-3D/Auro-CX compatibility is claimed**.

**Status (2026-09-25, v0.1):** manifest spec + validation (5 unit tests), `pack` / `inspect` / `extract` verified with real ffmpeg (DSD round-trips bit-exactly, fallback video+audio still decodes). Not yet: playback of the DSD main track in open-bar, export from make-disk, real immersive-DSD playback tests.

---

作者 / Author: 石 塚 正 浩（MASAHIRO ISHIZUKA） (https://github.com/aon-co-jp)

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, redaktionell überarbeitet; noch nicht von Muttersprachlern geprüft)
- **REST wird vollständig abgeschafft; neue APIs entstehen nicht mehr als REST.** APIs, die aus dem Browser oder per `curl` aufgerufen werden, verwenden **unäre Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body). Zwischen Diensten kommen **gRPC bzw. gRPC-web** zum Einsatz, im selben Stil wie die Admin-API von RPoem (`open_runo.admin.v1.Admin`). Streaming wird erst ergänzt, wenn es tatsächlich gebraucht wird.
- **Bestehende REST-Endpunkte werden schrittweise umgestellt, und zwar immer dann, wenn man ohnehin an ihnen arbeitet.** Sobald ein Endpunkt umgestellt ist, wird die alte REST-Route entfernt. Bis zum Abschluss der Migration bleiben die übrigen bestehen, es werden aber **keine neuen mehr hinzugefügt**.
- **JSON wird einheitlich über RS-JSON verarbeitet**: Gelesen wird mit `from_str_secure` bzw. `parse_secure` (mit Obergrenzen für Größe, Verschachtelungstiefe und doppelte Schlüssel), geschrieben mit `rust_json::fast`. `prost` wird nicht in das Browser-WASM aufgenommen, da es die Datei vergrößern würde.
- **Ausnahmen (Schnittstellen, deren Form ein fremder Standard vorgibt und die nicht zu unserer eigenen API gehören)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Drittanbietern (z. B. GitHub), Health-Checks, die Auslieferung statischer Dateien, OTLP und Ähnliches. Diese nehmen, wie vom jeweiligen Standard verlangt, gewöhnliches HTTP entgegen.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines einzelnen Repositorys diese Richtlinie bereits erfüllt, hängt vom jeweiligen Repository ab. Dieser Abschnitt dokumentiert lediglich die Richtlinie; er besagt nicht, dass der Code schon angepasst wurde.

### Русский (перевод выполнен с помощью ИИ и отредактирован; носителями языка пока не проверен)
- **От REST мы полностью отказываемся; новые API больше не создаются в стиле REST.** API, к которым обращаются из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело запроса — JSON). Для взаимодействия между сервисами применяются **gRPC и gRPC-web** — в том же стиле, что и административный API RPoem (`open_runo.admin.v1.Admin`). Потоковая передача данных добавляется лишь тогда, когда в ней действительно возникает необходимость.
- **Существующие REST-эндпоинты переводятся поэтапно — по мере того, как с ними приходится работать.** После перевода старый REST-маршрут удаляется. Пока миграция не завершена, оставшиеся эндпоинты сохраняются, но **новые не добавляются**.
- **Работа с JSON унифицирована на основе RS-JSON**: чтение — через `from_str_secure` или `parse_secure` (с ограничениями на размер, глубину вложенности и повторяющиеся ключи), запись — через `rust_json::fast`. `prost` в браузерный WASM не включается, поскольку это увеличило бы размер файла.
- **Исключения (интерфейсы, форму которых определяет чужой стандарт, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и тому подобное. Такие интерфейсы принимают обычный HTTP, как того требует соответствующий стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Данный раздел лишь фиксирует политику и не означает, что код уже изменён.

## API 方針の補足: 用語の整理と使い分け (2026-11-04)

### 日本語
**1つに統一するものではなく、別の層の話です。** 「REST の代わり」になるのは Connect・gRPC・GraphQL だけで、WASM は画面を作る技術です。

| 用語 | 何か | REST の代わりか |
|---|---|---|
| REST | URL と HTTP メソッドで操作を表す API の作り方 | 全廃する側 |
| **Connect** | `POST /サービス名/メソッド名` に JSON を送る形式。`curl` でもブラウザの `fetch` でも呼べる | はい(**採用**) |
| **gRPC** | 同じ「サービス/メソッド」の考え方を、バイナリ(protobuf)と HTTP/2 で行う形式。速く、ストリーミングも得意 | はい(**サービス同士に採用**) |
| gRPC-web | gRPC のブラウザ向けの変形版 | はい(必要なら) |
| GraphQL | 欲しい項目を問い合わせる形式 | 一部だけ(**問い合わせ(読み取り)専用**) |
| WASM | Rust をブラウザの中で動かす仕組み | いいえ(画面を作る技術) |

- **ブラウザの画面や `curl` から呼ぶ API は Connect**(POST、JSON を RS-JSON で読み書き)。ブラウザの WASM は素の gRPC を話せず、Connect なら `fetch` のままで、`prost` を足さずに済む(画面のサイズも増えない)。
- **サービス同士の通信(サーバー間、重い処理、ストリーミング)は gRPC。** 多くは Connect で足りるので、ストリーミングが必要になった所だけ gRPC に上げる。
- **GraphQL は、まとめて問い合わせたいときの読み取り専用。** 書き込み(操作)には使わない。
- **WASM は画面を作る技術として、そのまま使う。** 画面から呼ぶ API が Connect になっただけ。
- 外部の規格が形を決めるもの(ACME、OAuth のコールバック、第三者の Webhook、ヘルスチェック、静的ファイル、OTLP)は、規格どおりの HTTP のまま。
- **迷ったら Connect。**

### English
**These are not alternatives to unify; they sit at different layers.** Only Connect, gRPC and GraphQL can replace REST; WASM is a way to build the UI.

| Term | What it is | Replaces REST? |
|---|---|---|
| REST | API style that expresses operations through URLs and HTTP methods | The thing being abolished |
| **Connect** | `POST /Service/Method` with a JSON body; callable from `curl` and from a browser `fetch` | Yes (**adopted**) |
| **gRPC** | The same service/method idea over binary (protobuf) and HTTP/2; fast, good at streaming | Yes (**for service-to-service**) |
| gRPC-web | A browser-friendly variant of gRPC | Yes (if needed) |
| GraphQL | A query language: ask for exactly the fields you want | Partly (**read-only queries**) |
| WASM | Runs Rust inside the browser | No (a UI technology) |

- **APIs called from browser screens or `curl` use Connect** (POST, JSON read and written with RS-JSON). Browser WASM cannot speak plain gRPC; with Connect it stays a `fetch`, and `prost` is not needed (the binary does not grow).
- **Service-to-service calls (server to server, heavy work, streaming) use gRPC.** Connect is enough in most places; move a call to gRPC only where streaming is actually needed.
- **GraphQL is read-only**, for queries that gather several pieces of data. It is not used for writes (operations).
- **WASM stays as the way to build the UI.** The API it calls simply became Connect.
- Interfaces whose shape an outside standard dictates (ACME, OAuth callbacks, third-party webhooks, health checks, static files, OTLP) stay plain HTTP.
- **When in doubt, use Connect.**

### Deutsch (KI-Übersetzung, sprachlich überarbeitet; noch nicht von Muttersprachlern geprüft)
**Es geht nicht darum, eine einzige Technik auszuwählen – die Begriffe liegen auf unterschiedlichen Ebenen.** Als Ersatz für REST kommen nur Connect, gRPC und GraphQL in Frage; WASM dagegen ist eine Technik, mit der die Benutzeroberfläche gebaut wird.

| Begriff | Was ist das? | Ersatz für REST? |
|---|---|---|
| REST | Ein API-Stil, bei dem Operationen über URLs und HTTP-Methoden ausgedrückt werden | wird abgeschafft |
| **Connect** | `POST /Dienst/Methode` mit JSON im Body; aufrufbar mit `curl` ebenso wie mit `fetch` im Browser | ja (**gewählt**) |
| **gRPC** | Dasselbe Prinzip aus Dienst und Methode, aber über binäres protobuf und HTTP/2; schnell und gut für Streaming geeignet | ja (**zwischen Diensten**) |
| gRPC-web | Eine für den Browser angepasste Variante von gRPC | ja (bei Bedarf) |
| GraphQL | Abfragesprache, mit der man genau die benötigten Felder anfordert | teilweise (**nur für lesende Abfragen**) |
| WASM | Führt Rust-Code direkt im Browser aus | nein (Technik für die Oberfläche) |

- **APIs, die von Browseroberflächen oder per `curl` aufgerufen werden, verwenden Connect** (POST; JSON wird mit RS-JSON gelesen und geschrieben). WASM im Browser kann kein reines gRPC sprechen. Mit Connect bleibt es bei einem gewöhnlichen `fetch`, und `prost` ist nicht nötig – die WASM-Datei wird dadurch nicht größer.
- **Für die Kommunikation zwischen Diensten (Server zu Server, rechenintensive Aufgaben, Streaming) wird gRPC eingesetzt.** In den meisten Fällen reicht Connect aus; auf gRPC wird nur dort umgestellt, wo Streaming tatsächlich gebraucht wird.
- **GraphQL dient ausschließlich lesenden Abfragen**, wenn mehrere Informationen gebündelt abgefragt werden sollen. Für Schreibzugriffe (Operationen) wird es nicht verwendet.
- **WASM bleibt die Technik für die Oberfläche.** Geändert hat sich lediglich, dass die aufgerufene API nun auf Connect beruht.
- Schnittstellen, deren Form durch einen externen Standard festgelegt ist (ACME, OAuth-Callbacks, Webhooks von Drittanbietern, Health-Checks, statische Dateien, OTLP), bleiben gewöhnliches HTTP.
- **Im Zweifel: Connect.**

### Русский (перевод выполнен с помощью ИИ и литературно отредактирован; носителями языка пока не проверен)
**Это не варианты, из которых нужно выбрать один, а технологии разных уровней.** Заменить REST могут только Connect, gRPC и GraphQL; WASM же — это способ создания пользовательского интерфейса.

| Термин | Что это | Заменяет REST? |
|---|---|---|
| REST | Стиль построения API, при котором операции выражаются через URL и HTTP-методы | отменяется |
| **Connect** | `POST /Сервис/Метод` с телом в формате JSON; вызывается как из `curl`, так и из браузерного `fetch` | да (**выбран**) |
| **gRPC** | Тот же принцип «сервис — метод», но поверх двоичного формата protobuf и HTTP/2; работает быстро и хорошо подходит для потоковой передачи | да (**для связи между сервисами**) |
| gRPC-web | Адаптированный для браузера вариант gRPC | да (при необходимости) |
| GraphQL | Язык запросов: клиент запрашивает ровно те поля, которые ему нужны | частично (**только запросы на чтение**) |
| WASM | Позволяет выполнять код на Rust прямо в браузере | нет (технология для интерфейса) |

- **API, которые вызываются из браузерных интерфейсов или через `curl`, строятся на Connect** (POST; JSON читается и записывается через RS-JSON). Браузерный WASM не умеет работать с «чистым» gRPC. С Connect достаточно обычного `fetch`, а `prost` не требуется — размер WASM-файла не увеличивается.
- **Для обмена между сервисами (сервер — сервер, ресурсоёмкие задачи, потоковая передача) применяется gRPC.** В большинстве случаев достаточно Connect; на gRPC переходят только там, где потоковая передача действительно необходима.
- **GraphQL используется только для чтения**, когда нужно получить сразу несколько связанных данных. Для записи (операций) он не применяется.
- **WASM по-прежнему остаётся технологией для интерфейса.** Изменилось лишь то, что вызываемый им API теперь построен на Connect.
- Интерфейсы, форма которых задана внешним стандартом (ACME, обратные вызовы OAuth, вебхуки сторонних сервисов, проверки работоспособности, статические файлы, OTLP), остаются обычным HTTP.
- **Если есть сомнения — выбирайте Connect.**
