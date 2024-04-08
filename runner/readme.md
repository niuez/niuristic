## runner

```
Usage: runner <IN_FOLDER> <OUT_FOLDER> [PROGRAM]...
```

`<IN_FOLDER>`の各ファイルについて、`[PROGRAM]`にそれを入力した結果を`<OUT_FOLDER>`に保存します。
runnerは実行の最後に`<input_filename>\\t<プログラムが出力したstderrの最終行>\\n`を結合したものを出力します
タブ区切りで出力すると、spreadsheetに貼り付けられます。
