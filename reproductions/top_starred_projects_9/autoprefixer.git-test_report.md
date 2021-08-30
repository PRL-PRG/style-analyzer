# Test report for javascript / file:///tmp/top-repos-quality-repos-kh8l8bue/autoprefixer.git HEAD b5b5f5d01c03923d2750f827421b0f4db4b5e1e1

### Classification report

PPCR: 0.974

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 0.972| 0.966| 0.967| 0.965| 3207| 3225| 0.994 |
| `␣` | 0.941| 0.948| 0.943| 0.945| 0.942| 2139| 2151| 0.994 |
| `'` | 0.976| 0.988| 0.969| 0.982| 0.973| 738| 752| 0.981 |
| `⏎␣⁺␣⁺` | 0.944| 0.908| 0.876| 0.926| 0.908| 295| 306| 0.964 |
| `⏎␣⁻␣⁻` | 0.903| 0.919| 0.919| 0.911| 0.911| 295| 295| 1.000 |
| `⏎` | 0.777| 0.775| 0.596| 0.776| 0.675| 293| 381| 0.769 |
| `⏎⏎` | 0.947| 0.698| 0.553| 0.804| 0.698| 179| 226| 0.792 |
| `weighted avg` | 0.946| 0.947| 0.922| 0.946| 0.931| 7146| 7336| 0.974 |
| `micro avg` | 0.947| 0.947| 0.922| 0.947| 0.934| 7146| 7336| 0.974 |
| `macro avg` | 0.922| 0.887| 0.832| 0.901| 0.867| 7146| 7336| 0.974 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|18 |3116 |78 |0 |4 |0 |9 |0 |
|12 |48 |2028 |11 |16 |16 |20 |0 |
|14 |9 |0 |729 |0 |0 |0 |0 |
|88 |27 |32 |0 |227 |0 |0 |7 |
|11 |18 |2 |7 |0 |268 |0 |0 |
|0 |18 |5 |0 |1 |0 |271 |0 |
|47 |0 |10 |0 |44 |0 |0 |125 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9818181818181818, "precision": 0.9759036144578314, "recall": 0.9878048780487805, "support": 738}, "macro avg": {"f1-score": 0.9014612253020278, "precision": 0.9216071919934944, "recall": 0.8868175335546354, "support": 7146}, "micro avg": {"f1-score": 0.9465435208508256, "precision": 0.9465435208508256, "recall": 0.9465435208508256, "support": 7146}, "weighted avg": {"f1-score": 0.9459967841675684, "precision": 0.9464572880878268, "recall": 0.9465435208508256, "support": 7146}, "\u2205": {"f1-score": 0.9672512804594133, "precision": 0.9629171817058096, "recall": 0.9716245712503898, "support": 3207}, "\u23ce": {"f1-score": 0.7760683760683761, "precision": 0.7773972602739726, "recall": 0.7747440273037542, "support": 293}, "\u23ce\u23ce": {"f1-score": 0.8038585209003216, "precision": 0.946969696969697, "recall": 0.6983240223463687, "support": 179}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.92573402417962, "precision": 0.9436619718309859, "recall": 0.9084745762711864, "support": 295}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9109243697478991, "precision": 0.9033333333333333, "recall": 0.9186440677966101, "support": 295}, "\u2423": {"f1-score": 0.9445738239403818, "precision": 0.9410672853828306, "recall": 0.9481065918653576, "support": 2139}},
  "cl_report_full": {"\u0027": {"f1-score": 0.972648432288192, "precision": 0.9759036144578314, "recall": 0.9694148936170213, "support": 752}, "macro avg": {"f1-score": 0.8673515471035796, "precision": 0.9216071919934944, "recall": 0.8316846670877338, "support": 7336}, "micro avg": {"f1-score": 0.934125120839663, "precision": 0.9465435208508256, "recall": 0.9220283533260633, "support": 7336}, "weighted avg": {"f1-score": 0.9309977006473366, "precision": 0.9445161619854341, "recall": 0.9220283533260633, "support": 7336}, "\u2205": {"f1-score": 0.964556570190373, "precision": 0.9629171817058096, "recall": 0.9662015503875969, "support": 3225}, "\u23ce": {"f1-score": 0.6745913818722139, "precision": 0.7773972602739726, "recall": 0.5958005249343832, "support": 381}, "\u23ce\u23ce": {"f1-score": 0.6983240223463687, "precision": 0.946969696969697, "recall": 0.5530973451327433, "support": 226}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9084745762711863, "precision": 0.9436619718309859, "recall": 0.8758169934640523, "support": 306}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9109243697478991, "precision": 0.9033333333333333, "recall": 0.9186440677966101, "support": 295}, "\u2423": {"f1-score": 0.9419414770088249, "precision": 0.9410672853828306, "recall": 0.9428172942817294, "support": 2151}},
  "ppcr": 0.9741003271537623
}
```
</details>