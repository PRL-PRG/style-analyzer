# Train report for javascript / file:///tmp/top-repos-quality-repos-jcql76fi/thief.git HEAD d28e1c9f13c7c2ec1cea70c9a0c8914956d6f4f2

### Classification report

PPCR: 0.617

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.966| 0.996| 0.879| 0.981| 0.920| 3190| 3617| 0.882 |
| `␣` | 0.974| 0.935| 0.664| 0.954| 0.790| 1390| 1957| 0.710 |
| `⏎` | 0.916| 0.945| 0.497| 0.930| 0.644| 254| 483| 0.526 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 182| 0.121 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 179| 0.106 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 136| 0.037 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 159| 0.025 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 131| 0.015 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 967| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 114| 0.000 |
| `weighted avg` | 0.955| 0.965| 0.595| 0.960| 0.654| 4886| 7925| 0.617 |
| `macro avg` | 0.286| 0.288| 0.204| 0.287| 0.235| 4886| 7925| 0.617 |
| `micro avg` | 0.965| 0.965| 0.595| 0.965| 0.736| 4886| 7925| 0.617 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| "| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|427 |3178 |12 |0 |0 |0 |0 |0 |0 |0 |0 |
|567 |70 |1299 |0 |21 |0 |0 |0 |0 |0 |0 |
|967 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|229 |11 |3 |0 |240 |0 |0 |0 |0 |0 |0 |
|155 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|160 |9 |12 |0 |1 |0 |0 |0 |0 |0 |0 |
|160 |19 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|131 |2 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|114 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|129 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| .electron-vue/dev-runner.js | 31 |
| src/main/utils/osUtil.js | 24 |
| .electron-vue/build.js | 16 |
| src/renderer/utils/dialog.js | 16 |
| .electron-vue/webpack.renderer.config.js | 15 |
| src/main/utils/book.js | 11 |
| .electron-vue/webpack.web.config.js | 11 |
| src/main/utils/stock.js | 11 |
| .electron-vue/webpack.main.config.js | 10 |
| src/renderer/utils/key.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.28650401595335473, "precision": 0.28561880896925784, "recall": 0.28756525083786055, "support": 4886}, "micro avg": {"f1-score": 0.965411379451494, "precision": 0.965411379451494, "recall": 0.965411379451494, "support": 4886}, "weighted avg": {"f1-score": 0.9600787496978598, "precision": 0.9553184271661544, "recall": 0.965411379451494, "support": 4886}, "\u2205": {"f1-score": 0.9807128529547909, "precision": 0.9656639319355819, "recall": 0.9962382445141066, "support": 3190}, "\u23ce": {"f1-score": 0.9302325581395349, "precision": 0.916030534351145, "recall": 0.9448818897637795, "support": 254}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9540947484392215, "precision": 0.9744936234058514, "recall": 0.9345323741007194, "support": 1390}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 967}, "macro avg": {"f1-score": 0.23540536017161595, "precision": 0.28561880896925784, "recall": 0.20392941859346467, "support": 7925}, "micro avg": {"f1-score": 0.736398407618453, "precision": 0.965411379451494, "recall": 0.595205047318612, "support": 7925}, "weighted avg": {"f1-score": 0.654201314333735, "precision": 0.7372029288211803, "recall": 0.595205047318612, "support": 7925}, "\u2205": {"f1-score": 0.920092646207296, "precision": 0.9656639319355819, "recall": 0.8786286978158695, "support": 3617}, "\u23ce": {"f1-score": 0.6442953020134228, "precision": 0.916030534351145, "recall": 0.4968944099378882, "support": 483}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 159}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 136}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 179}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 131}, "\u2423": {"f1-score": 0.7896656534954406, "precision": 0.9744936234058514, "recall": 0.6637710781808891, "support": 1957}},
  "ppcr": 0.6165299684542587
}
```
</details>
