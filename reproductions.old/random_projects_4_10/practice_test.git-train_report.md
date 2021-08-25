# Train report for javascript / file:///tmp/top-repos-quality-repos-2onr0x6i/practice_test.git HEAD f604ccc8e1cccd7460980cdcf89f8f8c0f0c4241

### Classification report

PPCR: 0.418

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.990| 0.548| 0.986| 0.704| 2363| 4266| 0.554 |
| `␣` | 0.937| 0.956| 0.340| 0.946| 0.499| 587| 1648| 0.356 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 185| 0.092 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 612| 0.011 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 190| 0.037 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 239| 0.000 |
| `macro avg` | 0.320| 0.324| 0.148| 0.322| 0.201| 2981| 7140| 0.418 |
| `weighted avg` | 0.963| 0.973| 0.406| 0.968| 0.536| 2981| 7140| 0.418 |
| `micro avg` | 0.973| 0.973| 0.406| 0.973| 0.573| 2981| 7140| 0.418 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1903 |2339 |24 |0 |0 |0 |0 |
|1061 |26 |561 |0 |0 |0 |0 |
|605 |0 |7 |0 |0 |0 |0 |
|239 |0 |0 |0 |0 |0 |0 |
|183 |0 |7 |0 |0 |0 |0 |
|168 |17 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| wechat/xcx-canvas/pages/canvas/canvas.js | 15 |
| wechat/xcx-canvas/utils/util.js | 9 |
| wechat/项目学习/utils/util.js | 8 |
| wechat/基础篇/utils/util.js | 8 |
| html_study/第五章/5-21.js | 5 |
| wechat/基础篇/pages/demo3/demo3.js | 5 |
| html_study/第五章/5-10.js | 4 |
| html_study/第五章/5-18.js | 3 |
| wechat/xcx-canvas/app.js | 3 |
| wechat/项目学习/app.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.32198616217420073, "precision": 0.3197514796327679, "recall": 0.3242584006749906, "support": 2981}, "micro avg": {"f1-score": 0.9728279100972828, "precision": 0.9728279100972828, "recall": 0.9728279100972828, "support": 2981}, "weighted avg": {"f1-score": 0.9677819250601477, "precision": 0.9627991472211019, "recall": 0.9728279100972828, "support": 2981}, "\u2205": {"f1-score": 0.9858798735511065, "precision": 0.9819479429051218, "recall": 0.9898434193821414, "support": 2363}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.9460370994940979, "precision": 0.9365609348914858, "recall": 0.9557069846678024, "support": 587}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 239}, "macro avg": {"f1-score": 0.20050045333877445, "precision": 0.3197514796327679, "recall": 0.14811690274724357, "support": 7140}, "micro avg": {"f1-score": 0.5730659025787965, "precision": 0.9728279100972828, "recall": 0.4061624649859944, "support": 7140}, "weighted avg": {"f1-score": 0.5356802895253043, "precision": 0.8028630735482377, "recall": 0.4061624649859944, "support": 7140}, "\u2205": {"f1-score": 0.7036702767749698, "precision": 0.9819479429051218, "recall": 0.5482887951242381, "support": 4266}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 612}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 190}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 185}, "\u2423": {"f1-score": 0.49933244325767684, "precision": 0.9365609348914858, "recall": 0.3404126213592233, "support": 1648}},
  "ppcr": 0.41750700280112046
}
```
</details>
