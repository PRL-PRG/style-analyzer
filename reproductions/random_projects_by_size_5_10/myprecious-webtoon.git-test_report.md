# Test report for javascript / file:///tmp/top-repos-quality-repos-xn_ai3kh/myprecious-webtoon.git HEAD 1d37a9c9806131561b81ef312c453480956c0754

### Classification report

PPCR: 0.891

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.935| 0.938| 0.880| 0.936| 0.907| 1644| 1752| 0.938 |
| `␣` | 0.791| 0.820| 0.688| 0.805| 0.736| 549| 654| 0.839 |
| `⏎` | 0.792| 0.938| 0.689| 0.859| 0.737| 130| 177| 0.734 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.977| 0.773| 0.773| 0.863| 0.863| 110| 110| 1.000 |
| `'` | 0.563| 0.930| 0.678| 0.702| 0.615| 86| 118| 0.729 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.987| 0.893| 0.714| 0.938| 0.829| 84| 105| 0.800 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 54| 62| 0.871 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 26| 0.808 |
| `macro avg` | 0.631| 0.661| 0.553| 0.638| 0.586| 2678| 3004| 0.891 |
| `weighted avg` | 0.863| 0.879| 0.784| 0.869| 0.817| 2678| 3004| 0.891 |
| `micro avg` | 0.879| 0.879| 0.784| 0.879| 0.829| 2678| 3004| 0.891 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|108 |1542 |102 |0 |0 |0 |0 |0 |0 |
|105 |84 |450 |7 |6 |1 |1 |0 |0 |
|32 |5 |1 |80 |0 |0 |0 |0 |0 |
|47 |6 |0 |1 |122 |1 |0 |0 |0 |
|0 |9 |16 |0 |0 |85 |0 |0 |0 |
|21 |4 |0 |0 |5 |0 |75 |0 |0 |
|8 |0 |0 |54 |0 |0 |0 |0 |0 |
|5 |0 |0 |0 |21 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u0027": {"f1-score": 0.7017543859649122, "precision": 0.5633802816901409, "recall": 0.9302325581395349, "support": 86}, "macro avg": {"f1-score": 0.6378262682069653, "precision": 0.6306060359861401, "recall": 0.6614883559640741, "support": 2678}, "micro avg": {"f1-score": 0.8790141896938014, "precision": 0.8790141896938013, "recall": 0.8790141896938013, "support": 2678}, "weighted avg": {"f1-score": 0.8688782835292143, "precision": 0.8634724505214412, "recall": 0.8790141896938013, "support": 2678}, "\u2205": {"f1-score": 0.9362477231329691, "precision": 0.9345454545454546, "recall": 0.9379562043795621, "support": 1644}, "\u23ce": {"f1-score": 0.8591549295774646, "precision": 0.7922077922077922, "recall": 0.9384615384615385, "support": 130}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8629441624365481, "precision": 0.9770114942528736, "recall": 0.7727272727272727, "support": 110}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9375, "precision": 0.9868421052631579, "recall": 0.8928571428571429, "support": 84}, "\u2423": {"f1-score": 0.8050089445438283, "precision": 0.7908611599297012, "recall": 0.819672131147541, "support": 549}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u0027": {"f1-score": 0.6153846153846153, "precision": 0.5633802816901409, "recall": 0.6779661016949152, "support": 118}, "macro avg": {"f1-score": 0.5858298866195998, "precision": 0.6306060359861401, "recall": 0.5528068757784811, "support": 3004}, "micro avg": {"f1-score": 0.8285814853924673, "precision": 0.8790141896938013, "recall": 0.7836218375499334, "support": 3004}, "weighted avg": {"f1-score": 0.817091050137509, "precision": 0.8563036527424515, "recall": 0.7836218375499334, "support": 3004}, "\u2205": {"f1-score": 0.9065255731922399, "precision": 0.9345454545454546, "recall": 0.8801369863013698, "support": 1752}, "\u23ce": {"f1-score": 0.7371601208459215, "precision": 0.7922077922077922, "recall": 0.6892655367231638, "support": 177}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8629441624365481, "precision": 0.9770114942528736, "recall": 0.7727272727272727, "support": 110}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8287292817679558, "precision": 0.9868421052631579, "recall": 0.7142857142857143, "support": 105}, "\u2423": {"f1-score": 0.7358953393295176, "precision": 0.7908611599297012, "recall": 0.6880733944954128, "support": 654}},
  "ppcr": 0.8914780292942743
}
```
</details>