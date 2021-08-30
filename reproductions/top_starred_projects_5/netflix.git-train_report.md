# Train report for javascript / file:///tmp/top-repos-quality-repos-axjjbbuo/netflix.git HEAD 54ead17f6b8a240b4ab0d2eeff9d1c66c0a6c206

### Classification report

PPCR: 0.652

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.967| 0.998| 0.807| 0.982| 0.880| 8551| 10574| 0.809 |
| `'` | 0.996| 1.000| 0.991| 0.998| 0.993| 1569| 1584| 0.991 |
| `␣` | 0.920| 0.826| 0.213| 0.871| 0.345| 978| 3802| 0.257 |
| `⏎␣⁻␣⁻` | 1.000| 0.710| 0.415| 0.831| 0.587| 214| 366| 0.585 |
| `⏎⏎` | 0.979| 0.914| 0.463| 0.945| 0.629| 151| 298| 0.507 |
| `"` | 1.000| 0.953| 0.519| 0.976| 0.684| 127| 233| 0.545 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 77| 375| 0.205 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 699| 0.046 |
| `weighted avg` | 0.959| 0.968| 0.632| 0.963| 0.711| 11699| 17931| 0.652 |
| `macro avg` | 0.733| 0.675| 0.426| 0.700| 0.515| 11699| 17931| 0.652 |
| `micro avg` | 0.968| 0.968| 0.632| 0.968| 0.764| 11699| 17931| 0.652 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2023 |8537 |14 |0 |0 |0 |0 |0 |0 |
|2824 |170 |808 |0 |0 |0 |0 |0 |0 |
|15 |0 |0 |1569 |0 |0 |0 |0 |0 |
|667 |29 |0 |0 |0 |3 |0 |0 |0 |
|147 |13 |0 |0 |0 |138 |0 |0 |0 |
|298 |21 |56 |0 |0 |0 |0 |0 |0 |
|152 |62 |0 |0 |0 |0 |0 |152 |0 |
|106 |0 |0 |6 |0 |0 |0 |0 |121 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/seed.js | 57 |
| src/pages/signup.js | 25 |
| src/__tests__/pages/signup.test.js | 20 |
| src/helpers/routes.js | 20 |
| src/containers/browse.js | 19 |
| src/components/accordion/index.js | 16 |
| src/pages/signin.js | 15 |
| src/components/card/index.js | 14 |
| src/__tests__/pages/signin.test.js | 13 |
| src/components/player/index.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9758064516129031, "precision": 1.0, "recall": 0.952755905511811, "support": 127}, "\u0027": {"f1-score": 0.998091603053435, "precision": 0.9961904761904762, "recall": 1.0, "support": 1569}, "macro avg": {"f1-score": 0.7003272867690868, "precision": 0.7327232451061519, "recall": 0.6751852747276738, "support": 11699}, "micro avg": {"f1-score": 0.9680314556799726, "precision": 0.9680314556799726, "recall": 0.9680314556799726, "support": 11699}, "weighted avg": {"f1-score": 0.9625557581759261, "precision": 0.9588189744745643, "recall": 0.9680314556799726, "support": 11699}, "\u2205": {"f1-score": 0.9822240119657136, "precision": 0.966598731884058, "recall": 0.9983627645889369, "support": 8551}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u23ce": {"f1-score": 0.9452054794520547, "precision": 0.9787234042553191, "recall": 0.9139072847682119, "support": 151}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8306010928961748, "precision": 1.0, "recall": 0.7102803738317757, "support": 214}, "\u2423": {"f1-score": 0.8706896551724138, "precision": 0.9202733485193622, "recall": 0.8261758691206544, "support": 978}},
  "cl_report_full": {"\"": {"f1-score": 0.6836158192090395, "precision": 1.0, "recall": 0.51931330472103, "support": 233}, "\u0027": {"f1-score": 0.9933523266856601, "precision": 0.9961904761904762, "recall": 0.990530303030303, "support": 1584}, "macro avg": {"f1-score": 0.5147090565885882, "precision": 0.7327232451061519, "recall": 0.4260135998421664, "support": 17931}, "micro avg": {"f1-score": 0.7644279446506917, "precision": 0.9680314556799726, "recall": 0.6315877530533712, "support": 17931}, "weighted avg": {"f1-score": 0.7111182392178074, "precision": 0.9028118649694074, "recall": 0.6315877530533712, "support": 17931}, "\u2205": {"f1-score": 0.8798309801092445, "precision": 0.966598731884058, "recall": 0.8073576697560053, "support": 10574}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 699}, "\u23ce\u23ce": {"f1-score": 0.6287015945330297, "precision": 0.9787234042553191, "recall": 0.46308724832214765, "support": 298}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 375}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5868725868725869, "precision": 1.0, "recall": 0.41530054644808745, "support": 366}, "\u2423": {"f1-score": 0.3452991452991453, "precision": 0.9202733485193622, "recall": 0.21251972645975803, "support": 3802}},
  "ppcr": 0.6524454854720875
}
```
</details>
