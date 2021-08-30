# Train report for javascript / file:///tmp/top-repos-quality-repos-4i_imopo/clockpicker.git HEAD e6ac014b3c167281ac37cf122ab19b6967d8fae4

### Classification report

PPCR: 0.555

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.845| 0.999| 0.897| 0.916| 0.870| 7522| 8377| 0.898 |
| `␣` | 0.972| 0.287| 0.075| 0.443| 0.139| 1692| 6481| 0.261 |
| `'` | 1.000| 1.000| 0.959| 1.000| 0.979| 814| 849| 0.959 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 108| 616| 0.175 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 46| 544| 0.085 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 1134| 0.016 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 366| 0.005 |
| `weighted avg` | 0.864| 0.864| 0.480| 0.828| 0.491| 10202| 18367| 0.555 |
| `micro avg` | 0.864| 0.864| 0.480| 0.864| 0.617| 10202| 18367| 0.555 |
| `macro avg` | 0.402| 0.327| 0.276| 0.337| 0.284| 10202| 18367| 0.555 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⏎| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|855 |7513 |9 |0 |0 |0 |0 |0 |
|4789 |1206 |486 |0 |0 |0 |0 |0 |
|35 |0 |0 |814 |0 |0 |0 |0 |
|1116 |18 |0 |0 |0 |0 |0 |0 |
|508 |103 |5 |0 |0 |0 |0 |0 |
|364 |2 |0 |0 |0 |0 |0 |0 |
|498 |46 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/assets/qunit-1.14.0.js | 1245 |
| src/clockpicker.js | 93 |
| gulpfile.js | 30 |
| test/suites/basic.js | 14 |
| test/suites/data-api.js | 4 |
| test/assets/utils.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 814}, "macro avg": {"f1-score": 0.33701311987718263, "precision": 0.40247100424328147, "recall": 0.3265767931797225, "support": 10202}, "micro avg": {"f1-score": 0.863850225445991, "precision": 0.863850225445991, "recall": 0.863850225445991, "support": 10202}, "weighted avg": {"f1-score": 0.8284540368639647, "precision": 0.8642372336233819, "recall": 0.863850225445991, "support": 10202}, "\u2205": {"f1-score": 0.9156611822059719, "precision": 0.8452970297029703, "recall": 0.9988035097048658, "support": 7522}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.4434306569343066, "precision": 0.972, "recall": 0.2872340425531915, "support": 1692}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9789536981358989, "precision": 1.0, "recall": 0.9587750294464076, "support": 849}, "macro avg": {"f1-score": 0.2840720617543919, "precision": 0.40247100424328147, "recall": 0.2758034154848152, "support": 18367}, "micro avg": {"f1-score": 0.6169624418075536, "precision": 0.863850225445991, "recall": 0.47982795230576575, "support": 18367}, "weighted avg": {"f1-score": 0.491324032414514, "precision": 0.7747364957707727, "recall": 0.47982795230576575, "support": 18367}, "\u2205": {"f1-score": 0.8703156675354763, "precision": 0.8452970297029703, "recall": 0.8968604512355258, "support": 8377}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1134}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 616}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 544}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 366}, "\u2423": {"f1-score": 0.13923506660936827, "precision": 0.972, "recall": 0.07498842771177287, "support": 6481}},
  "ppcr": 0.5554527141068221
}
```
</details>
