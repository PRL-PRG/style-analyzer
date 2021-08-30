# Train report for javascript / file:///tmp/top-repos-quality-repos-a4ywgfc9/npm-check-updates.git HEAD cf0a52ca7d569422072954903081f6f8b4f8a39c

### Classification report

PPCR: 0.673

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.988| 1.000| 0.949| 0.994| 0.968| 13717| 14458| 0.949 |
| `'` | 1.000| 1.000| 0.983| 1.000| 0.991| 4621| 4702| 0.983 |
| `␣` | 0.975| 0.992| 0.230| 0.983| 0.372| 1746| 7541| 0.232 |
| `⏎` | 0.959| 0.866| 0.288| 0.910| 0.443| 543| 1630| 0.333 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 75| 921| 0.081 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 846| 0.038 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 31| 634| 0.049 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 114| 0.009 |
| `weighted avg` | 0.982| 0.989| 0.666| 0.986| 0.719| 20766| 30846| 0.673 |
| `micro avg` | 0.989| 0.989| 0.666| 0.989| 0.796| 20766| 30846| 0.673 |
| `macro avg` | 0.490| 0.482| 0.306| 0.486| 0.347| 20766| 30846| 0.673 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|741 |13717 |0 |0 |0 |0 |0 |0 |0 |
|5795 |6 |1732 |0 |8 |0 |0 |0 |0 |
|81 |0 |0 |4621 |0 |0 |0 |0 |0 |
|1087 |28 |45 |0 |470 |0 |0 |0 |0 |
|846 |73 |0 |0 |2 |0 |0 |0 |0 |
|814 |30 |0 |0 |2 |0 |0 |0 |0 |
|603 |24 |0 |0 |7 |0 |0 |0 |0 |
|113 |0 |0 |0 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/versionmanager.js | 38 |
| lib/package-managers/yarn.js | 28 |
| lib/package-managers/npm.js | 26 |
| test/index.test.js | 26 |
| test/bin.test.js | 21 |
| test/versionmanager.test.js | 14 |
| lib/index.js | 13 |
| lib/logging.js | 12 |
| lib/version-util.js | 10 |
| bin/cli.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4621}, "macro avg": {"f1-score": 0.4859236846922181, "precision": 0.49028237489310794, "recall": 0.4821929208356275, "support": 20766}, "micro avg": {"f1-score": 0.9891168255802755, "precision": 0.9891168255802755, "recall": 0.9891168255802755, "support": 20766}, "weighted avg": {"f1-score": 0.9856902378271495, "precision": 0.9824467659627066, "recall": 0.9891168255802755, "support": 20766}, "\u2205": {"f1-score": 0.9941656097119044, "precision": 0.9883989047413172, "recall": 1.0, "support": 13717}, "\u23ce": {"f1-score": 0.9099709583736689, "precision": 0.9591836734693877, "recall": 0.8655616942909761, "support": 543}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 0.9832529094521715, "precision": 0.9746764209341587, "recall": 0.9919816723940436, "support": 1746}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9913118095033787, "precision": 1.0, "recall": 0.9827732879625691, "support": 4702}, "macro avg": {"f1-score": 0.34682866662174106, "precision": 0.49028237489310794, "recall": 0.30619283821921484, "support": 30846}, "micro avg": {"f1-score": 0.7959389289312564, "precision": 0.9891168255802755, "recall": 0.6658886079232316, "support": 30846}, "weighted avg": {"f1-score": 0.7192195575512383, "precision": 0.9046805304664967, "recall": 0.6658886079232316, "support": 30846}, "\u2205": {"f1-score": 0.968167701863354, "precision": 0.9883989047413172, "recall": 0.9487480979388574, "support": 14458}, "\u23ce": {"f1-score": 0.4433962264150943, "precision": 0.9591836734693877, "recall": 0.2883435582822086, "support": 1630}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 634}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 921}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 846}, "\u2423": {"f1-score": 0.37175359519210127, "precision": 0.9746764209341587, "recall": 0.22967776157008354, "support": 7541}},
  "ppcr": 0.6732153277572457
}
```
</details>
