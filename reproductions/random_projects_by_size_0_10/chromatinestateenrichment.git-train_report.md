# Train report for javascript / file:///tmp/top-repos-quality-repos-ybybjd1v/chromatinestateenrichment.git HEAD cb02263d079c79abd44a20ca3cfb64fe837c3c5f

### Classification report

PPCR: 0.213

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.941| 0.990| 0.400| 0.965| 0.562| 873| 2159| 0.404 |
| `␣` | 0.944| 0.839| 0.107| 0.888| 0.192| 180| 1409| 0.128 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 386| 0.036 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 136| 0.051 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 155| 0.026 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 377| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 340| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 94| 0.000 |
| `micro avg` | 0.942| 0.942| 0.201| 0.942| 0.331| 1078| 5056| 0.213 |
| `weighted avg` | 0.920| 0.942| 0.201| 0.930| 0.293| 1078| 5056| 0.213 |
| `macro avg` | 0.236| 0.229| 0.063| 0.232| 0.094| 1078| 5056| 0.213 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1286 |864 |9 |0 |0 |0 |0 |0 |0 |
|1229 |29 |151 |0 |0 |0 |0 |0 |0 |
|372 |14 |0 |0 |0 |0 |0 |0 |0 |
|377 |0 |0 |0 |0 |0 |0 |0 |0 |
|340 |0 |0 |0 |0 |0 |0 |0 |0 |
|151 |4 |0 |0 |0 |0 |0 |0 |0 |
|129 |7 |0 |0 |0 |0 |0 |0 |0 |
|94 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| javascript/DataTables/DataTables-1.10.18/js/dataTables.jqueryui.js | 21 |
| javascript/DataTables/DataTables-1.10.18/js/dataTables.bootstrap4.js | 14 |
| javascript/DataTables/DataTables-1.10.18/js/dataTables.semanticui.js | 14 |
| javascript/DataTables/DataTables-1.10.18/js/dataTables.bootstrap.js | 14 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.23163242684008276, "precision": 0.2356158088235294, "recall": 0.2285724513172967, "support": 1078}, "micro avg": {"f1-score": 0.9415584415584416, "precision": 0.9415584415584416, "recall": 0.9415584415584416, "support": 1078}, "weighted avg": {"f1-score": 0.9296603063335886, "precision": 0.9197792753465022, "recall": 0.9415584415584416, "support": 1078}, "\u2205": {"f1-score": 0.964824120603015, "precision": 0.9411764705882353, "recall": 0.9896907216494846, "support": 873}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.888235294117647, "precision": 0.94375, "recall": 0.8388888888888889, "support": 180}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 340}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 377}, "macro avg": {"f1-score": 0.09425815581506575, "precision": 0.2356158088235294, "recall": 0.06341918441988263, "support": 5056}, "micro avg": {"f1-score": 0.3309422888816433, "precision": 0.9415584415584416, "recall": 0.200751582278481, "support": 5056}, "weighted avg": {"f1-score": 0.29344687551634974, "precision": 0.6649018492879746, "recall": 0.200751582278481, "support": 5056}, "\u2205": {"f1-score": 0.5615859603509912, "precision": 0.9411764705882353, "recall": 0.4001852709587772, "support": 2159}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 386}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u2423": {"f1-score": 0.19247928616953475, "precision": 0.94375, "recall": 0.10716820440028389, "support": 1409}},
  "ppcr": 0.2132120253164557
}
```
</details>
