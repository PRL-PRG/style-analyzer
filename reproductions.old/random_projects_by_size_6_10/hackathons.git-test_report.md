# Test report for javascript / file:///tmp/top-repos-quality-repos-ndrmo_t8/hackathons.git HEAD ce0349b5aa8807664b5fadc65ca5af1f855e8c25

### Classification report

PPCR: 0.704

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.942| 1.000| 0.870| 0.970| 0.904| 420| 483| 0.870 |
| `⏎` | 1.000| 0.962| 0.863| 0.980| 0.926| 235| 262| 0.897 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 94| 0.106 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 48| 0.146 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 68| 0.000 |
| `weighted avg` | 0.938| 0.961| 0.676| 0.949| 0.711| 672| 955| 0.704 |
| `micro avg` | 0.961| 0.961| 0.676| 0.961| 0.794| 672| 955| 0.704 |
| `macro avg` | 0.388| 0.392| 0.346| 0.390| 0.366| 672| 955| 0.704 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|63 |420 |0 |0 |0 |0 |
|84 |10 |0 |0 |0 |0 |
|27 |9 |0 |226 |0 |0 |
|68 |0 |0 |0 |0 |0 |
|41 |7 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.39009082574782206, "precision": 0.3883408071748879, "recall": 0.39234042553191484, "support": 672}, "micro avg": {"f1-score": 0.9613095238095238, "precision": 0.9613095238095238, "recall": 0.9613095238095238, "support": 672}, "weighted avg": {"f1-score": 0.949110785321979, "precision": 0.9382674033739056, "recall": 0.9613095238095238, "support": 672}, "\u2205": {"f1-score": 0.9699769053117783, "precision": 0.9417040358744395, "recall": 1.0, "support": 420}, "\u23ce": {"f1-score": 0.9804772234273319, "precision": 1.0, "recall": 0.9617021276595744, "support": 235}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "macro avg": {"f1-score": 0.36608551412588897, "precision": 0.3883408071748879, "recall": 0.3464321274477265, "support": 955}, "micro avg": {"f1-score": 0.7940995697602949, "precision": 0.9613095238095238, "recall": 0.6764397905759162, "support": 955}, "weighted avg": {"f1-score": 0.7114133982225617, "precision": 0.7506209940600568, "recall": 0.6764397905759162, "support": 955}, "\u2205": {"f1-score": 0.9041980624327234, "precision": 0.9417040358744395, "recall": 0.8695652173913043, "support": 483}, "\u23ce": {"f1-score": 0.9262295081967213, "precision": 1.0, "recall": 0.8625954198473282, "support": 262}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}},
  "ppcr": 0.7036649214659686
}
```
</details>
