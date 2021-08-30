# Test report for javascript / file:///tmp/top-repos-quality-repos-4i_imopo/clockpicker.git HEAD e6ac014b3c167281ac37cf122ab19b6967d8fae4

### Classification report

PPCR: 0.935

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.607| 0.970| 0.970| 0.746| 0.746| 4821| 4821| 1.000 |
| `␣` | 0.806| 0.519| 0.493| 0.631| 0.612| 4712| 4962| 0.950 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 411| 424| 0.969 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 406| 682| 0.595 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 379| 473| 0.801 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 115| 234| 0.491 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.620| 0.657| 0.614| 0.606| 0.572| 10844| 11596| 0.935 |
| `micro avg` | 0.657| 0.657| 0.614| 0.657| 0.635| 10844| 11596| 0.935 |
| `macro avg` | 0.202| 0.213| 0.209| 0.197| 0.194| 10844| 11596| 0.935 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⏎| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|0 |4676 |138 |7 |0 |0 |0 |0 |
|250 |2170 |2444 |98 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |
|276 |178 |228 |0 |0 |0 |0 |0 |
|94 |170 |209 |0 |0 |0 |0 |0 |
|119 |105 |10 |0 |0 |0 |0 |0 |
|13 |409 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19681545029439193, "precision": 0.2018538560654735, "recall": 0.2126569962856033, "support": 10844}, "micro avg": {"f1-score": 0.6565842862412394, "precision": 0.6565842862412394, "recall": 0.6565842862412394, "support": 10844}, "weighted avg": {"f1-score": 0.6061528491078167, "precision": 0.6200730003180127, "recall": 0.6565842862412394, "support": 10844}, "\u2205": {"f1-score": 0.7464282863756085, "precision": 0.6066424494032174, "recall": 0.9699232524372536, "support": 4821}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 406}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 379}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 411}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u2423": {"f1-score": 0.6312798656851349, "precision": 0.8063345430550973, "recall": 0.5186757215619694, "support": 4712}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19399476851173775, "precision": 0.2018538560654735, "recall": 0.208923797391422, "support": 11596}, "micro avg": {"f1-score": 0.6345811051693406, "precision": 0.6565842862412394, "recall": 0.614004829251466, "support": 11596}, "weighted avg": {"f1-score": 0.5720048207233304, "precision": 0.5972451924122373, "recall": 0.614004829251466, "support": 11596}, "\u2205": {"f1-score": 0.7464282863756085, "precision": 0.6066424494032174, "recall": 0.9699232524372536, "support": 4821}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 682}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 473}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 424}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 234}, "\u2423": {"f1-score": 0.6115350932065557, "precision": 0.8063345430550973, "recall": 0.4925433293027005, "support": 4962}},
  "ppcr": 0.93515005174198
}
```
</details>
