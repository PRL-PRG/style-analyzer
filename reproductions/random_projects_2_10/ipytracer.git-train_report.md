# Train report for javascript / file:///tmp/top-repos-quality-repos-_2gfujht/ipytracer.git HEAD fe6505f48a9c79ada9fec3bb61ab2969b0f30609

### Classification report

PPCR: 0.396

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.984| 1.000| 0.714| 0.992| 0.827| 1220| 1709| 0.714 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 715| 0.022 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 102| 0.039 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 300| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 202| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 101| 0.000 |
| `macro avg` | 0.164| 0.167| 0.119| 0.165| 0.138| 1240| 3129| 0.396 |
| `weighted avg` | 0.968| 0.984| 0.390| 0.976| 0.452| 1240| 3129| 0.396 |
| `micro avg` | 0.984| 0.984| 0.390| 0.984| 0.558| 1240| 3129| 0.396 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|489 |1220 |0 |0 |0 |0 |0 |
|699 |16 |0 |0 |0 |0 |0 |
|300 |0 |0 |0 |0 |0 |0 |
|202 |0 |0 |0 |0 |0 |0 |
|98 |4 |0 |0 |0 |0 |0 |
|101 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/src/list2D.js | 9 |
| js/webpack.config.js | 4 |
| js/src/list1D.js | 2 |
| js/src/chart.js | 1 |
| js/src/labplugin.js | 1 |
| js/src/tracer.js | 1 |
| js/src/tree.js | 1 |
| js/src/directed_graph.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.16531165311653115, "precision": 0.1639784946236559, "recall": 0.16666666666666666, "support": 1240}, "micro avg": {"f1-score": 0.9838709677419355, "precision": 0.9838709677419355, "recall": 0.9838709677419355, "support": 1240}, "weighted avg": {"f1-score": 0.9758720167846839, "precision": 0.9680020811654526, "recall": 0.9838709677419355, "support": 1240}, "\u2205": {"f1-score": 0.991869918699187, "precision": 0.9838709677419355, "recall": 1.0, "support": 1220}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 300}, "macro avg": {"f1-score": 0.13789985305753363, "precision": 0.1639784946236559, "recall": 0.11897795982055782, "support": 3129}, "micro avg": {"f1-score": 0.5584802014190889, "precision": 0.9838709677419355, "recall": 0.3899009268136785, "support": 3129}, "weighted avg": {"f1-score": 0.45190958557109295, "precision": 0.5373715192940134, "recall": 0.3899009268136785, "support": 3129}, "\u2205": {"f1-score": 0.8273991183452017, "precision": 0.9838709677419355, "recall": 0.713867758923347, "support": 1709}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 202}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 102}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 715}},
  "ppcr": 0.39629274528603387
}
```
</details>
