# Train report for javascript / file:///tmp/top-repos-quality-repos-0xta8ssd/visualization-for-transportation.git HEAD 5f4921b1dd5f48e8644a7aed60c88891e5d39ac3

### Classification report

PPCR: 0.224

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.940| 0.998| 0.380| 0.968| 0.541| 3354| 8798| 0.381 |
| `"` | 1.000| 1.000| 0.559| 1.000| 0.717| 681| 1219| 0.559 |
| `␣` | 0.929| 0.479| 0.018| 0.633| 0.036| 219| 5682| 0.039 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 68| 1140| 0.060 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 278| 0.058 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 399| 0.035 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 333| 0.009 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1120| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 254| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 245| 0.000 |
| `macro avg` | 0.287| 0.248| 0.096| 0.260| 0.129| 4355| 19468| 0.224 |
| `micro avg` | 0.949| 0.949| 0.212| 0.949| 0.347| 4355| 19468| 0.224 |
| `weighted avg` | 0.927| 0.949| 0.212| 0.933| 0.300| 4355| 19468| 0.224 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| '| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5444 |3346 |8 |0 |0 |0 |0 |0 |0 |0 |0 |
|5463 |114 |105 |0 |0 |0 |0 |0 |0 |0 |0 |
|538 |0 |0 |681 |0 |0 |0 |0 |0 |0 |0 |
|1072 |68 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1120 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|385 |14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|330 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|254 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|262 |16 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|245 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/js/jquery.ui.js | 121 |
| static/js/calendar.js | 82 |
| static/js/index.js | 19 |
| static/js/trail.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 681}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2600281385847076, "precision": 0.2868827241030535, "recall": 0.2477066843106983, "support": 4355}, "micro avg": {"f1-score": 0.9487944890929966, "precision": 0.9487944890929966, "recall": 0.9487944890929966, "support": 4355}, "weighted avg": {"f1-score": 0.9334929599128013, "precision": 0.9267493614400186, "recall": 0.9487944890929966, "support": 4355}, "\u2205": {"f1-score": 0.9677512653651483, "precision": 0.939623701207526, "recall": 0.9976147883124628, "support": 3354}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.6325301204819277, "precision": 0.9292035398230089, "recall": 0.4794520547945205, "support": 219}},
  "cl_report_full": {"\"": {"f1-score": 0.7168421052631578, "precision": 1.0, "recall": 0.5586546349466776, "support": 1219}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1120}, "macro avg": {"f1-score": 0.12945479978785376, "precision": 0.2868827241030535, "recall": 0.09574477512664324, "support": 19468}, "micro avg": {"f1-score": 0.34689165932082444, "precision": 0.9487944890929966, "recall": 0.21224573659338403, "support": 19468}, "weighted avg": {"f1-score": 0.30016277669812075, "precision": 0.7584520154354916, "recall": 0.21224573659338403, "support": 19468}, "\u2205": {"f1-score": 0.5414677562909621, "precision": 0.939623701207526, "recall": 0.380313707660832, "support": 8798}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1140}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 399}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 333}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 254}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 278}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "\u2423": {"f1-score": 0.0362381363244176, "precision": 0.9292035398230089, "recall": 0.018479408658922915, "support": 5682}},
  "ppcr": 0.22370043147729607
}
```
</details>
