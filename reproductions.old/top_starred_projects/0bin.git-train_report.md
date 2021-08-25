# Train report for javascript / file:///tmp/top-repos-quality-repos-v3vkr_ue/0bin.git HEAD 3e0c4da19302bee38f6cb009362e2f7071b88705

### Classification report

PPCR: 0.794

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 0.989| 0.951| 0.989| 0.969| 37703| 39187| 0.962 |
| `␣` | 0.954| 0.995| 0.960| 0.974| 0.957| 24866| 25772| 0.965 |
| `⏎` | 0.966| 0.714| 0.242| 0.821| 0.388| 1863| 5486| 0.340 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 366| 3271| 0.112 |
| `⏎⏎` | 0.968| 0.492| 0.102| 0.652| 0.185| 303| 1461| 0.207 |
| `⏎␣⁻␣⁻` | 1.000| 0.601| 0.048| 0.751| 0.092| 248| 3090| 0.080 |
| `'` | 1.000| 1.000| 0.061| 1.000| 0.114| 178| 2936| 0.061 |
| `"` | 1.000| 1.000| 0.082| 1.000| 0.151| 118| 1447| 0.082 |
| `weighted avg` | 0.969| 0.974| 0.774| 0.970| 0.797| 65645| 82650| 0.794 |
| `micro avg` | 0.974| 0.974| 0.774| 0.974| 0.863| 65645| 82650| 0.794 |
| `macro avg` | 0.859| 0.724| 0.306| 0.773| 0.357| 65645| 82650| 0.794 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1484 |37283 |418 |0 |0 |0 |0 |0 |2 |
|906 |119 |24747 |0 |0 |0 |0 |0 |0 |
|3623 |39 |491 |1330 |0 |0 |0 |0 |3 |
|2905 |215 |142 |9 |0 |0 |0 |0 |0 |
|2842 |69 |24 |6 |0 |149 |0 |0 |0 |
|2758 |0 |0 |0 |0 |0 |178 |0 |0 |
|1329 |0 |0 |0 |0 |0 |0 |118 |0 |
|1158 |1 |121 |32 |0 |0 |0 |0 |149 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| zerobin/static/js/vue.js | 1095 |
| docs/.build/html/_static/underscore-1.3.1.js | 149 |
| docs/.build/html/_static/websupport.js | 145 |
| docs/.build/html/_static/searchtools.js | 118 |
| docs/.build/html/_static/doctools.js | 79 |
| zerobin/static/js/behavior.js | 74 |
| docs/.build/html/_static/sidebar.js | 31 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 118}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 178}, "macro avg": {"f1-score": 0.7732967109344749, "precision": 0.8594445829513226, "recall": 0.7238165733049287, "support": 65645}, "micro avg": {"f1-score": 0.9742402315484805, "precision": 0.9742402315484805, "recall": 0.9742402315484805, "support": 65645}, "weighted avg": {"f1-score": 0.9704212759937698, "precision": 0.9690988655845698, "recall": 0.9742402315484805, "support": 65645}, "\u2205": {"f1-score": 0.9885587771281602, "precision": 0.9882574351905847, "recall": 0.9888603028936689, "support": 37703}, "\u23ce": {"f1-score": 0.8209876543209876, "precision": 0.9658678286129266, "recall": 0.7139023081052066, "support": 1863}, "\u23ce\u23ce": {"f1-score": 0.6520787746170679, "precision": 0.9675324675324676, "recall": 0.49174917491749176, "support": 303}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 366}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7506297229219143, "precision": 1.0, "recall": 0.6008064516129032, "support": 248}, "\u2423": {"f1-score": 0.9741187584876695, "precision": 0.953898932274602, "recall": 0.9952143489101585, "support": 24866}},
  "cl_report_full": {"\"": {"f1-score": 0.15079872204472844, "precision": 1.0, "recall": 0.08154803040774015, "support": 1447}, "\u0027": {"f1-score": 0.11432241490044959, "precision": 1.0, "recall": 0.060626702997275204, "support": 2936}, "macro avg": {"f1-score": 0.3569710814384166, "precision": 0.8594445829513226, "recall": 0.30580695532235197, "support": 82650}, "micro avg": {"f1-score": 0.8625240230621397, "precision": 0.9742402315484805, "recall": 0.7737931034482759, "support": 82650}, "weighted avg": {"f1-score": 0.797222106343144, "precision": 0.9376411765061093, "recall": 0.7737931034482759, "support": 82650}, "\u2205": {"f1-score": 0.9694850025353321, "precision": 0.9882574351905847, "recall": 0.951412458213183, "support": 39187}, "\u23ce": {"f1-score": 0.3875856039632814, "precision": 0.9658678286129266, "recall": 0.24243528982865475, "support": 5486}, "\u23ce\u23ce": {"f1-score": 0.18452012383900931, "precision": 0.9675324675324676, "recall": 0.10198494182067078, "support": 1461}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3271}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.09200370484717506, "precision": 1.0, "recall": 0.0482200647249191, "support": 3090}, "\u2423": {"f1-score": 0.9570530793773566, "precision": 0.953898932274602, "recall": 0.9602281545863728, "support": 25772}},
  "ppcr": 0.7942528735632184
}
```
</details>
