# Train report for javascript / file:///tmp/top-repos-quality-repos-z1xfkvat/kmera.git HEAD 77ba2c13238342eb1d768d26bcbf135580f49555

### Classification report

PPCR: 0.295

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.945| 0.998| 0.449| 0.971| 0.609| 617| 1372| 0.450 |
| `␣` | 0.987| 0.805| 0.260| 0.887| 0.412| 185| 573| 0.323 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 191| 0.005 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 460| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 122| 0.000 |
| `macro avg` | 0.386| 0.361| 0.142| 0.372| 0.204| 803| 2718| 0.295 |
| `weighted avg` | 0.953| 0.953| 0.281| 0.950| 0.394| 803| 2718| 0.295 |
| `micro avg` | 0.953| 0.953| 0.281| 0.953| 0.435| 803| 2718| 0.295 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|755 |616 |1 |0 |0 |0 |
|388 |36 |149 |0 |0 |0 |
|460 |0 |0 |0 |0 |0 |
|190 |0 |1 |0 |0 |0 |
|122 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/homepage/index.js | 7 |
| src/translate/index.js | 7 |
| src/picture-card/index.js | 7 |
| Gulpfile.js | 3 |
| server.js | 2 |
| src/landing/index.js | 2 |
| src/footer/index.js | 2 |
| src/layout/index.js | 1 |
| src/translate/es.js | 1 |
| src/homepage/template.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3715495891027806, "precision": 0.38630804859220735, "recall": 0.36075693197249115, "support": 803}, "micro avg": {"f1-score": 0.9526774595267746, "precision": 0.9526774595267746, "recall": 0.9526774595267746, "support": 803}, "weighted avg": {"f1-score": 0.9502959218421183, "precision": 0.9532779379970056, "recall": 0.9526774595267746, "support": 803}, "\u2205": {"f1-score": 0.970843183609141, "precision": 0.9447852760736196, "recall": 0.9983792544570502, "support": 617}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.8869047619047619, "precision": 0.9867549668874173, "recall": 0.8054054054054054, "support": 185}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 460}, "macro avg": {"f1-score": 0.2040595724237329, "precision": 0.38630804859220735, "recall": 0.14180289917013927, "support": 2718}, "micro avg": {"f1-score": 0.4345356432831582, "precision": 0.9526774595267746, "recall": 0.2814569536423841, "support": 2718}, "weighted avg": {"f1-score": 0.3940318252689298, "precision": 0.6849359804265991, "recall": 0.2814569536423841, "support": 2718}, "\u2205": {"f1-score": 0.6086956521739131, "precision": 0.9447852760736196, "recall": 0.4489795918367347, "support": 1372}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 191}, "\u2423": {"f1-score": 0.41160220994475133, "precision": 0.9867549668874173, "recall": 0.2600349040139616, "support": 573}},
  "ppcr": 0.29543782192788814
}
```
</details>
