# Train report for javascript / file:///tmp/top-repos-quality-repos-mqwb7mko/amphora.git HEAD cdddd55c3b6ea5008ff28642f5aff754ff7cd304

### Classification report

PPCR: 0.832

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.992| 0.970| 0.993| 0.982| 77729| 79495| 0.978 |
| `␣` | 0.950| 0.995| 0.941| 0.972| 0.946| 42229| 44673| 0.945 |
| `'` | 0.980| 1.000| 0.796| 0.990| 0.878| 8685| 10908| 0.796 |
| `⏎␣⁻␣⁻` | 0.976| 0.431| 0.103| 0.598| 0.186| 832| 3500| 0.238 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 633| 9863| 0.064 |
| `⏎␣⁺␣⁺` | 0.934| 0.339| 0.047| 0.497| 0.090| 502| 3593| 0.140 |
| `⏎⏎` | 0.922| 0.922| 0.078| 0.922| 0.144| 206| 2425| 0.085 |
| `"` | 1.000| 0.126| 0.044| 0.224| 0.084| 206| 594| 0.347 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 203| 1426| 0.142 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 125| 1352| 0.092 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 98| 0.429 |
| `macro avg` | 0.614| 0.437| 0.271| 0.472| 0.301| 131392| 157927| 0.832 |
| `micro avg` | 0.978| 0.978| 0.814| 0.978| 0.889| 131392| 157927| 0.832 |
| `weighted avg` | 0.971| 0.978| 0.814| 0.973| 0.831| 131392| 157927| 0.832 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1766 |77092 |635 |0 |0 |0 |2 |0 |0 |0 |0 |0 |
|2444 |198 |42031 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2223 |0 |0 |8685 |0 |0 |0 |0 |0 |0 |0 |0 |
|9230 |28 |589 |0 |0 |0 |0 |16 |0 |0 |0 |0 |
|3091 |36 |296 |0 |0 |170 |0 |0 |0 |0 |0 |0 |
|2668 |96 |377 |0 |0 |0 |359 |0 |0 |0 |0 |0 |
|2219 |3 |13 |0 |0 |0 |0 |190 |0 |0 |0 |0 |
|1223 |30 |161 |0 |0 |12 |0 |0 |0 |0 |0 |0 |
|1227 |37 |85 |0 |0 |0 |3 |0 |0 |0 |0 |0 |
|388 |0 |0 |180 |0 |0 |0 |0 |0 |0 |26 |0 |
|56 |0 |38 |0 |0 |0 |4 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| cookies/static/cookies/js/pdf.js | 873 |
| cookies/static/cookies/web/viewer.js | 642 |
| cookies/static/cookies/web/l10n.js | 280 |
| static/django_extensions/js/jquery.autocomplete.26e55daaf7c5.js | 80 |
| static/django_extensions/js/jquery.autocomplete.js | 80 |
| static/admin/js/inlines.js | 59 |
| static/django_extensions/js/jquery.ajaxQueue.ac504621bdd8.js | 59 |
| static/admin/js/inlines.8199258210ef.js | 59 |
| static/django_extensions/js/jquery.ajaxQueue.js | 59 |
| static/admin/js/admin/DateTimeShortcuts.1a1d41ec13f8.js | 46 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.22413793103448276, "precision": 1.0, "recall": 0.1262135922330097, "support": 206}, "\u0027": {"f1-score": 0.9897435897435898, "precision": 0.9796954314720813, "recall": 1.0, "support": 8685}, "macro avg": {"f1-score": 0.4724630122063893, "precision": 0.6142276214483374, "recall": 0.4368905119743775, "support": 131392}, "micro avg": {"f1-score": 0.9783929006332197, "precision": 0.9783929006332197, "recall": 0.9783929006332197, "support": 131392}, "weighted avg": {"f1-score": 0.9729347043978175, "precision": 0.9712849764980489, "recall": 0.9783929006332197, "support": 131392}, "\u2205": {"f1-score": 0.9931400524318996, "precision": 0.9944788441692467, "recall": 0.9918048604767847, "support": 77729}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 633}, "\u23ce\u23ce": {"f1-score": 0.9223300970873787, "precision": 0.9223300970873787, "recall": 0.9223300970873787, "support": 206}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.49707602339181284, "precision": 0.9340659340659341, "recall": 0.3386454183266932, "support": 502}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 203}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5983333333333334, "precision": 0.9755434782608695, "recall": 0.43149038461538464, "support": 832}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 125}, "\u2423": {"f1-score": 0.9723321072477848, "precision": 0.9503900508762012, "recall": 0.9953112789789007, "support": 42229}},
  "cl_report_full": {"\"": {"f1-score": 0.08387096774193549, "precision": 1.0, "recall": 0.04377104377104377, "support": 594}, "\u0027": {"f1-score": 0.8784706417842513, "precision": 0.9796954314720813, "recall": 0.7962046204620462, "support": 10908}, "macro avg": {"f1-score": 0.3009123370401517, "precision": 0.6142276214483374, "recall": 0.27080387691225344, "support": 157927}, "micro avg": {"f1-score": 0.8886592308144299, "precision": 0.9783929006332197, "recall": 0.8140026721206634, "support": 157927}, "weighted avg": {"f1-score": 0.831144969891158, "precision": 0.8978866171520582, "recall": 0.8140026721206634, "support": 157927}, "\u2205": {"f1-score": 0.9819698754896029, "precision": 0.9944788441692467, "recall": 0.9697716837536952, "support": 79495}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9863}, "\u23ce\u23ce": {"f1-score": 0.14443177499049792, "precision": 0.9223300970873787, "recall": 0.07835051546391752, "support": 2425}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.09006622516556291, "precision": 0.9340659340659341, "recall": 0.047314222098524907, "support": 3593}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1426}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.18562564632885212, "precision": 0.9755434782608695, "recall": 0.10257142857142858, "support": 3500}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1352}, "\u2423": {"f1-score": 0.945600575940966, "precision": 0.9503900508762012, "recall": 0.9408591319141316, "support": 44673}},
  "ppcr": 0.8319793322231157
}
```
</details>
