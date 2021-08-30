# Train report for javascript / file:///tmp/top-repos-quality-repos-svlqhn4t/rogue.js.git HEAD 7caf7d38d5bcf9bb97f10923ba8766aace049616

### Classification report

PPCR: 0.251

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 1.000| 0.442| 0.997| 0.612| 646| 1462| 0.442 |
| `'` | 1.000| 1.000| 0.494| 1.000| 0.661| 161| 326| 0.494 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 199| 0.010 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 98| 0.020 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1035| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 106| 0.000 |
| `micro avg` | 0.995| 0.995| 0.250| 0.995| 0.400| 811| 3226| 0.251 |
| `macro avg` | 0.332| 0.333| 0.156| 0.333| 0.212| 811| 3226| 0.251 |
| `weighted avg` | 0.990| 0.995| 0.250| 0.993| 0.344| 811| 3226| 0.251 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|816 |646 |0 |0 |0 |0 |0 |
|1035 |0 |0 |0 |0 |0 |0 |
|165 |0 |0 |161 |0 |0 |0 |
|197 |2 |0 |0 |0 |0 |0 |
|106 |0 |0 |0 |0 |0 |0 |
|96 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/cli/lib/commands/dev.js | 2 |
| packages/cli/lib/commands/build.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 161}, "macro avg": {"f1-score": 0.33281893004115226, "precision": 0.3323076923076923, "recall": 0.3333333333333333, "support": 811}, "micro avg": {"f1-score": 0.9950678175092479, "precision": 0.9950678175092479, "recall": 0.9950678175092479, "support": 811}, "weighted avg": {"f1-score": 0.9926093376566043, "precision": 0.9901659869107464, "recall": 0.9950678175092479, "support": 811}, "\u2205": {"f1-score": 0.9969135802469136, "precision": 0.9938461538461538, "recall": 1.0, "support": 646}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6611909650924024, "precision": 1.0, "recall": 0.4938650306748466, "support": 326}, "macro avg": {"f1-score": 0.21215556488913778, "precision": 0.3323076923076923, "recall": 0.15595424929852095, "support": 3226}, "micro avg": {"f1-score": 0.39980183304433986, "precision": 0.9950678175092479, "recall": 0.25015499070055797, "support": 3226}, "weighted avg": {"f1-score": 0.3440532172543544, "precision": 0.5514578663741714, "recall": 0.25015499070055797, "support": 3226}, "\u2205": {"f1-score": 0.6117424242424242, "precision": 0.9938461538461538, "recall": 0.4418604651162791, "support": 1462}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 199}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1035}},
  "ppcr": 0.2513949163050217
}
```
</details>
