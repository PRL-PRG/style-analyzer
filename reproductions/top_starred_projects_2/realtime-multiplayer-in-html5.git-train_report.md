# Train report for javascript / file:///tmp/top-repos-quality-repos-d1qpf08f/realtime-multiplayer-in-html5.git HEAD 00af50dd57baa29f30809925881a624bc50234f1

### Classification report

PPCR: 0.395

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.995| 1.000| 0.610| 0.997| 0.756| 3295| 5401| 0.610 |
| `'` | 1.000| 1.000| 0.379| 1.000| 0.550| 140| 369| 0.379 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 1873| 0.005 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 279| 0.025 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 92| 0.011 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 358| 0.000 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 172| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 112| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 92| 0.000 |
| `micro avg` | 0.995| 0.995| 0.393| 0.995| 0.563| 3453| 8748| 0.395 |
| `macro avg` | 0.222| 0.222| 0.110| 0.222| 0.145| 3453| 8748| 0.395 |
| `weighted avg` | 0.990| 0.995| 0.393| 0.992| 0.490| 3453| 8748| 0.395 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2106 |3295 |0 |0 |0 |0 |0 |0 |0 |0 |
|1863 |10 |0 |0 |0 |0 |0 |0 |0 |0 |
|229 |0 |0 |140 |0 |0 |0 |0 |0 |0 |
|358 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|272 |7 |0 |0 |0 |0 |0 |0 |0 |0 |
|172 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|112 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|91 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|92 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| game.core.js | 14 |
| game.server.js | 3 |
| lib/keyboard.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 140}, "macro avg": {"f1-score": 0.22191955878396558, "precision": 0.22161853975919776, "recall": 0.2222222222222222, "support": 3453}, "micro avg": {"f1-score": 0.9947871416159861, "precision": 0.9947871416159861, "recall": 0.9947871416159861, "support": 3453}, "weighted avg": {"f1-score": 0.9921878122613665, "precision": 0.9896026054326702, "recall": 0.9947871416159861, "support": 3453}, "\u2205": {"f1-score": 0.9972760290556901, "precision": 0.99456685783278, "recall": 1.0, "support": 3295}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}},
  "cl_report_full": {"\u0027": {"f1-score": 0.5500982318271119, "precision": 1.0, "recall": 0.3794037940379404, "support": 369}, "macro avg": {"f1-score": 0.14515028169409958, "precision": 0.22161853975919776, "recall": 0.1099417780986837, "support": 8748}, "micro avg": {"f1-score": 0.5630686009343497, "precision": 0.9947871416159861, "recall": 0.39266117969821673, "support": 8748}, "weighted avg": {"f1-score": 0.4901138249102034, "precision": 0.6562249198850989, "recall": 0.39266117969821673, "support": 8748}, "\u2205": {"f1-score": 0.7562543034197843, "precision": 0.99456685783278, "recall": 0.610072208850213, "support": 5401}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 358}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 172}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 279}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1873}},
  "ppcr": 0.394718792866941
}
```
</details>
