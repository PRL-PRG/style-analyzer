# Test report for javascript / file:///tmp/top-repos-quality-repos-34b8yf9b/travelroute.git HEAD 63478bb0511bccf4a75f93834978f093d5b54f47

### Classification report

PPCR: 0.629

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 0.982| 0.856| 0.975| 0.909| 334| 383| 0.872 |
| `␣` | 0.850| 0.927| 0.327| 0.887| 0.472| 55| 156| 0.353 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 18| 0.278 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 18| 0.167 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 17| 0.118 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 42| 0.000 |
| `micro avg` | 0.950| 0.950| 0.598| 0.950| 0.734| 399| 634| 0.629 |
| `weighted avg` | 0.927| 0.950| 0.598| 0.938| 0.665| 399| 634| 0.629 |
| `macro avg` | 0.303| 0.318| 0.197| 0.310| 0.230| 399| 634| 0.629 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|49 |328 |6 |0 |0 |0 |0 |
|101 |4 |51 |0 |0 |0 |0 |
|42 |0 |0 |0 |0 |0 |0 |
|13 |4 |1 |0 |0 |0 |0 |
|15 |0 |2 |0 |0 |0 |0 |
|15 |3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "macro avg": {"f1-score": 0.31028274867024136, "precision": 0.3029252704031465, "recall": 0.3182181092360733, "support": 399}, "micro avg": {"f1-score": 0.949874686716792, "precision": 0.949874686716792, "recall": 0.949874686716792, "support": 399}, "weighted avg": {"f1-score": 0.9382099217291889, "precision": 0.9270983505962547, "recall": 0.949874686716792, "support": 399}, "\u2205": {"f1-score": 0.9747399702823178, "precision": 0.967551622418879, "recall": 0.9820359281437125, "support": 334}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.8869565217391303, "precision": 0.85, "recall": 0.9272727272727272, "support": 55}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "macro avg": {"f1-score": 0.23013491330665845, "precision": 0.3029252704031465, "recall": 0.19721999062730133, "support": 634}, "micro avg": {"f1-score": 0.73378509196515, "precision": 0.949874686716792, "recall": 0.5977917981072555, "support": 634}, "weighted avg": {"f1-score": 0.6650719027354207, "precision": 0.793647115751468, "recall": 0.5977917981072555, "support": 634}, "\u2205": {"f1-score": 0.9085872576177285, "precision": 0.967551622418879, "recall": 0.856396866840731, "support": 383}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u2423": {"f1-score": 0.4722222222222222, "precision": 0.85, "recall": 0.3269230769230769, "support": 156}},
  "ppcr": 0.6293375394321766
}
```
</details>
