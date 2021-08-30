# Train report for javascript / file:///tmp/top-repos-quality-repos-u2h6bxfm/textures.git HEAD bfde14ecf2c051c499ae25eac5f9eb5ed60a4eb2

### Classification report

PPCR: 0.762

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.951| 0.996| 0.970| 0.973| 0.960| 5245| 5386| 0.974 |
| `␣` | 0.926| 0.861| 0.361| 0.892| 0.519| 713| 1702| 0.419 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 656| 1312| 0.500 |
| `⏎` | 0.830| 0.889| 0.675| 0.859| 0.745| 461| 607| 0.759 |
| `⏎⇥⁻` | 1.000| 0.693| 0.447| 0.819| 0.618| 176| 273| 0.645 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 108| 299| 0.361 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 66| 160| 0.412 |
| `micro avg` | 0.946| 0.946| 0.721| 0.946| 0.818| 7425| 9739| 0.762 |
| `macro avg` | 0.672| 0.634| 0.422| 0.649| 0.501| 7425| 9739| 0.762 |
| `weighted avg` | 0.924| 0.946| 0.721| 0.934| 0.775| 7425| 9739| 0.762 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|141 |5222 |23 |0 |0 |0 |0 |0 |
|989 |99 |614 |0 |0 |0 |0 |0 |
|656 |0 |0 |656 |0 |0 |0 |0 |
|146 |51 |0 |0 |410 |0 |0 |0 |
|191 |82 |26 |0 |0 |0 |0 |0 |
|97 |16 |0 |0 |38 |0 |122 |0 |
|94 |20 |0 |0 |46 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/lines.js | 123 |
| src/paths.js | 116 |
| src/circles.js | 112 |
| rollup.config.js | 28 |
| src/random.js | 5 |
| tests/circles-test.js | 4 |
| tests/paths-test.js | 4 |
| tests/lines-test.js | 4 |
| src/main.js | 3 |
| tests/jsdom.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 656}, "macro avg": {"f1-score": 0.6489664226057881, "precision": 0.6724624284792787, "recall": 0.6341882417669903, "support": 7425}, "micro avg": {"f1-score": 0.9459932659932659, "precision": 0.9459932659932659, "recall": 0.9459932659932659, "support": 7425}, "weighted avg": {"f1-score": 0.9340163730388656, "precision": 0.9244277358773177, "recall": 0.9459932659932659, "support": 7425}, "\u2205": {"f1-score": 0.9728924080111783, "precision": 0.951183970856102, "recall": 0.9956148713060057, "support": 5245}, "\u23ce": {"f1-score": 0.8586387434554974, "precision": 0.8299595141700404, "recall": 0.8893709327548807, "support": 461}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u21e5\u207b": {"f1-score": 0.8187919463087248, "precision": 1.0, "recall": 0.6931818181818182, "support": 176}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u2423": {"f1-score": 0.8924418604651163, "precision": 0.9260935143288085, "recall": 0.8611500701262272, "support": 713}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 1312}, "macro avg": {"f1-score": 0.5012405822626235, "precision": 0.6724624284792787, "recall": 0.42180603400469047, "support": 9739}, "micro avg": {"f1-score": 0.8184572360755068, "precision": 0.9459932659932659, "recall": 0.7212239449635486, "support": 9739}, "weighted avg": {"f1-score": 0.7753562627387885, "precision": 0.9023589129807795, "recall": 0.7212239449635486, "support": 9739}, "\u2205": {"f1-score": 0.9602795145273997, "precision": 0.951183970856102, "recall": 0.9695506869662087, "support": 5386}, "\u23ce": {"f1-score": 0.7447774750227066, "precision": 0.8299595141700404, "recall": 0.6754530477759473, "support": 607}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 299}, "\u23ce\u21e5\u207b": {"f1-score": 0.6177215189873418, "precision": 1.0, "recall": 0.4468864468864469, "support": 273}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 160}, "\u2423": {"f1-score": 0.5192389006342494, "precision": 0.9260935143288085, "recall": 0.3607520564042303, "support": 1702}},
  "ppcr": 0.7623986035527262
}
```
</details>
