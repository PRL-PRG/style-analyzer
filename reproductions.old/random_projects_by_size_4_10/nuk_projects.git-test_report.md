# Test report for javascript / file:///tmp/top-repos-quality-repos-1s6pl1mw/nuk_projects.git HEAD d33f7d0147e45526ab884805dd599cfb651e36fd

### Classification report

PPCR: 0.507

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.952| 0.966| 0.827| 0.959| 0.885| 326| 381| 0.856 |
| `␣` | 0.857| 0.825| 0.251| 0.841| 0.388| 80| 263| 0.304 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 60| 0.033 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 82| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 13| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `macro avg` | 0.258| 0.256| 0.154| 0.257| 0.182| 408| 805| 0.507 |
| `weighted avg` | 0.928| 0.934| 0.473| 0.931| 0.546| 408| 805| 0.507 |
| `micro avg` | 0.934| 0.934| 0.473| 0.934| 0.628| 408| 805| 0.507 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|55 |315 |11 |0 |0 |0 |0 |0 |
|183 |14 |66 |0 |0 |0 |0 |0 |
|58 |2 |0 |0 |0 |0 |0 |0 |
|82 |0 |0 |0 |0 |0 |0 |0 |
|13 |0 |0 |0 |0 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.25709549154274747, "precision": 0.2584006412232567, "recall": 0.2558939526730938, "support": 408}, "micro avg": {"f1-score": 0.9338235294117647, "precision": 0.9338235294117647, "recall": 0.9338235294117647, "support": 408}, "weighted avg": {"f1-score": 0.9310389368206928, "precision": 0.9284635304272766, "recall": 0.9338235294117647, "support": 408}, "\u2205": {"f1-score": 0.958904109589041, "precision": 0.9516616314199395, "recall": 0.9662576687116564, "support": 326}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8407643312101911, "precision": 0.8571428571428571, "recall": 0.825, "support": 80}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "macro avg": {"f1-score": 0.18186667925597202, "precision": 0.2584006412232567, "recall": 0.15396031769793034, "support": 805}, "micro avg": {"f1-score": 0.6281945589447651, "precision": 0.9338235294117647, "recall": 0.47329192546583854, "support": 805}, "weighted avg": {"f1-score": 0.5456231911426025, "precision": 0.730449258384557, "recall": 0.47329192546583854, "support": 805}, "\u2205": {"f1-score": 0.8848314606741573, "precision": 0.9516616314199395, "recall": 0.8267716535433071, "support": 381}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 60}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.388235294117647, "precision": 0.8571428571428571, "recall": 0.2509505703422053, "support": 263}},
  "ppcr": 0.506832298136646
}
```
</details>
