# Train report for javascript / file:///tmp/top-repos-quality-repos-hnk66v9m/chatbots.git HEAD d7ef87bd3e910e65204bde5fe8f682fb02632d53

### Classification report

PPCR: 0.232

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.387| 1.000| 0.558| 182| 470| 0.387 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 189| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 124| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.232| 1.000| 0.335| 182| 783| 0.232 |
| `macro avg` | 0.333| 0.333| 0.129| 0.333| 0.186| 182| 783| 0.232 |
| `micro avg` | 1.000| 1.000| 0.232| 1.000| 0.377| 182| 783| 0.232 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|288 |182 |0 |0 |
|189 |0 |0 |0 |
|124 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 182}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 182}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 182}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 182}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 124}, "macro avg": {"f1-score": 0.18609406952965235, "precision": 0.3333333333333333, "recall": 0.12907801418439716, "support": 783}, "micro avg": {"f1-score": 0.3772020725388601, "precision": 1.0, "recall": 0.23243933588761176, "support": 783}, "weighted avg": {"f1-score": 0.3351119259729372, "precision": 0.6002554278416348, "recall": 0.23243933588761176, "support": 783}, "\u2205": {"f1-score": 0.558282208588957, "precision": 1.0, "recall": 0.3872340425531915, "support": 470}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 189}},
  "ppcr": 0.23243933588761176
}
```
</details>
