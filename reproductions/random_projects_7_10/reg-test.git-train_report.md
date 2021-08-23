# Train report for javascript / file:///tmp/top-repos-quality-repos-2sp8_qp0/reg-test.git HEAD 618f7a9e5ffbc716cc61c15c7798968447bfe749

### Classification report

PPCR: 0.638

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.976| 0.992| 0.948| 0.984| 0.962| 7806| 8168| 0.956 |
| `␣` | 0.968| 0.919| 0.350| 0.943| 0.514| 1744| 4576| 0.381 |
| `'` | 1.000| 1.000| 0.319| 1.000| 0.484| 879| 2756| 0.319 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.962| 0.958| 0.935| 0.960| 0.948| 448| 459| 0.976 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.954| 0.957| 0.801| 0.955| 0.871| 392| 468| 0.838 |
| `⏎` | 0.975| 0.826| 0.177| 0.894| 0.299| 235| 1097| 0.214 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 139| 0.029 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 377| 0.000 |
| `macro avg` | 0.729| 0.706| 0.441| 0.717| 0.510| 11508| 18040| 0.638 |
| `micro avg` | 0.975| 0.975| 0.622| 0.975| 0.760| 11508| 18040| 0.638 |
| `weighted avg` | 0.975| 0.975| 0.622| 0.975| 0.705| 11508| 18040| 0.638 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|362 |7747 |49 |0 |0 |5 |5 |0 |0 |
|2832 |119 |1602 |0 |0 |11 |12 |0 |0 |
|1877 |0 |0 |879 |0 |0 |0 |0 |0 |
|862 |39 |0 |0 |194 |2 |0 |0 |0 |
|76 |10 |4 |0 |3 |375 |0 |0 |0 |
|11 |19 |0 |0 |0 |0 |429 |0 |0 |
|377 |0 |0 |0 |0 |0 |0 |0 |0 |
|135 |2 |0 |0 |2 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/admin/js/core.js | 63 |
| static/admin/js/SelectFilter2.js | 50 |
| static/admin/js/inlines.js | 39 |
| static/admin/js/SelectBox.js | 31 |
| static/admin/js/actions.js | 28 |
| static/admin/js/admin/DateTimeShortcuts.js | 19 |
| static/admin/js/calendar.js | 16 |
| static/admin/js/urlify.js | 9 |
| static/admin/js/admin/RelatedObjectLookups.js | 8 |
| static/admin/js/timeparse.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 879}, "macro avg": {"f1-score": 0.7170038655633206, "precision": 0.7293895699794561, "recall": 0.7063466933530593, "support": 11508}, "micro avg": {"f1-score": 0.9754953076120959, "precision": 0.9754953076120959, "recall": 0.9754953076120959, "support": 11508}, "weighted avg": {"f1-score": 0.9750213109037676, "precision": 0.9750878443532808, "recall": 0.9754953076120959, "support": 11508}, "\u2205": {"f1-score": 0.9842459662050566, "precision": 0.9761844758064516, "recall": 0.9924417115039713, "support": 7806}, "\u23ce": {"f1-score": 0.8940092165898617, "precision": 0.9748743718592965, "recall": 0.825531914893617, "support": 235}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9554140127388535, "precision": 0.9541984732824428, "recall": 0.9566326530612245, "support": 392}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.959731543624161, "precision": 0.9618834080717489, "recall": 0.9575892857142857, "support": 448}, "\u2423": {"f1-score": 0.942630185348632, "precision": 0.9679758308157099, "recall": 0.9185779816513762, "support": 1744}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 377}, "\u0027": {"f1-score": 0.4836313617606603, "precision": 1.0, "recall": 0.3189404934687954, "support": 2756}, "macro avg": {"f1-score": 0.5098106131633382, "precision": 0.7293895699794561, "recall": 0.441281727300921, "support": 18040}, "micro avg": {"f1-score": 0.7598483822932178, "precision": 0.9754953076120959, "recall": 0.6222838137472284, "support": 18040}, "weighted avg": {"f1-score": 0.7048635593272105, "precision": 0.9488048090870593, "recall": 0.6222838137472284, "support": 18040}, "\u2205": {"f1-score": 0.9621212121212122, "precision": 0.9761844758064516, "recall": 0.9484573947110676, "support": 8168}, "\u23ce": {"f1-score": 0.2993827160493827, "precision": 0.9748743718592965, "recall": 0.17684594348222424, "support": 1097}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 139}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8710801393728224, "precision": 0.9541984732824428, "recall": 0.8012820512820513, "support": 468}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9480662983425414, "precision": 0.9618834080717489, "recall": 0.934640522875817, "support": 459}, "\u2423": {"f1-score": 0.5142031776600867, "precision": 0.9679758308157099, "recall": 0.3500874125874126, "support": 4576}},
  "ppcr": 0.6379157427937916
}
```
</details>
