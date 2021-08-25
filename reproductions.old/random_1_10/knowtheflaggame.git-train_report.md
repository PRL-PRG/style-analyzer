# Train report for javascript / file:///tmp/top-repos-quality-repos-s_0j3cac/knowtheflaggame.git HEAD 58405af88dec8eef5b589983cf8cd33b5b004bed

### Classification report

PPCR: 0.580

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 878| 878| 1.000 |
| `␣` | 0.994| 0.987| 0.811| 0.991| 0.893| 638| 777| 0.821 |
| `⏎` | 0.965| 0.982| 0.863| 0.973| 0.911| 224| 255| 0.878 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1092| 0.000 |
| `weighted avg` | 0.993| 0.993| 0.576| 0.993| 0.601| 1740| 3002| 0.580 |
| `micro avg` | 0.993| 0.993| 0.576| 0.993| 0.729| 1740| 3002| 0.580 |
| `macro avg` | 0.740| 0.742| 0.668| 0.741| 0.701| 1740| 3002| 0.580 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|1092 |0 |0 |0 |0 |
|0 |0 |878 |0 |0 |
|139 |0 |0 |630 |8 |
|31 |0 |0 |4 |220 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/router.js | 4 |
| src/store/modules/players.js | 4 |
| src/store/index.js | 1 |
| src/main.js | 1 |
| server.js | 1 |
| src/services/playersService.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 878}, "macro avg": {"f1-score": 0.7410043412923694, "precision": 0.7396507831091925, "recall": 0.7424009180474698, "support": 1740}, "micro avg": {"f1-score": 0.993103448275862, "precision": 0.993103448275862, "recall": 0.993103448275862, "support": 1740}, "weighted avg": {"f1-score": 0.9931231203566693, "precision": 0.9931696059104212, "recall": 0.993103448275862, "support": 1740}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.9734513274336283, "precision": 0.9649122807017544, "recall": 0.9821428571428571, "support": 224}, "\u2423": {"f1-score": 0.9905660377358492, "precision": 0.9936908517350158, "recall": 0.987460815047022, "support": 638}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 878}, "macro avg": {"f1-score": 0.7009891960975067, "precision": 0.7396507831091925, "recall": 0.6683889772125067, "support": 3002}, "micro avg": {"f1-score": 0.728806410797132, "precision": 0.993103448275862, "recall": 0.5756162558294471, "support": 3002}, "weighted avg": {"f1-score": 0.6009815027183516, "precision": 0.6316290550889588, "recall": 0.5756162558294471, "support": 3002}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1092}, "\u23ce": {"f1-score": 0.9109730848861285, "precision": 0.9649122807017544, "recall": 0.8627450980392157, "support": 255}, "\u2423": {"f1-score": 0.892983699503898, "precision": 0.9936908517350158, "recall": 0.8108108108108109, "support": 777}},
  "ppcr": 0.5796135909393737
}
```
</details>
