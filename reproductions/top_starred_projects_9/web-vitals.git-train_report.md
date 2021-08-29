# Train report for javascript / file:///tmp/top-repos-quality-repos-d0fkp1fk/web-vitals.git HEAD ad735bf7dbc811f36f9673b25b0f5cd3ab042961

### Classification report

PPCR: 0.773

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.954| 1.000| 0.964| 0.977| 0.959| 5408| 5610| 0.964 |
| `␣` | 0.991| 0.752| 0.423| 0.855| 0.593| 835| 1485| 0.562 |
| `⏎` | 0.977| 0.921| 0.583| 0.948| 0.730| 453| 715| 0.634 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 339| 678| 0.500 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 342| 0.061 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 155| 0.071 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 154| 0.006 |
| `weighted avg` | 0.958| 0.961| 0.743| 0.957| 0.792| 7068| 9139| 0.773 |
| `micro avg` | 0.961| 0.961| 0.743| 0.961| 0.838| 7068| 9139| 0.773 |
| `macro avg` | 0.560| 0.525| 0.353| 0.540| 0.421| 7068| 9139| 0.773 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|202 |5408 |0 |0 |0 |0 |0 |0 |
|650 |207 |628 |0 |0 |0 |0 |0 |
|262 |31 |5 |417 |0 |0 |0 |0 |
|339 |0 |0 |0 |339 |0 |0 |0 |
|321 |14 |0 |7 |0 |0 |0 |0 |
|144 |7 |1 |3 |0 |0 |0 |0 |
|153 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/e2e/getCLS-test.js | 60 |
| rollup.config.js | 40 |
| wdio.conf.js | 36 |
| test/e2e/getLCP-test.js | 22 |
| test/e2e/getTTFB-test.js | 19 |
| test/e2e/getFID-test.js | 19 |
| test/e2e/getFCP-test.js | 18 |
| test/server.js | 15 |
| test/utils/beacons.js | 13 |
| test/utils/stubForwardBack.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 339}, "macro avg": {"f1-score": 0.5398937854284982, "precision": 0.5601779306032034, "recall": 0.5246608013868196, "support": 7068}, "micro avg": {"f1-score": 0.9609507640067911, "precision": 0.9609507640067911, "recall": 0.9609507640067911, "support": 7068}, "weighted avg": {"f1-score": 0.9568902008782368, "precision": 0.95761396406413, "recall": 0.9609507640067911, "support": 7068}, "\u2205": {"f1-score": 0.9765258215962441, "precision": 0.9541284403669725, "recall": 1.0, "support": 5408}, "\u23ce": {"f1-score": 0.9477272727272728, "precision": 0.9765807962529274, "recall": 0.9205298013245033, "support": 453}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.85500340367597, "precision": 0.9905362776025236, "recall": 0.7520958083832335, "support": 835}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 678}, "macro avg": {"f1-score": 0.4212474429796345, "precision": 0.5601779306032034, "recall": 0.352872182283947, "support": 9139}, "micro avg": {"f1-score": 0.8381563521934966, "precision": 0.9609507640067911, "recall": 0.7431885326622168, "support": 9139}, "weighted avg": {"f1-score": 0.7916137973368346, "precision": 0.8972384497230886, "recall": 0.7431885326622168, "support": 9139}, "\u2205": {"f1-score": 0.9590352899450258, "precision": 0.9541284403669725, "recall": 0.9639928698752228, "support": 5610}, "\u23ce": {"f1-score": 0.7302977232924693, "precision": 0.9765807962529274, "recall": 0.5832167832167832, "support": 715}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 342}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u2423": {"f1-score": 0.5927324209532798, "precision": 0.9905362776025236, "recall": 0.4228956228956229, "support": 1485}},
  "ppcr": 0.7733887733887734
}
```
</details>
