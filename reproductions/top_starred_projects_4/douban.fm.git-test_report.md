# Test report for javascript / file:///tmp/top-repos-quality-repos-wjoo6l8x/douban.fm.git HEAD 777928a28ab43c12733505a9374a8323d44df4b5

### Classification report

PPCR: 0.474

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.892| 0.900| 0.664| 0.896| 0.762| 110| 149| 0.738 |
| `'` | 0.879| 0.795| 0.500| 0.835| 0.637| 73| 116| 0.629 |
| `␣` | 0.633| 0.756| 0.258| 0.689| 0.367| 41| 120| 0.342 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 4| 0.250 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 21| 0.048 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 43| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 24| 0.000 |
| `macro avg` | 0.343| 0.350| 0.203| 0.346| 0.252| 226| 477| 0.474 |
| `weighted avg` | 0.833| 0.832| 0.394| 0.831| 0.485| 226| 477| 0.474 |
| `micro avg` | 0.832| 0.832| 0.394| 0.832| 0.535| 226| 477| 0.474 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|39 |99 |5 |6 |0 |0 |0 |0 |
|79 |9 |31 |1 |0 |0 |0 |0 |
|43 |3 |12 |58 |0 |0 |0 |0 |
|43 |0 |0 |0 |0 |0 |0 |0 |
|24 |0 |0 |0 |0 |0 |0 |0 |
|20 |0 |0 |1 |0 |0 |0 |0 |
|3 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8345323741007193, "precision": 0.8787878787878788, "recall": 0.7945205479452054, "support": 73}, "macro avg": {"f1-score": 0.34562126639993757, "precision": 0.3433332617006086, "recall": 0.3500883012744022, "support": 226}, "micro avg": {"f1-score": 0.831858407079646, "precision": 0.831858407079646, "recall": 0.831858407079646, "support": 226}, "weighted avg": {"f1-score": 0.8306077166057166, "precision": 0.8327362777425988, "recall": 0.831858407079646, "support": 226}, "\u2205": {"f1-score": 0.8959276018099547, "precision": 0.8918918918918919, "recall": 0.9, "support": 110}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.6888888888888889, "precision": 0.6326530612244898, "recall": 0.7560975609756098, "support": 41}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6373626373626373, "precision": 0.8787878787878788, "recall": 0.5, "support": 116}, "macro avg": {"f1-score": 0.25225214346093466, "precision": 0.3433332617006086, "recall": 0.2032518376478108, "support": 477}, "micro avg": {"f1-score": 0.534850640113798, "precision": 0.831858407079646, "recall": 0.3941299790356394, "support": 477}, "weighted avg": {"f1-score": 0.48517183509926615, "precision": 0.6514667781514143, "recall": 0.3941299790356394, "support": 477}, "\u2205": {"f1-score": 0.7615384615384615, "precision": 0.8918918918918919, "recall": 0.6644295302013423, "support": 149}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.36686390532544383, "precision": 0.6326530612244898, "recall": 0.25833333333333336, "support": 120}},
  "ppcr": 0.47379454926624737
}
```
</details>
