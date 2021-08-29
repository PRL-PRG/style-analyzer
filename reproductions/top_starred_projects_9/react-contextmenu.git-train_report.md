# Train report for javascript / file:///tmp/top-repos-quality-repos-b1wc6w99/react-contextmenu.git HEAD d9018dbfbd6e21423cb2b753b3762adf5a6d77b0

### Classification report

PPCR: 0.683

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 0.990| 0.816| 0.980| 0.887| 6307| 7654| 0.824 |
| `␣` | 0.938| 0.907| 0.515| 0.922| 0.665| 1874| 3299| 0.568 |
| `'` | 1.000| 1.000| 0.828| 1.000| 0.906| 605| 731| 0.828 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.900| 0.909| 0.484| 0.905| 0.629| 198| 372| 0.532 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 44| 530| 0.083 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 381| 0.047 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 286| 0.014 |
| `weighted avg` | 0.957| 0.965| 0.659| 0.961| 0.745| 9050| 13253| 0.683 |
| `micro avg` | 0.965| 0.965| 0.659| 0.965| 0.783| 9050| 13253| 0.683 |
| `macro avg` | 0.544| 0.544| 0.378| 0.544| 0.441| 9050| 13253| 0.683 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1347 |6246 |61 |0 |0 |0 |0 |0 |
|1425 |168 |1699 |0 |0 |0 |7 |0 |
|126 |0 |0 |605 |0 |0 |0 |0 |
|486 |0 |32 |0 |0 |0 |12 |0 |
|363 |2 |16 |0 |0 |0 |0 |0 |
|174 |18 |0 |0 |0 |0 |180 |0 |
|282 |0 |3 |0 |0 |0 |1 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/ContextMenu.js | 57 |
| src/AbstractMenu.js | 27 |
| tests/ContextMenu.test.js | 26 |
| examples/DynamicMenu.js | 21 |
| src/SubMenu.js | 20 |
| tests/sub-menu-test.js | 19 |
| examples/index.js | 18 |
| src/MenuItem.js | 17 |
| examples/RTLSubMenu.js | 13 |
| src/ContextMenuTrigger.js | 12 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 605}, "macro avg": {"f1-score": 0.5438708707680814, "precision": 0.5441337064432172, "recall": 0.5437194254531261, "support": 9050}, "micro avg": {"f1-score": 0.9646408839779006, "precision": 0.9646408839779006, "recall": 0.9646408839779006, "support": 9050}, "weighted avg": {"f1-score": 0.9608710659383263, "precision": 0.9573496929100712, "recall": 0.9646408839779006, "support": 9050}, "\u2205": {"f1-score": 0.9804567930303745, "precision": 0.9707802300279764, "recall": 0.9903282067543999, "support": 6307}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9045226130653266, "precision": 0.9, "recall": 0.9090909090909091, "support": 198}, "\u2423": {"f1-score": 0.9221166892808683, "precision": 0.9381557150745444, "recall": 0.9066168623265741, "support": 1874}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9056886227544909, "precision": 1.0, "recall": 0.8276333789329685, "support": 731}, "macro avg": {"f1-score": 0.4409631419025749, "precision": 0.5441337064432172, "recall": 0.3775075417317687, "support": 13253}, "micro avg": {"f1-score": 0.7828543245303322, "precision": 0.9646408839779006, "recall": 0.6587187806534369, "support": 13253}, "weighted avg": {"f1-score": 0.7452513993830288, "precision": 0.8746040583011434, "recall": 0.6587187806534369, "support": 13253}, "\u2205": {"f1-score": 0.8867120954003408, "precision": 0.9707802300279764, "recall": 0.8160438986151032, "support": 7654}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 530}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 286}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 381}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.6293706293706294, "precision": 0.9, "recall": 0.4838709677419355, "support": 372}, "\u2423": {"f1-score": 0.6649706457925635, "precision": 0.9381557150745444, "recall": 0.5150045468323734, "support": 3299}},
  "ppcr": 0.6828642571493246
}
```
</details>
