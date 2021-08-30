# Train report for javascript / file:///tmp/top-repos-quality-repos-vk_0mqpi/seen.git HEAD d8946b3b97b9814e78f79334b9fd6349b9022289

### Classification report

PPCR: 0.456

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 1.000| 0.821| 0.986| 0.890| 4013| 4886| 0.821 |
| `␣` | 1.000| 0.515| 0.031| 0.680| 0.060| 198| 3289| 0.060 |
| `"` | 1.000| 1.000| 0.419| 1.000| 0.591| 52| 124| 0.419 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 463| 0.032 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 182| 0.027 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 179| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 264| 0.000 |
| `weighted avg` | 0.969| 0.973| 0.444| 0.967| 0.492| 4283| 9387| 0.456 |
| `macro avg` | 0.425| 0.359| 0.182| 0.381| 0.220| 4283| 9387| 0.456 |
| `micro avg` | 0.973| 0.973| 0.444| 0.973| 0.610| 4283| 9387| 0.456 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|873 |4013 |0 |0 |0 |0 |0 |0 |
|3091 |96 |102 |0 |0 |0 |0 |0 |
|448 |15 |0 |0 |0 |0 |0 |0 |
|179 |0 |0 |0 |0 |0 |0 |0 |
|177 |5 |0 |0 |0 |0 |0 |0 |
|72 |0 |0 |0 |0 |0 |52 |0 |
|264 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| site/lib/StackBlur.js | 77 |
| site/lib/2048/game_manager.js | 13 |
| site/lib/2048/keyboard_input_manager.js | 9 |
| site/lib/2048/html_actuator.js | 5 |
| site/lib/2048/grid.js | 4 |
| site/lib/2048/local_storage_manager.js | 3 |
| site/lib/2048/tile.js | 3 |
| site/lib/2048/classlist_polyfill.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 52}, "macro avg": {"f1-score": 0.38082184089553284, "precision": 0.42455800435940905, "recall": 0.3593073593073593, "support": 4283}, "micro avg": {"f1-score": 0.97291618024749, "precision": 0.97291618024749, "recall": 0.97291618024749, "support": 4283}, "weighted avg": {"f1-score": 0.9671880300248457, "precision": 0.9690074481578705, "recall": 0.97291618024749, "support": 4283}, "\u2205": {"f1-score": 0.9857528862687301, "precision": 0.9719060305158634, "recall": 1.0, "support": 4013}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.6799999999999999, "precision": 1.0, "recall": 0.5151515151515151, "support": 198}},
  "cl_report_full": {"\"": {"f1-score": 0.5909090909090909, "precision": 1.0, "recall": 0.41935483870967744, "support": 124}, "macro avg": {"f1-score": 0.2201946129271127, "precision": 0.42455800435940905, "recall": 0.18167050610520488, "support": 9387}, "micro avg": {"f1-score": 0.6096561814191661, "precision": 0.97291618024749, "recall": 0.4439117929050815, "support": 9387}, "weighted avg": {"f1-score": 0.4922885635519839, "precision": 0.8694719148929912, "recall": 0.4439117929050815, "support": 9387}, "\u2205": {"f1-score": 0.8902939545202441, "precision": 0.9719060305158634, "recall": 0.8213262382316824, "support": 4886}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 463}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 264}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 179}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "\u2423": {"f1-score": 0.06015924506045414, "precision": 1.0, "recall": 0.031012465795074492, "support": 3289}},
  "ppcr": 0.4562693086183019
}
```
</details>
