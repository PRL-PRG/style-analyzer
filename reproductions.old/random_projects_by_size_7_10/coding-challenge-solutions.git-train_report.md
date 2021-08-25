# Train report for javascript / file:///tmp/top-repos-quality-repos-h48wcfa1/coding-challenge-solutions.git HEAD 9d17e58b043d1d5443fa03ad935232494da1234c

### Classification report

PPCR: 0.787

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.953| 0.996| 0.934| 0.974| 0.943| 8004| 8536| 0.938 |
| `␣` | 0.971| 0.956| 0.600| 0.963| 0.742| 3087| 4915| 0.628 |
| `'` | 1.000| 1.000| 0.989| 1.000| 0.994| 969| 980| 0.989 |
| `⏎␣⁺␣⁺` | 0.940| 0.699| 0.661| 0.802| 0.776| 472| 499| 0.946 |
| `⏎␣⁻␣⁻` | 1.000| 0.650| 0.595| 0.788| 0.746| 377| 412| 0.915 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 439| 0.084 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 690| 0.030 |
| `weighted avg` | 0.957| 0.961| 0.757| 0.957| 0.812| 12967| 16471| 0.787 |
| `micro avg` | 0.961| 0.961| 0.757| 0.961| 0.847| 12967| 16471| 0.787 |
| `macro avg` | 0.695| 0.614| 0.540| 0.647| 0.600| 12967| 16471| 0.787 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|532 |7969 |35 |0 |0 |0 |0 |0 |
|1828 |115 |2951 |0 |0 |21 |0 |0 |
|11 |0 |0 |969 |0 |0 |0 |0 |
|669 |16 |5 |0 |0 |0 |0 |0 |
|27 |119 |23 |0 |0 |330 |0 |0 |
|35 |109 |23 |0 |0 |0 |245 |0 |
|402 |35 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| advent-of-code/2016/day-01/partTwo.js | 31 |
| advent-of-code/2016/day-01/partOne.js | 28 |
| advent-of-code/2016/day-02/partTwo.js | 23 |
| advent-of-code/2016/day-02/partOne.js | 22 |
| advent-of-code/2018/day-04/partOne.js | 22 |
| advent-of-code/2016/day-04/partTwo.js | 19 |
| advent-of-code/2016/day-04/partOne.js | 17 |
| advent-of-code/2015/day-06/part-2/solution.js | 17 |
| advent-of-code/2015/day-06/part-1/solution.js | 16 |
| advent-of-code/2015/day-13/part-1/solution.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 969}, "macro avg": {"f1-score": 0.6467069599878404, "precision": 0.6948716808819582, "recall": 0.6143701978942683, "support": 12967}, "micro avg": {"f1-score": 0.9612092234132799, "precision": 0.9612092234132799, "recall": 0.9612092234132799, "support": 12967}, "weighted avg": {"f1-score": 0.9572637110461418, "precision": 0.9573751878987946, "recall": 0.9612092234132799, "support": 12967}, "\u2205": {"f1-score": 0.9737887212073073, "precision": 0.9528877197178046, "recall": 0.9956271864067966, "support": 8004}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8019441069258809, "precision": 0.9401709401709402, "recall": 0.6991525423728814, "support": 472}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7877813504823151, "precision": 1.0, "recall": 0.649867374005305, "support": 377}, "\u2423": {"f1-score": 0.9634345412993798, "precision": 0.9710431062849622, "recall": 0.9559442824748947, "support": 3087}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9943560800410467, "precision": 1.0, "recall": 0.9887755102040816, "support": 980}, "macro avg": {"f1-score": 0.600255757272213, "precision": 0.6948716808819582, "recall": 0.5398201017774271, "support": 16471}, "micro avg": {"f1-score": 0.8467966573816156, "precision": 0.9612092234132799, "recall": 0.7567239390443811, "support": 16471}, "weighted avg": {"f1-score": 0.8115351723801197, "precision": 0.8965862268257585, "recall": 0.7567239390443811, "support": 16471}, "\u2205": {"f1-score": 0.9431327297473223, "precision": 0.9528877197178046, "recall": 0.9335754451733833, "support": 8536}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 690}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 439}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7764705882352941, "precision": 0.9401709401709402, "recall": 0.6613226452905812, "support": 499}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.745814307458143, "precision": 1.0, "recall": 0.5946601941747572, "support": 412}, "\u2423": {"f1-score": 0.7420165954236861, "precision": 0.9710431062849622, "recall": 0.6004069175991862, "support": 4915}},
  "ppcr": 0.7872624612956105
}
```
</details>
