# Train report for javascript / file:///tmp/top-repos-quality-repos-f2fbyuei/small-useful-and-fun-codes.git HEAD 4b69d0c4c85ae59e03c342bd2119ad7befece072

### Classification report

PPCR: 0.281

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 0.994| 0.395| 0.997| 0.566| 1496| 3767| 0.397 |
| `␣` | 0.983| 1.000| 0.217| 0.991| 0.356| 519| 2389| 0.217 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 427| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 182| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 213| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 188| 0.000 |
| `macro avg` | 0.330| 0.332| 0.102| 0.331| 0.154| 2015| 7166| 0.281 |
| `weighted avg` | 0.996| 0.996| 0.280| 0.996| 0.416| 2015| 7166| 0.281 |
| `micro avg` | 0.996| 0.996| 0.280| 0.996| 0.437| 2015| 7166| 0.281 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|2271 |1487 |9 |0 |0 |0 |0 |
|1870 |0 |519 |0 |0 |0 |0 |
|427 |0 |0 |0 |0 |0 |0 |
|182 |0 |0 |0 |0 |0 |0 |
|213 |0 |0 |0 |0 |0 |0 |
|188 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| javascript/Its-raining-boxes/game.js | 5 |
| javascript/punjabi_university_patiala_result_searcher.js | 3 |
| javascript/Its-raining-boxes/game_old.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.33139781909649746, "precision": 0.33049242424242425, "recall": 0.3323306595365419, "support": 2015}, "micro avg": {"f1-score": 0.9955334987593052, "precision": 0.9955334987593052, "recall": 0.9955334987593052, "support": 2015}, "weighted avg": {"f1-score": 0.9955459578225575, "precision": 0.9956096323031807, "recall": 0.9955334987593052, "support": 2015}, "\u2205": {"f1-score": 0.9969829031176667, "precision": 1.0, "recall": 0.9939839572192514, "support": 1496}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9914040114613181, "precision": 0.9829545454545454, "recall": 1.0, "support": 519}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "macro avg": {"f1-score": 0.15364832740633655, "precision": 0.33049242424242425, "recall": 0.1019982562469514, "support": 7166}, "micro avg": {"f1-score": 0.43698943470210216, "precision": 0.9955334987593052, "recall": 0.2799330170248395, "support": 7166}, "weighted avg": {"f1-score": 0.4161882531766108, "precision": 0.8533740453657422, "recall": 0.2799330170248395, "support": 7166}, "\u2205": {"f1-score": 0.5660449181575942, "precision": 1.0, "recall": 0.39474382797982477, "support": 3767}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 427}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 213}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 188}, "\u2423": {"f1-score": 0.3558450462804251, "precision": 0.9829545454545454, "recall": 0.21724570950188363, "support": 2389}},
  "ppcr": 0.28118894780909853
}
```
</details>
