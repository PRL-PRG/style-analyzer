# Train report for javascript / file:///tmp/top-repos-quality-repos-9yrgsbn7/robot-management-application.git HEAD 62d8414fac3f513a13d7bc31981ad83035cf0848

### Classification report

PPCR: 0.692

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.932| 0.986| 0.928| 0.958| 0.930| 63831| 67768| 0.942 |
| `␣` | 0.939| 0.822| 0.473| 0.877| 0.629| 22030| 38262| 0.576 |
| `'` | 1.000| 1.000| 0.262| 1.000| 0.415| 1115| 4255| 0.262 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 306| 7197| 0.043 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 301| 2680| 0.112 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 158| 1943| 0.081 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 95| 555| 0.171 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 543| 0.066 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 316| 0.076 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 1963| 0.005 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 249| 0.016 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1359| 0.000 |
| `macro avg` | 0.239| 0.234| 0.139| 0.236| 0.165| 87909| 127090| 0.692 |
| `micro avg` | 0.934| 0.934| 0.646| 0.934| 0.764| 87909| 127090| 0.692 |
| `weighted avg` | 0.925| 0.934| 0.646| 0.928| 0.699| 87909| 127090| 0.692 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3937 |62908 |923 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|16232 |3926 |18104 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6891 |230 |76 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3140 |0 |0 |0 |1115 |0 |0 |0 |0 |0 |0 |0 |0 |
|1359 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1954 |9 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1785 |52 |106 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2379 |296 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|460 |39 |56 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|507 |36 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|292 |19 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|245 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/assets/ColladaLoader.js | 2735 |
| client/src/assets/ColladaLoader2.js | 687 |
| client/src/assets/ros3d.js | 497 |
| client/src/assets/roslib.js | 365 |
| client/src/assets/roslibjs/roslib.js | 365 |
| client/src/assets/ros2d.js | 198 |
| client/src/assets/ros2djs/ros2d.js | 198 |
| client/src/assets/STLLoader.js | 118 |
| client/src/app/home/husky/husky.component.js | 46 |
| client/src/app/home/jackal/jackal.component.js | 43 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1115}, "macro avg": {"f1-score": 0.23620576555011394, "precision": 0.23924632016760403, "recall": 0.23394403430629973, "support": 87909}, "micro avg": {"f1-score": 0.9342274397388208, "precision": 0.9342274397388208, "recall": 0.9342274397388208, "support": 87909}, "weighted avg": {"f1-score": 0.9278709643568611, "precision": 0.9245752764844686, "recall": 0.9342274397388208, "support": 87909}, "\u2205": {"f1-score": 0.9578682908260373, "precision": 0.9317081117907552, "recall": 0.9855399414077799, "support": 63831}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 306}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 301}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 158}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u2423": {"f1-score": 0.87660089577533, "precision": 0.9392477302204929, "recall": 0.8217884702678167, "support": 22030}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1359}, "\u0027": {"f1-score": 0.415270018621974, "precision": 1.0, "recall": 0.2620446533490012, "support": 4255}, "macro avg": {"f1-score": 0.16454689162658626, "precision": 0.23924632016760403, "recall": 0.1386240114241765, "support": 127090}, "micro avg": {"f1-score": 0.7639756463983555, "precision": 0.9342274397388208, "recall": 0.6462113462900307, "support": 127090}, "weighted avg": {"f1-score": 0.6992604612114002, "precision": 0.8130654809468282, "recall": 0.6462113462900307, "support": 127090}, "\u2205": {"f1-score": 0.9299932735591742, "precision": 0.9317081117907552, "recall": 0.9282847361586589, "support": 67768}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7197}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 316}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 249}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2680}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1943}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 555}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1963}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 543}, "\u2423": {"f1-score": 0.6292994073378869, "precision": 0.9392477302204929, "recall": 0.4731587475824578, "support": 38262}},
  "ppcr": 0.6917066645684161
}
```
</details>
