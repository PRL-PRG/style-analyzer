# Test report for javascript / file:///tmp/top-repos-quality-repos-g7bm6_qh/structurefinder.git HEAD eed4e45d323f7deb639935e28d425574af000962

### Classification report

PPCR: 0.401

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.888| 1.000| 0.756| 0.941| 0.817| 294| 389| 0.756 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 194| 0.139 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 106| 0.047 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 22| 0.182 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 23| 0.043 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 69| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 22| 0.000 |
| `macro avg` | 0.127| 0.143| 0.108| 0.134| 0.117| 331| 825| 0.401 |
| `weighted avg` | 0.789| 0.888| 0.356| 0.836| 0.385| 331| 825| 0.401 |
| `micro avg` | 0.888| 0.888| 0.356| 0.888| 0.509| 331| 825| 0.401 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|95 |294 |0 |0 |0 |0 |0 |0 |
|167 |27 |0 |0 |0 |0 |0 |0 |
|101 |5 |0 |0 |0 |0 |0 |0 |
|69 |0 |0 |0 |0 |0 |0 |0 |
|18 |4 |0 |0 |0 |0 |0 |0 |
|22 |0 |0 |0 |0 |0 |0 |0 |
|22 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "macro avg": {"f1-score": 0.13440000000000002, "precision": 0.1268882175226586, "recall": 0.14285714285714285, "support": 331}, "micro avg": {"f1-score": 0.8882175226586103, "precision": 0.8882175226586103, "recall": 0.8882175226586103, "support": 331}, "weighted avg": {"f1-score": 0.8356350453172207, "precision": 0.788930367557799, "recall": 0.8882175226586103, "support": 331}, "\u2205": {"f1-score": 0.9408000000000001, "precision": 0.8882175226586103, "recall": 1.0, "support": 294}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "macro avg": {"f1-score": 0.11666666666666667, "precision": 0.1268882175226586, "recall": 0.10796915167095116, "support": 825}, "micro avg": {"f1-score": 0.5086505190311419, "precision": 0.8882175226586103, "recall": 0.3563636363636364, "support": 825}, "weighted avg": {"f1-score": 0.38507070707070706, "precision": 0.4188080197747872, "recall": 0.3563636363636364, "support": 825}, "\u2205": {"f1-score": 0.8166666666666667, "precision": 0.8882175226586103, "recall": 0.7557840616966581, "support": 389}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 69}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 194}},
  "ppcr": 0.4012121212121212
}
```
</details>
