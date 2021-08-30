# Train report for javascript / file:///tmp/top-repos-quality-repos-1flcresv/flexslider.git HEAD 690832b7f972298e76e2965714657a2beec9e35c

### Classification report

PPCR: 0.508

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 0.998| 0.868| 0.993| 0.925| 11072| 12723| 0.870 |
| `␣` | 0.894| 0.811| 0.074| 0.850| 0.137| 624| 6806| 0.092 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 1014| 0.034 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 395| 0.005 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 251| 0.004 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 822| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 577| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 409| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 106| 0.000 |
| `weighted avg` | 0.981| 0.984| 0.500| 0.983| 0.550| 11733| 23103| 0.508 |
| `micro avg` | 0.984| 0.984| 0.500| 0.984| 0.663| 11733| 23103| 0.508 |
| `macro avg` | 0.209| 0.201| 0.105| 0.205| 0.118| 11733| 23103| 0.508 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1651 |11045 |27 |0 |0 |0 |0 |0 |0 |0 |
|6182 |118 |506 |0 |0 |0 |0 |0 |0 |0 |
|980 |2 |32 |0 |0 |0 |0 |0 |0 |0 |
|822 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|577 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|409 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|393 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|250 |0 |1 |0 |0 |0 |0 |0 |0 |0 |
|106 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| demo/js/jquery.easing.js | 59 |
| demo/js/jquery.flexslider.js | 44 |
| jquery.flexslider.js | 44 |
| demo/js/jquery.mousewheel.js | 24 |
| demo/js/jquery.fitvid.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2048578029470103, "precision": 0.20922976509429653, "recall": 0.2009398724536008, "support": 11733}, "micro avg": {"f1-score": 0.9844881956873774, "precision": 0.9844881956873774, "recall": 0.9844881956873774, "support": 11733}, "weighted avg": {"f1-score": 0.9825688598054457, "precision": 0.9808991280637767, "recall": 0.9844881956873774, "support": 11733}, "\u2205": {"f1-score": 0.9933000584558658, "precision": 0.989074952986478, "recall": 0.9975614161849711, "support": 11072}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.850420168067227, "precision": 0.8939929328621908, "recall": 0.8108974358974359, "support": 624}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 822}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 577}, "macro avg": {"f1-score": 0.11799231637398384, "precision": 0.20922976509429653, "recall": 0.10471767017896787, "support": 23103}, "micro avg": {"f1-score": 0.663164542427374, "precision": 0.9844881956873774, "recall": 0.4999783577890317, "support": 23103}, "weighted avg": {"f1-score": 0.5496551535792334, "precision": 0.8080559463232926, "recall": 0.4999783577890317, "support": 23103}, "\u2205": {"f1-score": 0.9246546672247802, "precision": 0.989074952986478, "recall": 0.8681128664623123, "support": 12723}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1014}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 251}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 409}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 395}, "\u2423": {"f1-score": 0.13727618014107434, "precision": 0.8939929328621908, "recall": 0.07434616514839847, "support": 6806}},
  "ppcr": 0.5078561225814829
}
```
</details>
