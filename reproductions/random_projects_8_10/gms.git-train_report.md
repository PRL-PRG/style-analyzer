# Train report for javascript / file:///tmp/top-repos-quality-repos-jt8edkz7/gms.git HEAD 7fceb57ebaeb78e497572dc3686545aff2920365

### Classification report

PPCR: 0.879

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.942| 0.993| 0.978| 0.967| 0.960| 45549| 46234| 0.985 |
| `␣` | 0.970| 0.901| 0.801| 0.934| 0.877| 15057| 16923| 0.890 |
| `"` | 0.980| 0.999| 0.959| 0.989| 0.969| 11151| 11624| 0.959 |
| `⏎` | 0.942| 0.801| 0.491| 0.866| 0.645| 3181| 5192| 0.613 |
| `'` | 0.997| 0.886| 0.458| 0.938| 0.628| 2028| 3919| 0.517 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 309| 1160| 0.266 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 208| 932| 0.223 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 147| 707| 0.208 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 128| 1066| 0.120 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 127| 723| 0.176 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 103| 0.097 |
| `macro avg` | 0.439| 0.416| 0.335| 0.427| 0.371| 77895| 88583| 0.879 |
| `micro avg` | 0.954| 0.954| 0.839| 0.954| 0.892| 77895| 88583| 0.879 |
| `weighted avg` | 0.943| 0.954| 0.839| 0.947| 0.861| 77895| 88583| 0.879 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|685 |45236 |298 |0 |15 |0 |0 |0 |0 |0 |0 |0 |
|1866 |1451 |13560 |0 |46 |0 |0 |0 |0 |0 |0 |0 |
|473 |0 |0 |11145 |0 |6 |0 |0 |0 |0 |0 |0 |
|2011 |551 |81 |0 |2549 |0 |0 |0 |0 |0 |0 |0 |
|1891 |0 |0 |232 |0 |1796 |0 |0 |0 |0 |0 |0 |
|851 |267 |25 |0 |17 |0 |0 |0 |0 |0 |0 |0 |
|938 |97 |0 |0 |31 |0 |0 |0 |0 |0 |0 |0 |
|724 |171 |5 |0 |32 |0 |0 |0 |0 |0 |0 |0 |
|560 |129 |2 |0 |16 |0 |0 |0 |0 |0 |0 |0 |
|596 |115 |11 |0 |1 |0 |0 |0 |0 |0 |0 |0 |
|93 |7 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| device/web/static/underscore/underscore.js | 433 |
| device/web/static/ngmap/testapp/scripts/markerclusterer.js | 278 |
| device/web/static/ngmap/config/jsdoc/template/publish.js | 219 |
| device/web/static/admin/js/admin/DateTimeShortcuts.js | 176 |
| device/web/static/gis/js/OLMapWidget.js | 173 |
| device/web/static/admin/js/SelectFilter2.js | 129 |
| device/web/static/ngmap/gulpfile.js | 124 |
| device/web/static/admin/js/calendar.js | 116 |
| device/web/static/admin/js/core.js | 111 |
| device/web/static/ngmap/services/attr2-map-options.js | 108 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9894353693181818, "precision": 0.9796079810143271, "recall": 0.9994619316653215, "support": 11151}, "\u0027": {"f1-score": 0.9378590078328981, "precision": 0.9966703662597114, "recall": 0.8856015779092702, "support": 2028}, "macro avg": {"f1-score": 0.4267093877164664, "precision": 0.43904246742565467, "recall": 0.4163718120790269, "support": 77895}, "micro avg": {"f1-score": 0.9536683997689197, "precision": 0.9536683997689197, "recall": 0.9536683997689197, "support": 77895}, "weighted avg": {"f1-score": 0.9472932978718497, "precision": 0.9428627857243068, "recall": 0.9536683997689197, "support": 77895}, "\u2205": {"f1-score": 0.9668600985326964, "precision": 0.9419456938197568, "recall": 0.9931282794353333, "support": 45549}, "\u23ce": {"f1-score": 0.8658288043478262, "precision": 0.9416328038418914, "recall": 0.8013203395158756, "support": 3181}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 208}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 309}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 127}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 147}, "\u2423": {"f1-score": 0.9338199848495282, "precision": 0.9696102967465141, "recall": 0.9005778043434948, "support": 15057}},
  "cl_report_full": {"\"": {"f1-score": 0.9690883005086735, "precision": 0.9796079810143271, "recall": 0.958792154163799, "support": 11624}, "\u0027": {"f1-score": 0.6278622618423353, "precision": 0.9966703662597114, "recall": 0.45828017351365147, "support": 3919}, "macro avg": {"f1-score": 0.3708750086691843, "precision": 0.43904246742565467, "recall": 0.3352464057192323, "support": 88583}, "micro avg": {"f1-score": 0.8924422446209109, "precision": 0.9536683997689197, "recall": 0.8386033437567028, "support": 88583}, "weighted avg": {"f1-score": 0.8613627227895831, "precision": 0.9046939493371566, "recall": 0.8386033437567028, "support": 88583}, "\u2205": {"f1-score": 0.9598336480723122, "precision": 0.9419456938197568, "recall": 0.9784141540857377, "support": 46234}, "\u23ce": {"f1-score": 0.6453981516647677, "precision": 0.9416328038418914, "recall": 0.4909476117103236, "support": 5192}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 932}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 103}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1160}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 723}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1066}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 707}, "\u2423": {"f1-score": 0.8774427332729391, "precision": 0.9696102967465141, "recall": 0.8012763694380429, "support": 16923}},
  "ppcr": 0.8793447952767461
}
```
</details>
