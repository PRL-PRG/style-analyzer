# Test report for javascript / file:///tmp/top-repos-quality-repos-thr8jp55/opensource.builders.git HEAD 28b6a2c6d42127b16e790913df0140981ecf3c64

### Classification report

PPCR: 0.671

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.888| 1.000| 0.903| 0.941| 0.896| 390| 432| 0.903 |
| `"` | 0.990| 1.000| 0.850| 0.995| 0.915| 102| 120| 0.850 |
| `␣` | 1.000| 0.375| 0.079| 0.545| 0.147| 40| 189| 0.212 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 18| 0.556 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 59| 0.153 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 12| 0.500 |
| `micro avg` | 0.910| 0.910| 0.611| 0.910| 0.731| 557| 830| 0.671 |
| `weighted avg` | 0.875| 0.910| 0.611| 0.880| 0.632| 557| 830| 0.671 |
| `macro avg` | 0.480| 0.396| 0.305| 0.414| 0.326| 557| 830| 0.671 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|42 |390 |0 |0 |0 |0 |0 |
|149 |24 |15 |1 |0 |0 |0 |
|18 |0 |0 |102 |0 |0 |0 |
|50 |9 |0 |0 |0 |0 |0 |
|8 |10 |0 |0 |0 |0 |0 |
|6 |6 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9951219512195122, "precision": 0.9902912621359223, "recall": 1.0, "support": 102}, "macro avg": {"f1-score": 0.41357818973518173, "precision": 0.47977899167717153, "recall": 0.3958333333333333, "support": 557}, "micro avg": {"f1-score": 0.9102333931777378, "precision": 0.9102333931777379, "recall": 0.9102333931777379, "support": 557}, "weighted avg": {"f1-score": 0.8801952443806369, "precision": 0.8751866373957555, "recall": 0.9102333931777379, "support": 557}, "\u2205": {"f1-score": 0.9408926417370326, "precision": 0.8883826879271071, "recall": 1.0, "support": 390}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u2423": {"f1-score": 0.5454545454545454, "precision": 1.0, "recall": 0.375, "support": 40}},
  "cl_report_full": {"\"": {"f1-score": 0.9147982062780268, "precision": 0.9902912621359223, "recall": 0.85, "support": 120}, "macro avg": {"f1-score": 0.32622990297785665, "precision": 0.47977899167717153, "recall": 0.3053571428571429, "support": 830}, "micro avg": {"f1-score": 0.7310742609949531, "precision": 0.9102333931777379, "recall": 0.6108433734939759, "support": 830}, "weighted avg": {"f1-score": 0.6318500892074858, "precision": 0.8332726176395433, "recall": 0.6108433734939759, "support": 830}, "\u2205": {"f1-score": 0.8955223880597015, "precision": 0.8883826879271071, "recall": 0.9027777777777778, "support": 432}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u2423": {"f1-score": 0.14705882352941177, "precision": 1.0, "recall": 0.07936507936507936, "support": 189}},
  "ppcr": 0.6710843373493975
}
```
</details>
