# Train report for javascript / file:///tmp/top-repos-quality-repos-akky18gy/fdir.git HEAD f832c2996d85407b209d0b2d6578c5971261c379

### Classification report

PPCR: 0.609

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 0.996| 0.793| 0.982| 0.872| 3286| 4129| 0.796 |
| `␣` | 0.975| 0.897| 0.357| 0.934| 0.523| 687| 1725| 0.398 |
| `"` | 1.000| 1.000| 0.505| 1.000| 0.671| 217| 430| 0.505 |
| `⏎␣⁻␣⁻` | 1.000| 0.969| 0.745| 0.984| 0.854| 163| 212| 0.769 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 232| 0.091 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 344| 0.044 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 137| 0.000 |
| `weighted avg` | 0.964| 0.972| 0.591| 0.967| 0.689| 4389| 7209| 0.609 |
| `micro avg` | 0.972| 0.972| 0.591| 0.972| 0.735| 4389| 7209| 0.609 |
| `macro avg` | 0.563| 0.552| 0.343| 0.557| 0.417| 4389| 7209| 0.609 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|843 |3273 |13 |0 |0 |0 |0 |0 |
|1038 |71 |616 |0 |0 |0 |0 |0 |
|213 |0 |0 |217 |0 |0 |0 |0 |
|329 |15 |0 |0 |0 |0 |0 |0 |
|211 |19 |2 |0 |0 |0 |0 |0 |
|49 |4 |1 |0 |0 |0 |158 |0 |
|137 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/api/shared.js | 44 |
| benchmarks/glob-benchmark.js | 13 |
| benchmarks/benchmark.js | 11 |
| src/api/fns.js | 10 |
| src/compat/fs.js | 10 |
| benchmarks/fdir-benchmark.js | 10 |
| __tests__/fdir.test.js | 9 |
| src/builder/index.js | 7 |
| benchmarks/export.js | 6 |
| benchmarks/sysinfo.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 217}, "macro avg": {"f1-score": 0.5571668964838212, "precision": 0.5632077277534613, "recall": 0.5517172980394952, "support": 4389}, "micro avg": {"f1-score": 0.9715197083618137, "precision": 0.9715197083618137, "recall": 0.9715197083618137, "support": 4389}, "weighted avg": {"f1-score": 0.9671964933693505, "precision": 0.9637050859283542, "recall": 0.9715197083618137, "support": 4389}, "\u2205": {"f1-score": 0.9817036592681464, "precision": 0.9677705499704317, "recall": 0.9960438222763238, "support": 3286}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9844236760124611, "precision": 1.0, "recall": 0.9693251533742331, "support": 163}, "\u2423": {"f1-score": 0.934040940106141, "precision": 0.9746835443037974, "recall": 0.8966521106259098, "support": 687}},
  "cl_report_full": {"\"": {"f1-score": 0.6707882534775889, "precision": 1.0, "recall": 0.5046511627906977, "support": 430}, "macro avg": {"f1-score": 0.4170089172799142, "precision": 0.5632077277534613, "recall": 0.3428173587560607, "support": 7209}, "micro avg": {"f1-score": 0.7352991895154337, "precision": 0.9715197083618137, "recall": 0.5914828686364267, "support": 7209}, "weighted avg": {"f1-score": 0.6893697370739876, "precision": 0.876578404043829, "recall": 0.5914828686364267, "support": 7209}, "\u2205": {"f1-score": 0.8715217680734922, "precision": 0.9677705499704317, "recall": 0.7926858803584403, "support": 4129}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 344}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 137}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 232}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.854054054054054, "precision": 1.0, "recall": 0.7452830188679245, "support": 212}, "\u2423": {"f1-score": 0.522698345354264, "precision": 0.9746835443037974, "recall": 0.3571014492753623, "support": 1725}},
  "ppcr": 0.6088223054515189
}
```
</details>
