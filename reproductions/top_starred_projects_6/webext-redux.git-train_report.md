# Train report for javascript / file:///tmp/top-repos-quality-repos-lsin22na/webext-redux.git HEAD 2a77acb1bd9e8059df10fe58e30293ffa46d770d

### Classification report

PPCR: 0.712

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.940| 0.996| 0.955| 0.968| 0.948| 8789| 9171| 0.958 |
| `␣` | 0.972| 0.882| 0.583| 0.924| 0.728| 3185| 4820| 0.661 |
| `⏎␣⁻␣⁻` | 1.000| 0.860| 0.528| 0.925| 0.691| 342| 557| 0.614 |
| `"` | 1.000| 1.000| 0.272| 1.000| 0.428| 312| 1145| 0.272 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 98| 815| 0.120 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 46| 596| 0.077 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 402| 0.092 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 491| 0.000 |
| `micro avg` | 0.950| 0.950| 0.676| 0.950| 0.790| 12809| 17997| 0.712 |
| `weighted avg` | 0.938| 0.950| 0.676| 0.943| 0.727| 12809| 17997| 0.712 |
| `macro avg` | 0.489| 0.467| 0.292| 0.477| 0.349| 12809| 17997| 0.712 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|382 |8757 |32 |0 |0 |0 |0 |0 |0 |
|1635 |377 |2808 |0 |0 |0 |0 |0 |0 |
|833 |0 |0 |312 |0 |0 |0 |0 |0 |
|717 |64 |34 |0 |0 |0 |0 |0 |0 |
|550 |39 |7 |0 |0 |0 |0 |0 |0 |
|215 |41 |7 |0 |0 |0 |294 |0 |0 |
|491 |0 |0 |0 |0 |0 |0 |0 |0 |
|365 |35 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/strategies/deepDiff/arrayDiff/diff/lcs.js | 129 |
| test/wrapStore.test.js | 66 |
| test/Store.test.js | 60 |
| test/arrayDiff/same.test.js | 43 |
| test/applyMiddleware.test.js | 40 |
| test/deepDiff.test.js | 32 |
| test/arrayDiff/apply.test.js | 32 |
| src/serialization.js | 27 |
| src/strategies/deepDiff/arrayDiff/diff/patch.js | 25 |
| test/serialization.test.js | 22 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 312}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4770612684654185, "precision": 0.48899060063006766, "recall": 0.46720510763605044, "support": 12809}, "micro avg": {"f1-score": 0.9501912717620423, "precision": 0.9501912717620423, "recall": 0.9501912717620423, "support": 12809}, "weighted avg": {"f1-score": 0.9427788801339765, "precision": 0.9378494292977609, "recall": 0.9501912717620423, "support": 12809}, "\u2205": {"f1-score": 0.9675174013921114, "precision": 0.9402985074626866, "recall": 0.9963590852201616, "support": 8789}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9245283018867925, "precision": 1.0, "recall": 0.8596491228070176, "support": 342}, "\u2423": {"f1-score": 0.9244444444444444, "precision": 0.9716262975778547, "recall": 0.8816326530612245, "support": 3185}},
  "cl_report_full": {"\"": {"f1-score": 0.428277282086479, "precision": 1.0, "recall": 0.2724890829694323, "support": 1145}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 491}, "macro avg": {"f1-score": 0.3493944942599766, "precision": 0.48899060063006766, "recall": 0.2922183811029035, "support": 17997}, "micro avg": {"f1-score": 0.7901707459585795, "precision": 0.9501912717620423, "recall": 0.6762793798966494, "support": 17997}, "weighted avg": {"f1-score": 0.7265585421716045, "precision": 0.8339565686650863, "recall": 0.6762793798966494, "support": 17997}, "\u2205": {"f1-score": 0.9475221813460291, "precision": 0.9402985074626866, "recall": 0.9548577036310107, "support": 9171}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 815}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 402}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 596}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.690951821386604, "precision": 1.0, "recall": 0.5278276481149012, "support": 557}, "\u2423": {"f1-score": 0.7284046692607005, "precision": 0.9716262975778547, "recall": 0.5825726141078839, "support": 4820}},
  "ppcr": 0.7117297327332334
}
```
</details>
