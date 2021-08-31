# Train report for javascript / file:///tmp/top-repos-quality-repos-f8kwghfu/ac-nh-turnip-prices.git HEAD 8581f153ca60ba6b8f31791a4eebc40dc3f4afbd

### Classification report

PPCR: 0.488

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.998| 0.779| 0.996| 0.874| 3146| 4029| 0.781 |
| `⏎␣⁻␣⁻` | 0.985| 0.974| 0.918| 0.979| 0.950| 265| 281| 0.943 |
| `⏎␣⁺␣⁺` | 0.977| 0.989| 0.921| 0.983| 0.949| 261| 280| 0.932 |
| `␣` | 0.995| 0.967| 0.084| 0.981| 0.155| 209| 2397| 0.087 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 479| 0.013 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 190| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 150| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 159| 0.000 |
| `macro avg` | 0.494| 0.491| 0.338| 0.492| 0.366| 3887| 7965| 0.488 |
| `weighted avg` | 0.991| 0.993| 0.484| 0.992| 0.556| 3887| 7965| 0.488 |
| `micro avg` | 0.993| 0.993| 0.484| 0.993| 0.651| 3887| 7965| 0.488 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| "| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|883 |3140 |1 |0 |3 |2 |0 |0 |0 |
|2188 |5 |202 |0 |1 |1 |0 |0 |0 |
|473 |6 |0 |0 |0 |0 |0 |0 |0 |
|16 |4 |0 |0 |258 |3 |0 |0 |0 |
|19 |3 |0 |0 |0 |258 |0 |0 |0 |
|190 |0 |0 |0 |0 |0 |0 |0 |0 |
|150 |0 |0 |0 |0 |0 |0 |0 |0 |
|159 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/predictions.js | 11 |
| js/scripts.js | 9 |
| js/translations.js | 5 |
| service-worker.js | 2 |
| js/contributors.js | 1 |
| js/chart.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4923449619068575, "precision": 0.4939224541649364, "recall": 0.49083633076186767, "support": 3887}, "micro avg": {"f1-score": 0.992539233341909, "precision": 0.992539233341909, "recall": 0.992539233341909, "support": 3887}, "weighted avg": {"f1-score": 0.9917568806792414, "precision": 0.9910113767330316, "recall": 0.992539233341909, "support": 3887}, "\u2205": {"f1-score": 0.9961928934010152, "precision": 0.9943001899936669, "recall": 0.9980928162746344, "support": 3146}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9828571428571428, "precision": 0.9772727272727273, "recall": 0.9885057471264368, "support": 261}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9791271347248577, "precision": 0.9847328244274809, "recall": 0.9735849056603774, "support": 265}, "\u2423": {"f1-score": 0.9805825242718447, "precision": 0.9950738916256158, "recall": 0.9665071770334929, "support": 209}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 190}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 150}, "macro avg": {"f1-score": 0.36599877334489694, "precision": 0.4939224541649364, "recall": 0.3378999698581406, "support": 7965}, "micro avg": {"f1-score": 0.6510293621329734, "precision": 0.992539233341909, "recall": 0.48436911487758944, "support": 7965}, "weighted avg": {"f1-score": 0.555632472859149, "precision": 0.8715095883253698, "recall": 0.48436911487758944, "support": 7965}, "\u2205": {"f1-score": 0.8737999165159315, "precision": 0.9943001899936669, "recall": 0.7793497145693721, "support": 4029}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 479}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 159}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9485294117647058, "precision": 0.9772727272727273, "recall": 0.9214285714285714, "support": 280}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9502762430939226, "precision": 0.9847328244274809, "recall": 0.9181494661921709, "support": 281}, "\u2423": {"f1-score": 0.1553846153846154, "precision": 0.9950738916256158, "recall": 0.08427200667501043, "support": 2397}},
  "ppcr": 0.48801004394224734
}
```
</details>
