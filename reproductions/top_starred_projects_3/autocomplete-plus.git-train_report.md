# Train report for javascript / file:///tmp/top-repos-quality-repos-btoso_kg/autocomplete-plus.git HEAD d2a02b1fb7d228b2ea00065d6fe19f96f5731861

### Classification report

PPCR: 0.847

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.994| 0.960| 0.994| 0.977| 32761| 33910| 0.966 |
| `␣` | 0.971| 0.996| 0.894| 0.983| 0.931| 11410| 12705| 0.898 |
| `'` | 1.000| 1.000| 0.965| 1.000| 0.982| 5461| 5657| 0.965 |
| `⏎⏎` | 0.950| 0.956| 0.319| 0.953| 0.477| 481| 1444| 0.333 |
| `⏎` | 0.969| 0.757| 0.122| 0.850| 0.218| 461| 2849| 0.162 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 99| 1703| 0.058 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 80| 1659| 0.048 |
| `weighted avg` | 0.986| 0.989| 0.838| 0.987| 0.865| 50753| 59927| 0.847 |
| `micro avg` | 0.989| 0.989| 0.838| 0.989| 0.907| 50753| 59927| 0.847 |
| `macro avg` | 0.698| 0.672| 0.466| 0.683| 0.512| 50753| 59927| 0.847 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1149 |32567 |190 |0 |0 |0 |0 |4 |
|1295 |49 |11360 |0 |0 |0 |0 |1 |
|196 |0 |0 |5461 |0 |0 |0 |0 |
|2388 |30 |63 |0 |349 |0 |0 |19 |
|1604 |23 |76 |0 |0 |0 |0 |0 |
|1579 |79 |1 |0 |0 |0 |0 |0 |
|963 |0 |10 |0 |11 |0 |0 |460 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| spec/autocomplete-manager-integration-spec.js | 68 |
| lib/autocomplete-manager.js | 60 |
| lib/suggestion-list-element.js | 44 |
| lib/subsequence-provider.js | 42 |
| spec/subsequence-provider-spec.js | 40 |
| lib/provider-manager.js | 37 |
| lib/symbol-provider.js | 32 |
| lib/suggestion-list.js | 25 |
| lib/snippet-parser.js | 24 |
| lib/fuzzy-provider.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5461}, "macro avg": {"f1-score": 0.6829929091499637, "precision": 0.6978958262053968, "recall": 0.6718695788245724, "support": 50753}, "micro avg": {"f1-score": 0.9890449825626071, "precision": 0.9890449825626071, "recall": 0.9890449825626071, "support": 50753}, "weighted avg": {"f1-score": 0.9871813283698614, "precision": 0.9856248526772937, "recall": 0.9890449825626071, "support": 50753}, "\u2205": {"f1-score": 0.9942755957196722, "precision": 0.9944729449126665, "recall": 0.9940783248374592, "support": 32761}, "\u23ce": {"f1-score": 0.8501827040194884, "precision": 0.9694444444444444, "recall": 0.7570498915401301, "support": 461}, "\u23ce\u23ce": {"f1-score": 0.9533678756476685, "precision": 0.9504132231404959, "recall": 0.9563409563409564, "support": 481}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 99}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 80}, "\u2423": {"f1-score": 0.9831241886629165, "precision": 0.9709401709401709, "recall": 0.9956178790534619, "support": 11410}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9823709300233855, "precision": 1.0, "recall": 0.9653526604207177, "support": 5657}, "macro avg": {"f1-score": 0.5121651994611736, "precision": 0.6978958262053968, "recall": 0.46584895289121675, "support": 59927}, "micro avg": {"f1-score": 0.9070654138055655, "precision": 0.9890449825626071, "recall": 0.8376357902114239, "support": 59927}, "weighted avg": {"f1-score": 0.8648611747145644, "precision": 0.9319624935374787, "recall": 0.8376357902114239, "support": 59927}, "\u2205": {"f1-score": 0.9771370278136159, "precision": 0.9944729449126665, "recall": 0.9603951636685344, "support": 33910}, "\u23ce": {"f1-score": 0.2175132440012465, "precision": 0.9694444444444444, "recall": 0.1224991224991225, "support": 2849}, "\u23ce\u23ce": {"f1-score": 0.47717842323651455, "precision": 0.9504132231404959, "recall": 0.3185595567867036, "support": 1444}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1703}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1659}, "\u2423": {"f1-score": 0.9309567711534521, "precision": 0.9709401709401709, "recall": 0.8941361668634396, "support": 12705}},
  "ppcr": 0.8469137450564854
}
```
</details>
