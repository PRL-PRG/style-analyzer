# Train report for javascript / file:///tmp/top-repos-quality-repos-zqejew8w/api.git HEAD 3ba2eac0e7c9093852bdd3400c7f2581cd74ac9e

### Classification report

PPCR: 0.969

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.971| 0.997| 0.982| 0.984| 0.976| 43869| 44529| 0.985 |
| `␣` | 0.967| 0.931| 0.895| 0.948| 0.930| 13033| 13550| 0.962 |
| `'` | 1.000| 1.000| 0.991| 1.000| 0.995| 7485| 7556| 0.991 |
| `⏎` | 0.934| 0.861| 0.784| 0.896| 0.853| 3594| 3947| 0.911 |
| `⏎⇥⁺` | 0.981| 0.905| 0.869| 0.942| 0.922| 1520| 1584| 0.960 |
| `⏎⇥⁻` | 0.951| 0.796| 0.682| 0.866| 0.794| 799| 933| 0.856 |
| `⏎⏎` | 0.986| 0.656| 0.485| 0.787| 0.650| 418| 565| 0.740 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 315| 0.051 |
| `weighted avg` | 0.971| 0.972| 0.942| 0.971| 0.953| 70734| 72979| 0.969 |
| `macro avg` | 0.849| 0.768| 0.711| 0.803| 0.765| 70734| 72979| 0.969 |
| `micro avg` | 0.972| 0.972| 0.942| 0.972| 0.956| 70734| 72979| 0.969 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|660 |43724 |107 |0 |23 |15 |0 |0 |0 |
|517 |857 |12134 |0 |28 |11 |3 |0 |0 |
|71 |0 |0 |7485 |0 |0 |0 |0 |0 |
|353 |250 |229 |0 |3096 |0 |15 |4 |0 |
|64 |83 |56 |0 |5 |1376 |0 |0 |0 |
|134 |109 |23 |0 |31 |0 |636 |0 |0 |
|147 |5 |3 |0 |133 |0 |3 |274 |0 |
|299 |2 |2 |0 |0 |0 |12 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| routes/v3/covid-19/apiWorldometers.js | 117 |
| scrapers/covid-19/historical.js | 116 |
| routes/v2/apiWorldometers.js | 109 |
| updateCheck.js | 105 |
| routes/apiDeprecated.js | 81 |
| utils/nameUtils.js | 69 |
| tests/v3/covid-19/api_gov.spec.js | 54 |
| routes/v2/apiHistorical.js | 53 |
| routes/v3/covid-19/apiHistorical.js | 53 |
| scrapers/covid-19/getWorldometers.js | 49 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 7485}, "macro avg": {"f1-score": 0.8029853896506465, "precision": 0.8486169834291244, "recall": 0.7682390285776535, "support": 70734}, "micro avg": {"f1-score": 0.9715978171742019, "precision": 0.9715978171742019, "recall": 0.9715978171742019, "support": 70734}, "weighted avg": {"f1-score": 0.9708582787399749, "precision": 0.9712102811266706, "recall": 0.9715978171742019, "support": 70734}, "\u2205": {"f1-score": 0.9836781066153725, "precision": 0.970997113035754, "recall": 0.9966947046889603, "support": 43869}, "\u23ce": {"f1-score": 0.8960926193921853, "precision": 0.9336550060313631, "recall": 0.8614357262103506, "support": 3594}, "\u23ce\u21e5\u207a": {"f1-score": 0.9418206707734429, "precision": 0.9814550641940085, "recall": 0.9052631578947369, "support": 1520}, "\u23ce\u21e5\u207b": {"f1-score": 0.8664850136239782, "precision": 0.9506726457399103, "recall": 0.7959949937421777, "support": 799}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u23ce": {"f1-score": 0.7873563218390803, "precision": 0.9856115107913669, "recall": 0.6555023923444976, "support": 418}, "\u2423": {"f1-score": 0.948450384961113, "precision": 0.9665445276405926, "recall": 0.9310212537405048, "support": 13033}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9952795691775812, "precision": 1.0, "recall": 0.9906034939121228, "support": 7556}, "macro avg": {"f1-score": 0.7649519794548135, "precision": 0.8486169834291244, "recall": 0.7109664248933464, "support": 72979}, "micro avg": {"f1-score": 0.9564200872572419, "precision": 0.9715978171742019, "recall": 0.941709258827882, "support": 72979}, "weighted avg": {"f1-score": 0.9527350099931239, "precision": 0.9670428206533478, "recall": 0.941709258827882, "support": 72979}, "\u2205": {"f1-score": 0.9764289462812226, "precision": 0.970997113035754, "recall": 0.9819218935974309, "support": 44529}, "\u23ce": {"f1-score": 0.8525402726146222, "precision": 0.9336550060313631, "recall": 0.7843932100329364, "support": 3947}, "\u23ce\u21e5\u207a": {"f1-score": 0.9216342933690557, "precision": 0.9814550641940085, "recall": 0.8686868686868687, "support": 1584}, "\u23ce\u21e5\u207b": {"f1-score": 0.7940074906367042, "precision": 0.9506726457399103, "recall": 0.6816720257234726, "support": 933}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 315}, "\u23ce\u23ce": {"f1-score": 0.6500593119810202, "precision": 0.9856115107913669, "recall": 0.4849557522123894, "support": 565}, "\u2423": {"f1-score": 0.9296659515783021, "precision": 0.9665445276405926, "recall": 0.8954981549815498, "support": 13550}},
  "ppcr": 0.9692377259211554
}
```
</details>
