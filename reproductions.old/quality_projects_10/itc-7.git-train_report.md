# Train report for javascript / file:///tmp/top-repos-quality-repos-pl9gcvp7/itc-7.git HEAD d2bfcdc3a1faea4cdc1d7e555404445ac8768c66

### Classification report

PPCR: 0.766

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 0.985| 0.916| 0.982| 0.946| 149245| 160463| 0.930 |
| `␣` | 0.935| 0.950| 0.759| 0.942| 0.838| 53729| 67220| 0.799 |
| `⏎` | 0.941| 0.844| 0.383| 0.890| 0.544| 7552| 16663| 0.453 |
| `"` | 1.000| 1.000| 0.211| 1.000| 0.349| 1421| 6727| 0.211 |
| `⏎⇥⁺` | 0.960| 0.343| 0.034| 0.505| 0.065| 420| 4272| 0.098 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 293| 4421| 0.066 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 204| 1426| 0.143 |
| `'` | 1.000| 1.000| 0.021| 1.000| 0.042| 168| 7907| 0.021 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 163| 1567| 0.104 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 95| 3871| 0.025 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 57| 1349| 0.042 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 357| 0.048 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 1384| 0.007 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 218| 0.014 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 424| 0.002 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 150| 0.007 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 282| 0.000 |
| `macro avg` | 0.342| 0.301| 0.137| 0.313| 0.164| 213378| 278701| 0.766 |
| `micro avg` | 0.966| 0.966| 0.740| 0.966| 0.838| 213378| 278701| 0.766 |
| `weighted avg` | 0.962| 0.966| 0.740| 0.964| 0.790| 213378| 278701| 0.766 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⇥⁺| ⏎⏎| ⏎⇥⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⇥⁻| ⏎⏎⏎| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|11218 |147004 |2157 |81 |0 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|13491 |2651 |51032 |46 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|9111 |367 |810 |6375 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|7739 |0 |0 |0 |168 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5306 |0 |0 |0 |0 |1421 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3852 |78 |195 |3 |0 |0 |144 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4128 |34 |32 |227 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3776 |61 |23 |11 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1404 |52 |99 |10 |0 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1375 |6 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1222 |15 |187 |1 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1292 |9 |44 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|423 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|340 |3 |0 |14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|282 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|215 |0 |1 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|149 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Meruzh_Janoyan/Homeworks/angularJs/public/js/lib/src/Chart.Core.js | 423 |
| Gevor_Vardanyan/Homeworks/AngularJs/public/js/lib/src/Chart.Core.js | 423 |
| Hakob_Torosyan/Vagharsh_Hakob_hdm_selenium/public/js/lib/src/Chart.Core.js | 423 |
| Gevor_Vardanyan/Homeworks/AngularJs/js/lib/src/Chart.Core.js | 423 |
| Hakob_Torosyan/Selenium/Vagharsh_Hakob_hdm_selenium/public/js/lib/src/Chart.Core.js | 423 |
| Ani_Saghatelyan/Homework/angular/public/js/lib/src/Chart.Core.js | 423 |
| Ani_Saghatelyan/Homework/sphinx/ITC7/itc7build/html/_static/underscore-1.3.1.js | 169 |
| Smbat_Sargsyan/homework/Sphinx/New_Summary_Sphinx/_build/html/_static/underscore-1.3.1.js | 169 |
| Araksya_Hambaryan/Homework/Sphinx/16_02_2016/Araksbuild/html/_static/underscore-1.3.1.js | 169 |
| Ani_Saghatelyan/Homework/sphinx/ITC7/itc7build/html/_static/searchtools.js | 120 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1421}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 168}, "macro avg": {"f1-score": 0.3128778477973803, "precision": 0.34198421290571124, "recall": 0.301281909083706, "support": 213378}, "micro avg": {"f1-score": 0.9660977232891863, "precision": 0.9660977232891863, "recall": 0.9660977232891863, "support": 213378}, "weighted avg": {"f1-score": 0.963769073552901, "precision": 0.9622458300959438, "recall": 0.9660977232891863, "support": 213378}, "\u2205": {"f1-score": 0.9815775592102188, "precision": 0.9781941828973723, "recall": 0.984984421588663, "support": 149245}, "\u23ce": {"f1-score": 0.8897418004187021, "precision": 0.9405429330185896, "recall": 0.8441472457627118, "support": 7552}, "\u23ce\u21e5\u207a": {"f1-score": 0.5052631578947367, "precision": 0.96, "recall": 0.34285714285714286, "support": 420}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 293}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 204}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u2423": {"f1-score": 0.9423408950318072, "precision": 0.9349945034811287, "recall": 0.9498036442144838, "support": 53729}},
  "cl_report_full": {"\"": {"f1-score": 0.3487972508591065, "precision": 1.0, "recall": 0.21123829344432882, "support": 6727}, "\u0027": {"f1-score": 0.04160990712074303, "precision": 1.0, "recall": 0.021246996332363727, "support": 7907}, "macro avg": {"f1-score": 0.16373887476283033, "precision": 0.34198421290571124, "recall": 0.13671059444491335, "support": 278701}, "micro avg": {"f1-score": 0.8378492071395041, "precision": 0.9660977232891863, "recall": 0.7396600658052895, "support": 278701}, "weighted avg": {"f1-score": 0.7899708264040972, "precision": 0.9121664098340202, "recall": 0.7396600658052895, "support": 278701}, "\u2205": {"f1-score": 0.9461421620369178, "precision": 0.9781941828973723, "recall": 0.9161239662725986, "support": 160463}, "\u23ce": {"f1-score": 0.5439187747962971, "precision": 0.9405429330185896, "recall": 0.3825841685170738, "support": 16663}, "\u23ce\u21e5\u207a": {"f1-score": 0.06512890094979647, "precision": 0.96, "recall": 0.033707865168539325, "support": 4272}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3871}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4421}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 150}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 424}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 357}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 218}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 282}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1567}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1426}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1384}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1349}, "\u2423": {"f1-score": 0.8379638752052545, "precision": 0.9349945034811287, "recall": 0.7591788158286225, "support": 67220}},
  "ppcr": 0.7656161980043129
}
```
</details>
