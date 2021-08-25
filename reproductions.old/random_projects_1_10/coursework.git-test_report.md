# Test report for javascript / file:///tmp/top-repos-quality-repos-ri36l8us/coursework.git HEAD 35eca505bda907082a222fc60e2af98890d145da

### Classification report

PPCR: 0.880

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 0.812| 0.721| 0.896| 0.838| 181| 204| 0.887 |
| `␣` | 0.606| 1.000| 0.989| 0.754| 0.751| 86| 87| 0.989 |
| `⏎` | 1.000| 0.143| 0.125| 0.250| 0.222| 14| 16| 0.875 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 10| 0.800 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 6| 0.667 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 10| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `micro avg` | 0.802| 0.802| 0.706| 0.802| 0.751| 293| 333| 0.880 |
| `weighted avg` | 0.843| 0.802| 0.706| 0.787| 0.720| 293| 333| 0.880 |
| `macro avg` | 0.200| 0.150| 0.141| 0.146| 0.139| 293| 333| 0.880 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| '| ⏎⏎⇥⁻| ⏎⏎⇥⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⇥| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|23 |147 |34 |0 |0 |0 |0 |0 |
|1 |0 |86 |0 |0 |0 |0 |0 |
|2 |0 |12 |2 |0 |0 |0 |0 |
|2 |0 |8 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |
|10 |0 |0 |0 |0 |0 |0 |0 |
|2 |0 |2 |0 |0 |2 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.14620980217899343, "precision": 0.20043336944745396, "recall": 0.1503855260761338, "support": 293}, "micro avg": {"f1-score": 0.8020477815699659, "precision": 0.8020477815699659, "recall": 0.8020477815699659, "support": 293}, "weighted avg": {"f1-score": 0.7870819039607677, "precision": 0.843291832908715, "recall": 0.8020477815699659, "support": 293}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2205": {"f1-score": 0.8963414634146342, "precision": 1.0, "recall": 0.8121546961325967, "support": 181}, "\u23ce": {"f1-score": 0.25, "precision": 1.0, "recall": 0.14285714285714285, "support": 14}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.7543859649122806, "precision": 0.6056338028169014, "recall": 1.0, "support": 86}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1393015971450637, "precision": 0.20043336944745396, "recall": 0.1410841524938888, "support": 333}, "micro avg": {"f1-score": 0.7507987220447284, "precision": 0.8020477815699659, "recall": 0.7057057057057057, "support": 333}, "weighted avg": {"f1-score": 0.7200370228026705, "precision": 0.8188893118470583, "recall": 0.7057057057057057, "support": 333}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2205": {"f1-score": 0.8376068376068375, "precision": 1.0, "recall": 0.7205882352941176, "support": 204}, "\u23ce": {"f1-score": 0.2222222222222222, "precision": 1.0, "recall": 0.125, "support": 16}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.7510917030567685, "precision": 0.6056338028169014, "recall": 0.9885057471264368, "support": 87}},
  "ppcr": 0.8798798798798799
}
```
</details>
