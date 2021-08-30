# Train report for javascript / file:///tmp/top-repos-quality-repos-juvoghh_/diyhue.git HEAD d386e072bec57decaf14c76ba53b2e4caf8fa4f1

### Classification report

PPCR: 0.076

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 1.000| 0.475| 1.000| 0.644| 153| 322| 0.475 |
| `␣` | 0.918| 1.000| 0.119| 0.957| 0.210| 123| 1037| 0.119 |
| `∅` | 1.000| 0.222| 0.001| 0.364| 0.002| 9| 2002| 0.004 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 107| 0.037 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 221| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 92| 0.000 |
| `micro avg` | 0.962| 0.962| 0.074| 0.962| 0.137| 289| 3781| 0.076 |
| `weighted avg` | 0.951| 0.962| 0.074| 0.948| 0.114| 289| 3781| 0.076 |
| `macro avg` | 0.486| 0.370| 0.099| 0.387| 0.143| 289| 3781| 0.076 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1993 |2 |7 |0 |0 |0 |0 |
|914 |0 |123 |0 |0 |0 |0 |
|221 |0 |0 |0 |0 |0 |0 |
|169 |0 |0 |0 |153 |0 |0 |
|103 |0 |4 |0 |0 |0 |0 |
|92 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| BridgeEmulator/web-ui-src/src/color.js | 5 |
| BridgeEmulator/web-ui-src/src/App.js | 4 |
| BridgeEmulator/web-ui-src/src/Room.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 153}, "macro avg": {"f1-score": 0.3868058012026883, "precision": 0.486318407960199, "recall": 0.3703703703703704, "support": 289}, "micro avg": {"f1-score": 0.9619377162629758, "precision": 0.9619377162629758, "recall": 0.9619377162629758, "support": 289}, "weighted avg": {"f1-score": 0.9481250374845623, "precision": 0.9512214016423075, "recall": 0.9619377162629758, "support": 289}, "\u2205": {"f1-score": 0.3636363636363636, "precision": 1.0, "recall": 0.2222222222222222, "support": 9}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9571984435797666, "precision": 0.917910447761194, "recall": 1.0, "support": 123}},
  "cl_report_full": {"\"": {"f1-score": 0.6442105263157895, "precision": 1.0, "recall": 0.4751552795031056, "support": 322}, "macro avg": {"f1-score": 0.14271389861444503, "precision": 0.486318407960199, "recall": 0.0991276099133212, "support": 3781}, "micro avg": {"f1-score": 0.13660933660933663, "precision": 0.9619377162629758, "recall": 0.07352552234858503, "support": 3781}, "weighted avg": {"f1-score": 0.11353649790211014, "precision": 0.8664038969395287, "recall": 0.07352552234858503, "support": 3781}, "\u2205": {"f1-score": 0.001996007984031936, "precision": 1.0, "recall": 0.000999000999000999, "support": 2002}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 221}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 107}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u2423": {"f1-score": 0.21007685738684886, "precision": 0.917910447761194, "recall": 0.11861137897782063, "support": 1037}},
  "ppcr": 0.07643480560698228
}
```
</details>
