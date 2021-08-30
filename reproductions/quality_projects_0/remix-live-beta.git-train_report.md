# Train report for javascript / file:///tmp/top-repos-quality-repos-gmtq_2m9/remix-live-beta.git HEAD b74e67604b13c5f5eb29bfa1ccc02582addc20a6

### Classification report

PPCR: 0.878

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.991| 0.969| 0.988| 0.977| 42689| 43641| 0.978 |
| `␣` | 0.964| 0.975| 0.933| 0.969| 0.948| 20641| 21558| 0.957 |
| `'` | 1.000| 1.000| 0.975| 1.000| 0.987| 6926| 7107| 0.975 |
| `⏎␣⁻␣⁻` | 0.948| 0.940| 0.759| 0.944| 0.843| 2023| 2504| 0.808 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 288| 4165| 0.069 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 142| 2563| 0.055 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 1291| 0.010 |
| `macro avg` | 0.557| 0.558| 0.519| 0.557| 0.537| 72722| 82829| 0.878 |
| `micro avg` | 0.980| 0.980| 0.860| 0.980| 0.916| 72722| 82829| 0.878 |
| `weighted avg` | 0.974| 0.980| 0.860| 0.977| 0.872| 72722| 82829| 0.878 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|952 |42285 |380 |0 |0 |0 |24 |0 |
|917 |436 |20124 |0 |0 |0 |81 |0 |
|181 |0 |0 |6926 |0 |0 |0 |0 |
|3877 |22 |266 |0 |0 |0 |0 |0 |
|2421 |47 |95 |0 |0 |0 |0 |0 |
|481 |111 |11 |0 |0 |0 |1901 |0 |
|1278 |6 |7 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/app/files/file-explorer.js | 82 |
| src/app/panels/terminal.js | 70 |
| src/app.js | 44 |
| src/app/tabs/debugger/debuggerUI/vmDebugger/DropdownPanel.js | 39 |
| src/app/editor/editor.js | 36 |
| src/app/ui/modal-dialog-custom.js | 32 |
| src/app/tabs/runTab/model/recorder.js | 32 |
| test-browser/tests/transactionExecution.js | 32 |
| src/app/compiler/compiler-imports.js | 31 |
| src/app/tabs/runTab/model/dropdownlogic.js | 30 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6926}, "macro avg": {"f1-score": 0.5572775311118604, "precision": 0.5566878867323489, "recall": 0.5578832131558514, "support": 72722}, "micro avg": {"f1-score": 0.9795660185363438, "precision": 0.9795660185363438, "recall": 0.9795660185363438, "support": 72722}, "weighted avg": {"f1-score": 0.9765826828923768, "precision": 0.9736264950785919, "recall": 0.9795660185363438, "support": 72722}, "\u2205": {"f1-score": 0.9880134585728304, "precision": 0.9855035308923952, "recall": 0.9905362037058727, "support": 42689}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 288}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9436584760486474, "precision": 0.9476570289132602, "recall": 0.939693524468611, "support": 2023}, "\u2423": {"f1-score": 0.9692707831615451, "precision": 0.9636546473207872, "recall": 0.9749527639164769, "support": 20641}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9871018313974205, "precision": 1.0, "recall": 0.9745321514000281, "support": 7107}, "macro avg": {"f1-score": 0.5365130357299238, "precision": 0.5566878867323489, "recall": 0.5194467894844185, "support": 82829}, "micro avg": {"f1-score": 0.9159182518916625, "precision": 0.9795660185363438, "recall": 0.8600369435825617, "support": 82829}, "weighted avg": {"f1-score": 0.8718433980493863, "precision": 0.8845061473519584, "recall": 0.8600369435825617, "support": 82829}, "\u2205": {"f1-score": 0.9771456301705412, "precision": 0.9855035308923952, "recall": 0.9689283013679797, "support": 43641}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4165}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1291}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2563}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8430155210643014, "precision": 0.9476570289132602, "recall": 0.759185303514377, "support": 2504}, "\u2423": {"f1-score": 0.9483282674772036, "precision": 0.9636546473207872, "recall": 0.9334817701085444, "support": 21558}},
  "ppcr": 0.8779775199507419
}
```
</details>
