# Train report for javascript / file:///tmp/top-repos-quality-repos-aqu8kliu/sgtk.git HEAD 1ab078d4c33e29143ee7c699420f300e8836cec5

### Classification report

PPCR: 0.874

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.998| 0.985| 0.964| 0.991| 0.981| 50319| 51395| 0.979 |
| `␣` | 0.964| 0.997| 0.917| 0.980| 0.940| 21711| 23608| 0.920 |
| `"` | 1.000| 1.000| 0.651| 1.000| 0.788| 6317| 9710| 0.651 |
| `⏎` | 0.993| 0.959| 0.507| 0.976| 0.672| 3342| 6320| 0.529 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.937| 0.968| 0.939| 0.953| 0.938| 2500| 2579| 0.969 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.989| 0.984| 0.936| 0.987| 0.962| 2347| 2467| 0.951 |
| `'` | 1.000| 1.000| 0.095| 1.000| 0.173| 153| 1616| 0.095 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 1331| 0.006 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 115| 0.000 |
| `macro avg` | 0.765| 0.766| 0.556| 0.765| 0.606| 86697| 99141| 0.874 |
| `micro avg` | 0.987| 0.987| 0.863| 0.987| 0.921| 86697| 99141| 0.874 |
| `weighted avg` | 0.988| 0.987| 0.863| 0.987| 0.904| 86697| 99141| 0.874 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎⏎| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1076 |49545 |668 |0 |4 |81 |21 |0 |0 |0 |
|1897 |29 |21650 |0 |4 |28 |0 |0 |0 |0 |
|3393 |0 |0 |6317 |0 |0 |0 |0 |0 |0 |
|2978 |26 |56 |0 |3206 |50 |4 |0 |0 |0 |
|79 |12 |58 |0 |9 |2421 |0 |0 |0 |0 |
|120 |13 |22 |0 |0 |3 |2309 |0 |0 |0 |
|1463 |0 |0 |0 |0 |0 |0 |153 |0 |0 |
|1323 |4 |0 |0 |4 |0 |0 |0 |0 |0 |
|115 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| UI/scripts/handleAlongChromosomes.js | 101 |
| resources/E.coli/scripts/handleAlongChromosomes.js | 87 |
| resources/examples/S.smargdinae_scaff_mp/scripts/handleAlongChromosomes.js | 87 |
| resources/examples/S.smaragdinae_GFA_PE_MP/scripts/handleAlongChromosomes.js | 87 |
| resources/E.coli/scripts/freeLayout.js | 54 |
| resources/examples/S.smargdinae_scaff_mp/scripts/freeLayout.js | 54 |
| resources/examples/S.smaragdinae_GFA_PE_MP/scripts/freeLayout.js | 54 |
| UI/scripts/zoomChoose.js | 48 |
| resources/examples/S.smaragdinae_GFA_PE_MP/scripts/script.js | 40 |
| resources/examples/S.smargdinae_scaff_mp/scripts/script.js | 40 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6317}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 153}, "macro avg": {"f1-score": 0.765228682948914, "precision": 0.7647293684236952, "recall": 0.765924824839129, "support": 86697}, "micro avg": {"f1-score": 0.9873582707590804, "precision": 0.9873582707590804, "recall": 0.9873582707590804, "support": 86697}, "weighted avg": {"f1-score": 0.987367732735912, "precision": 0.9876091946423985, "recall": 0.9873582707590804, "support": 86697}, "\u2205": {"f1-score": 0.9914155360787611, "precision": 0.9983074412138064, "recall": 0.9846181362904668, "support": 50319}, "\u23ce": {"f1-score": 0.9760998629928452, "precision": 0.9934924078091106, "recall": 0.9593058049072412, "support": 3342}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9525870548888452, "precision": 0.9372822299651568, "recall": 0.9684, "support": 2500}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9865413373210852, "precision": 0.9892887746358183, "recall": 0.9838091180230081, "support": 2347}, "\u2423": {"f1-score": 0.980414355258689, "precision": 0.964193462189365, "recall": 0.9971903643314449, "support": 21711}},
  "cl_report_full": {"\"": {"f1-score": 0.7882947526049792, "precision": 1.0, "recall": 0.6505664263645726, "support": 9710}, "\u0027": {"f1-score": 0.1729790842283776, "precision": 1.0, "recall": 0.09467821782178218, "support": 1616}, "macro avg": {"f1-score": 0.6059648078903902, "precision": 0.7647293684236952, "recall": 0.5564755515046755, "support": 99141}, "micro avg": {"f1-score": 0.9212432333537812, "precision": 0.9873582707590804, "recall": 0.8634268365257562, "support": 99141}, "weighted avg": {"f1-score": 0.9035023885696732, "precision": 0.9736979503132951, "recall": 0.8634268365257562, "support": 99141}, "\u2205": {"f1-score": 0.9808560342096928, "precision": 0.9983074412138064, "recall": 0.9640042805720401, "support": 51395}, "\u23ce": {"f1-score": 0.6716245941133342, "precision": 0.9934924078091106, "recall": 0.5072784810126583, "support": 6320}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1331}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9380085238279736, "precision": 0.9372822299651568, "recall": 0.9387359441644048, "support": 2579}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9618829410539471, "precision": 0.9892887746358183, "recall": 0.9359546007296311, "support": 2467}, "\u2423": {"f1-score": 0.9400373409752073, "precision": 0.964193462189365, "recall": 0.9170620128769909, "support": 23608}},
  "ppcr": 0.874481798650407
}
```
</details>
