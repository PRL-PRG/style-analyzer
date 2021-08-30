# Train report for javascript / file:///tmp/top-repos-quality-repos-2epzhbcy/front-end-roadmap.git HEAD 18d638411ab9b282cb1ccd352e1edb1fed2a141c

### Classification report

PPCR: 0.217

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.947| 1.000| 0.434| 0.973| 0.595| 570| 1313| 0.434 |
| `"` | 1.000| 1.000| 0.924| 1.000| 0.961| 438| 474| 0.924 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 162| 0.099 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 449| 0.031 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 180| 0.011 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2225| 0.000 |
| `micro avg` | 0.969| 0.969| 0.210| 0.969| 0.345| 1040| 4803| 0.217 |
| `macro avg` | 0.324| 0.333| 0.226| 0.329| 0.259| 1040| 4803| 0.217 |
| `weighted avg` | 0.940| 0.969| 0.210| 0.954| 0.258| 1040| 4803| 0.217 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|2225 |0 |0 |0 |0 |0 |0 |
|743 |0 |570 |0 |0 |0 |0 |
|36 |0 |0 |438 |0 |0 |0 |
|435 |0 |14 |0 |0 |0 |0 |
|178 |0 |2 |0 |0 |0 |0 |
|146 |0 |16 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/page/index/drawRoadmap.js | 23 |
| src/page/index/roadmap.js | 4 |
| config-overrides.js | 3 |
| src/index.js | 1 |
| src/serviceWorker.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 438}, "macro avg": {"f1-score": 0.32878270762229805, "precision": 0.32447397563676633, "recall": 0.3333333333333333, "support": 1040}, "micro avg": {"f1-score": 0.9692307692307692, "precision": 0.9692307692307692, "recall": 0.9692307692307692, "support": 1040}, "weighted avg": {"f1-score": 0.9542662116040955, "precision": 0.9400971121901354, "recall": 0.9692307692307692, "support": 1040}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.9726962457337883, "precision": 0.946843853820598, "recall": 1.0, "support": 570}},
  "cl_report_full": {"\"": {"f1-score": 0.9605263157894737, "precision": 1.0, "recall": 0.9240506329113924, "support": 474}, "macro avg": {"f1-score": 0.25930442948101323, "precision": 0.32447397563676633, "recall": 0.22636182800363777, "support": 4803}, "micro avg": {"f1-score": 0.34502823891836387, "precision": 0.9692307692307692, "recall": 0.2098688319800125, "support": 4803}, "weighted avg": {"f1-score": 0.2575304427449623, "precision": 0.35752779097781495, "recall": 0.2098688319800125, "support": 4803}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2225}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 449}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 162}, "\u2423": {"f1-score": 0.5953002610966057, "precision": 0.946843853820598, "recall": 0.43412033511043413, "support": 1313}},
  "ppcr": 0.21653133458255258
}
```
</details>
