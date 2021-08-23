# Test report for javascript / file:///tmp/top-repos-quality-repos-ybybjd1v/chromatinestateenrichment.git HEAD cb02263d079c79abd44a20ca3cfb64fe837c3c5f

### Classification report

PPCR: 0.414

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.820| 1.000| 0.746| 0.901| 0.781| 173| 232| 0.746 |
| `"` | 0.658| 0.521| 0.260| 0.581| 0.373| 48| 96| 0.500 |
| `'` | 0.452| 0.514| 0.257| 0.481| 0.328| 37| 74| 0.500 |
| `␣` | 1.000| 0.400| 0.062| 0.571| 0.118| 35| 224| 0.156 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 51| 0.118 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 31| 0.161 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 22| 0.136 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `micro avg` | 0.752| 0.752| 0.311| 0.752| 0.440| 307| 742| 0.414 |
| `weighted avg` | 0.733| 0.752| 0.311| 0.722| 0.361| 307| 742| 0.414 |
| `macro avg` | 0.366| 0.304| 0.166| 0.317| 0.200| 307| 742| 0.414 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|59 |173 |0 |0 |0 |0 |0 |0 |0 |
|189 |21 |14 |0 |0 |0 |0 |0 |0 |
|45 |6 |0 |0 |0 |0 |0 |0 |0 |
|37 |5 |0 |0 |19 |13 |0 |0 |0 |
|48 |0 |0 |0 |23 |25 |0 |0 |0 |
|26 |4 |0 |1 |0 |0 |0 |0 |0 |
|19 |2 |0 |1 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.5813953488372092, "precision": 0.6578947368421053, "recall": 0.5208333333333334, "support": 48}, "\u0027": {"f1-score": 0.48101265822784806, "precision": 0.4523809523809524, "recall": 0.5135135135135135, "support": 37}, "macro avg": {"f1-score": 0.31685978064503695, "precision": 0.36627261281165, "recall": 0.3042933558558559, "support": 307}, "micro avg": {"f1-score": 0.752442996742671, "precision": 0.752442996742671, "recall": 0.752442996742671, "support": 307}, "weighted avg": {"f1-score": 0.721774115413517, "precision": 0.7334222947956055, "recall": 0.752442996742671, "support": 307}, "\u2205": {"f1-score": 0.9010416666666666, "precision": 0.8199052132701422, "recall": 1.0, "support": 173}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.5714285714285715, "precision": 1.0, "recall": 0.4, "support": 35}},
  "cl_report_full": {"\"": {"f1-score": 0.3731343283582089, "precision": 0.6578947368421053, "recall": 0.2604166666666667, "support": 96}, "\u0027": {"f1-score": 0.3275862068965517, "precision": 0.4523809523809524, "recall": 0.25675675675675674, "support": 74}, "macro avg": {"f1-score": 0.19992574609951538, "precision": 0.36627261281165, "recall": 0.16567038482447966, "support": 742}, "micro avg": {"f1-score": 0.44041944709246905, "precision": 0.752442996742671, "recall": 0.3113207547169811, "support": 742}, "weighted avg": {"f1-score": 0.3606686239133972, "precision": 0.6884799119834307, "recall": 0.3113207547169811, "support": 742}, "\u2205": {"f1-score": 0.781038374717833, "precision": 0.8199052132701422, "recall": 0.7456896551724138, "support": 232}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.11764705882352941, "precision": 1.0, "recall": 0.0625, "support": 224}},
  "ppcr": 0.4137466307277628
}
```
</details>
