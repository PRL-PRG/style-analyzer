# Test report for javascript / file:///tmp/top-repos-quality-repos-hid2uo_j/avalon.git HEAD f8e2d458993647a98853a84222b480012ae16e39

### Classification report

PPCR: 0.945

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.948| 0.973| 0.972| 0.960| 0.960| 1396| 1397| 0.999 |
| `␣` | 0.801| 0.985| 0.976| 0.884| 0.880| 732| 739| 0.991 |
| `⏎` | 0.940| 0.624| 0.521| 0.750| 0.670| 202| 242| 0.835 |
| `'` | 1.000| 0.486| 0.486| 0.654| 0.654| 144| 144| 1.000 |
| `⏎␣⁻␣⁻` | 0.933| 0.660| 0.660| 0.773| 0.773| 106| 106| 1.000 |
| `⏎⏎` | 0.625| 0.377| 0.351| 0.471| 0.449| 53| 57| 0.930 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 114| 0.096 |
| `micro avg` | 0.894| 0.894| 0.845| 0.894| 0.869| 2644| 2799| 0.945 |
| `macro avg` | 0.750| 0.586| 0.567| 0.642| 0.627| 2644| 2799| 0.945 |
| `weighted avg` | 0.898| 0.894| 0.845| 0.885| 0.841| 2644| 2799| 0.945 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1 |1358 |37 |1 |0 |0 |0 |0 |
|7 |9 |721 |0 |0 |0 |2 |0 |
|40 |5 |57 |126 |0 |0 |3 |11 |
|0 |28 |46 |0 |70 |0 |0 |0 |
|103 |4 |7 |0 |0 |0 |0 |0 |
|0 |28 |6 |1 |0 |0 |70 |1 |
|4 |1 |26 |6 |0 |0 |0 |20 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.6542056074766355, "precision": 1.0, "recall": 0.4861111111111111, "support": 144}, "macro avg": {"f1-score": 0.6417013563162911, "precision": 0.7496293141344751, "recall": 0.5864801976612107, "support": 2644}, "micro avg": {"f1-score": 0.8944780635400907, "precision": 0.8944780635400907, "recall": 0.8944780635400907, "support": 2644}, "weighted avg": {"f1-score": 0.8848918682609901, "precision": 0.8983920053078155, "recall": 0.8944780635400907, "support": 2644}, "\u2205": {"f1-score": 0.9600565570873101, "precision": 0.947662247034194, "recall": 0.9727793696275072, "support": 1396}, "\u23ce": {"f1-score": 0.7500000000000001, "precision": 0.9402985074626866, "recall": 0.6237623762376238, "support": 202}, "\u23ce\u23ce": {"f1-score": 0.4705882352941177, "precision": 0.625, "recall": 0.37735849056603776, "support": 53}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7734806629834254, "precision": 0.9333333333333333, "recall": 0.660377358490566, "support": 106}, "\u2423": {"f1-score": 0.883578431372549, "precision": 0.8011111111111111, "recall": 0.9849726775956285, "support": 732}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6542056074766355, "precision": 1.0, "recall": 0.4861111111111111, "support": 144}, "macro avg": {"f1-score": 0.626694187450246, "precision": 0.7496293141344751, "recall": 0.5665360878816047, "support": 2799}, "micro avg": {"f1-score": 0.8690060628329964, "precision": 0.8944780635400907, "recall": 0.8449446230796713, "support": 2799}, "weighted avg": {"f1-score": 0.8413379170712548, "precision": 0.8653146989486187, "recall": 0.8449446230796713, "support": 2799}, "\u2205": {"f1-score": 0.9597173144876325, "precision": 0.947662247034194, "recall": 0.972083035075161, "support": 1397}, "\u23ce": {"f1-score": 0.6702127659574468, "precision": 0.9402985074626866, "recall": 0.5206611570247934, "support": 242}, "\u23ce\u23ce": {"f1-score": 0.449438202247191, "precision": 0.625, "recall": 0.3508771929824561, "support": 57}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7734806629834254, "precision": 0.9333333333333333, "recall": 0.660377358490566, "support": 106}, "\u2423": {"f1-score": 0.8798047589993898, "precision": 0.8011111111111111, "recall": 0.9756427604871448, "support": 739}},
  "ppcr": 0.9446230796713112
}
```
</details>
