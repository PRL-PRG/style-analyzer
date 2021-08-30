# Train report for javascript / file:///tmp/top-repos-quality-repos-69woa327/dashboard.git HEAD 265e8beaa7cf94381ac208074d1405a3b046b475

### Classification report

PPCR: 0.893

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.998| 0.835| 0.994| 0.906| 16301| 19469| 0.837 |
| `␣` | 0.996| 0.974| 0.926| 0.985| 0.960| 12788| 13452| 0.951 |
| `⏎` | 0.985| 0.999| 0.972| 0.992| 0.978| 7181| 7382| 0.973 |
| `⏎␣⁻␣⁻` | 0.934| 0.990| 0.869| 0.961| 0.900| 1071| 1220| 0.878 |
| `⏎␣⁺␣⁺` | 0.993| 0.983| 0.839| 0.988| 0.909| 1048| 1228| 0.853 |
| `"` | 1.000| 1.000| 0.466| 1.000| 0.636| 246| 528| 0.466 |
| `macro avg` | 0.983| 0.990| 0.818| 0.987| 0.882| 38635| 43279| 0.893 |
| `micro avg` | 0.989| 0.989| 0.883| 0.989| 0.933| 38635| 43279| 0.893 |
| `weighted avg` | 0.990| 0.989| 0.883| 0.989| 0.932| 38635| 43279| 0.893 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|3168 |16265 |16 |0 |2 |18 |0 |
|664 |167 |12452 |109 |3 |57 |0 |
|201 |0 |8 |7173 |0 |0 |0 |
|180 |1 |17 |0 |1030 |0 |0 |
|149 |4 |4 |1 |2 |1060 |0 |
|282 |0 |0 |0 |0 |0 |246 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| front/src/components/MarketcapChart/MarketcapChart.js | 126 |
| front/src/components/TokenInfo/TokenInfo.js | 47 |
| front/src/components/CommunityInfo/CommunityInfo.js | 43 |
| front/src/utils/formatNumValueWithCurrency.js | 43 |
| front/src/components/ExchangesStats/ExchangesStats.js | 38 |
| front/src/utils/formatNumValue.js | 26 |
| front/src/index.js | 21 |
| front/src/components/StoreStats/StoreStats.js | 16 |
| front/src/components/TokenInfo/mock.js | 13 |
| front/src/utils/index.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 246}, "macro avg": {"f1-score": 0.9865741872837188, "precision": 0.9830002894761897, "recall": 0.9904927525998959, "support": 38635}, "micro avg": {"f1-score": 0.9894137440144947, "precision": 0.9894137440144947, "recall": 0.9894137440144947, "support": 38635}, "weighted avg": {"f1-score": 0.9894095311728097, "precision": 0.9895708729244095, "recall": 0.9894137440144947, "support": 38635}, "\u2205": {"f1-score": 0.9936465269717148, "precision": 0.9895358033704448, "recall": 0.9977915465308876, "support": 16301}, "\u23ce": {"f1-score": 0.9918418141592921, "precision": 0.9848963339283262, "recall": 0.9988859490321682, "support": 7181}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.988009592326139, "precision": 0.9932497589199615, "recall": 0.982824427480916, "support": 1048}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9610154125113326, "precision": 0.933920704845815, "recall": 0.9897292250233427, "support": 1071}, "\u2423": {"f1-score": 0.9849317777338343, "precision": 0.9963991357925902, "recall": 0.9737253675320613, "support": 12788}},
  "cl_report_full": {"\"": {"f1-score": 0.6356589147286822, "precision": 1.0, "recall": 0.4659090909090909, "support": 528}, "macro avg": {"f1-score": 0.8815527410661587, "precision": 0.9830002894761897, "recall": 0.8177173251174189, "support": 43279}, "micro avg": {"f1-score": 0.9333203115462557, "precision": 0.9894137440144947, "recall": 0.88324591603318, "support": 43279}, "weighted avg": {"f1-score": 0.9316504043911259, "precision": 0.9895430215907398, "recall": 0.88324591603318, "support": 43279}, "\u2205": {"f1-score": 0.905976716983234, "precision": 0.9895358033704448, "recall": 0.8354306846782064, "support": 19469}, "\u23ce": {"f1-score": 0.9782475281281965, "precision": 0.9848963339283262, "recall": 0.9716878894608507, "support": 7382}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9094922737306843, "precision": 0.9932497589199615, "recall": 0.8387622149837134, "support": 1228}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9002123142250531, "precision": 0.933920704845815, "recall": 0.8688524590163934, "support": 1220}, "\u2423": {"f1-score": 0.9597286986011021, "precision": 0.9963991357925902, "recall": 0.9256616116562593, "support": 13452}},
  "ppcr": 0.8926962268074586
}
```
</details>
