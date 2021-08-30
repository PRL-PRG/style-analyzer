# Test report for javascript / file:///tmp/top-repos-quality-repos-2epzhbcy/front-end-roadmap.git HEAD 18d638411ab9b282cb1ccd352e1edb1fed2a141c

### Classification report

PPCR: 0.264

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.591| 1.000| 0.255| 0.743| 0.356| 13| 51| 0.255 |
| `␣` | 0.500| 0.100| 0.026| 0.167| 0.050| 10| 38| 0.263 |
| `"` | 0.800| 1.000| 0.500| 0.889| 0.615| 4| 8| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 11| 0.091 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 1| 1.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `micro avg` | 0.621| 0.621| 0.164| 0.621| 0.259| 29| 110| 0.264 |
| `macro avg` | 0.315| 0.350| 0.130| 0.300| 0.170| 29| 110| 0.264 |
| `weighted avg` | 0.548| 0.621| 0.164| 0.513| 0.227| 29| 110| 0.264 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|38 |13 |0 |0 |0 |0 |0 |
|28 |8 |1 |1 |0 |0 |0 |
|4 |0 |0 |4 |0 |0 |0 |
|10 |0 |1 |0 |0 |0 |0 |
|0 |1 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.888888888888889, "precision": 0.8, "recall": 1.0, "support": 4}, "macro avg": {"f1-score": 0.2997354497354498, "precision": 0.3151515151515151, "recall": 0.35000000000000003, "support": 29}, "micro avg": {"f1-score": 0.6206896551724138, "precision": 0.6206896551724138, "recall": 0.6206896551724138, "support": 29}, "weighted avg": {"f1-score": 0.5130815544608649, "precision": 0.5476489028213166, "recall": 0.6206896551724138, "support": 29}, "\u2205": {"f1-score": 0.7428571428571429, "precision": 0.5909090909090909, "recall": 1.0, "support": 13}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.16666666666666669, "precision": 0.5, "recall": 0.1, "support": 10}},
  "cl_report_full": {"\"": {"f1-score": 0.6153846153846154, "precision": 0.8, "recall": 0.5, "support": 8}, "macro avg": {"f1-score": 0.1702581664910432, "precision": 0.3151515151515151, "recall": 0.13020295837633297, "support": 110}, "micro avg": {"f1-score": 0.2589928057553957, "precision": 0.6206896551724138, "recall": 0.16363636363636364, "support": 110}, "weighted avg": {"f1-score": 0.22715873167927964, "precision": 0.5048760330578512, "recall": 0.16363636363636364, "support": 110}, "\u2205": {"f1-score": 0.35616438356164387, "precision": 0.5909090909090909, "recall": 0.2549019607843137, "support": 51}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.05, "precision": 0.5, "recall": 0.02631578947368421, "support": 38}},
  "ppcr": 0.2636363636363636
}
```
</details>
