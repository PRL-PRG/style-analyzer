# Train report for javascript / file:///tmp/top-repos-quality-repos-60bwy3oa/sync-settings.git HEAD a3c22b57d613ef007d5f8fc78a260b1fa7a79bb0

### Classification report

PPCR: 0.806

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 0.987| 0.943| 0.975| 0.953| 15334| 16048| 0.956 |
| `␣` | 0.941| 0.826| 0.468| 0.880| 0.626| 3648| 6434| 0.567 |
| `'` | 1.000| 1.000| 0.939| 1.000| 0.969| 2497| 2659| 0.939 |
| `⏎⇥⁻` | 0.895| 0.890| 0.884| 0.892| 0.889| 1002| 1009| 0.993 |
| `⏎⇥⁺` | 0.864| 0.966| 0.948| 0.912| 0.904| 1000| 1019| 0.981 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 43| 1482| 0.029 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 536| 0.000 |
| `micro avg` | 0.956| 0.956| 0.771| 0.956| 0.854| 23524| 29187| 0.806 |
| `weighted avg` | 0.955| 0.956| 0.771| 0.955| 0.812| 23524| 29187| 0.806 |
| `macro avg` | 0.666| 0.667| 0.597| 0.666| 0.620| 23524| 29187| 0.806 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|714 |15127 |160 |0 |0 |15 |32 |0 |
|2786 |424 |3014 |0 |0 |137 |73 |0 |
|162 |0 |0 |2497 |0 |0 |0 |0 |
|1439 |23 |20 |0 |0 |0 |0 |0 |
|19 |25 |9 |0 |0 |966 |0 |0 |
|7 |110 |0 |0 |0 |0 |892 |0 |
|536 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/views/diff-view.js | 323 |
| spec/sync-settings-spec.js | 143 |
| lib/utils/utils.js | 106 |
| lib/sync-settings.js | 84 |
| lib/location/gist.js | 78 |
| lib/utils/notify.js | 60 |
| spec/utils-spec.js | 41 |
| spec/diff-view-spec.js | 38 |
| spec/gist-api-mock.js | 38 |
| spec/integration-spec.js | 35 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2497}, "macro avg": {"f1-score": 0.665583245695845, "precision": 0.6660958542406827, "recall": 0.6669894697371604, "support": 23524}, "micro avg": {"f1-score": 0.9562999489882673, "precision": 0.9562999489882673, "recall": 0.9562999489882673, "support": 23524}, "weighted avg": {"f1-score": 0.9546611718254177, "precision": 0.9546059967776377, "recall": 0.9562999489882673, "support": 23524}, "\u2205": {"f1-score": 0.9745836420449054, "precision": 0.9629511744859635, "recall": 0.986500586931003, "support": 15334}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u21e5\u207a": {"f1-score": 0.9121813031161473, "precision": 0.8640429338103757, "recall": 0.966, "support": 1000}, "\u23ce\u21e5\u207b": {"f1-score": 0.8924462231115557, "precision": 0.8946840521564694, "recall": 0.8902195608782435, "support": 1002}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8798715515983068, "precision": 0.94099281923197, "recall": 0.8262061403508771, "support": 3648}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9685802948021722, "precision": 1.0, "recall": 0.9390748401654757, "support": 2659}, "macro avg": {"f1-score": 0.620023020176421, "precision": 0.6660958542406827, "recall": 0.5974521725480068, "support": 29187}, "micro avg": {"f1-score": 0.8535599779932084, "precision": 0.9562999489882673, "recall": 0.7707541028540104, "support": 29187}, "weighted avg": {"f1-score": 0.812245933989738, "precision": 0.8890939187058583, "recall": 0.7707541028540104, "support": 29187}, "\u2205": {"f1-score": 0.9526718518751771, "precision": 0.9629511744859635, "recall": 0.9426096709870389, "support": 16048}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1482}, "\u23ce\u21e5\u207a": {"f1-score": 0.9040711277491811, "precision": 0.8640429338103757, "recall": 0.9479882237487733, "support": 1019}, "\u23ce\u21e5\u207b": {"f1-score": 0.8893320039880359, "precision": 0.8946840521564694, "recall": 0.8840436075322101, "support": 1009}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 536}, "\u2423": {"f1-score": 0.6255058628203798, "precision": 0.94099281923197, "recall": 0.468448865402549, "support": 6434}},
  "ppcr": 0.8059752629595368
}
```
</details>
