# Train report for javascript / file:///tmp/top-repos-quality-repos-ke1ue5c3/demo.git HEAD 2511ba1002cf1dc3be40d2ad647e428c48802bc0

### Classification report

PPCR: 0.681

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.993| 0.852| 0.983| 0.908| 29170| 33989| 0.858 |
| `␣` | 0.957| 0.954| 0.475| 0.956| 0.635| 7337| 14720| 0.498 |
| `'` | 0.981| 1.000| 0.891| 0.990| 0.934| 5484| 6152| 0.891 |
| `⏎` | 0.953| 0.941| 0.248| 0.947| 0.393| 1144| 4347| 0.263 |
| `⏎␣⁻␣⁻` | 0.992| 0.877| 0.354| 0.931| 0.522| 669| 1659| 0.403 |
| `"` | 1.000| 0.800| 0.705| 0.889| 0.827| 540| 613| 0.881 |
| `⏎⇥⁻` | 1.000| 0.660| 0.337| 0.795| 0.504| 473| 926| 0.511 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 200| 1674| 0.119 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 46| 149| 0.309 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 983| 0.036 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 971| 0.034 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 152| 0.112 |
| `macro avg` | 0.571| 0.519| 0.322| 0.541| 0.394| 45148| 66335| 0.681 |
| `micro avg` | 0.971| 0.971| 0.661| 0.971| 0.787| 45148| 66335| 0.681 |
| `weighted avg` | 0.964| 0.971| 0.661| 0.967| 0.747| 45148| 66335| 0.681 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4819 |28964 |199 |0 |6 |0 |1 |0 |0 |0 |0 |0 |0 |
|7383 |328 |6999 |0 |10 |0 |0 |0 |0 |0 |0 |0 |0 |
|668 |0 |0 |5484 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3203 |48 |20 |0 |1076 |0 |0 |0 |0 |0 |0 |0 |0 |
|1474 |142 |58 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|990 |70 |6 |0 |6 |0 |587 |0 |0 |0 |0 |0 |0 |
|938 |4 |0 |0 |29 |0 |0 |0 |0 |0 |0 |0 |0 |
|948 |33 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|453 |159 |0 |0 |2 |0 |0 |0 |0 |312 |0 |0 |0 |
|73 |0 |0 |108 |0 |0 |0 |0 |0 |0 |432 |0 |0 |
|103 |23 |23 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|135 |9 |4 |0 |0 |0 |4 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| html5/longlist_other/src/list.js | 54 |
| javascript/float/index.js | 43 |
| draftjs/highlight/src/index.js | 37 |
| antd/helloworld/src/components/PageHeader/index.js | 30 |
| antd/helloworld/src/components/SiderMenu/SiderMenu.js | 29 |
| antd/helloworld/src/components/Ellipsis/index.js | 28 |
| draftjs/entity/src/index.js | 26 |
| antd/helloworld/src/routes/Forms/AdvancedForm.js | 22 |
| draftjs/highlight2/src/index.js | 21 |
| antd/helloworld/src/routes/Forms/TableForm.js | 21 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.888888888888889, "precision": 1.0, "recall": 0.8, "support": 540}, "\u0027": {"f1-score": 0.9902491874322861, "precision": 0.98068669527897, "recall": 1.0, "support": 5484}, "macro avg": {"f1-score": 0.5408420562875461, "precision": 0.5712683491749931, "recall": 0.5187064970147535, "support": 45148}, "micro avg": {"f1-score": 0.9713387082484274, "precision": 0.9713387082484274, "recall": 0.9713387082484274, "support": 45148}, "weighted avg": {"f1-score": 0.967222727615119, "precision": 0.9643691049149389, "recall": 0.9713387082484274, "support": 45148}, "\u2205": {"f1-score": 0.982663273960984, "precision": 0.9725990597716588, "recall": 0.9929379499485773, "support": 29170}, "\u23ce": {"f1-score": 0.9467663880334359, "precision": 0.9530558015943312, "recall": 0.9405594405594405, "support": 1144}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u21e5\u207b": {"f1-score": 0.7949044585987262, "precision": 1.0, "recall": 0.6596194503171248, "support": 473}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 200}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9310071371927041, "precision": 0.9915540540540541, "recall": 0.8774289985052317, "support": 669}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.9556253413435281, "precision": 0.9573245794009028, "recall": 0.9539321248466676, "support": 7337}},
  "cl_report_full": {"\"": {"f1-score": 0.8267942583732057, "precision": 1.0, "recall": 0.7047308319738989, "support": 613}, "\u0027": {"f1-score": 0.933923705722071, "precision": 0.98068669527897, "recall": 0.8914174252275683, "support": 6152}, "macro avg": {"f1-score": 0.39358929088577393, "precision": 0.5712683491749931, "recall": 0.32183912785818164, "support": 66335}, "micro avg": {"f1-score": 0.786738785285649, "precision": 0.9713387082484274, "recall": 0.6610989673626291, "support": 66335}, "weighted avg": {"f1-score": 0.7465307396851417, "precision": 0.9121820086820329, "recall": 0.6610989673626291, "support": 66335}, "\u2205": {"f1-score": 0.908403769856827, "precision": 0.9725990597716588, "recall": 0.8521580511341905, "support": 33989}, "\u23ce": {"f1-score": 0.3929875821767714, "precision": 0.9530558015943312, "recall": 0.2475270301357258, "support": 4347}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 983}, "\u23ce\u21e5\u207b": {"f1-score": 0.5040387722132471, "precision": 1.0, "recall": 0.3369330453563715, "support": 926}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 971}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1674}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 149}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.521545979564638, "precision": 0.9915540540540541, "recall": 0.353827606992164, "support": 1659}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "\u2423": {"f1-score": 0.6353774227225274, "precision": 0.9573245794009028, "recall": 0.4754755434782609, "support": 14720}},
  "ppcr": 0.6806060149242481
}
```
</details>
