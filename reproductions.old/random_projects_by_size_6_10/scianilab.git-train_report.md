# Train report for javascript / file:///tmp/top-repos-quality-repos-x0qwt7e6/scianilab.git HEAD 71ccc5cf057a1233f1b93df203435437abaeb14c

### Classification report

PPCR: 0.615

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 0.979| 0.860| 0.975| 0.912| 23077| 26278| 0.878 |
| `␣` | 0.924| 0.922| 0.456| 0.923| 0.611| 7866| 15896| 0.495 |
| `'` | 1.000| 1.000| 0.219| 1.000| 0.359| 336| 1537| 0.219 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 79| 3422| 0.023 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 320| 0.131 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 557| 0.039 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 178| 0.090 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 332| 0.042 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 1209| 0.009 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 522| 0.017 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 314| 0.029 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 341| 0.009 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 177| 0.000 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 121| 0.000 |
| `weighted avg` | 0.953| 0.959| 0.590| 0.956| 0.668| 31484| 51204| 0.615 |
| `micro avg` | 0.959| 0.959| 0.590| 0.959| 0.730| 31484| 51204| 0.615 |
| `macro avg` | 0.207| 0.207| 0.110| 0.207| 0.134| 31484| 51204| 0.615 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎⇥⁻| ⏎⇥⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⇥⁺| "| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3201 |22602 |475 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8030 |613 |7253 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3343 |22 |57 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1201 |0 |0 |0 |336 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1198 |11 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|535 |20 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|513 |0 |9 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|278 |5 |37 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|305 |8 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|338 |2 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|318 |14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|162 |0 |16 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|177 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|121 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| data-visualizations/chemical-periodic-table/TrackballControls.js | 299 |
| data-visualizations/chemical-periodic-table/CSS3DRenderer.js | 169 |
| data-visualizations/bili-video-view/client.js | 112 |
| data-visualizations/_tools/p5js/p5.dom.js | 85 |
| data-visualizations/_tools/jsgif/Demos/canvascycle/main.js | 68 |
| data-visualizations/_tools/jsgif/Demos/canvascycle/tools.js | 54 |
| science-popularizations/sci-tech-history/logo/assistant-functions.js | 37 |
| data-visualizations/_tools/jsgif/GIFEncoder.js | 37 |
| data-visualizations/bili-video-view/assistant-functions.js | 37 |
| data-visualizations/_opening/logo/GIFEncoder.js | 37 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 336}, "macro avg": {"f1-score": 0.20698000127010843, "precision": 0.20671422798790767, "recall": 0.2072490287284625, "support": 31484}, "micro avg": {"f1-score": 0.9589315207724558, "precision": 0.9589315207724558, "recall": 0.9589315207724558, "support": 31484}, "weighted avg": {"f1-score": 0.9557456925594818, "precision": 0.9525924452794098, "recall": 0.9589315207724558, "support": 31484}, "\u2205": {"f1-score": 0.974770345452193, "precision": 0.9701678327681675, "recall": 0.979416735277549, "support": 23077}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 79}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.9229496723293249, "precision": 0.9238313590625398, "recall": 0.9220696669209255, "support": 7866}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 177}, "\u0027": {"f1-score": 0.3587827015483182, "precision": 1.0, "recall": 0.21860767729342875, "support": 1537}, "macro avg": {"f1-score": 0.13439066924615975, "precision": 0.20671422798790767, "recall": 0.10964265041926312, "support": 51204}, "micro avg": {"f1-score": 0.7302389705882353, "precision": 0.9589315207724558, "recall": 0.5896219045387079, "support": 51204}, "weighted avg": {"f1-score": 0.6683599163462235, "precision": 0.814707710201157, "recall": 0.5896219045387079, "support": 51204}, "\u2205": {"f1-score": 0.9118305597579425, "precision": 0.9701678327681675, "recall": 0.8601111195676993, "support": 26278}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3422}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 522}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 557}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1209}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 178}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 320}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 341}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 314}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 332}, "\u2423": {"f1-score": 0.6108561081399755, "precision": 0.9238313590625398, "recall": 0.4562783090085556, "support": 15896}},
  "ppcr": 0.6148738379814077
}
```
</details>
