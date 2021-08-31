# Train report for javascript / file:///tmp/top-repos-quality-repos-muw4dbdq/shifty.git HEAD 520ba431598d3c8e662b9347804b95ddca482134

### Classification report

PPCR: 0.743

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.984| 0.995| 0.966| 0.990| 0.975| 6015| 6197| 0.971 |
| `␣` | 0.969| 0.980| 0.604| 0.974| 0.744| 2520| 4088| 0.616 |
| `⏎␣⁺␣⁺` | 0.957| 0.846| 0.704| 0.898| 0.811| 338| 406| 0.833 |
| `⏎␣⁻␣⁻` | 0.932| 0.848| 0.683| 0.888| 0.788| 290| 360| 0.806 |
| `'` | 1.000| 1.000| 0.487| 1.000| 0.655| 204| 419| 0.487 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 732| 0.041 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 453| 0.002 |
| `macro avg` | 0.692| 0.667| 0.492| 0.679| 0.568| 9398| 12655| 0.743 |
| `weighted avg` | 0.975| 0.978| 0.726| 0.976| 0.788| 9398| 12655| 0.743 |
| `micro avg` | 0.978| 0.978| 0.726| 0.978| 0.834| 9398| 12655| 0.743 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|182 |5987 |28 |0 |0 |0 |0 |0 |
|1568 |20 |2469 |0 |0 |0 |13 |18 |
|702 |4 |26 |0 |0 |0 |0 |0 |
|215 |0 |0 |0 |204 |0 |0 |0 |
|452 |1 |0 |0 |0 |0 |0 |0 |
|68 |38 |14 |0 |0 |0 |286 |0 |
|70 |32 |12 |0 |0 |0 |0 |246 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/token.js | 43 |
| src/token.test.js | 34 |
| src/easing-functions.js | 31 |
| src/tweenable.js | 25 |
| src/bezier.test.js | 20 |
| src/tweenable.test.js | 18 |
| webpack.config.js | 9 |
| src/interpolate.js | 6 |
| src/index.js | 6 |
| src/bezier.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 204}, "macro avg": {"f1-score": 0.6785763792088695, "precision": 0.6916193146077841, "recall": 0.6670766548415407, "support": 9398}, "micro avg": {"f1-score": 0.9780804426473718, "precision": 0.9780804426473718, "recall": 0.9780804426473718, "support": 9398}, "weighted avg": {"f1-score": 0.976140740670408, "precision": 0.9746210164021296, "recall": 0.9780804426473718, "support": 9398}, "\u2205": {"f1-score": 0.9898321897991238, "precision": 0.984380138112463, "recall": 0.9953449709060682, "support": 6015}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8979591836734695, "precision": 0.9565217391304348, "recall": 0.8461538461538461, "support": 338}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8880866425992779, "precision": 0.9318181818181818, "recall": 0.8482758620689655, "support": 290}, "\u2423": {"f1-score": 0.974156638390215, "precision": 0.9686151431934091, "recall": 0.9797619047619047, "support": 2520}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6548956661316212, "precision": 1.0, "recall": 0.48687350835322196, "support": 419}, "macro avg": {"f1-score": 0.5676966306164849, "precision": 0.6916193146077841, "recall": 0.4921022560533505, "support": 12655}, "micro avg": {"f1-score": 0.8336280778125426, "precision": 0.9780804426473718, "recall": 0.7263532200711181, "support": 12655}, "weighted avg": {"f1-score": 0.7880081095547282, "precision": 0.8852394146818721, "recall": 0.7263532200711181, "support": 12655}, "\u2205": {"f1-score": 0.9751608437169151, "precision": 0.984380138112463, "recall": 0.9661126351460384, "support": 6197}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 732}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 453}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8113475177304964, "precision": 0.9565217391304348, "recall": 0.7044334975369458, "support": 406}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7884615384615384, "precision": 0.9318181818181818, "recall": 0.6833333333333333, "support": 360}, "\u2423": {"f1-score": 0.7440108482748231, "precision": 0.9686151431934091, "recall": 0.6039628180039139, "support": 4088}},
  "ppcr": 0.7426313709996049
}
```
</details>
