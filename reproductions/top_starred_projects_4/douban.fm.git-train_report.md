# Train report for javascript / file:///tmp/top-repos-quality-repos-wjoo6l8x/douban.fm.git HEAD 777928a28ab43c12733505a9374a8323d44df4b5

### Classification report

PPCR: 0.490

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 0.991| 0.719| 0.992| 0.833| 2101| 2899| 0.725 |
| `␣` | 0.932| 0.997| 0.268| 0.964| 0.416| 360| 1341| 0.268 |
| `'` | 1.000| 1.000| 0.492| 1.000| 0.660| 217| 441| 0.492 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 153| 0.072 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 204| 0.039 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 357| 0.014 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 122| 0.000 |
| `macro avg` | 0.418| 0.427| 0.211| 0.422| 0.273| 2702| 5517| 0.490 |
| `weighted avg` | 0.976| 0.984| 0.482| 0.980| 0.592| 2702| 5517| 0.490 |
| `micro avg` | 0.984| 0.984| 0.482| 0.984| 0.647| 2702| 5517| 0.490 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|798 |2083 |18 |0 |0 |0 |0 |0 |
|981 |1 |359 |0 |0 |0 |0 |0 |
|224 |0 |0 |217 |0 |0 |0 |0 |
|352 |0 |5 |0 |0 |0 |0 |0 |
|196 |5 |3 |0 |0 |0 |0 |0 |
|142 |11 |0 |0 |0 |0 |0 |0 |
|122 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| libs/fm.js | 21 |
| libs/template.js | 7 |
| libs/utils.js | 6 |
| libs/commands.js | 5 |
| libs/sdk.js | 2 |
| libs/menu.js | 1 |
| libs/errors.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 217}, "macro avg": {"f1-score": 0.4222038627975853, "precision": 0.4177674706246135, "recall": 0.42695069619153386, "support": 2702}, "micro avg": {"f1-score": 0.9840858623242043, "precision": 0.9840858623242043, "recall": 0.9840858623242043, "support": 2702}, "weighted avg": {"f1-score": 0.9798108269649847, "precision": 0.9758253946891995, "recall": 0.9840858623242043, "support": 2702}, "\u2205": {"f1-score": 0.991668650321352, "precision": 0.991904761904762, "recall": 0.991432651118515, "support": 2101}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.963758389261745, "precision": 0.9324675324675324, "recall": 0.9972222222222222, "support": 360}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6595744680851063, "precision": 1.0, "recall": 0.49206349206349204, "support": 441}, "macro avg": {"f1-score": 0.27270455306162295, "precision": 0.4177674706246135, "recall": 0.21118539779783435, "support": 5517}, "micro avg": {"f1-score": 0.6470373524759703, "precision": 0.9840858623242043, "recall": 0.4819648359615733, "support": 5517}, "weighted avg": {"f1-score": 0.5917429572898804, "precision": 0.8277996856626547, "recall": 0.4819648359615733, "support": 5517}, "\u2205": {"f1-score": 0.833366673334667, "precision": 0.991904761904762, "recall": 0.7185236288375302, "support": 2899}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 357}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 204}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 153}, "\u2423": {"f1-score": 0.41599073001158754, "precision": 0.9324675324675324, "recall": 0.2677106636838181, "support": 1341}},
  "ppcr": 0.4897589269530542
}
```
</details>
