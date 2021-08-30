# Train report for javascript / file:///tmp/top-repos-quality-repos-djntt0u3/0bin.git HEAD c65d5c4d090d385eb5c40df8e0001af9267f3b4b

### Classification report

PPCR: 0.575

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.999| 0.929| 0.992| 0.957| 10750| 11556| 0.930 |
| `⏎␣⁺␣⁺` | 0.949| 0.966| 0.837| 0.958| 0.890| 563| 650| 0.866 |
| `⏎␣⁻␣⁻` | 0.966| 0.947| 0.883| 0.956| 0.922| 533| 572| 0.932 |
| `␣` | 0.994| 0.768| 0.059| 0.867| 0.112| 453| 5883| 0.077 |
| `⏎` | 0.964| 0.908| 0.254| 0.935| 0.402| 379| 1356| 0.279 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 373| 0.040 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1380| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 294| 0.000 |
| `weighted avg` | 0.982| 0.983| 0.565| 0.982| 0.606| 12693| 22064| 0.575 |
| `macro avg` | 0.607| 0.574| 0.370| 0.588| 0.410| 12693| 22064| 0.575 |
| `micro avg` | 0.983| 0.983| 0.565| 0.983| 0.718| 12693| 22064| 0.575 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|806 |10735 |2 |0 |0 |10 |3 |0 |0 |
|5430 |79 |348 |0 |0 |13 |13 |0 |0 |
|977 |30 |0 |344 |0 |3 |2 |0 |0 |
|1380 |0 |0 |0 |0 |0 |0 |0 |0 |
|87 |19 |0 |0 |0 |544 |0 |0 |0 |
|39 |27 |0 |0 |0 |1 |505 |0 |0 |
|358 |0 |0 |13 |0 |2 |0 |0 |0 |
|294 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| docs/.build/html/_static/underscore-1.3.1.js | 75 |
| docs/.build/html/_static/websupport.js | 59 |
| docs/.build/html/_static/searchtools.js | 29 |
| docs/.build/html/_static/doctools.js | 28 |
| zerobin/static/js/behavior.js | 23 |
| docs/.build/html/_static/sidebar.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5884827934531189, "precision": 0.6073262825881578, "recall": 0.573523459245025, "support": 12693}, "micro avg": {"f1-score": 0.9829039628141495, "precision": 0.9829039628141495, "recall": 0.9829039628141495, "support": 12693}, "weighted avg": {"f1-score": 0.9817587323576709, "precision": 0.9817824708009526, "recall": 0.9829039628141495, "support": 12693}, "\u2205": {"f1-score": 0.9921441774491682, "precision": 0.9857667584940312, "recall": 0.9986046511627907, "support": 10750}, "\u23ce": {"f1-score": 0.9347826086956522, "precision": 0.9635854341736695, "recall": 0.9076517150395779, "support": 379}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9577464788732395, "precision": 0.9493891797556719, "recall": 0.9662522202486679, "support": 563}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9564393939393939, "precision": 0.9655831739961759, "recall": 0.9474671669793621, "support": 533}, "\u2423": {"f1-score": 0.8667496886674969, "precision": 0.9942857142857143, "recall": 0.7682119205298014, "support": 453}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 294}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1380}, "macro avg": {"f1-score": 0.41022578475105675, "precision": 0.6073262825881578, "recall": 0.3701982092662944, "support": 22064}, "micro avg": {"f1-score": 0.717898552809506, "precision": 0.9829039628141495, "recall": 0.5654459753444525, "support": 22064}, "weighted avg": {"f1-score": 0.6055521657672853, "precision": 0.8936249958940525, "recall": 0.5654459753444525, "support": 22064}, "\u2205": {"f1-score": 0.9565178650984586, "precision": 0.9857667584940312, "recall": 0.9289546555901697, "support": 11556}, "\u23ce": {"f1-score": 0.40163455925277297, "precision": 0.9635854341736695, "recall": 0.2536873156342183, "support": 1356}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 373}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8896156991005724, "precision": 0.9493891797556719, "recall": 0.8369230769230769, "support": 650}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9223744292237444, "precision": 0.9655831739961759, "recall": 0.8828671328671329, "support": 572}, "\u2423": {"f1-score": 0.1116637253329055, "precision": 0.9942857142857143, "recall": 0.059153493115757266, "support": 5883}},
  "ppcr": 0.5752810007251632
}
```
</details>
