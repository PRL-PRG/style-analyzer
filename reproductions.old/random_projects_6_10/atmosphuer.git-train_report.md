# Train report for javascript / file:///tmp/top-repos-quality-repos-q2j8j72f/atmosphuer.git HEAD 97851fb2aa9e5ca9638c391c8bf288a439c83bdd

### Classification report

PPCR: 0.913

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.955| 0.994| 0.975| 0.974| 0.965| 12275| 12511| 0.981 |
| `␣` | 0.972| 0.924| 0.799| 0.948| 0.877| 5981| 6919| 0.864 |
| `'` | 0.977| 1.000| 0.962| 0.988| 0.970| 3648| 3791| 0.962 |
| `⏎` | 0.977| 0.871| 0.538| 0.921| 0.693| 1577| 2554| 0.617 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.960| 0.892| 0.865| 0.925| 0.910| 676| 697| 0.970 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.977| 0.984| 0.914| 0.980| 0.944| 612| 659| 0.929 |
| `"` | 1.000| 0.157| 0.151| 0.271| 0.262| 102| 106| 0.962 |
| `micro avg` | 0.964| 0.964| 0.880| 0.964| 0.920| 24871| 27237| 0.913 |
| `weighted avg` | 0.964| 0.964| 0.880| 0.962| 0.913| 24871| 27237| 0.913 |
| `macro avg` | 0.974| 0.832| 0.743| 0.858| 0.803| 24871| 27237| 0.913 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|236 |12201 |37 |0 |0 |23 |14 |0 |
|938 |417 |5529 |0 |33 |2 |0 |0 |
|143 |0 |0 |3648 |0 |0 |0 |0 |
|977 |128 |76 |0 |1373 |0 |0 |0 |
|21 |35 |38 |0 |0 |603 |0 |0 |
|47 |1 |9 |0 |0 |0 |602 |0 |
|4 |0 |0 |86 |0 |0 |0 |16 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/js/fileinput.js | 791 |
| static/js/index.js | 58 |
| static/js/fileinput_locale_zh-TW.js | 5 |
| static/js/fileinput_locale_ja.js | 5 |
| static/js/fileinput_locale_zh.js | 4 |
| static/js/fileinput_locale_es.js | 2 |
| static/js/fileinput_locale_ca.js | 2 |
| static/js/fileinput_locale_ro.js | 2 |
| static/js/fileinput_locale_ar.js | 2 |
| static/js/fileinput_locale_bg.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.2711864406779661, "precision": 1.0, "recall": 0.1568627450980392, "support": 102}, "\u0027": {"f1-score": 0.9883500406393932, "precision": 0.9769683985002678, "recall": 1.0, "support": 3648}, "macro avg": {"f1-score": 0.8581151937100191, "precision": 0.9739117675951946, "recall": 0.831653429535247, "support": 24871}, "micro avg": {"f1-score": 0.963853483977323, "precision": 0.963853483977323, "recall": 0.963853483977323, "support": 24871}, "weighted avg": {"f1-score": 0.9622280734445389, "precision": 0.9642949780959192, "recall": 0.963853483977323, "support": 24871}, "\u2205": {"f1-score": 0.9738596001117453, "precision": 0.9545454545454546, "recall": 0.9939714867617108, "support": 12275}, "\u23ce": {"f1-score": 0.9205497820985585, "precision": 0.9765291607396871, "recall": 0.8706404565630945, "support": 1577}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.924846625766871, "precision": 0.9601910828025477, "recall": 0.8920118343195266, "support": 676}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9804560260586319, "precision": 0.9772727272727273, "recall": 0.9836601307189542, "support": 612}, "\u2423": {"f1-score": 0.9475578406169667, "precision": 0.9718755493056777, "recall": 0.9244273532854038, "support": 5981}},
  "cl_report_full": {"\"": {"f1-score": 0.26229508196721313, "precision": 1.0, "recall": 0.1509433962264151, "support": 106}, "\u0027": {"f1-score": 0.9695681063122923, "precision": 0.9769683985002678, "recall": 0.962279082036402, "support": 3791}, "macro avg": {"f1-score": 0.8030907116067344, "precision": 0.9739117675951946, "recall": 0.7433968437753344, "support": 27237}, "micro avg": {"f1-score": 0.920089045827896, "precision": 0.963853483977323, "recall": 0.8801262987847414, "support": 27237}, "weighted avg": {"f1-score": 0.9130902102639367, "precision": 0.9650014206665178, "recall": 0.8801262987847414, "support": 27237}, "\u2205": {"f1-score": 0.9647728620566955, "precision": 0.9545454545454546, "recall": 0.9752218048117657, "support": 12511}, "\u23ce": {"f1-score": 0.6934343434343434, "precision": 0.9765291607396871, "recall": 0.5375880971025842, "support": 2554}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9101886792452831, "precision": 0.9601910828025477, "recall": 0.8651362984218077, "support": 697}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9443137254901961, "precision": 0.9772727272727273, "recall": 0.91350531107739, "support": 659}, "\u2423": {"f1-score": 0.8770621827411167, "precision": 0.9718755493056777, "recall": 0.7991039167509756, "support": 6919}},
  "ppcr": 0.9131328707273194
}
```
</details>
