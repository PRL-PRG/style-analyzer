# Train report for javascript / file:///tmp/top-repos-quality-repos-fx1fho1c/bilibili-helper-o.git HEAD 06fb124cbc35526eac2b83b4e50fd31f9580d47a

### Classification report

PPCR: 0.775

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.994| 0.951| 0.992| 0.970| 39635| 41412| 0.957 |
| `␣` | 0.984| 0.974| 0.604| 0.979| 0.749| 12751| 20555| 0.620 |
| `'` | 0.991| 1.000| 0.744| 0.995| 0.850| 4415| 5933| 0.744 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.940| 0.944| 0.833| 0.942| 0.883| 2009| 2277| 0.882 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.911| 0.952| 0.598| 0.931| 0.722| 1513| 2411| 0.628 |
| `⏎` | 0.997| 0.839| 0.073| 0.911| 0.136| 385| 4410| 0.087 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 373| 0.107 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 66| 0.545 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 947| 0.002 |
| `weighted avg` | 0.984| 0.985| 0.764| 0.985| 0.829| 60786| 78384| 0.775 |
| `micro avg` | 0.985| 0.985| 0.764| 0.985| 0.861| 60786| 78384| 0.775 |
| `macro avg` | 0.646| 0.634| 0.423| 0.639| 0.479| 60786| 78384| 0.775 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1777 |39400 |159 |0 |0 |29 |47 |0 |0 |0 |
|7804 |182 |12417 |0 |0 |106 |46 |0 |0 |0 |
|1518 |0 |0 |4415 |0 |0 |0 |0 |0 |0 |
|4025 |32 |25 |0 |323 |5 |0 |0 |0 |0 |
|898 |65 |7 |0 |0 |1441 |0 |0 |0 |0 |
|268 |108 |4 |0 |0 |0 |1897 |0 |0 |0 |
|333 |0 |0 |40 |0 |0 |0 |0 |0 |0 |
|945 |0 |1 |0 |1 |0 |0 |0 |0 |0 |
|30 |7 |0 |0 |0 |0 |29 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/js/pages/config/index.js | 82 |
| src/js/modules/danmu/UI/danmu.js | 54 |
| src/js/modules/treasure/tfModule.js | 38 |
| src/js/libs/permissionManager/index.js | 37 |
| src/js/modules/videoDownload/UI/videoDownload.js | 36 |
| src/js/modules/dynamicCheck/UI/DynamicBox.js | 32 |
| src/js/libs/FeatureManager.js | 30 |
| src/js/modules/danmu/proto-bili-gen.js | 27 |
| src/js/modules/danmu/index.js | 26 |
| src/js/modules/menu/UI/Menu.js | 25 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u0027": {"f1-score": 0.995490417136415, "precision": 0.9910213243546577, "recall": 1.0, "support": 4415}, "macro avg": {"f1-score": 0.6390227810397894, "precision": 0.6459463209659427, "recall": 0.6337223565135559, "support": 60786}, "micro avg": {"f1-score": 0.9853091172309414, "precision": 0.9853091172309414, "recall": 0.9853091172309414, "support": 60786}, "weighted avg": {"f1-score": 0.9846541626120547, "precision": 0.9841283592212478, "recall": 0.9853091172309414, "support": 60786}, "\u2205": {"f1-score": 0.9920809779803347, "precision": 0.9900990099009901, "recall": 0.9940708969345275, "support": 39635}, "\u23ce": {"f1-score": 0.9111424541607898, "precision": 0.9969135802469136, "recall": 0.8389610389610389, "support": 385}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9314802844214609, "precision": 0.9114484503478811, "recall": 0.9524124256444151, "support": 1513}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9419066534260179, "precision": 0.9395740465577018, "recall": 0.9442508710801394, "support": 2009}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.9791042422330862, "precision": 0.9844604772853405, "recall": 0.9738059760018822, "support": 12751}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 373}, "\u0027": {"f1-score": 0.8500192529842124, "precision": 0.9910213243546577, "recall": 0.7441429293780549, "support": 5933}, "macro avg": {"f1-score": 0.47896391939848976, "precision": 0.6459463209659427, "recall": 0.42263091821694343, "support": 78384}, "micro avg": {"f1-score": 0.8607171085722498, "precision": 0.9853091172309414, "recall": 0.7640972647479077, "support": 78384}, "weighted avg": {"f1-score": 0.8288902869612391, "precision": 0.9676795778820382, "recall": 0.7640972647479077, "support": 78384}, "\u2205": {"f1-score": 0.9703716474152156, "precision": 0.9900990099009901, "recall": 0.951415048778132, "support": 41412}, "\u23ce": {"f1-score": 0.13645965356991974, "precision": 0.9969135802469136, "recall": 0.07324263038548753, "support": 4410}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 947}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.7219438877755512, "precision": 0.9114484503478811, "recall": 0.59767731231854, "support": 2411}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8831471135940411, "precision": 0.9395740465577018, "recall": 0.8331137461572244, "support": 2277}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u2423": {"f1-score": 0.7487337192474675, "precision": 0.9844604772853405, "recall": 0.6040865969350523, "support": 20555}},
  "ppcr": 0.7754898958971219
}
```
</details>
