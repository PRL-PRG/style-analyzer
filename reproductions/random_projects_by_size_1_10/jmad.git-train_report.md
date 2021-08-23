# Train report for javascript / file:///tmp/top-repos-quality-repos-v8tpeb4c/jmad.git HEAD a966b479b9e1259fc889911d0f88a7a6bf532de6

### Classification report

PPCR: 0.492

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.998| 0.815| 0.981| 0.883| 6200| 7596| 0.816 |
| `␣` | 0.974| 0.852| 0.235| 0.909| 0.378| 1181| 4288| 0.275 |
| `⏎` | 0.925| 0.890| 0.310| 0.907| 0.464| 374| 1075| 0.348 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.963| 0.921| 0.775| 0.941| 0.859| 366| 435| 0.841 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.983| 0.874| 0.669| 0.925| 0.796| 326| 426| 0.765 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 118| 0.034 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2944| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 290| 0.000 |
| `weighted avg` | 0.964| 0.965| 0.475| 0.963| 0.556| 8451| 17172| 0.492 |
| `macro avg` | 0.601| 0.567| 0.350| 0.583| 0.423| 8451| 17172| 0.492 |
| `micro avg` | 0.965| 0.965| 0.475| 0.965| 0.636| 8451| 17172| 0.492 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1396 |6190 |5 |0 |0 |5 |0 |0 |0 |
|3107 |150 |1006 |0 |15 |5 |5 |0 |0 |
|2944 |0 |0 |0 |0 |0 |0 |0 |0 |
|701 |20 |18 |0 |333 |3 |0 |0 |0 |
|69 |23 |4 |0 |2 |337 |0 |0 |0 |
|100 |34 |0 |0 |7 |0 |285 |0 |0 |
|290 |0 |0 |0 |0 |0 |0 |0 |0 |
|114 |1 |0 |0 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| jmad/static/admin/js/core.js | 49 |
| jmad/static/admin/js/inlines.js | 46 |
| jmad/static/admin/js/admin/DateTimeShortcuts.js | 42 |
| jmad/static/admin/js/SelectBox.js | 34 |
| jmad/static/admin/js/SelectFilter2.js | 32 |
| jmad/static/admin/js/actions.js | 24 |
| jmad/static/admin/js/urlify.js | 23 |
| jmad/static/admin/js/calendar.js | 21 |
| jmad/static/admin/js/admin/RelatedObjectLookups.js | 9 |
| jmad/static/admin/js/autocomplete.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5829903604222924, "precision": 0.601119151769044, "recall": 0.5669475094488732, "support": 8451}, "micro avg": {"f1-score": 0.964501242456514, "precision": 0.964501242456514, "recall": 0.964501242456514, "support": 8451}, "weighted avg": {"f1-score": 0.9634176357846593, "precision": 0.9642185716110608, "recall": 0.964501242456514, "support": 8451}, "\u2205": {"f1-score": 0.9811380567443334, "precision": 0.9644749143035214, "recall": 0.9983870967741936, "support": 6200}, "\u23ce": {"f1-score": 0.9073569482288828, "precision": 0.925, "recall": 0.8903743315508021, "support": 374}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.941340782122905, "precision": 0.9628571428571429, "recall": 0.9207650273224044, "support": 366}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9253246753246752, "precision": 0.9827586206896551, "recall": 0.8742331288343558, "support": 326}, "\u2423": {"f1-score": 0.9087624209575428, "precision": 0.9738625363020329, "recall": 0.8518204911092294, "support": 1181}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 290}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2944}, "macro avg": {"f1-score": 0.42254079181568366, "precision": 0.601119151769044, "recall": 0.3503756199132895, "support": 17172}, "micro avg": {"f1-score": 0.6362252663622526, "precision": 0.964501242456514, "recall": 0.4746680642907058, "support": 17172}, "weighted avg": {"f1-score": 0.5557457388847108, "precision": 0.7764935379844699, "recall": 0.4746680642907058, "support": 17172}, "\u2205": {"f1-score": 0.8834023119737406, "precision": 0.9644749143035214, "recall": 0.814902580305424, "support": 7596}, "\u23ce": {"f1-score": 0.46411149825783965, "precision": 0.925, "recall": 0.3097674418604651, "support": 1075}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8585987261146497, "precision": 0.9628571428571429, "recall": 0.774712643678161, "support": 435}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7960893854748603, "precision": 0.9827586206896551, "recall": 0.6690140845070423, "support": 426}, "\u2423": {"f1-score": 0.3781244127043789, "precision": 0.9738625363020329, "recall": 0.23460820895522388, "support": 4288}},
  "ppcr": 0.4921383647798742
}
```
</details>
