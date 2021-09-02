# Test report for javascript / file:///tmp/top-repos-quality-repos-rv0csyb4/cuckoo.git HEAD 50452a39ff7c3e0c4c94d114bc6317101633b958

### Classification report

PPCR: 0.910

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.981| 0.968| 0.978| 0.972| 12183| 12342| 0.987 |
| `␣` | 0.903| 0.968| 0.951| 0.934| 0.926| 4778| 4866| 0.982 |
| `⏎` | 0.865| 0.943| 0.933| 0.902| 0.898| 1287| 1301| 0.989 |
| `⏎⏎` | 0.849| 0.612| 0.534| 0.711| 0.656| 636| 728| 0.874 |
| `'` | 0.948| 0.773| 0.320| 0.852| 0.479| 260| 628| 0.414 |
| `"` | 0.857| 0.455| 0.148| 0.595| 0.252| 145| 446| 0.325 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.881| 0.993| 0.765| 0.933| 0.819| 141| 183| 0.770 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 80| 215| 0.372 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 76| 291| 0.261 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 69| 79| 0.873 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 139| 0.137 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 164| 0.073 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 145| 0.062 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 38| 0.105 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 41| 0.024 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 40| 0.000 |
| `macro avg` | 0.392| 0.358| 0.289| 0.369| 0.313| 19700| 21646| 0.910 |
| `weighted avg` | 0.931| 0.944| 0.859| 0.936| 0.864| 19700| 21646| 0.910 |
| `micro avg` | 0.944| 0.944| 0.859| 0.944| 0.899| 19700| 21646| 0.910 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁻| ⏎⇥⁺| ␣␣| ⏎⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|159 |11953 |221 |0 |9 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|88 |146 |4627 |0 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|14 |44 |29 |1214 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|92 |27 |54 |166 |389 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|368 |15 |33 |0 |0 |201 |11 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|301 |36 |27 |5 |0 |11 |66 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|215 |25 |41 |0 |0 |0 |0 |0 |0 |0 |10 |0 |0 |0 |0 |0 |0 |
|152 |3 |2 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|135 |0 |18 |11 |51 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|42 |0 |1 |0 |0 |0 |0 |0 |0 |0 |140 |0 |0 |0 |0 |0 |0 |
|40 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|40 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|136 |8 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|120 |6 |4 |0 |0 |0 |0 |0 |0 |0 |9 |0 |0 |0 |0 |0 |0 |
|10 |2 |67 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|34 |0 |0 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.5945945945945945, "precision": 0.8571428571428571, "recall": 0.45517241379310347, "support": 145}, "\u0027": {"f1-score": 0.8516949152542372, "precision": 0.9481132075471698, "recall": 0.7730769230769231, "support": 260}, "macro avg": {"f1-score": 0.36908351227293024, "precision": 0.3923229738489967, "recall": 0.35784933468455704, "support": 19700}, "micro avg": {"f1-score": 0.9436548223350254, "precision": 0.9436548223350254, "recall": 0.9436548223350254, "support": 19700}, "weighted avg": {"f1-score": 0.935560719947431, "precision": 0.930698833024497, "recall": 0.9436548223350254, "support": 19700}, "\u2205": {"f1-score": 0.9778304973821988, "precision": 0.9745617611088463, "recall": 0.9811212345071001, "support": 12183}, "\u23ce": {"f1-score": 0.9022668153102936, "precision": 0.8646723646723646, "recall": 0.9432789432789432, "support": 1287}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.7111517367458867, "precision": 0.8493449781659389, "recall": 0.6116352201257862, "support": 636}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9333333333333335, "precision": 0.8805031446540881, "recall": 0.9929078014184397, "support": 141}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 80}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.9344643037463396, "precision": 0.902829268292683, "recall": 0.9683968187526162, "support": 4778}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 69}},
  "cl_report_full": {"\"": {"f1-score": 0.2523900573613767, "precision": 0.8571428571428571, "recall": 0.14798206278026907, "support": 446}, "\u0027": {"f1-score": 0.47857142857142854, "precision": 0.9481132075471698, "recall": 0.3200636942675159, "support": 628}, "macro avg": {"f1-score": 0.3125627668497828, "precision": 0.3923229738489967, "recall": 0.2887442119878727, "support": 21646}, "micro avg": {"f1-score": 0.8992405553136943, "precision": 0.9436548223350254, "recall": 0.858819181373002, "support": 21646}, "weighted avg": {"f1-score": 0.8641652629243852, "precision": 0.8917723020307752, "recall": 0.858819181373002, "support": 21646}, "\u2205": {"f1-score": 0.9715121713333604, "precision": 0.9745617611088463, "recall": 0.9684816075190407, "support": 12342}, "\u23ce": {"f1-score": 0.8975970425138632, "precision": 0.8646723646723646, "recall": 0.9331283627978478, "support": 1301}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 139}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 145}, "\u23ce\u23ce": {"f1-score": 0.6559865092748736, "precision": 0.8493449781659389, "recall": 0.5343406593406593, "support": 728}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8187134502923977, "precision": 0.8805031446540881, "recall": 0.7650273224043715, "support": 183}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 215}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 291}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "\u2423": {"f1-score": 0.9262336102492243, "precision": 0.902829268292683, "recall": 0.9508836826962598, "support": 4866}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 79}},
  "ppcr": 0.9100988635313684
}
```
</details>