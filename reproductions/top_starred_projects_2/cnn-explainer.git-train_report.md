# Train report for javascript / file:///tmp/top-repos-quality-repos-5jk3sdmg/cnn-explainer.git HEAD 09342105af93e2d363871b600f3612a871406728

### Classification report

PPCR: 0.810

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.965| 0.992| 0.960| 0.978| 0.963| 18055| 18647| 0.968 |
| `␣` | 0.974| 0.962| 0.747| 0.968| 0.845| 8043| 10359| 0.776 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 3856| 3856| 1.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 149| 446| 0.334 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 144| 463| 0.311 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 72| 2546| 0.028 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 884| 0.055 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 295| 0.003 |
| `micro avg` | 0.972| 0.972| 0.787| 0.972| 0.869| 30369| 37496| 0.810 |
| `macro avg` | 0.367| 0.369| 0.338| 0.368| 0.351| 30369| 37496| 0.810 |
| `weighted avg` | 0.958| 0.972| 0.787| 0.965| 0.815| 30369| 37496| 0.810 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| ⏎⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|592 |17910 |145 |0 |0 |0 |0 |0 |0 |
|2316 |305 |7738 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |3856 |0 |0 |0 |0 |0 |
|2474 |55 |17 |0 |0 |0 |0 |0 |0 |
|835 |9 |40 |0 |0 |0 |0 |0 |0 |
|319 |144 |0 |0 |0 |0 |0 |0 |0 |
|297 |143 |6 |0 |0 |0 |0 |0 |0 |
|294 |0 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/overview/flatten-draw.js | 218 |
| src/overview/intermediate-draw.js | 209 |
| src/overview/overview-draw.js | 160 |
| src/utils/cnn.js | 67 |
| src/utils/cnn-tf.js | 62 |
| src/overview/intermediate-utils.js | 46 |
| src/detail-view/DetailviewUtils.js | 43 |
| src/overview/draw-utils.js | 38 |
| src/config.js | 14 |
| rollup.config.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3856}, "macro avg": {"f1-score": 0.36824777666552777, "precision": 0.36729592030450686, "recall": 0.3692559762462034, "support": 30369}, "micro avg": {"f1-score": 0.9715170074747275, "precision": 0.9715170074747275, "recall": 0.9715170074747275, "support": 30369}, "weighted avg": {"f1-score": 0.9648175939106909, "precision": 0.958363154655057, "recall": 0.9715170074747275, "support": 30369}, "\u2205": {"f1-score": 0.9781273040058982, "precision": 0.9646665948508025, "recall": 0.9919689836610357, "support": 18055}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 144}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 149}, "\u2423": {"f1-score": 0.967854909318324, "precision": 0.9737007675852523, "recall": 0.9620788263085913, "support": 8043}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3856}, "macro avg": {"f1-score": 0.3509965903467552, "precision": 0.36729592030450686, "recall": 0.3384324394449485, "support": 37496}, "micro avg": {"f1-score": 0.8694909010535622, "precision": 0.9715170074747275, "recall": 0.7868572647749094, "support": 37496}, "weighted avg": {"f1-score": 0.8150881012884437, "precision": 0.8515762813526387, "recall": 0.7868572647749094, "support": 37496}, "\u2205": {"f1-score": 0.9625668449197861, "precision": 0.9646665948508025, "recall": 0.9604762160132997, "support": 18647}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2546}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 463}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 295}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 884}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 446}, "\u2423": {"f1-score": 0.8454058778542554, "precision": 0.9737007675852523, "recall": 0.7469832995462883, "support": 10359}},
  "ppcr": 0.8099263921484958
}
```
</details>
