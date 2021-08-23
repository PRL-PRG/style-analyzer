# Train report for javascript / file:///tmp/top-repos-quality-repos-9ln8gubq/sknikodbackendserver.git HEAD 937d44fbf930931f1901421a04be0a62e7a012f4

### Classification report

PPCR: 0.619

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.958| 0.998| 0.987| 0.978| 0.972| 9302| 9405| 0.989 |
| `␣` | 0.967| 0.888| 0.469| 0.926| 0.632| 2794| 5285| 0.529 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.904| 0.912| 0.677| 0.908| 0.774| 342| 461| 0.742 |
| `⏎` | 0.951| 0.792| 0.212| 0.864| 0.347| 341| 1272| 0.268 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 105| 0.467 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 474| 0.101 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 112| 0.107 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 192| 0.026 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3205| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 308| 0.000 |
| `macro avg` | 0.378| 0.359| 0.235| 0.368| 0.273| 12893| 20819| 0.619 |
| `weighted avg` | 0.949| 0.958| 0.593| 0.953| 0.638| 12893| 20819| 0.619 |
| `micro avg` | 0.958| 0.958| 0.593| 0.958| 0.733| 12893| 20819| 0.619 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|103 |9287 |15 |0 |0 |0 |0 |0 |0 |0 |0 |
|2491 |308 |2480 |0 |0 |0 |6 |0 |0 |0 |0 |
|3205 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|931 |47 |24 |0 |270 |0 |0 |0 |0 |0 |0 |
|426 |9 |36 |0 |3 |0 |0 |0 |0 |0 |0 |
|119 |24 |0 |0 |6 |0 |312 |0 |0 |0 |0 |
|308 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|187 |2 |0 |0 |3 |0 |0 |0 |0 |0 |0 |
|100 |3 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|56 |19 |1 |0 |2 |0 |27 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| staticfolder/rest_framework/docs/js/api.js | 85 |
| staticfolder/admin/js/core.js | 63 |
| staticfolder/admin/js/admin/DateTimeShortcuts.js | 53 |
| staticfolder/admin/js/SelectFilter2.js | 49 |
| staticfolder/admin/js/inlines.js | 39 |
| staticfolder/admin/js/urlify.js | 36 |
| staticfolder/admin/js/actions.js | 35 |
| staticfolder/rest_framework/js/ajax-form.js | 34 |
| staticfolder/admin/js/calendar.js | 29 |
| staticfolder/admin/js/SelectBox.js | 25 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3675370252569743, "precision": 0.3779435043837755, "recall": 0.35900733223070824, "support": 12893}, "micro avg": {"f1-score": 0.9578065617001473, "precision": 0.9578065617001473, "recall": 0.9578065617001473, "support": 12893}, "weighted avg": {"f1-score": 0.9527803724080813, "precision": 0.9494882812385257, "recall": 0.9578065617001473, "support": 12893}, "\u2205": {"f1-score": 0.9775274985527078, "precision": 0.9575213939581401, "recall": 0.9983874435605247, "support": 9302}, "\u23ce": {"f1-score": 0.8639999999999999, "precision": 0.9507042253521126, "recall": 0.7917888563049853, "support": 341}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9082969432314411, "precision": 0.9043478260869565, "recall": 0.9122807017543859, "support": 342}, "\u2423": {"f1-score": 0.9255458107855943, "precision": 0.9668615984405458, "recall": 0.8876163206871869, "support": 2794}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 308}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3205}, "macro avg": {"f1-score": 0.2725341502872517, "precision": 0.3779435043837755, "recall": 0.2345759822689148, "support": 20819}, "micro avg": {"f1-score": 0.7326174655908874, "precision": 0.9578065617001473, "recall": 0.5931600941447716, "support": 20819}, "weighted avg": {"f1-score": 0.6379621076864311, "precision": 0.7561147211877883, "recall": 0.5931600941447716, "support": 20819}, "\u2205": {"f1-score": 0.9722571189279732, "precision": 0.9575213939581401, "recall": 0.9874534821903242, "support": 9405}, "\u23ce": {"f1-score": 0.3470437017994859, "precision": 0.9507042253521126, "recall": 0.21226415094339623, "support": 1272}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 192}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 474}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 105}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7741935483870968, "precision": 0.9043478260869565, "recall": 0.6767895878524945, "support": 461}, "\u2423": {"f1-score": 0.6318471337579618, "precision": 0.9668615984405458, "recall": 0.4692526017029328, "support": 5285}},
  "ppcr": 0.6192900715692397
}
```
</details>
