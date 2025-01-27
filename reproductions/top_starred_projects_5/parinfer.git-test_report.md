# Test report for javascript / file:///tmp/top-repos-quality-repos-vvda93vz/parinfer.git HEAD 41c74d03534a5adbdcb7430fb666899e8dbf746d

### Classification report

PPCR: 0.879

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.957| 0.929| 0.960| 0.946| 2624| 2702| 0.971 |
| `␣` | 0.910| 0.941| 0.869| 0.925| 0.889| 1560| 1690| 0.923 |
| `⏎` | 0.872| 0.796| 0.391| 0.832| 0.540| 162| 330| 0.491 |
| `⏎␣⁻␣⁻` | 0.965| 0.830| 0.822| 0.892| 0.888| 100| 101| 0.990 |
| `⏎␣⁺␣⁺` | 0.900| 0.957| 0.918| 0.928| 0.909| 94| 98| 0.959 |
| `⏎⏎` | 0.774| 0.732| 0.360| 0.752| 0.491| 56| 114| 0.491 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 120| 0.125 |
| `'` | 0.267| 0.444| 0.039| 0.333| 0.068| 9| 102| 0.088 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 2| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.934| 0.936| 0.822| 0.935| 0.852| 4622| 5259| 0.879 |
| `macro avg` | 0.565| 0.566| 0.433| 0.562| 0.473| 4622| 5259| 0.879 |
| `micro avg` | 0.936| 0.936| 0.822| 0.936| 0.875| 4622| 5259| 0.879 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|78 |2510 |110 |0 |0 |0 |1 |3 |0 |0 |
|130 |76 |1468 |5 |0 |3 |6 |2 |0 |0 |
|168 |2 |24 |129 |0 |0 |0 |7 |0 |0 |
|105 |2 |2 |0 |0 |0 |0 |0 |11 |0 |
|1 |9 |5 |1 |0 |83 |2 |0 |0 |0 |
|4 |4 |0 |0 |0 |0 |90 |0 |0 |0 |
|58 |0 |1 |13 |0 |0 |1 |41 |0 |0 |
|93 |0 |2 |0 |3 |0 |0 |0 |4 |0 |
|0 |0 |2 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u0027": {"f1-score": 0.33333333333333337, "precision": 0.26666666666666666, "recall": 0.4444444444444444, "support": 9}, "macro avg": {"f1-score": 0.5623606832455386, "precision": 0.5650802978643675, "recall": 0.5657910925468658, "support": 4622}, "micro avg": {"f1-score": 0.9357421029857205, "precision": 0.9357421029857205, "recall": 0.9357421029857205, "support": 4622}, "weighted avg": {"f1-score": 0.9345579416845619, "precision": 0.9340479349982456, "recall": 0.9357421029857205, "support": 4622}, "\u2205": {"f1-score": 0.9603979338052421, "precision": 0.9642719938532462, "recall": 0.9565548780487805, "support": 2624}, "\u23ce": {"f1-score": 0.8322580645161292, "precision": 0.8716216216216216, "recall": 0.7962962962962963, "support": 162}, "\u23ce\u23ce": {"f1-score": 0.7522935779816513, "precision": 0.7735849056603774, "recall": 0.7321428571428571, "support": 56}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9278350515463918, "precision": 0.9, "recall": 0.9574468085106383, "support": 94}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8924731182795699, "precision": 0.9651162790697675, "recall": 0.83, "support": 100}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9250157529930686, "precision": 0.909541511771995, "recall": 0.941025641025641, "support": 1560}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 120}, "\u0027": {"f1-score": 0.06837606837606837, "precision": 0.26666666666666666, "recall": 0.0392156862745098, "support": 102}, "macro avg": {"f1-score": 0.47308313820728654, "precision": 0.5650802978643675, "recall": 0.43275040031981005, "support": 5259}, "micro avg": {"f1-score": 0.8754174678676249, "precision": 0.9357421029857205, "recall": 0.8223996957596501, "support": 5259}, "weighted avg": {"f1-score": 0.8515733971433089, "precision": 0.8996555696620285, "recall": 0.8223996957596501, "support": 5259}, "\u2205": {"f1-score": 0.9462770970782282, "precision": 0.9642719938532462, "recall": 0.9289415247964471, "support": 2702}, "\u23ce": {"f1-score": 0.5397489539748954, "precision": 0.8716216216216216, "recall": 0.39090909090909093, "support": 330}, "\u23ce\u23ce": {"f1-score": 0.49101796407185627, "precision": 0.7735849056603774, "recall": 0.35964912280701755, "support": 114}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9090909090909091, "precision": 0.9, "recall": 0.9183673469387755, "support": 98}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8877005347593583, "precision": 0.9651162790697675, "recall": 0.8217821782178217, "support": 101}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8886198547215496, "precision": 0.909541511771995, "recall": 0.8686390532544379, "support": 1690}},
  "ppcr": 0.8788743107054573
}
```
</details>
