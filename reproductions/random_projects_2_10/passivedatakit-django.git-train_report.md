# Train report for javascript / file:///tmp/top-repos-quality-repos-bsx51bvb/passivedatakit-django.git HEAD e5db20d26d8b45d3288c787cae85e3c2bf4b80a7

### Classification report

PPCR: 0.823

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 0.980| 0.937| 0.964| 0.943| 15982| 16722| 0.956 |
| `␣` | 0.940| 0.909| 0.816| 0.924| 0.874| 6724| 7486| 0.898 |
| `'` | 1.000| 1.000| 0.433| 1.000| 0.604| 466| 1076| 0.433 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 208| 1424| 0.146 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 101| 817| 0.124 |
| `⏎␣⁻␣⁻` | 0.472| 0.708| 0.362| 0.567| 0.410| 24| 47| 0.511 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 45| 0.156 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 0.750| 0.143| 0.857| 0.250| 4| 21| 0.190 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 29| 0.034 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 905| 0.000 |
| `macro avg` | 0.436| 0.435| 0.269| 0.431| 0.308| 23517| 28572| 0.823 |
| `weighted avg` | 0.934| 0.947| 0.779| 0.940| 0.804| 23517| 28572| 0.823 |
| `micro avg` | 0.947| 0.947| 0.779| 0.947| 0.855| 23517| 28572| 0.823 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|740 |15668 |314 |0 |0 |0 |0 |0 |0 |0 |0 |
|762 |613 |6110 |0 |0 |0 |0 |0 |0 |1 |0 |
|610 |0 |0 |466 |0 |0 |0 |0 |0 |0 |0 |
|1216 |136 |56 |0 |0 |0 |0 |0 |0 |16 |0 |
|716 |84 |15 |0 |0 |0 |0 |0 |0 |2 |0 |
|28 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|17 |1 |0 |0 |0 |0 |0 |3 |0 |0 |0 |
|38 |2 |5 |0 |0 |0 |0 |0 |0 |0 |0 |
|23 |7 |0 |0 |0 |0 |0 |0 |0 |17 |0 |
|905 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/pdk/js/lib/rickshaw.js | 1157 |
| static/pdk/js/issues.js | 49 |
| static/pdk/js/home.js | 18 |
| static/pdk/js/generator.js | 14 |
| static/pdk/js/export.js | 5 |
| static/pdk/js/system-health.js | 3 |
| static/pdk/js/basic.js | 3 |
| static/pdk/js/source.js | 3 |
| static/pdk/js/common.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 466}, "macro avg": {"f1-score": 0.431220941590203, "precision": 0.4361020757905914, "recall": 0.4347371536707727, "support": 23517}, "micro avg": {"f1-score": 0.9467193944805885, "precision": 0.9467193944805885, "recall": 0.9467193944805885, "support": 23517}, "weighted avg": {"f1-score": 0.9401272969796441, "precision": 0.9340872634228746, "recall": 0.9467193944805885, "support": 23517}, "\u2205": {"f1-score": 0.9643923306558335, "precision": 0.9489431288232087, "recall": 0.9803528970091353, "support": 15982}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 208}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5666666666666667, "precision": 0.4722222222222222, "recall": 0.7083333333333334, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8571428571428571, "precision": 1.0, "recall": 0.75, "support": 4}, "\u2423": {"f1-score": 0.924007561436673, "precision": 0.939855406860483, "recall": 0.9086853063652588, "support": 6724}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 905}, "\u0027": {"f1-score": 0.6044098573281452, "precision": 1.0, "recall": 0.43308550185873607, "support": 1076}, "macro avg": {"f1-score": 0.30806350016553996, "precision": 0.4361020757905914, "recall": 0.2690804256172709, "support": 28572}, "micro avg": {"f1-score": 0.8548445929082916, "precision": 0.9467193944805885, "recall": 0.7792244155116897, "support": 28572}, "weighted avg": {"f1-score": 0.8043751776934746, "precision": 0.8407944498244335, "recall": 0.7792244155116897, "support": 28572}, "\u2205": {"f1-score": 0.9429181837330364, "precision": 0.9489431288232087, "recall": 0.936969262049994, "support": 16722}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1424}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 817}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.40963855421686746, "precision": 0.4722222222222222, "recall": 0.3617021276595745, "support": 47}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.25, "precision": 1.0, "recall": 0.14285714285714285, "support": 21}, "\u2423": {"f1-score": 0.8736684063773504, "precision": 0.939855406860483, "recall": 0.8161902217472615, "support": 7486}},
  "ppcr": 0.8230785384292314
}
```
</details>
