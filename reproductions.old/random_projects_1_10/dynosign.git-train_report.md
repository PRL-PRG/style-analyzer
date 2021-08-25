# Train report for javascript / file:///tmp/top-repos-quality-repos-wsmbqhds/dynosign.git HEAD 0337f13569a3ac2e28372bc79089478e69c1bbeb

### Classification report

PPCR: 0.329

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 0.996| 0.420| 0.993| 0.590| 3038| 7203| 0.422 |
| `'` | 1.000| 1.000| 0.937| 1.000| 0.968| 1076| 1148| 0.937 |
| `␣` | 0.971| 0.989| 0.141| 0.980| 0.246| 545| 3832| 0.142 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 465| 0.047 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 442| 0.005 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 977| 0.001 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 190| 0.000 |
| `micro avg` | 0.991| 0.991| 0.325| 0.991| 0.490| 4684| 14257| 0.329 |
| `weighted avg` | 0.985| 0.991| 0.325| 0.988| 0.442| 4684| 14257| 0.329 |
| `macro avg` | 0.423| 0.426| 0.214| 0.425| 0.258| 4684| 14257| 0.329 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|4165 |3025 |13 |0 |0 |0 |0 |0 |
|3287 |6 |539 |0 |0 |0 |0 |0 |
|72 |0 |0 |1076 |0 |0 |0 |0 |
|976 |1 |0 |0 |0 |0 |0 |0 |
|443 |19 |3 |0 |0 |0 |0 |0 |
|440 |2 |0 |0 |0 |0 |0 |0 |
|190 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| gh-pages/documentationjs_theme/assets_theme/split.js | 14 |
| gh-pages/documentationjs_theme/index.js | 13 |
| ProgramEntry/reducers.test.js | 7 |
| gh-pages/documentationjs_theme/assets_theme/site.js | 3 |
| ProgramEntry/reducers.js | 2 |
| ProgramEntry/MainArea/DrawingLayer.test.js | 2 |
| ProgramEntry/MenuActionPanel/optionsHelpers.js | 2 |
| ProgramEntry/MainArea/drawingHelpers.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1076}, "macro avg": {"f1-score": 0.42475267959753266, "precision": 0.4231428377514194, "recall": 0.42638738495440454, "support": 4684}, "micro avg": {"f1-score": 0.9906063193851409, "precision": 0.9906063193851409, "recall": 0.9906063193851409, "support": 4684}, "weighted avg": {"f1-score": 0.9879697874297886, "precision": 0.9853599184227063, "recall": 0.9906063193851409, "support": 4684}, "\u2205": {"f1-score": 0.9932687571827287, "precision": 0.9908286930887652, "recall": 0.9957208689927584, "support": 3038}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.98, "precision": 0.9711711711711711, "recall": 0.9889908256880734, "support": 545}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9676258992805754, "precision": 1.0, "recall": 0.9372822299651568, "support": 1148}, "macro avg": {"f1-score": 0.2576072148409098, "precision": 0.4231428377514194, "recall": 0.21398625056226125, "support": 14257}, "micro avg": {"f1-score": 0.4899424528799957, "precision": 0.9906063193851409, "recall": 0.3254541628673634, "support": 14257}, "weighted avg": {"f1-score": 0.4419931391001037, "precision": 0.8421454025563796, "recall": 0.3254541628673634, "support": 14257}, "\u2205": {"f1-score": 0.5898985959438378, "precision": 0.9908286930887652, "recall": 0.4199639039289185, "support": 7203}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 977}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 190}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 465}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 442}, "\u2423": {"f1-score": 0.2457260086619558, "precision": 0.9711711711711711, "recall": 0.14065762004175367, "support": 3832}},
  "ppcr": 0.32854036613593324
}
```
</details>
