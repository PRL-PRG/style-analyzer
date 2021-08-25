# Train report for javascript / file:///tmp/top-repos-quality-repos-9alsofqb/avimehenwal.in.git HEAD c8dd7a458c1882832238980000b145ae1b6d32a9

### Classification report

PPCR: 0.754

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.991| 0.955| 0.992| 0.973| 22441| 23293| 0.963 |
| `␣` | 0.968| 0.992| 0.916| 0.980| 0.941| 13406| 14510| 0.924 |
| `'` | 1.000| 1.000| 0.221| 1.000| 0.362| 489| 2211| 0.221 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.947| 0.871| 0.133| 0.908| 0.233| 124| 814| 0.152 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 100| 2810| 0.036 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 87| 515| 0.169 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 913| 0.051 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 470| 0.051 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 1196| 0.018 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 247| 0.016 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 224| 0.004 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1510| 0.000 |
| `weighted avg` | 0.976| 0.983| 0.742| 0.980| 0.766| 36745| 48713| 0.754 |
| `micro avg` | 0.983| 0.983| 0.742| 0.983| 0.846| 36745| 48713| 0.754 |
| `macro avg` | 0.326| 0.321| 0.185| 0.323| 0.209| 36745| 48713| 0.754 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|852 |22240 |201 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1104 |108 |13298 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2710 |4 |96 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1722 |0 |0 |0 |489 |0 |0 |0 |0 |0 |0 |0 |0 |
|1174 |10 |12 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|866 |4 |43 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|690 |7 |9 |0 |0 |0 |0 |108 |0 |0 |0 |0 |0 |
|428 |12 |75 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|446 |17 |1 |0 |0 |0 |0 |6 |0 |0 |0 |0 |0 |
|1510 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|243 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|223 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/js/moment.js | 323 |
| themes/cupper-hugo-theme/assets/js/lunr.js | 178 |
| static/js/moment-timezone-with-data-2012-2022.js | 31 |
| static/js/moment-timezone.js | 28 |
| themes/cupper-hugo-theme/assets/js/search.js | 19 |
| themes/cupper-hugo-theme/static/js/dom-scripts.js | 14 |
| themes/cupper-hugo-theme/static/js/service-worker-registration.js | 10 |
| static/js/search.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 489}, "macro avg": {"f1-score": 0.3232672753666412, "precision": 0.3256640061493417, "recall": 0.32116290229527394, "support": 36745}, "micro avg": {"f1-score": 0.9833991019186283, "precision": 0.9833991019186283, "recall": 0.9833991019186283, "support": 36745}, "weighted avg": {"f1-score": 0.979596231323737, "precision": 0.9759133501631383, "recall": 0.9833991019186283, "support": 36745}, "\u2205": {"f1-score": 0.9919050910956002, "precision": 0.9927685028122489, "recall": 0.9910431798939441, "support": 22441}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 100}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 87}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9075630252100839, "precision": 0.9473684210526315, "recall": 0.8709677419354839, "support": 124}, "\u2423": {"f1-score": 0.9797391880940101, "precision": 0.9678311499272197, "recall": 0.9919439057138595, "support": 13406}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1510}, "\u0027": {"f1-score": 0.3622222222222222, "precision": 1.0, "recall": 0.22116689280868385, "support": 2211}, "macro avg": {"f1-score": 0.20915356899722412, "precision": 0.3256640061493417, "recall": 0.1854258091715327, "support": 48713}, "micro avg": {"f1-score": 0.8456785789510636, "precision": 0.9833991019186283, "recall": 0.7417937716831235, "support": 48713}, "weighted avg": {"f1-score": 0.7662113205839693, "precision": 0.8242141649290028, "recall": 0.7417937716831235, "support": 48713}, "\u2205": {"f1-score": 0.973410657621184, "precision": 0.9927685028122489, "recall": 0.9547932855364273, "support": 23293}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2810}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 247}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 224}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1196}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 515}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 913}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 470}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.23275862068965514, "precision": 0.9473684210526315, "recall": 0.13267813267813267, "support": 814}, "\u2423": {"f1-score": 0.9414513274336282, "precision": 0.9678311499272197, "recall": 0.9164713990351482, "support": 14510}},
  "ppcr": 0.7543160963192577
}
```
</details>
