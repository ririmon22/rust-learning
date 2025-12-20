## フェーズ1：言語機能を「設計として使う」（001–030）

### Option / Result / enum（007–015）

- Option を設計として使う（Cache / Slot）
- Option::take / replace を使い切る
- Result を返す API 設計
- 自作 Error enum を作る
- Result の合成（map_err / and_then）
- if を match に置き換える
- if let / while let 禁止ノック
- 状態 enum（Init / Running / Done）
- match exhaustiveness 破壊テスト

→ 「ある / ない」「成功 / 失敗」「状態遷移」を  
　分岐ではなく型で表現する練習

---

### 所有権・借用（016–023）

- clone 禁止コレクション操作
- &T と &mut T の API 設計
- String / &str 境界ノック
- Vec<T> を安全にラップ
- Box<T> を使う理由を作る
- 関数境界で所有権を移す
- 所有権パズル（通すだけ）
- borrow checker を文章で説明

→ borrow checker の判断を  
　**説明できる状態**を目指す

---

### trait / generics（024–030）

- trait を定義する
- trait を実装する
- trait object を使う
- ジェネリクス境界を設計
- where 句ノック
- Into / From 実装
- ゼロコスト抽象化体験

→ 抽象化を「便利さ」ではなく  
　**境界設計**として理解する

---

## フェーズ2：Rustで「小さな道具」を作る（031–070）

**コード量：100〜300行**

### Iterator / Collection（031–040）

- 自作 Iterator
- map 相当を実装
- filter 相当を実装
- fold で集約
- 状態付き Iterator
- Peekable 実装
- Window Iterator
- Zip Iterator
- flatten 的構造
- Iterator API 設計レビュー

---

### CLI / I/O（041–050）

- std::fs を使う
- 行数カウンタ
- wc-lite
- grep-lite
- 引数パーサ手書き
- stdin / stdout 対応
- エラーメッセージ改善
- テストを書く
- CLI 設計見直し
- ミニ CLI ツール完成

---

### 設計・構造（051–060）

- モジュール分割
- public / private 設計
- 内部構造を隠す
- Builder pattern
- 状態遷移 enum
- 設定ファイル読み込み
- JSON パース
- エラー設計整理
- ログ設計
- 設計リファクタ

---

### 低レイヤ前段（061–070）

- Vec の内部を観察
- Drop トレイト観察
- RAII 実験
- Rc / RefCell
- Weak で循環回避
- Cell / RefCell 比較
- Copy vs Clone
- Pin を触る
- unsafe の入口に立つ
- 安全ラッパを設計

---

## フェーズ3：Rustで「小さな世界」を作る（071–100）

**コード量：300〜800行**

### 実行モデル（071–080）

- スタックマシン
- 命令 enum 設計
- 実行ループ
- エラー停止
- デバッグ表示
- トレース実行
- ステップ実行
- 命令追加
- 前処理最適化
- ミニ VM 完成

---

### unsafe / メモリ（081–090）

- raw pointer 基礎
- Box を自作
- bump allocator
- arena allocator
- lifetime 手動管理
- unsafe API 設計
- soundness 検討
- Miri 実行
- UB 回避設計
- 安全ラッパ完成

---

### 仕上げ（091–100）

- 全体リファクタ
- README 作成
- ベンチマーク
- テスト拡充
- API 固定
- rustdoc 整備
- unsafe 部分解説
- 設計振り返り
- ポートフォリオ整理
- 完走

---