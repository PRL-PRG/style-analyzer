# Test report for javascript / file:///tmp/top-repos-quality-repos-dtiwvh3g/api-sieg-down-v2.git HEAD 6a5fa6ff3c549b224d2cc33757d06184ddda6e5a

### Classification report

PPCR: 0.775

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.943| 0.968| 0.880| 0.956| 0.911| 379| 417| 0.909 |
| `␣` | 0.830| 0.897| 0.775| 0.862| 0.802| 223| 258| 0.864 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.853| 0.967| 0.967| 0.906| 0.906| 30| 30| 1.000 |
| `'` | 1.000| 0.920| 0.500| 0.958| 0.667| 25| 46| 0.543 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 82| 0.244 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 25| 0.400 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 29| 0.000 |
| `weighted avg` | 0.863| 0.901| 0.698| 0.882| 0.727| 687| 887| 0.775 |
| `micro avg` | 0.901| 0.901| 0.698| 0.901| 0.787| 687| 887| 0.775 |
| `macro avg` | 0.518| 0.536| 0.446| 0.526| 0.469| 687| 887| 0.775 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|38 |367 |10 |0 |0 |2 |0 |0 |
|35 |20 |200 |0 |0 |3 |0 |0 |
|21 |2 |0 |23 |0 |0 |0 |0 |
|62 |0 |20 |0 |0 |0 |0 |0 |
|0 |0 |1 |0 |0 |29 |0 |0 |
|29 |0 |0 |0 |0 |0 |0 |0 |
|15 |0 |10 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9583333333333334, "precision": 1.0, "recall": 0.92, "support": 25}, "macro avg": {"f1-score": 0.5260544950738917, "precision": 0.5180373464599869, "recall": 0.5359807691549234, "support": 687}, "micro avg": {"f1-score": 0.901018922852984, "precision": 0.901018922852984, "recall": 0.901018922852984, "support": 687}, "weighted avg": {"f1-score": 0.8815262981227727, "precision": 0.8634876690789621, "recall": 0.901018922852984, "support": 687}, "\u2205": {"f1-score": 0.9557291666666666, "precision": 0.9434447300771208, "recall": 0.9683377308707124, "support": 379}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.90625, "precision": 0.8529411764705882, "recall": 0.9666666666666667, "support": 30}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8620689655172414, "precision": 0.8298755186721992, "recall": 0.8968609865470852, "support": 223}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 46}, "macro avg": {"f1-score": 0.46931283546651376, "precision": 0.5180373464599869, "recall": 0.44599376976823857, "support": 887}, "micro avg": {"f1-score": 0.7865311308767471, "precision": 0.901018922852984, "recall": 0.6978579481397971, "support": 887}, "weighted avg": {"f1-score": 0.72651316073707, "precision": 0.7656286037809521, "recall": 0.6978579481397971, "support": 887}, "\u2205": {"f1-score": 0.9106699751861043, "precision": 0.9434447300771208, "recall": 0.8800959232613909, "support": 417}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.90625, "precision": 0.8529411764705882, "recall": 0.9666666666666667, "support": 30}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u2423": {"f1-score": 0.8016032064128257, "precision": 0.8298755186721992, "recall": 0.7751937984496124, "support": 258}},
  "ppcr": 0.7745208568207441
}
```
</details>
