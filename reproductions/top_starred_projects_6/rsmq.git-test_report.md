# Test report for javascript / file:///tmp/top-repos-quality-repos-adzanq3z/rsmq.git HEAD 5c507c2ae97145fbdef7369da52dc69b105e0cc6

### Classification report

PPCR: 0.924

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.846| 0.984| 0.933| 0.910| 0.887| 184| 194| 0.948 |
| `␣` | 0.930| 0.580| 0.482| 0.714| 0.635| 69| 83| 0.831 |
| `"` | 0.864| 0.950| 0.950| 0.905| 0.905| 20| 20| 1.000 |
| `⏎` | 1.000| 0.353| 0.353| 0.522| 0.522| 17| 17| 1.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.848| 0.848| 0.783| 0.848| 0.815| 290| 314| 0.924 |
| `weighted avg` | 0.876| 0.848| 0.783| 0.840| 0.802| 290| 314| 0.924 |
| `macro avg` | 0.404| 0.318| 0.302| 0.339| 0.328| 290| 314| 0.924 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|10 |181 |2 |0 |0 |1 |
|14 |22 |40 |0 |3 |4 |
|0 |11 |0 |6 |0 |0 |
|0 |0 |1 |0 |19 |0 |
|0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9047619047619048, "precision": 0.8636363636363636, "recall": 0.95, "support": 20}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3389260542417632, "precision": 0.40440703492214036, "recall": 0.3184829970635597, "support": 290}, "micro avg": {"f1-score": 0.8482758620689655, "precision": 0.8482758620689655, "recall": 0.8482758620689655, "support": 290}, "weighted avg": {"f1-score": 0.8400251776480747, "precision": 0.8761549724436355, "recall": 0.8482758620689655, "support": 290}, "\u2205": {"f1-score": 0.9095477386934674, "precision": 0.8457943925233645, "recall": 0.9836956521739131, "support": 184}, "\u23ce": {"f1-score": 0.5217391304347826, "precision": 1.0, "recall": 0.35294117647058826, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.7142857142857143, "precision": 0.9302325581395349, "recall": 0.5797101449275363, "support": 69}},
  "cl_report_full": {"\"": {"f1-score": 0.9047619047619048, "precision": 0.8636363636363636, "recall": 0.95, "support": 20}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3276307302309007, "precision": 0.40440703492214036, "recall": 0.30198428644840125, "support": 314}, "micro avg": {"f1-score": 0.814569536423841, "precision": 0.8482758620689655, "recall": 0.7834394904458599, "support": 314}, "weighted avg": {"f1-score": 0.8018811050682619, "precision": 0.877599177540896, "recall": 0.7834394904458599, "support": 314}, "\u2205": {"f1-score": 0.8872549019607844, "precision": 0.8457943925233645, "recall": 0.9329896907216495, "support": 194}, "\u23ce": {"f1-score": 0.5217391304347826, "precision": 1.0, "recall": 0.35294117647058826, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.6349206349206349, "precision": 0.9302325581395349, "recall": 0.4819277108433735, "support": 83}},
  "ppcr": 0.9235668789808917
}
```
</details>
