# Train report for javascript / file:///tmp/top-repos-quality-repos-skeutftm/tads.git HEAD a39116a8182e7cf99e23ee1692854f3260646cd0

### Classification report

PPCR: 0.317

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.988| 0.322| 0.986| 0.485| 661| 2028| 0.326 |
| `␣` | 0.952| 0.997| 0.365| 0.974| 0.528| 379| 1036| 0.366 |
| `'` | 1.000| 1.000| 0.490| 1.000| 0.657| 143| 292| 0.490 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 108| 0.130 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 218| 0.032 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 115| 0.000 |
| `weighted avg` | 0.958| 0.975| 0.309| 0.967| 0.454| 1204| 3797| 0.317 |
| `micro avg` | 0.975| 0.975| 0.309| 0.975| 0.470| 1204| 3797| 0.317 |
| `macro avg` | 0.489| 0.498| 0.196| 0.493| 0.278| 1204| 3797| 0.317 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1367 |653 |8 |0 |0 |0 |0 |
|657 |1 |378 |0 |0 |0 |0 |
|149 |0 |0 |143 |0 |0 |0 |
|211 |7 |0 |0 |0 |0 |0 |
|115 |0 |0 |0 |0 |0 |0 |
|94 |3 |11 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| react/huntweb/src/serviceWorker.js | 5 |
| react/huntweb/src/pages/main/index.js | 4 |
| jsBasic/JS assíncrono/desafio1.js | 3 |
| ES6/Conceitos/public/async.js | 3 |
| ES6/Conceitos/public/spread.js | 3 |
| jsBasic/JS assíncrono/main.js | 2 |
| ES6/src/main.js | 2 |
| ES6/Conceitos/public/desafio1.js | 2 |
| jsBasic/JS assíncrono/desafio2.js | 2 |
| ES6/Conceitos/public/function.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 143}, "macro avg": {"f1-score": 0.49331453024703364, "precision": 0.4892624654790447, "recall": 0.4975431005233136, "support": 1204}, "micro avg": {"f1-score": 0.9750830564784053, "precision": 0.9750830564784053, "recall": 0.9750830564784053, "support": 1204}, "weighted avg": {"f1-score": 0.9665726480040273, "precision": 0.9583979732162454, "recall": 0.9750830564784053, "support": 1204}, "\u2205": {"f1-score": 0.9856603773584907, "precision": 0.983433734939759, "recall": 0.9878971255673222, "support": 661}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.9742268041237113, "precision": 0.9521410579345088, "recall": 0.9973614775725593, "support": 379}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6574712643678161, "precision": 1.0, "recall": 0.4897260273972603, "support": 292}, "macro avg": {"f1-score": 0.27836282887545655, "precision": 0.4892624654790447, "recall": 0.19609716711929567, "support": 3797}, "micro avg": {"f1-score": 0.4695060987802439, "precision": 0.9750830564784053, "recall": 0.3091914669475902, "support": 3797}, "weighted avg": {"f1-score": 0.45362253181929546, "precision": 0.8619493680479279, "recall": 0.3091914669475902, "support": 3797}, "\u2205": {"f1-score": 0.4851411589895988, "precision": 0.983433734939759, "recall": 0.3219921104536489, "support": 2028}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 218}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u2423": {"f1-score": 0.5275645498953244, "precision": 0.9521410579345088, "recall": 0.36486486486486486, "support": 1036}},
  "ppcr": 0.31709244140110615
}
```
</details>
