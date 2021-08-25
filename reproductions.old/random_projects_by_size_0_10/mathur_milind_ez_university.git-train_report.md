# Train report for javascript / file:///tmp/top-repos-quality-repos-lf2e4na8/mathur_milind_ez_university.git HEAD 0bdd3ad78c2a34c4331314192556ed9137b30e5c

### Classification report

PPCR: 0.520

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 0.993| 0.737| 0.983| 0.839| 5856| 7886| 0.743 |
| `␣` | 0.961| 0.928| 0.268| 0.944| 0.419| 1292| 4472| 0.289 |
| `'` | 1.000| 1.000| 0.359| 1.000| 0.528| 1063| 2964| 0.359 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.971| 0.917| 0.784| 0.943| 0.868| 408| 477| 0.855 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.981| 0.881| 0.675| 0.928| 0.799| 353| 461| 0.766 |
| `⏎` | 0.940| 0.909| 0.200| 0.925| 0.330| 243| 1105| 0.220 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 122| 0.033 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 249| 0.000 |
| `micro avg` | 0.974| 0.974| 0.506| 0.974| 0.666| 9219| 17736| 0.520 |
| `weighted avg` | 0.974| 0.974| 0.506| 0.974| 0.632| 9219| 17736| 0.520 |
| `macro avg` | 0.728| 0.703| 0.378| 0.715| 0.473| 9219| 17736| 0.520 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2030 |5813 |38 |0 |0 |5 |0 |0 |0 |
|3180 |83 |1199 |0 |0 |4 |6 |0 |0 |
|1901 |0 |0 |1063 |0 |0 |0 |0 |0 |
|862 |20 |0 |0 |221 |2 |0 |0 |0 |
|69 |20 |11 |0 |3 |374 |0 |0 |0 |
|108 |34 |0 |0 |8 |0 |311 |0 |0 |
|249 |0 |0 |0 |0 |0 |0 |0 |0 |
|118 |1 |0 |0 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/admin/js/core.js | 45 |
| static/admin/js/inlines.js | 36 |
| static/admin/js/actions.js | 33 |
| static/admin/js/SelectFilter2.js | 29 |
| static/admin/js/urlify.js | 23 |
| static/admin/js/admin/DateTimeShortcuts.js | 19 |
| static/admin/js/SelectBox.js | 17 |
| static/admin/js/calendar.js | 10 |
| static/admin/js/admin/RelatedObjectLookups.js | 5 |
| static/admin/js/autocomplete.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1063}, "macro avg": {"f1-score": 0.7154246682552364, "precision": 0.728400326095108, "recall": 0.703478399618457, "support": 9219}, "micro avg": {"f1-score": 0.9741837509491268, "precision": 0.9741837509491268, "recall": 0.9741837509491268, "support": 9219}, "weighted avg": {"f1-score": 0.9736960515574102, "precision": 0.9736956666286579, "recall": 0.9741837509491268, "support": 9219}, "\u2205": {"f1-score": 0.9830049885854402, "precision": 0.9735387707251717, "recall": 0.9926571038251366, "support": 5856}, "\u23ce": {"f1-score": 0.9246861924686193, "precision": 0.9404255319148936, "recall": 0.9094650205761317, "support": 243}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9432534678436317, "precision": 0.9714285714285714, "recall": 0.9166666666666666, "support": 408}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9283582089552238, "precision": 0.9810725552050473, "recall": 0.8810198300283286, "support": 353}, "\u2423": {"f1-score": 0.9440944881889763, "precision": 0.9607371794871795, "recall": 0.9280185758513931, "support": 1292}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 249}, "\u0027": {"f1-score": 0.527936429103551, "precision": 1.0, "recall": 0.3586369770580297, "support": 2964}, "macro avg": {"f1-score": 0.47290644574102086, "precision": 0.728400326095108, "recall": 0.37782078053067725, "support": 17736}, "micro avg": {"f1-score": 0.6663698757187906, "precision": 0.9741837509491268, "recall": 0.5063712223725756, "support": 17736}, "weighted avg": {"f1-score": 0.6316480049923228, "precision": 0.9524447170665473, "recall": 0.5063712223725756, "support": 17736}, "\u2205": {"f1-score": 0.8389983401890742, "precision": 0.9735387707251717, "recall": 0.7371290895257419, "support": 7886}, "\u23ce": {"f1-score": 0.32985074626865674, "precision": 0.9404255319148936, "recall": 0.2, "support": 1105}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8677494199535963, "precision": 0.9714285714285714, "recall": 0.7840670859538784, "support": 477}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7994858611825193, "precision": 0.9810725552050473, "recall": 0.6746203904555315, "support": 461}, "\u2423": {"f1-score": 0.4192307692307692, "precision": 0.9607371794871795, "recall": 0.26811270125223613, "support": 4472}},
  "ppcr": 0.5197902571041949
}
```
</details>
