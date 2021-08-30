# Train report for javascript / file:///tmp/top-repos-quality-repos-5n7_0qpf/webrtc-book-cn.git HEAD 8de08ace70827edffa4f4c1aa9c600b53b9975f3

### Classification report

PPCR: 0.558

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 1.000| 0.915| 0.989| 0.945| 1811| 1980| 0.915 |
| `'` | 1.000| 1.000| 0.499| 1.000| 0.666| 243| 487| 0.499 |
| `⏎` | 0.975| 1.000| 0.483| 0.987| 0.646| 153| 317| 0.483 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 922| 0.041 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 96| 0.052 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 139| 0.022 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 100| 0.000 |
| `weighted avg` | 0.960| 0.980| 0.546| 0.969| 0.594| 2253| 4041| 0.558 |
| `micro avg` | 0.980| 0.980| 0.546| 0.980| 0.701| 2253| 4041| 0.558 |
| `macro avg` | 0.422| 0.429| 0.271| 0.425| 0.322| 2253| 4041| 0.558 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|169 |1811 |0 |0 |0 |0 |0 |0 |
|884 |38 |0 |0 |0 |0 |0 |0 |
|244 |0 |0 |243 |0 |0 |0 |0 |
|164 |0 |0 |0 |153 |0 |0 |0 |
|136 |0 |0 |0 |3 |0 |0 |0 |
|100 |0 |0 |0 |0 |0 |0 |0 |
|91 |4 |0 |0 |1 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/completeNodeClientWithDataChannel.js | 22 |
| js/dataChannel.js | 9 |
| js/NodeSocketIOServer.js | 8 |
| js/simpleNodeClient.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 243}, "macro avg": {"f1-score": 0.42509055601392554, "precision": 0.42169376369726297, "recall": 0.42857142857142855, "support": 2253}, "micro avg": {"f1-score": 0.9795827785175322, "precision": 0.9795827785175322, "recall": 0.9795827785175322, "support": 2253}, "weighted avg": {"f1-score": 0.9694924664783091, "precision": 0.9596333247255938, "recall": 0.9795827785175322, "support": 2253}, "\u2205": {"f1-score": 0.9885371179039301, "precision": 0.97733405288721, "recall": 1.0, "support": 1811}, "\u23ce": {"f1-score": 0.9870967741935484, "precision": 0.9745222929936306, "recall": 1.0, "support": 153}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6657534246575343, "precision": 1.0, "recall": 0.4989733059548255, "support": 487}, "macro avg": {"f1-score": 0.3223249685491673, "precision": 0.42169376369726297, "recall": 0.2708956589817976, "support": 4041}, "micro avg": {"f1-score": 0.7013028280902447, "precision": 0.9795827785175322, "recall": 0.5461519425884682, "support": 4041}, "weighted avg": {"f1-score": 0.593880703438938, "precision": 0.675833949912313, "recall": 0.5461519425884682, "support": 4041}, "\u2205": {"f1-score": 0.9449517349334725, "precision": 0.97733405288721, "recall": 0.9146464646464646, "support": 1980}, "\u23ce": {"f1-score": 0.6455696202531646, "precision": 0.9745222929936306, "recall": 0.48264984227129337, "support": 317}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 139}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 100}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 922}},
  "ppcr": 0.5575352635486266
}
```
</details>
