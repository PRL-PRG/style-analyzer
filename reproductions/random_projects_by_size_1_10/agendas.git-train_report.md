# Train report for javascript / file:///tmp/top-repos-quality-repos-zqyoi1jt/agendas.git HEAD 31b24a2d6350605b638b59062f297ef3f58e9879

### Classification report

PPCR: 0.861

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.991| 0.972| 0.989| 0.979| 63548| 64805| 0.981 |
| `␣` | 0.954| 0.989| 0.881| 0.971| 0.916| 31976| 35887| 0.891 |
| `'` | 0.974| 1.000| 0.961| 0.987| 0.967| 14806| 15413| 0.961 |
| `⏎` | 0.955| 0.807| 0.416| 0.875| 0.579| 3469| 6730| 0.515 |
| `⏎⏎` | 0.941| 0.958| 0.434| 0.950| 0.594| 1807| 3989| 0.453 |
| `"` | 1.000| 0.010| 0.007| 0.020| 0.014| 400| 574| 0.697 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 323| 2136| 0.151 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 256| 2054| 0.125 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 226| 2028| 0.111 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 86| 97| 0.887 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 75| 1772| 0.042 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 356| 0.014 |
| `weighted avg` | 0.966| 0.974| 0.839| 0.968| 0.865| 116977| 135841| 0.861 |
| `macro avg` | 0.484| 0.396| 0.306| 0.399| 0.338| 116977| 135841| 0.861 |
| `micro avg` | 0.974| 0.974| 0.839| 0.974| 0.901| 116977| 135841| 0.861 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ␣␣| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1257 |63006 |534 |0 |0 |8 |0 |0 |0 |0 |0 |0 |0 |
|3911 |293 |31611 |0 |66 |6 |0 |0 |0 |0 |0 |0 |0 |
|607 |0 |0 |14806 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3261 |103 |478 |0 |2798 |90 |0 |0 |0 |0 |0 |0 |0 |
|2182 |4 |36 |0 |35 |1732 |0 |0 |0 |0 |0 |0 |0 |
|1802 |90 |134 |0 |0 |2 |0 |0 |0 |0 |0 |0 |0 |
|1813 |242 |62 |0 |17 |2 |0 |0 |0 |0 |0 |0 |0 |
|1798 |55 |197 |0 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|1697 |56 |10 |0 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|174 |0 |0 |396 |0 |0 |0 |0 |0 |0 |4 |0 |0 |
|351 |2 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|11 |18 |68 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| web/static/app/js/slick.js | 568 |
| web/static/app/plugins/select2/js/select2.full.js | 556 |
| web/static/app/js/map.js | 502 |
| web/static/app/plugins/moment/moment.js | 445 |
| web/static/app/plugins/select2/js/select2.js | 440 |
| web/static/app/plugins/daterangepicker/daterangepicker.js | 157 |
| web/static/app/plugins/fontawesome/js/conflict-detection.js | 134 |
| web/static/app/js/script.js | 69 |
| web/static/app/plugins/fullcalendar/jquery.fullcalendar.js | 55 |
| web/static/app/plugins/theia-sticky-sidebar/theia-sticky-sidebar.js | 27 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.019801980198019802, "precision": 1.0, "recall": 0.01, "support": 400}, "\u0027": {"f1-score": 0.9868035190615835, "precision": 0.9739507959479016, "recall": 1.0, "support": 14806}, "macro avg": {"f1-score": 0.3992555536460331, "precision": 0.4842568227787953, "recall": 0.3962602870729199, "support": 116977}, "micro avg": {"f1-score": 0.9741829590432307, "precision": 0.9741829590432307, "recall": 0.9741829590432307, "support": 116977}, "weighted avg": {"f1-score": 0.9682719908024036, "precision": 0.9662718846943092, "recall": 0.9741829590432307, "support": 116977}, "\u2205": {"f1-score": 0.9889732139353462, "precision": 0.9864879675585965, "recall": 0.9914710140366337, "support": 63548}, "\u23ce": {"f1-score": 0.8746483276023758, "precision": 0.9552748378286104, "recall": 0.8065724992793312, "support": 3469}, "\u23ce\u23ce": {"f1-score": 0.9498217713188921, "precision": 0.941304347826087, "recall": 0.9584947426674045, "support": 1807}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 226}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 256}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 323}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}, "\u2423": {"f1-score": 0.9710178316361793, "precision": 0.9540639241843479, "recall": 0.9885851888916688, "support": 31976}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}},
  "cl_report_full": {"\"": {"f1-score": 0.01384083044982699, "precision": 1.0, "recall": 0.006968641114982578, "support": 574}, "\u0027": {"f1-score": 0.9672382818879635, "precision": 0.9739507959479016, "recall": 0.9606176604165315, "support": 15413}, "macro avg": {"f1-score": 0.3375010505081217, "precision": 0.4842568227787953, "recall": 0.30588489341038316, "support": 135841}, "micro avg": {"f1-score": 0.901494355623413, "precision": 0.9741829590432307, "recall": 0.8388998903129394, "support": 135841}, "weighted avg": {"f1-score": 0.8651455837154045, "precision": 0.9123696903353801, "recall": 0.8388998903129394, "support": 135841}, "\u2205": {"f1-score": 0.9793120599344077, "precision": 0.9864879675585965, "recall": 0.972239796312013, "support": 64805}, "\u23ce": {"f1-score": 0.579356040998033, "precision": 0.9552748378286104, "recall": 0.4157503714710253, "support": 6730}, "\u23ce\u23ce": {"f1-score": 0.5942700291645222, "precision": 0.941304347826087, "recall": 0.43419403359237907, "support": 3989}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 356}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2028}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2054}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2136}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1772}, "\u2423": {"f1-score": 0.9159953636627065, "precision": 0.9540639241843479, "recall": 0.8808482180176666, "support": 35887}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}},
  "ppcr": 0.8611317643421353
}
```
</details>
