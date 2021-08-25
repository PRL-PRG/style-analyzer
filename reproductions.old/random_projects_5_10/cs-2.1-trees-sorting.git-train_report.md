# Train report for javascript / file:///tmp/top-repos-quality-repos-5yw7tiei/cs-2.1-trees-sorting.git HEAD 6a03ffdeb043435640af62ee52852d0dce635f59

### Classification report

PPCR: 0.791

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.976| 0.940| 0.980| 0.962| 12851| 13341| 0.963 |
| `␣` | 0.947| 0.981| 0.776| 0.964| 0.853| 9123| 11533| 0.791 |
| `'` | 1.000| 1.000| 0.953| 1.000| 0.976| 2330| 2444| 0.953 |
| `⏎` | 0.990| 0.868| 0.510| 0.925| 0.673| 1135| 1931| 0.588 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 29| 728| 0.040 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 1013| 0.017 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 688| 0.019 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 300| 0.003 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 266| 0.000 |
| `weighted avg` | 0.971| 0.973| 0.769| 0.971| 0.817| 25499| 32244| 0.791 |
| `macro avg` | 0.436| 0.425| 0.353| 0.430| 0.385| 25499| 32244| 0.791 |
| `micro avg` | 0.973| 0.973| 0.769| 0.973| 0.859| 25499| 32244| 0.791 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎⇥⁻| ⏎⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|490 |12537 |314 |0 |0 |0 |0 |0 |0 |0 |
|2410 |176 |8947 |0 |0 |0 |0 |0 |0 |0 |
|114 |0 |0 |2330 |0 |0 |0 |0 |0 |0 |
|796 |9 |141 |0 |985 |0 |0 |0 |0 |0 |
|996 |0 |7 |0 |10 |0 |0 |0 |0 |0 |
|699 |0 |29 |0 |0 |0 |0 |0 |0 |0 |
|675 |5 |8 |0 |0 |0 |0 |0 |0 |0 |
|299 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|266 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Slides/js/reveal.js | 429 |
| Slides/plugin/markdown/markdown.js | 53 |
| Slides/plugin/zoom-js/zoom.js | 49 |
| Slides/plugin/multiplex/index.js | 43 |
| Slides/plugin/notes/notes.js | 39 |
| Web/sw.js | 35 |
| Slides/plugin/notes-server/index.js | 13 |
| Slides/plugin/math/math.js | 12 |
| Slides/plugin/multiplex/client.js | 10 |
| Slides/plugin/print-pdf/print-pdf.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2330}, "macro avg": {"f1-score": 0.4298654404389638, "precision": 0.43579076334684497, "recall": 0.42490173487803884, "support": 25499}, "micro avg": {"f1-score": 0.9725479430565904, "precision": 0.9725479430565904, "recall": 0.9725479430565904, "support": 25499}, "weighted avg": {"f1-score": 0.9713478283848959, "precision": 0.9707365065826378, "recall": 0.9725479430565904, "support": 25499}, "\u2205": {"f1-score": 0.9802572422690489, "precision": 0.9849937146448774, "recall": 0.9755661038051513, "support": 12851}, "\u23ce": {"f1-score": 0.9248826291079811, "precision": 0.9899497487437185, "recall": 0.8678414096916299, "support": 1135}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9636490925736442, "precision": 0.9471734067330086, "recall": 0.9807081004055683, "support": 9123}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9761206535400083, "precision": 1.0, "recall": 0.953355155482815, "support": 2444}, "macro avg": {"f1-score": 0.38490833280188397, "precision": 0.43579076334684497, "recall": 0.35321800766316536, "support": 32244}, "micro avg": {"f1-score": 0.858943941256949, "precision": 0.9725479430565904, "recall": 0.7691043294876566, "support": 32244}, "weighted avg": {"f1-score": 0.8173484714210089, "precision": 0.8814087895966141, "recall": 0.7691043294876566, "support": 32244}, "\u2205": {"f1-score": 0.9618320610687022, "precision": 0.9849937146448774, "recall": 0.9397346525747695, "support": 13341}, "\u23ce": {"f1-score": 0.6732740943267258, "precision": 0.9899497487437185, "recall": 0.5100983946141895, "support": 1931}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 728}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 688}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1013}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 266}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 300}, "\u2423": {"f1-score": 0.8529481862815196, "precision": 0.9471734067330086, "recall": 0.7757738662967137, "support": 11533}},
  "ppcr": 0.7908137948145392
}
```
</details>
