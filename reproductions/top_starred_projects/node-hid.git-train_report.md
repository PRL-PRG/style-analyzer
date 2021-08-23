# Train report for javascript / file:///tmp/top-repos-quality-repos-qouhqcfs/node-hid.git HEAD 2cdb2eb5060f1c861dd77d69575a505904cb400a

### Classification report

PPCR: 0.429

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 0.998| 0.740| 0.989| 0.843| 1749| 2359| 0.741 |
| `␣` | 0.983| 0.837| 0.139| 0.904| 0.243| 203| 1224| 0.166 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 121| 0.017 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 275| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 185| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 130| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 133| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 130| 0.000 |
| `weighted avg` | 0.980| 0.981| 0.420| 0.979| 0.502| 1954| 4557| 0.429 |
| `micro avg` | 0.981| 0.981| 0.420| 0.981| 0.589| 1954| 4557| 0.429 |
| `macro avg` | 0.245| 0.229| 0.110| 0.237| 0.136| 1954| 4557| 0.429 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|610 |1746 |3 |0 |0 |0 |0 |0 |0 |
|1021 |33 |170 |0 |0 |0 |0 |0 |0 |
|275 |0 |0 |0 |0 |0 |0 |0 |0 |
|185 |0 |0 |0 |0 |0 |0 |0 |0 |
|130 |0 |0 |0 |0 |0 |0 |0 |0 |
|133 |0 |0 |0 |0 |0 |0 |0 |0 |
|119 |2 |0 |0 |0 |0 |0 |0 |0 |
|130 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/test-blink1.js | 10 |
| src/test-teensyrawhid.js | 9 |
| src/test-bigredbutton.js | 5 |
| src/buzzers.js | 4 |
| src/test-ps3-rumbleled.js | 3 |
| src/test-macbookprotrackpad.js | 3 |
| src/powermate.js | 1 |
| nodehid.js | 1 |
| src/testReadSync.js | 1 |
| src/show-devices.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2366863058284612, "precision": 0.24537588482147782, "recall": 0.22946539472238886, "support": 1954}, "micro avg": {"f1-score": 0.9805527123848515, "precision": 0.9805527123848515, "recall": 0.9805527123848515, "support": 1954}, "weighted avg": {"f1-score": 0.9793940981307951, "precision": 0.9795847640619403, "recall": 0.9805527123848515, "support": 1954}, "\u2205": {"f1-score": 0.9892351274787534, "precision": 0.9803481190342505, "recall": 0.9982847341337907, "support": 1749}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9042553191489362, "precision": 0.9826589595375722, "recall": 0.8374384236453202, "support": 203}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 185}, "macro avg": {"f1-score": 0.13585711618063553, "precision": 0.24537588482147782, "recall": 0.10987912721963167, "support": 4557}, "micro avg": {"f1-score": 0.588542466594993, "precision": 0.9805527123848515, "recall": 0.4204520517884573, "support": 4557}, "weighted avg": {"f1-score": 0.5020102496659512, "precision": 0.7714320340741244, "recall": 0.4204520517884573, "support": 4557}, "\u2205": {"f1-score": 0.8434782608695652, "precision": 0.9803481190342505, "recall": 0.7401441288681645, "support": 2359}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 275}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 133}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u2423": {"f1-score": 0.24337866857551899, "precision": 0.9826589595375722, "recall": 0.1388888888888889, "support": 1224}},
  "ppcr": 0.42879087118718456
}
```
</details>
