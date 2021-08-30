# Test report for javascript / file:///tmp/top-repos-quality-repos-57tahmqm/react-keeper.git HEAD e2d9a62e06d79744b48f11875be862921aec0819

### Classification report

PPCR: 0.619

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.944| 0.981| 0.857| 0.962| 0.899| 858| 982| 0.874 |
| `␣` | 0.907| 0.805| 0.290| 0.853| 0.439| 205| 569| 0.360 |
| `'` | 0.971| 1.000| 0.971| 0.985| 0.971| 99| 102| 0.971 |
| `⏎␣⁺␣⁺` | 0.986| 0.961| 0.802| 0.973| 0.885| 76| 91| 0.835 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 136| 0.074 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 91| 0.022 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 50| 0.000 |
| `weighted avg` | 0.933| 0.943| 0.583| 0.938| 0.649| 1250| 2021| 0.619 |
| `macro avg` | 0.544| 0.535| 0.417| 0.539| 0.456| 1250| 2021| 0.619 |
| `micro avg` | 0.943| 0.943| 0.583| 0.943| 0.721| 1250| 2021| 0.619 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|124 |842 |16 |0 |0 |0 |0 |0 |
|364 |36 |165 |3 |0 |0 |1 |0 |
|3 |0 |0 |99 |0 |0 |0 |0 |
|126 |9 |1 |0 |0 |0 |0 |0 |
|89 |2 |0 |0 |0 |0 |0 |0 |
|15 |3 |0 |0 |0 |0 |73 |0 |
|50 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9850746268656716, "precision": 0.9705882352941176, "recall": 1.0, "support": 99}, "macro avg": {"f1-score": 0.5390581218256132, "precision": 0.5439449023878311, "recall": 0.5352509065602776, "support": 1250}, "micro avg": {"f1-score": 0.9432, "precision": 0.9432, "recall": 0.9432, "support": 1250}, "weighted avg": {"f1-score": 0.9375544526404523, "precision": 0.9334549489721212, "recall": 0.9432, "support": 1250}, "\u2205": {"f1-score": 0.9622857142857143, "precision": 0.9439461883408071, "recall": 0.9813519813519813, "support": 858}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9733333333333333, "precision": 0.9864864864864865, "recall": 0.9605263157894737, "support": 76}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.8527131782945737, "precision": 0.9065934065934066, "recall": 0.8048780487804879, "support": 205}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9705882352941176, "precision": 0.9705882352941176, "recall": 0.9705882352941176, "support": 102}, "macro avg": {"f1-score": 0.4562090611485316, "precision": 0.5439449023878311, "recall": 0.4171717530504927, "support": 2021}, "micro avg": {"f1-score": 0.7208804646896975, "precision": 0.9432, "recall": 0.5833745670460169, "support": 2021}, "weighted avg": {"f1-score": 0.6491763532814986, "precision": 0.8073117642615493, "recall": 0.5833745670460169, "support": 2021}, "\u2205": {"f1-score": 0.8986125933831376, "precision": 0.9439461883408071, "recall": 0.8574338085539714, "support": 982}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8848484848484848, "precision": 0.9864864864864865, "recall": 0.8021978021978022, "support": 91}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 91}, "\u2423": {"f1-score": 0.4394141145139813, "precision": 0.9065934065934066, "recall": 0.28998242530755713, "support": 569}},
  "ppcr": 0.6185056902523504
}
```
</details>
