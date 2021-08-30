# Train report for javascript / file:///tmp/top-repos-quality-repos-acyjc8oy/create-snowpack-app.git HEAD 45660ff36399f50de2d60d4af5743c3c0b65bdad

### Classification report

PPCR: 0.739

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.909| 0.981| 0.945| 0.944| 0.926| 3779| 3926| 0.963 |
| `␣` | 0.899| 0.922| 0.844| 0.911| 0.871| 1738| 1899| 0.915 |
| `⏎` | 0.907| 0.417| 0.139| 0.571| 0.240| 211| 635| 0.332 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 104| 347| 0.300 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 94| 340| 0.276 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 117| 0.291 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 1| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 726| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 77| 0.000 |
| `weighted avg` | 0.870| 0.906| 0.669| 0.884| 0.675| 5961| 8070| 0.739 |
| `micro avg` | 0.906| 0.906| 0.669| 0.906| 0.770| 5961| 8070| 0.739 |
| `macro avg` | 0.272| 0.232| 0.193| 0.243| 0.204| 5961| 8070| 0.739 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|147 |3709 |70 |0 |0 |0 |0 |0 |0 |0 |0 |
|161 |135 |1603 |0 |0 |0 |0 |0 |0 |0 |0 |
|424 |52 |71 |88 |0 |0 |0 |0 |0 |0 |0 |
|0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|726 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|77 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|243 |72 |32 |0 |0 |0 |0 |0 |0 |0 |0 |
|246 |87 |0 |7 |0 |0 |0 |0 |0 |0 |0 |
|83 |25 |7 |2 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/create-snowpack-app/index.js | 103 |
| packages/babel-plugin-package-import/plugin.js | 101 |
| packages/plugin-webpack/plugin.js | 76 |
| packages/plugin-parcel/plugin.js | 41 |
| packages/plugin-vue/plugin.js | 26 |
| packages/plugin-react-refresh/plugin.js | 18 |
| packages/plugin-webpack/import-meta-plugin/plugin.js | 16 |
| packages/app-scripts-react/jest.config.js | 15 |
| packages/app-scripts-preact/jest.config.js | 15 |
| packages/plugin-svelte/plugin.js | 14 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24257312540605153, "precision": 0.27151089167125336, "recall": 0.23208627034126258, "support": 5961}, "micro avg": {"f1-score": 0.9058882737795672, "precision": 0.9058882737795672, "recall": 0.9058882737795672, "support": 5961}, "weighted avg": {"f1-score": 0.8840086729805647, "precision": 0.8704066654173919, "recall": 0.9058882737795672, "support": 5961}, "\u2205": {"f1-score": 0.9437659033078881, "precision": 0.908845871110022, "recall": 0.9814765811061127, "support": 3779}, "\u23ce": {"f1-score": 0.5714285714285714, "precision": 0.9072164948453608, "recall": 0.41706161137440756, "support": 211}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 94}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9105367793240556, "precision": 0.8990465507571509, "recall": 0.9223245109321059, "support": 1738}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 726}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "macro avg": {"f1-score": 0.20375989574851605, "precision": 0.27151089167125336, "recall": 0.19274386238160973, "support": 8070}, "micro avg": {"f1-score": 0.7697241821680564, "precision": 0.9058882737795672, "recall": 0.6691449814126395, "support": 8070}, "weighted avg": {"f1-score": 0.6745205013554618, "precision": 0.7250930314860694, "recall": 0.6691449814126395, "support": 8070}, "\u2205": {"f1-score": 0.9264393655551393, "precision": 0.908845871110022, "recall": 0.9447274579724911, "support": 3926}, "\u23ce": {"f1-score": 0.24043715846994537, "precision": 0.9072164948453608, "recall": 0.13858267716535433, "support": 635}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 117}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 347}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 340}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.870722433460076, "precision": 0.8990465507571509, "recall": 0.8441284886782517, "support": 1899}},
  "ppcr": 0.7386617100371747
}
```
</details>
