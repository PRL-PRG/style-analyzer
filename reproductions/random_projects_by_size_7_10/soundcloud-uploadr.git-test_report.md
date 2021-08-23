# Test report for javascript / file:///tmp/top-repos-quality-repos-j5lewy8n/soundcloud-uploadr.git HEAD 528833f9ee4004b19547de73e704d6315d2ab43e

### Classification report

PPCR: 0.571

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.928| 0.982| 0.758| 0.954| 0.834| 223| 289| 0.772 |
| `␣` | 0.780| 0.796| 0.265| 0.788| 0.396| 49| 147| 0.333 |
| `⏎` | 0.950| 0.633| 0.432| 0.760| 0.594| 30| 44| 0.682 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 12| 0.250 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 44| 0.023 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.894| 0.905| 0.517| 0.896| 0.607| 306| 536| 0.571 |
| `micro avg` | 0.905| 0.905| 0.517| 0.905| 0.658| 306| 536| 0.571 |
| `macro avg` | 0.332| 0.301| 0.182| 0.313| 0.228| 306| 536| 0.571 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|66 |219 |4 |0 |0 |0 |
|98 |10 |39 |0 |0 |0 |
|14 |4 |7 |19 |0 |0 |
|43 |1 |0 |0 |0 |0 |
|9 |2 |0 |1 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.31276589423648243, "precision": 0.3322457627118644, "recall": 0.30141431011866626, "support": 306}, "micro avg": {"f1-score": 0.9052287581699346, "precision": 0.9052287581699346, "recall": 0.9052287581699346, "support": 306}, "weighted avg": {"f1-score": 0.896089693552208, "precision": 0.8943020937188434, "recall": 0.9052287581699346, "support": 306}, "\u2205": {"f1-score": 0.954248366013072, "precision": 0.9279661016949152, "recall": 0.9820627802690582, "support": 223}, "\u23ce": {"f1-score": 0.7599999999999999, "precision": 0.95, "recall": 0.6333333333333333, "support": 30}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.7878787878787878, "precision": 0.78, "recall": 0.7959183673469388, "support": 49}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "macro avg": {"f1-score": 0.2279968500725163, "precision": 0.3322457627118644, "recall": 0.18186372142439863, "support": 536}, "micro avg": {"f1-score": 0.6579572446555819, "precision": 0.9052287581699346, "recall": 0.5167910447761194, "support": 536}, "weighted avg": {"f1-score": 0.6071578677497212, "precision": 0.7922429167720718, "recall": 0.5167910447761194, "support": 536}, "\u2205": {"f1-score": 0.8342857142857143, "precision": 0.9279661016949152, "recall": 0.7577854671280276, "support": 289}, "\u23ce": {"f1-score": 0.59375, "precision": 0.95, "recall": 0.4318181818181818, "support": 44}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.3959390862944162, "precision": 0.78, "recall": 0.2653061224489796, "support": 147}},
  "ppcr": 0.5708955223880597
}
```
</details>
