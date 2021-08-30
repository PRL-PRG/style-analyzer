# Test report for javascript / file:///tmp/top-repos-quality-repos-v0sn10is/diyprojects.git HEAD ec45e529788a55fbb1d69c2790c6ff3b2db4c267

### Classification report

PPCR: 0.956

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 0.979| 0.962| 0.974| 0.966| 2491| 2533| 0.983 |
| `␣` | 0.938| 0.951| 0.947| 0.944| 0.942| 916| 920| 0.996 |
| `'` | 0.919| 0.973| 0.856| 0.945| 0.886| 445| 506| 0.879 |
| `⏎` | 0.914| 0.878| 0.805| 0.896| 0.856| 254| 277| 0.917 |
| `⏎␣⁺␣⁺` | 0.901| 0.845| 0.820| 0.872| 0.858| 129| 133| 0.970 |
| `⏎␣⁻␣⁻` | 0.937| 0.908| 0.748| 0.922| 0.832| 98| 119| 0.824 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 44| 0.659 |
| `⏎⏎` | 0.706| 0.444| 0.197| 0.545| 0.308| 27| 61| 0.443 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 5| 1.000 |
| `weighted avg` | 0.943| 0.950| 0.908| 0.946| 0.920| 4394| 4598| 0.956 |
| `micro avg` | 0.950| 0.950| 0.908| 0.950| 0.929| 4394| 4598| 0.956 |
| `macro avg` | 0.698| 0.664| 0.593| 0.678| 0.628| 4394| 4598| 0.956 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|42 |2438 |48 |0 |0 |5 |0 |0 |0 |0 |
|4 |31 |871 |0 |9 |2 |2 |1 |0 |0 |
|23 |25 |0 |223 |0 |2 |1 |3 |0 |0 |
|61 |0 |5 |0 |433 |3 |0 |0 |4 |0 |
|4 |15 |5 |0 |0 |109 |0 |0 |0 |0 |
|21 |0 |0 |8 |0 |0 |89 |1 |0 |0 |
|34 |2 |0 |13 |0 |0 |0 |12 |0 |0 |
|15 |0 |0 |0 |29 |0 |0 |0 |0 |0 |
|0 |2 |0 |0 |0 |0 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u0027": {"f1-score": 0.9454148471615721, "precision": 0.9193205944798302, "recall": 0.9730337078651685, "support": 445}, "macro avg": {"f1-score": 0.6777028244492239, "precision": 0.6982809327591815, "recall": 0.6642391311702287, "support": 4394}, "micro avg": {"f1-score": 0.9501593081474738, "precision": 0.9501593081474738, "recall": 0.9501593081474738, "support": 4394}, "weighted avg": {"f1-score": 0.9462745695445344, "precision": 0.9430544336319999, "recall": 0.9501593081474738, "support": 4394}, "\u2205": {"f1-score": 0.9744204636290967, "precision": 0.9701551929964186, "recall": 0.9787234042553191, "support": 2491}, "\u23ce": {"f1-score": 0.895582329317269, "precision": 0.9139344262295082, "recall": 0.8779527559055118, "support": 254}, "\u23ce\u23ce": {"f1-score": 0.5454545454545455, "precision": 0.7058823529411765, "recall": 0.4444444444444444, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8719999999999999, "precision": 0.9008264462809917, "recall": 0.8449612403100775, "support": 129}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.922279792746114, "precision": 0.9368421052631579, "recall": 0.9081632653061225, "support": 98}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.9441734417344174, "precision": 0.93756727664155, "recall": 0.9508733624454149, "support": 916}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u0027": {"f1-score": 0.8863868986693961, "precision": 0.9193205944798302, "recall": 0.8557312252964426, "support": 506}, "macro avg": {"f1-score": 0.6276232799024968, "precision": 0.6982809327591815, "recall": 0.5926876573128501, "support": 4598}, "micro avg": {"f1-score": 0.9286032028469751, "precision": 0.9501593081474738, "recall": 0.9080034797738147, "support": 4598}, "weighted avg": {"f1-score": 0.9203915151807045, "precision": 0.9379412802639456, "recall": 0.9080034797738147, "support": 4598}, "\u2205": {"f1-score": 0.9663099484740388, "precision": 0.9701551929964186, "recall": 0.96249506514015, "support": 2533}, "\u23ce": {"f1-score": 0.856046065259117, "precision": 0.9139344262295082, "recall": 0.8050541516245487, "support": 277}, "\u23ce\u23ce": {"f1-score": 0.30769230769230765, "precision": 0.7058823529411765, "recall": 0.19672131147540983, "support": 61}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8582677165354331, "precision": 0.9008264462809917, "recall": 0.8195488721804511, "support": 133}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8317757009345795, "precision": 0.9368421052631579, "recall": 0.7478991596638656, "support": 119}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.9421308815575987, "precision": 0.93756727664155, "recall": 0.9467391304347826, "support": 920}},
  "ppcr": 0.955632883862549
}
```
</details>