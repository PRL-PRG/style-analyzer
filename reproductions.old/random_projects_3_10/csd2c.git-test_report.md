# Test report for javascript / file:///tmp/top-repos-quality-repos-u4xgdhbw/csd2c.git HEAD c54a22757bfff916871c5ac950d9f06356330ee3

### Classification report

PPCR: 0.279

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 1.000| 0.449| 0.996| 0.618| 2150| 4789| 0.449 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 2433| 0.007 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 543| 0.000 |
| `weighted avg` | 0.984| 0.992| 0.277| 0.988| 0.381| 2167| 7765| 0.279 |
| `micro avg` | 0.992| 0.992| 0.277| 0.992| 0.433| 2167| 7765| 0.279 |
| `macro avg` | 0.331| 0.333| 0.150| 0.332| 0.206| 2167| 7765| 0.279 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|2639 |2150 |0 |0 |
|2416 |17 |0 |0 |
|543 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.33202069338275036, "precision": 0.3307183510229195, "recall": 0.3333333333333333, "support": 2167}, "micro avg": {"f1-score": 0.9921550530687586, "precision": 0.9921550530687586, "recall": 0.9921550530687586, "support": 2167}, "weighted avg": {"f1-score": 0.9882480259892662, "precision": 0.9843716493298712, "recall": 0.9921550530687586, "support": 2167}, "\u2205": {"f1-score": 0.996062080148251, "precision": 0.9921550530687586, "recall": 1.0, "support": 2150}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}},
  "cl_report_full": {"macro avg": {"f1-score": 0.20605712095073794, "precision": 0.3307183510229195, "recall": 0.14964850003480198, "support": 7765}, "micro avg": {"f1-score": 0.432944019331454, "precision": 0.9921550530687586, "recall": 0.2768834513844173, "support": 7765}, "weighted avg": {"f1-score": 0.3812521129039603, "precision": 0.6119034834702234, "recall": 0.2768834513844173, "support": 7765}, "\u2205": {"f1-score": 0.6181713628522139, "precision": 0.9921550530687586, "recall": 0.4489455001044059, "support": 4789}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 543}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2433}},
  "ppcr": 0.2790727623953638
}
```
</details>
