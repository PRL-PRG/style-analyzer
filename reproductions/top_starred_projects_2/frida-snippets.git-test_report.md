# Test report for javascript / file:///tmp/top-repos-quality-repos-ztfk92sj/frida-snippets.git HEAD e627f950c1aa26856b64aaff530d86599793e9fc

### Classification report

PPCR: 0.417

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.942| 1.000| 0.686| 0.970| 0.794| 212| 309| 0.686 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 38| 0.237 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 18| 0.222 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 35| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 139| 0.000 |
| `micro avg` | 0.942| 0.942| 0.393| 0.942| 0.555| 225| 539| 0.417 |
| `macro avg` | 0.188| 0.200| 0.137| 0.194| 0.159| 225| 539| 0.417 |
| `weighted avg` | 0.888| 0.942| 0.393| 0.914| 0.455| 225| 539| 0.417 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|97 |212 |0 |0 |0 |0 |
|139 |0 |0 |0 |0 |0 |
|35 |0 |0 |0 |0 |0 |
|14 |4 |0 |0 |0 |0 |
|29 |9 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "macro avg": {"f1-score": 0.1940503432494279, "precision": 0.18844444444444444, "recall": 0.2, "support": 225}, "micro avg": {"f1-score": 0.9422222222222222, "precision": 0.9422222222222222, "recall": 0.9422222222222222, "support": 225}, "weighted avg": {"f1-score": 0.9141927281973048, "precision": 0.8877827160493826, "recall": 0.9422222222222222, "support": 225}, "\u2205": {"f1-score": 0.9702517162471396, "precision": 0.9422222222222222, "recall": 1.0, "support": 212}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "macro avg": {"f1-score": 0.15880149812734082, "precision": 0.18844444444444444, "recall": 0.13721682847896438, "support": 539}, "micro avg": {"f1-score": 0.5549738219895287, "precision": 0.9422222222222222, "recall": 0.39332096474953615, "support": 539}, "weighted avg": {"f1-score": 0.45519167830564294, "precision": 0.540160791589363, "recall": 0.39332096474953615, "support": 539}, "\u2205": {"f1-score": 0.7940074906367041, "precision": 0.9422222222222222, "recall": 0.686084142394822, "support": 309}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 139}},
  "ppcr": 0.4174397031539889
}
```
</details>
