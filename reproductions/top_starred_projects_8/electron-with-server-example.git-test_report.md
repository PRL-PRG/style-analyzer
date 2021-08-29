# Test report for javascript / file:///tmp/top-repos-quality-repos-z9pqb3_a/electron-with-server-example.git HEAD 7afdab9354186d796d28ef6e38a7dbb5dc5aab8d

### Classification report

PPCR: 0.384

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.407| 1.000| 0.579| 44| 108| 0.407 |
| `␣` | 0.974| 1.000| 0.432| 0.987| 0.598| 38| 88| 0.432 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 20| 0.050 |
| `weighted avg` | 0.976| 0.988| 0.380| 0.982| 0.533| 83| 216| 0.384 |
| `macro avg` | 0.658| 0.667| 0.280| 0.662| 0.392| 83| 216| 0.384 |
| `micro avg` | 0.988| 0.988| 0.380| 0.988| 0.548| 83| 216| 0.384 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|64 |44 |0 |0 |
|50 |0 |38 |0 |
|19 |0 |1 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.6623376623376623, "precision": 0.6581196581196581, "recall": 0.6666666666666666, "support": 83}, "micro avg": {"f1-score": 0.9879518072289156, "precision": 0.9879518072289156, "recall": 0.9879518072289156, "support": 83}, "weighted avg": {"f1-score": 0.9820059458613675, "precision": 0.9762125424776027, "recall": 0.9879518072289156, "support": 83}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 44}, "\u2423": {"f1-score": 0.9870129870129869, "precision": 0.9743589743589743, "recall": 1.0, "support": 38}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "macro avg": {"f1-score": 0.3924575217571487, "precision": 0.6581196581196581, "recall": 0.2797418630751964, "support": 216}, "micro avg": {"f1-score": 0.5484949832775919, "precision": 0.9879518072289156, "recall": 0.37962962962962965, "support": 216}, "weighted avg": {"f1-score": 0.5332765421866127, "precision": 0.8969610636277302, "recall": 0.37962962962962965, "support": 216}, "\u2205": {"f1-score": 0.5789473684210525, "precision": 1.0, "recall": 0.4074074074074074, "support": 108}, "\u2423": {"f1-score": 0.5984251968503937, "precision": 0.9743589743589743, "recall": 0.4318181818181818, "support": 88}},
  "ppcr": 0.38425925925925924
}
```
</details>
