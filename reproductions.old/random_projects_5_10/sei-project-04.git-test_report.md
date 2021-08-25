# Test report for javascript / file:///tmp/top-repos-quality-repos-kli0n8xe/sei-project-04.git HEAD dfb3484a5f1667d1855d6413dc40b0fae7362bbc

### Classification report

PPCR: 0.854

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.886| 0.986| 0.939| 0.933| 0.912| 1452| 1525| 0.952 |
| `␣` | 0.852| 0.641| 0.469| 0.732| 0.605| 368| 503| 0.732 |
| `'` | 0.958| 1.000| 0.994| 0.978| 0.975| 338| 340| 0.994 |
| `⏎⇥⁻` | 0.857| 0.590| 0.590| 0.699| 0.699| 61| 61| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 40| 169| 0.237 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 55| 0.527 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 27| 0.037 |
| `weighted avg` | 0.863| 0.892| 0.762| 0.873| 0.772| 2289| 2680| 0.854 |
| `macro avg` | 0.507| 0.460| 0.427| 0.477| 0.456| 2289| 2680| 0.854 |
| `micro avg` | 0.892| 0.892| 0.762| 0.892| 0.822| 2289| 2680| 0.854 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁻| ⏎⇥⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|73 |1432 |16 |0 |0 |4 |0 |0 |
|135 |115 |236 |15 |0 |2 |0 |0 |
|2 |0 |0 |338 |0 |0 |0 |0 |
|129 |30 |10 |0 |0 |0 |0 |0 |
|0 |24 |1 |0 |0 |36 |0 |0 |
|26 |16 |13 |0 |0 |0 |0 |0 |
|26 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9782923299565847, "precision": 0.9575070821529745, "recall": 1.0, "support": 338}, "macro avg": {"f1-score": 0.4774724856608197, "precision": 0.507460871248419, "recall": 0.4596705967955887, "support": 2289}, "micro avg": {"f1-score": 0.892092616863259, "precision": 0.892092616863259, "recall": 0.892092616863259, "support": 2289}, "weighted avg": {"f1-score": 0.8727000703117234, "precision": 0.8629669484466602, "recall": 0.892092616863259, "support": 2289}, "\u2205": {"f1-score": 0.9332029977191267, "precision": 0.8855905998763142, "recall": 0.9862258953168044, "support": 1452}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u21e5\u207b": {"f1-score": 0.6990291262135924, "precision": 0.8571428571428571, "recall": 0.5901639344262295, "support": 61}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.731782945736434, "precision": 0.851985559566787, "recall": 0.6413043478260869, "support": 368}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9754689754689755, "precision": 0.9575070821529745, "recall": 0.9941176470588236, "support": 340}, "macro avg": {"f1-score": 0.4558782329726038, "precision": 0.507460871248419, "recall": 0.4274975522262485, "support": 2680}, "micro avg": {"f1-score": 0.8218957536727711, "precision": 0.892092616863259, "recall": 0.7619402985074627, "support": 2680}, "weighted avg": {"f1-score": 0.7719215427739123, "precision": 0.804818105780298, "recall": 0.7619402985074627, "support": 2680}, "\u2205": {"f1-score": 0.9115213239974539, "precision": 0.8855905998763142, "recall": 0.939016393442623, "support": 1525}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 169}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u23ce\u21e5\u207b": {"f1-score": 0.6990291262135924, "precision": 0.8571428571428571, "recall": 0.5901639344262295, "support": 61}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u2423": {"f1-score": 0.6051282051282052, "precision": 0.851985559566787, "recall": 0.4691848906560636, "support": 503}},
  "ppcr": 0.8541044776119403
}
```
</details>
