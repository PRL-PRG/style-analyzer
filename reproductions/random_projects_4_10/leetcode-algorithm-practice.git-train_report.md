# Train report for javascript / file:///tmp/top-repos-quality-repos-p5_5lijk/leetcode-algorithm-practice.git HEAD 1a54196c7156f22f2e4b593126548f1a9f14b553

### Classification report

PPCR: 0.717

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 0.998| 0.884| 0.984| 0.926| 5817| 6564| 0.886 |
| `␣` | 0.974| 0.838| 0.274| 0.901| 0.428| 890| 2722| 0.327 |
| `⏎` | 0.975| 0.942| 0.489| 0.958| 0.651| 535| 1031| 0.519 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.981| 1.000| 1.000| 0.991| 0.991| 420| 420| 1.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.993| 0.995| 0.965| 0.994| 0.978| 410| 423| 0.969 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 135| 0.200 |
| `macro avg` | 0.816| 0.796| 0.602| 0.805| 0.662| 8099| 11295| 0.717 |
| `weighted avg` | 0.970| 0.973| 0.698| 0.971| 0.774| 8099| 11295| 0.717 |
| `micro avg` | 0.973| 0.973| 0.698| 0.973| 0.813| 8099| 11295| 0.717 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|747 |5804 |12 |1 |0 |0 |0 |
|1832 |134 |746 |0 |3 |7 |0 |
|496 |27 |3 |504 |0 |1 |0 |
|13 |0 |0 |2 |408 |0 |0 |
|0 |0 |0 |0 |0 |420 |0 |
|108 |12 |5 |10 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| promise.js | 15 |
| 0003findLongestSubString.js | 15 |
| arrayFlatten.js | 13 |
| 0021mergeTwoOrderedLinkedList.js | 13 |
| 0583minDistance.js | 13 |
| 0155theMinStackO1.js | 12 |
| 0001sumOfTwoNum.js | 12 |
| 0232stackToQueue.js | 9 |
| 0242validAnagram.js | 9 |
| 0148sortLinkedList.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.8046410435651064, "precision": 0.8156350210729948, "recall": 0.7955242407045414, "support": 8099}, "micro avg": {"f1-score": 0.9732065687121867, "precision": 0.9732065687121867, "recall": 0.9732065687121867, "support": 8099}, "weighted avg": {"f1-score": 0.9708959028167164, "precision": 0.9700083601283576, "recall": 0.9732065687121867, "support": 8099}, "\u2205": {"f1-score": 0.9842292691198915, "precision": 0.9710557135686799, "recall": 0.9977651710503697, "support": 5817}, "\u23ce": {"f1-score": 0.9581749049429658, "precision": 0.9748549323017408, "recall": 0.9420560747663551, "support": 535}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9939098660170523, "precision": 0.9927007299270073, "recall": 0.9951219512195122, "support": 410}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9905660377358491, "precision": 0.9813084112149533, "recall": 1.0, "support": 420}, "\u2423": {"f1-score": 0.9009661835748792, "precision": 0.9738903394255874, "recall": 0.8382022471910112, "support": 890}},
  "cl_report_full": {"macro avg": {"f1-score": 0.6622504011694719, "precision": 0.8156350210729948, "recall": 0.601944152934831, "support": 11295}, "micro avg": {"f1-score": 0.8128287099102816, "precision": 0.9732065687121867, "recall": 0.6978308986277114, "support": 11295}, "weighted avg": {"f1-score": 0.7739055864805588, "precision": 0.9616712336833787, "recall": 0.6978308986277114, "support": 11295}, "\u2205": {"f1-score": 0.925604018818276, "precision": 0.9710557135686799, "recall": 0.8842169408897014, "support": 6564}, "\u23ce": {"f1-score": 0.6511627906976745, "precision": 0.9748549323017408, "recall": 0.48884578079534435, "support": 1031}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 135}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9784172661870503, "precision": 0.9927007299270073, "recall": 0.9645390070921985, "support": 423}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9905660377358491, "precision": 0.9813084112149533, "recall": 1.0, "support": 420}, "\u2423": {"f1-score": 0.42775229357798167, "precision": 0.9738903394255874, "recall": 0.27406318883174136, "support": 2722}},
  "ppcr": 0.7170429393536963
}
```
</details>
