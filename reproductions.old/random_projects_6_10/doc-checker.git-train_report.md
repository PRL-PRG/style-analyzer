# Train report for javascript / file:///tmp/top-repos-quality-repos-3fmtlt0p/doc-checker.git HEAD b9691476481da9fe77d094913e828c6f25c667e7

### Classification report

PPCR: 0.883

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 0.986| 0.961| 0.988| 0.975| 419624| 430675| 0.974 |
| `␣` | 0.936| 0.975| 0.792| 0.955| 0.858| 123569| 151950| 0.813 |
| `⏎` | 0.950| 0.967| 0.891| 0.958| 0.919| 47523| 51590| 0.921 |
| `"` | 0.981| 1.000| 0.801| 0.990| 0.882| 23241| 29007| 0.801 |
| `⏎⇥⁻` | 0.975| 0.791| 0.474| 0.874| 0.638| 7027| 11723| 0.599 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.989| 0.882| 0.484| 0.932| 0.650| 6722| 12245| 0.549 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.952| 0.771| 0.159| 0.852| 0.272| 2656| 12875| 0.206 |
| `⏎⇥⁺` | 0.946| 0.514| 0.057| 0.666| 0.108| 1440| 12920| 0.111 |
| `'` | 1.000| 0.635| 0.208| 0.777| 0.345| 1239| 3777| 0.328 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 639| 639| 1.000 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 122| 332| 0.367 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 73| 338| 0.216 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 114| 0.430 |
| `micro avg` | 0.976| 0.976| 0.861| 0.976| 0.915| 633924| 718185| 0.883 |
| `weighted avg` | 0.975| 0.976| 0.861| 0.975| 0.898| 633924| 718185| 0.883 |
| `macro avg` | 0.671| 0.579| 0.371| 0.615| 0.435| 633924| 718185| 0.883 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| '| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁻⇥⁻| ␣␣| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|11051 |413782 |5669 |67 |0 |59 |16 |15 |16 |0 |0 |0 |0 |0 |
|28381 |2496 |120418 |618 |0 |4 |20 |9 |4 |0 |0 |0 |0 |0 |
|4067 |334 |1191 |45971 |0 |2 |0 |1 |24 |0 |0 |0 |0 |0 |
|5766 |0 |0 |0 |23241 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|10219 |15 |561 |16 |0 |2047 |0 |17 |0 |0 |0 |0 |0 |0 |
|5523 |153 |24 |616 |0 |0 |5929 |0 |0 |0 |0 |0 |0 |0 |
|11480 |168 |479 |14 |0 |39 |0 |740 |0 |0 |0 |0 |0 |0 |
|4696 |683 |265 |519 |0 |0 |0 |0 |5560 |0 |0 |0 |0 |0 |
|2538 |0 |0 |0 |452 |0 |0 |0 |0 |787 |0 |0 |0 |0 |
|0 |45 |57 |537 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|265 |0 |1 |42 |0 |0 |30 |0 |0 |0 |0 |0 |0 |0 |
|210 |14 |0 |11 |0 |0 |0 |0 |97 |0 |0 |0 |0 |0 |
|65 |9 |40 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| checker_app/checker_appWeb/Scripts/Office/1/o15apptofilemappingtable.debug.js | 1992 |
| checker_app/checker_appWeb/Scripts/Office/1/outlook-web-16.00.debug.js | 1783 |
| checker_app/checker_appWeb/Scripts/Office/1/outlook-15.04.debug.js | 1378 |
| checker_app/checker_appWeb/Scripts/Office/1/outlook-15.02.debug.js | 930 |
| checker_app/checker_appWeb/Scripts/Office/1/outlook-15.01.debug.js | 858 |
| checker_app/checker_appWeb/Scripts/Office/1/access-web-16.00.debug.js | 817 |
| checker_app/checker_appWeb/Scripts/Office/1/outlookwebapp-15.01.debug.js | 816 |
| checker_app/checker_appWeb/Scripts/Office/1/powerpoint-win32-16.00.debug.js | 626 |
| checker_app/checker_appWeb/Scripts/Office/1/powerpoint-mac-16.00.debug.js | 561 |
| checker_app/checker_appWeb/Scripts/Office/1/powerpoint-win32-16.01.debug.js | 559 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9903694549793326, "precision": 0.9809226353775377, "recall": 1.0, "support": 23241}, "\u0027": {"f1-score": 0.7769002961500493, "precision": 1.0, "recall": 0.6351896690879741, "support": 1239}, "macro avg": {"f1-score": 0.6148093423312682, "precision": 0.6706889250766838, "recall": 0.5785361214138574, "support": 633924}, "micro avg": {"f1-score": 0.9756295707371861, "precision": 0.9756295707371861, "recall": 0.9756295707371861, "support": 633924}, "weighted avg": {"f1-score": 0.9746488783589559, "precision": 0.9746554267125176, "recall": 0.9756295707371861, "support": 633924}, "\u2205": {"f1-score": 0.9883449994804873, "precision": 0.9906224338578737, "recall": 0.9860780126970812, "support": 419624}, "\u23ce": {"f1-score": 0.9583880584568557, "precision": 0.9495982318068208, "recall": 0.9673421290743429, "support": 47523}, "\u23ce\u21e5\u207a": {"f1-score": 0.666066606660666, "precision": 0.9462915601023018, "recall": 0.5138888888888888, "support": 1440}, "\u23ce\u21e5\u207b": {"f1-score": 0.873664362036455, "precision": 0.9752674969303631, "recall": 0.7912338124377402, "support": 7027}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 639}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8516746411483254, "precision": 0.9516503951650395, "recall": 0.7707078313253012, "support": 2656}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.932452622473854, "precision": 0.9889908256880734, "recall": 0.8820291579886939, "support": 6722}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 73}, "\u2423": {"f1-score": 0.9546604089204594, "precision": 0.9356124470688785, "recall": 0.9745000768801236, "support": 123569}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}},
  "cl_report_full": {"\"": {"f1-score": 0.8820113851992409, "precision": 0.9809226353775377, "recall": 0.8012203950770503, "support": 29007}, "\u0027": {"f1-score": 0.3448729184925504, "precision": 1.0, "recall": 0.208366428382314, "support": 3777}, "macro avg": {"f1-score": 0.4345131802574419, "precision": 0.6706889250766838, "recall": 0.3714365427291587, "support": 718185}, "micro avg": {"f1-score": 0.9148300913609775, "precision": 0.9756295707371861, "recall": 0.8611639062358585, "support": 718185}, "weighted avg": {"f1-score": 0.8983305741138118, "precision": 0.9719568955331881, "recall": 0.8611639062358585, "support": 718185}, "\u2205": {"f1-score": 0.9754707239967279, "precision": 0.9906224338578737, "recall": 0.9607755267893423, "support": 430675}, "\u23ce": {"f1-score": 0.919410805891941, "precision": 0.9495982318068208, "recall": 0.8910835433223493, "support": 51590}, "\u23ce\u21e5\u207a": {"f1-score": 0.10801342869654065, "precision": 0.9462915601023018, "recall": 0.05727554179566564, "support": 12920}, "\u23ce\u21e5\u207b": {"f1-score": 0.6382001836547291, "precision": 0.9752674969303631, "recall": 0.47428132730529726, "support": 11723}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 332}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 639}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.27246106748302945, "precision": 0.9516503951650395, "recall": 0.15899029126213593, "support": 12875}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.650109649122807, "precision": 0.9889908256880734, "recall": 0.4841976316864026, "support": 12245}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 338}, "\u2423": {"f1-score": 0.8581211808091784, "precision": 0.9356124470688785, "recall": 0.7924843698585061, "support": 151950}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}},
  "ppcr": 0.8826750767559891
}
```
</details>
