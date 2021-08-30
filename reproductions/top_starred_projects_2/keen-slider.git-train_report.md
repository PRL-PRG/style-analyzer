# Train report for javascript / file:///tmp/top-repos-quality-repos-q8jykux0/keen-slider.git HEAD bfb2816b46d9f59874843f13c1982b4ad17bd2f3

### Classification report

PPCR: 0.206

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.987| 0.295| 0.985| 0.454| 538| 1798| 0.299 |
| `␣` | 0.961| 1.000| 0.200| 0.980| 0.331| 319| 1593| 0.200 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 180| 0.056 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 365| 0.014 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 151| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 152| 0.000 |
| `micro avg` | 0.975| 0.975| 0.201| 0.975| 0.333| 872| 4239| 0.206 |
| `macro avg` | 0.324| 0.331| 0.083| 0.328| 0.131| 872| 4239| 0.206 |
| `weighted avg` | 0.958| 0.975| 0.201| 0.966| 0.317| 872| 4239| 0.206 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1260 |531 |7 |0 |0 |0 |0 |
|1274 |0 |319 |0 |0 |0 |0 |
|360 |2 |3 |0 |0 |0 |0 |
|170 |7 |3 |0 |0 |0 |0 |
|151 |0 |0 |0 |0 |0 |0 |
|152 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/keen-slider.js | 20 |
| rollup.config.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.32753140356826993, "precision": 0.3240294511378849, "recall": 0.33116480793060715, "support": 872}, "micro avg": {"f1-score": 0.9747706422018348, "precision": 0.9747706422018348, "recall": 0.9747706422018348, "support": 872}, "weighted avg": {"f1-score": 0.9663355993208442, "precision": 0.9581907906856785, "recall": 0.9747706422018348, "support": 872}, "\u2205": {"f1-score": 0.9851576994434137, "precision": 0.9833333333333333, "recall": 0.9869888475836431, "support": 538}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9800307219662058, "precision": 0.9608433734939759, "recall": 1.0, "support": 319}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "macro avg": {"f1-score": 0.13094382663244938, "precision": 0.3240294511378849, "recall": 0.082596540156101, "support": 4239}, "micro avg": {"f1-score": 0.33261592643318333, "precision": 0.9747706422018348, "recall": 0.20051899032790751, "support": 4239}, "weighted avg": {"f1-score": 0.3172161228044366, "precision": 0.7781686311180083, "recall": 0.20051899032790751, "support": 4239}, "\u2205": {"f1-score": 0.45423438836612495, "precision": 0.9833333333333333, "recall": 0.2953281423804227, "support": 1798}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 365}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 151}, "\u2423": {"f1-score": 0.3314285714285714, "precision": 0.9608433734939759, "recall": 0.2002510985561833, "support": 1593}},
  "ppcr": 0.20570889360698277
}
```
</details>
