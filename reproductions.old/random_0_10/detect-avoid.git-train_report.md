# Train report for javascript / file:///tmp/top-repos-quality-repos-971ma9tt/detect-avoid.git HEAD 79d8021b942b6746e42b12039dfc04d30cb55e2e

### Classification report

PPCR: 0.817

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.989| 0.920| 0.983| 0.948| 247261| 265831| 0.930 |
| `␣` | 0.961| 0.977| 0.892| 0.969| 0.925| 127066| 139122| 0.913 |
| `'` | 0.999| 1.000| 0.904| 0.999| 0.949| 18604| 20591| 0.904 |
| `⏎` | 0.945| 0.863| 0.419| 0.902| 0.580| 18093| 37290| 0.485 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.933| 0.818| 0.292| 0.872| 0.445| 6069| 16986| 0.357 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.956| 0.204| 0.025| 0.336| 0.048| 2449| 20190| 0.121 |
| `⏎⏎` | 0.956| 0.475| 0.077| 0.635| 0.143| 1683| 10331| 0.163 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.320| 0.442| 0.090| 0.371| 0.140| 532| 2623| 0.203 |
| `⏎⇥⁻` | 0.961| 0.947| 0.239| 0.954| 0.383| 208| 823| 0.253 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 129| 635| 0.203 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 45| 591| 0.076 |
| `"` | 1.000| 0.524| 0.210| 0.688| 0.346| 42| 105| 0.400 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 916| 0.045 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 835| 0.013 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 135| 0.059 |
| `macro avg` | 0.601| 0.483| 0.271| 0.514| 0.327| 422241| 517004| 0.817 |
| `weighted avg` | 0.970| 0.970| 0.792| 0.968| 0.837| 422241| 517004| 0.817 |
| `micro avg` | 0.970| 0.970| 0.792| 0.970| 0.872| 422241| 517004| 0.817 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⏎⏎| "| ⏎⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|18570 |244596 |2644 |10 |0 |0 |5 |5 |1 |0 |0 |0 |0 |0 |0 |0 |
|12056 |2389 |124113 |79 |0 |0 |0 |0 |485 |0 |0 |0 |0 |0 |0 |0 |
|19197 |889 |1548 |15608 |0 |23 |3 |12 |10 |0 |0 |0 |0 |0 |0 |0 |
|1987 |0 |0 |0 |18604 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|17741 |1082 |706 |162 |0 |499 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|10917 |989 |42 |74 |0 |0 |4964 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8648 |345 |11 |524 |0 |0 |0 |800 |3 |0 |0 |0 |0 |0 |0 |0 |
|2091 |2 |10 |20 |0 |0 |265 |0 |235 |0 |0 |0 |0 |0 |0 |0 |
|875 |7 |31 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|615 |11 |0 |0 |0 |0 |0 |0 |0 |0 |197 |0 |0 |0 |0 |0 |
|824 |1 |0 |9 |0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|506 |43 |0 |1 |0 |0 |85 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|546 |6 |1 |19 |0 |0 |0 |19 |0 |0 |0 |0 |0 |0 |0 |0 |
|63 |0 |0 |0 |20 |0 |0 |0 |0 |0 |0 |0 |0 |0 |22 |0 |
|127 |0 |0 |0 |0 |0 |0 |0 |0 |0 |8 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| backup/uRad-Monitor/templates/assets/javascripts/highchart/modules/stock.src.js | 863 |
| backup/uRad-Monitor/templates/assets/javascripts/highchart/lib/canvg.src.js | 818 |
| backup/uRad-Monitor/templates/assets/javascripts/highchart/modules/annotations-advanced.src.js | 686 |
| backup/uRad-Monitor/templates/assets/javascripts/highchart/modules/gantt.src.js | 538 |
| backup/uRad-Monitor/templates/assets/javascripts/highchart/modules/annotations.src.js | 472 |
| backup/uRad-Monitor/templates/assets/javascripts/highchart/highcharts-3d.src.js | 463 |
| backup/uRad-Monitor/templates/assets/javascripts/highchart/es-modules/parts/Axis.js | 433 |
| backup/uRad-Monitor/templates/assets/javascripts/highchart/es-modules/parts/Series.js | 287 |
| backup/uRad-Monitor/templates/assets/javascripts/highchart/modules/accessibility.src.js | 263 |
| backup/uRad-Monitor/templates/assets/javascripts/highchart/modules/sunburst.src.js | 232 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.6875000000000001, "precision": 1.0, "recall": 0.5238095238095238, "support": 42}, "\u0027": {"f1-score": 0.999462769958096, "precision": 0.998926116838488, "recall": 1.0, "support": 18604}, "macro avg": {"f1-score": 0.5139195507233497, "precision": 0.6005505123080531, "recall": 0.48255438527380246, "support": 422241}, "micro avg": {"f1-score": 0.9701521169190107, "precision": 0.9701521169190107, "recall": 0.9701521169190107, "support": 422241}, "weighted avg": {"f1-score": 0.9679767067644166, "precision": 0.9696653604098271, "recall": 0.9701521169190107, "support": 422241}, "\u2205": {"f1-score": 0.9830614061705596, "precision": 0.9769771528998242, "recall": 0.9892219153040714, "support": 247261}, "\u23ce": {"f1-score": 0.9021443847176465, "precision": 0.945423708280332, "recall": 0.8626540651080529, "support": 18093}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u21e5\u207b": {"f1-score": 0.9539951573849879, "precision": 0.9609756097560975, "recall": 0.9471153846153846, "support": 208}, "\u23ce\u23ce": {"f1-score": 0.634920634920635, "precision": 0.955794504181601, "recall": 0.47534165181224003, "support": 1683}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.37124802527646134, "precision": 0.3201634877384196, "recall": 0.4417293233082707, "support": 532}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.3359138337260182, "precision": 0.9559386973180076, "recall": 0.20375663536137198, "support": 2449}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8715652708278465, "precision": 0.9327320556181886, "recall": 0.8179271708683473, "support": 6069}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 129}, "\u2423": {"f1-score": 0.968981777867995, "precision": 0.9613263519898378, "recall": 0.9767601089197739, "support": 127066}},
  "cl_report_full": {"\"": {"f1-score": 0.3464566929133859, "precision": 1.0, "recall": 0.20952380952380953, "support": 105}, "\u0027": {"f1-score": 0.9488206043605762, "precision": 0.998926116838488, "recall": 0.9035015297945704, "support": 20591}, "macro avg": {"f1-score": 0.32722690939332805, "precision": 0.6005505123080531, "recall": 0.27114468244886825, "support": 517004}, "micro avg": {"f1-score": 0.8722708132595862, "precision": 0.9701521169190107, "recall": 0.7923304268438929, "support": 517004}, "weighted avg": {"f1-score": 0.8367053191015474, "precision": 0.9594315900865114, "recall": 0.7923304268438929, "support": 517004}, "\u2205": {"f1-score": 0.9476957172829438, "precision": 0.9769771528998242, "recall": 0.9201184211021288, "support": 265831}, "\u23ce": {"f1-score": 0.5802338333426271, "precision": 0.945423708280332, "recall": 0.41855725395548404, "support": 37290}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 916}, "\u23ce\u21e5\u207b": {"f1-score": 0.38326848249027234, "precision": 0.9609756097560975, "recall": 0.2393681652490887, "support": 823}, "\u23ce\u23ce": {"f1-score": 0.14326647564469913, "precision": 0.955794504181601, "recall": 0.07743684057690446, "support": 10331}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 135}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 591}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 835}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.1400059577003277, "precision": 0.3201634877384196, "recall": 0.08959207014868471, "support": 2623}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.048184627269215914, "precision": 0.9559386973180076, "recall": 0.024715205547300643, "support": 20190}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.4450421373498296, "precision": 0.9327320556181886, "recall": 0.2922406687860591, "support": 16986}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 635}, "\u2423": {"f1-score": 0.9254291125460429, "precision": 0.9613263519898378, "recall": 0.892116272048993, "support": 139122}},
  "ppcr": 0.8167074142559826
}
```
</details>
