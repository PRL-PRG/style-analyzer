# Test report for javascript / file:///tmp/top-repos-quality-repos-dcsc9gu1/react-query-devtools.git HEAD d8cd2cb107a7cfaabb42b0fbc9fb144b343ec51c

### Classification report

PPCR: 0.142

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.975| 1.000| 0.987| 39| 40| 0.975 |
| `␣` | 0.567| 1.000| 0.127| 0.723| 0.207| 34| 268| 0.127 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 350| 0.074 |
| `⏎` | 1.000| 1.000| 0.286| 1.000| 0.444| 12| 42| 0.286 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 44| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 36| 0.000 |
| `weighted avg` | 0.633| 0.766| 0.109| 0.681| 0.146| 111| 780| 0.142 |
| `micro avg` | 0.766| 0.766| 0.109| 0.766| 0.191| 111| 780| 0.142 |
| `macro avg` | 0.428| 0.500| 0.231| 0.454| 0.273| 111| 780| 0.142 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|324 |0 |26 |0 |0 |0 |0 |
|234 |0 |34 |0 |0 |0 |0 |
|1 |0 |0 |39 |0 |0 |0 |
|30 |0 |0 |0 |12 |0 |0 |
|36 |0 |0 |0 |0 |0 |0 |
|44 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 39}, "macro avg": {"f1-score": 0.45390070921985815, "precision": 0.42777777777777776, "recall": 0.5, "support": 111}, "micro avg": {"f1-score": 0.7657657657657657, "precision": 0.7657657657657657, "recall": 0.7657657657657657, "support": 111}, "weighted avg": {"f1-score": 0.681042744872532, "precision": 0.633033033033033, "recall": 0.7657657657657657, "support": 111}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.7234042553191489, "precision": 0.5666666666666667, "recall": 1.0, "support": 34}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9873417721518987, "precision": 1.0, "recall": 0.975, "support": 40}, "macro avg": {"f1-score": 0.2731838816278458, "precision": 0.42777777777777776, "recall": 0.23126332622601278, "support": 780}, "micro avg": {"f1-score": 0.19079685746352412, "precision": 0.7657657657657657, "recall": 0.10897435897435898, "support": 780}, "weighted avg": {"f1-score": 0.1457965553365368, "precision": 0.29982905982905983, "recall": 0.10897435897435898, "support": 780}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 350}, "\u23ce": {"f1-score": 0.4444444444444445, "precision": 1.0, "recall": 0.2857142857142857, "support": 42}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.2073170731707317, "precision": 0.5666666666666667, "recall": 0.12686567164179105, "support": 268}},
  "ppcr": 0.1423076923076923
}
```
</details>
