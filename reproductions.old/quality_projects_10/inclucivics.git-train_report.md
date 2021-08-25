# Train report for javascript / file:///tmp/top-repos-quality-repos-xg5skbws/inclucivics.git HEAD 1a89419c5b414b0aca7c72316fa9fd5f22d5f1c9

### Classification report

PPCR: 0.084

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.495| 1.000| 0.662| 197| 398| 0.495 |
| `␣` | 0.985| 1.000| 0.162| 0.992| 0.279| 196| 1207| 0.162 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 185| 0.016 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2327| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 316| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 182| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 108| 0.000 |
| `macro avg` | 0.284| 0.286| 0.094| 0.285| 0.134| 396| 4723| 0.084 |
| `micro avg` | 0.992| 0.992| 0.083| 0.992| 0.154| 396| 4723| 0.084 |
| `weighted avg` | 0.985| 0.992| 0.083| 0.989| 0.127| 396| 4723| 0.084 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2327 |0 |0 |0 |0 |0 |0 |0 |
|1011 |0 |196 |0 |0 |0 |0 |0 |
|201 |0 |0 |197 |0 |0 |0 |0 |
|316 |0 |0 |0 |0 |0 |0 |0 |
|182 |0 |3 |0 |0 |0 |0 |0 |
|182 |0 |0 |0 |0 |0 |0 |0 |
|108 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/ChartTabs.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 197}, "macro avg": {"f1-score": 0.284629294755877, "precision": 0.28356066044508255, "recall": 0.2857142857142857, "support": 396}, "micro avg": {"f1-score": 0.9924242424242424, "precision": 0.9924242424242424, "recall": 0.9924242424242424, "support": 396}, "weighted avg": {"f1-score": 0.9886651323360185, "precision": 0.9849626922491245, "recall": 0.9924242424242424, "support": 396}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9924050632911393, "precision": 0.9849246231155779, "recall": 1.0, "support": 196}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6621848739495798, "precision": 1.0, "recall": 0.4949748743718593, "support": 398}, "macro avg": {"f1-score": 0.13442714212285198, "precision": 0.28356066044508255, "recall": 0.09390870793784283, "support": 4723}, "micro avg": {"f1-score": 0.15354561437780817, "precision": 0.9924242424242424, "recall": 0.08320982426423883, "support": 4723}, "weighted avg": {"f1-score": 0.12705216192478644, "precision": 0.3359737497566171, "recall": 0.08320982426423883, "support": 4723}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2327}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 316}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 185}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "\u2423": {"f1-score": 0.27880512091038406, "precision": 0.9849246231155779, "recall": 0.1623860811930406, "support": 1207}},
  "ppcr": 0.08384501376243912
}
```
</details>
