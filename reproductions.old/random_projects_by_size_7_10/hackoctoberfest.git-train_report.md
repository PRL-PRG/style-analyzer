# Train report for javascript / file:///tmp/top-repos-quality-repos-zhh05yep/hackoctoberfest.git HEAD 5bd052f6bb8df4f1a622368e336231e7b5c414fb

### Classification report

PPCR: 0.862

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.972| 0.945| 0.983| 0.969| 11797| 12140| 0.972 |
| `␣` | 0.936| 0.993| 0.936| 0.964| 0.936| 9714| 10315| 0.942 |
| `'` | 1.000| 1.000| 0.946| 1.000| 0.972| 2104| 2224| 0.946 |
| `⏎` | 0.990| 0.775| 0.501| 0.870| 0.665| 1058| 1637| 0.646 |
| `⏎⏎` | 0.975| 0.831| 0.217| 0.897| 0.355| 231| 886| 0.261 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 63| 664| 0.095 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 619| 0.018 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 268| 0.000 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 231| 0.000 |
| `weighted avg` | 0.969| 0.970| 0.836| 0.968| 0.862| 24978| 28984| 0.862 |
| `micro avg` | 0.970| 0.970| 0.836| 0.970| 0.898| 24978| 28984| 0.862 |
| `macro avg` | 0.544| 0.508| 0.394| 0.524| 0.433| 24978| 28984| 0.862 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎⇥⁻| ⏎⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|343 |11469 |327 |0 |0 |1 |0 |0 |0 |0 |
|601 |64 |9650 |0 |0 |0 |0 |0 |0 |0 |
|120 |0 |0 |2104 |0 |0 |0 |0 |0 |0 |
|579 |5 |229 |0 |820 |4 |0 |0 |0 |0 |
|655 |0 |31 |0 |8 |192 |0 |0 |0 |0 |
|601 |0 |63 |0 |0 |0 |0 |0 |0 |0 |
|608 |3 |8 |0 |0 |0 |0 |0 |0 |0 |
|268 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|231 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/reveal.js | 507 |
| plugin/markdown/markdown.js | 73 |
| plugin/multiplex/index.js | 50 |
| plugin/zoom-js/zoom.js | 33 |
| plugin/notes/notes.js | 23 |
| plugin/multiplex/client.js | 12 |
| plugin/math/math.js | 11 |
| plugin/notes-server/index.js | 11 |
| plugin/print-pdf/print-pdf.js | 11 |
| plugin/notes-server/client.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2104}, "macro avg": {"f1-score": 0.5237290795759486, "precision": 0.5438761011870944, "recall": 0.5079804424639086, "support": 24978}, "micro avg": {"f1-score": 0.9702538233645608, "precision": 0.9702538233645608, "recall": 0.9702538233645608, "support": 24978}, "weighted avg": {"f1-score": 0.9684429661539236, "precision": 0.968621790804937, "recall": 0.9702538233645608, "support": 24978}, "\u2205": {"f1-score": 0.9828605707429943, "precision": 0.9937613724980504, "recall": 0.9721963210985843, "support": 11797}, "\u23ce": {"f1-score": 0.8695652173913044, "precision": 0.9903381642512077, "recall": 0.775047258979206, "support": 1058}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u23ce": {"f1-score": 0.8971962616822431, "precision": 0.9746192893401016, "recall": 0.8311688311688312, "support": 231}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9639396663669962, "precision": 0.9361660845944897, "recall": 0.9934115709285567, "support": 9714}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9722735674676526, "precision": 1.0, "recall": 0.9460431654676259, "support": 2224}, "macro avg": {"f1-score": 0.43295906296438624, "precision": 0.5438761011870944, "recall": 0.39376919072019345, "support": 28984}, "micro avg": {"f1-score": 0.8982246766242912, "precision": 0.9702538233645608, "recall": 0.8361509798509522, "support": 28984}, "weighted avg": {"f1-score": 0.8617852684772869, "precision": 0.9118655979144719, "recall": 0.8361509798509522, "support": 28984}, "\u2205": {"f1-score": 0.9686246357839619, "precision": 0.9937613724980504, "recall": 0.9447281713344317, "support": 12140}, "\u23ce": {"f1-score": 0.665314401622718, "precision": 0.9903381642512077, "recall": 0.5009163103237629, "support": 1637}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 664}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 619}, "\u23ce\u23ce": {"f1-score": 0.3545706371191136, "precision": 0.9746192893401016, "recall": 0.21670428893905191, "support": 886}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 231}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 268}, "\u2423": {"f1-score": 0.9358483246860301, "precision": 0.9361660845944897, "recall": 0.9355307804168687, "support": 10315}},
  "ppcr": 0.8617858128622689
}
```
</details>
