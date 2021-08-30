# Train report for javascript / file:///tmp/top-repos-quality-repos-lhgniwdb/qrbtf.git HEAD 04730a769a5a134e76a114f9f7d61fcf6dc48079

### Classification report

PPCR: 0.544

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 0.993| 0.755| 0.980| 0.848| 11143| 14667| 0.760 |
| `␣` | 0.962| 0.945| 0.309| 0.953| 0.468| 2754| 8424| 0.327 |
| `"` | 0.981| 1.000| 0.618| 0.991| 0.758| 631| 1021| 0.618 |
| `⏎` | 1.000| 0.866| 0.111| 0.928| 0.200| 157| 1226| 0.128 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 105| 430| 0.244 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 79| 410| 0.193 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 160| 0.250 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 489| 0.025 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 468| 0.002 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 115| 0.000 |
| `micro avg` | 0.967| 0.967| 0.527| 0.967| 0.682| 14922| 27410| 0.544 |
| `macro avg` | 0.391| 0.380| 0.179| 0.385| 0.227| 14922| 27410| 0.544 |
| `weighted avg` | 0.952| 0.967| 0.527| 0.960| 0.635| 14922| 27410| 0.544 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁻| ⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3524 |11067 |76 |0 |0 |0 |0 |0 |0 |0 |0 |
|5670 |152 |2602 |0 |0 |0 |0 |0 |0 |0 |0 |
|1069 |17 |4 |136 |0 |0 |0 |0 |0 |0 |0 |
|390 |0 |0 |0 |631 |0 |0 |0 |0 |0 |0 |
|467 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|477 |0 |0 |0 |12 |0 |0 |0 |0 |0 |0 |
|325 |100 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|331 |61 |18 |0 |0 |0 |0 |0 |0 |0 |0 |
|120 |40 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|115 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/utils/qrcodeEncoder.js | 69 |
| src/api/TcbHandler.js | 38 |
| src/utils/gaHelper.js | 32 |
| src/components/renderer/RendererResImage.js | 31 |
| src/components/param/ParamColor.js | 24 |
| src/serviceWorker.js | 23 |
| src/components/style/Renderer.js | 22 |
| src/components/renderer/RendererImage.js | 22 |
| src/components/app/PartMore.js | 20 |
| src/components/app/App.js | 19 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9905808477237049, "precision": 0.9813374805598756, "recall": 1.0, "support": 631}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "macro avg": {"f1-score": 0.3852401237572264, "precision": 0.3910824106034709, "recall": 0.38042291654880883, "support": 14922}, "micro avg": {"f1-score": 0.9674306393244874, "precision": 0.9674306393244874, "recall": 0.9674306393244874, "support": 14922}, "weighted avg": {"f1-score": 0.959561676503634, "precision": 0.9520792581167665, "recall": 0.9674306393244874, "support": 14922}, "\u2205": {"f1-score": 0.9802045967849077, "precision": 0.9675642594859241, "recall": 0.9931795746208382, "support": 11143}, "\u23ce": {"f1-score": 0.9283276450511946, "precision": 1.0, "recall": 0.8662420382165605, "support": 157}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 105}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 79}, "\u2423": {"f1-score": 0.9532881480124564, "precision": 0.9619223659889095, "recall": 0.9448075526506899, "support": 2754}},
  "cl_report_full": {"\"": {"f1-score": 0.7584134615384615, "precision": 0.9813374805598756, "recall": 0.6180215475024485, "support": 1021}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 489}, "macro avg": {"f1-score": 0.22736104754798983, "precision": 0.3910824106034709, "recall": 0.17923818258273203, "support": 27410}, "micro avg": {"f1-score": 0.68203722951904, "precision": 0.9674306393244874, "recall": 0.5266690988690259, "support": 27410}, "weighted avg": {"f1-score": 0.6345936054359256, "precision": 0.8946532131565945, "recall": 0.5266690988690259, "support": 27410}, "\u2205": {"f1-score": 0.8478835472131776, "precision": 0.9675642594859241, "recall": 0.7545510329310697, "support": 14667}, "\u23ce": {"f1-score": 0.19970631424375918, "precision": 1.0, "recall": 0.11092985318107668, "support": 1226}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 160}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 468}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 430}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 410}, "\u2423": {"f1-score": 0.46760715248449997, "precision": 0.9619223659889095, "recall": 0.30887939221272553, "support": 8424}},
  "ppcr": 0.5443998540678584
}
```
</details>
