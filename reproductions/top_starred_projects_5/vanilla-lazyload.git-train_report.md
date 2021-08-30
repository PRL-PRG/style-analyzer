# Train report for javascript / file:///tmp/top-repos-quality-repos-e4r9sw52/vanilla-lazyload.git HEAD 7b2d8b26bde0a6e03ed411b83f80c62b491fc81d

### Classification report

PPCR: 0.636

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.918| 1.000| 0.968| 0.957| 0.942| 3216| 3323| 0.968 |
| `␣` | 0.995| 0.716| 0.282| 0.832| 0.439| 809| 2055| 0.394 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 272| 544| 0.500 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 52| 175| 0.297 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 452| 0.011 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 183| 0.027 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 122| 0.000 |
| `weighted avg` | 0.924| 0.933| 0.593| 0.923| 0.641| 4359| 6854| 0.636 |
| `macro avg` | 0.416| 0.388| 0.250| 0.398| 0.293| 4359| 6854| 0.636 |
| `micro avg` | 0.933| 0.933| 0.593| 0.933| 0.725| 4359| 6854| 0.636 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|107 |3216 |0 |0 |0 |0 |0 |0 |
|1246 |230 |579 |0 |0 |0 |0 |0 |
|272 |0 |0 |272 |0 |0 |0 |0 |
|447 |5 |0 |0 |0 |0 |0 |0 |
|178 |2 |3 |0 |0 |0 |0 |0 |
|123 |52 |0 |0 |0 |0 |0 |0 |
|122 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/lazyload.setSources.js | 38 |
| src/lazyload.js | 27 |
| rollup.config.js | 27 |
| src/lazyload.event.js | 25 |
| src/lazyload.defaults.js | 19 |
| src/lazyload.intersectionObserver.js | 18 |
| __tests__/lazyload.setSources.test.js | 16 |
| __tests__/lib/expectExtend.js | 14 |
| src/lazyload.counters.js | 11 |
| src/lazyload.data.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 272}, "macro avg": {"f1-score": 0.3984992935082416, "precision": 0.41605596045203674, "recall": 0.3879569132968391, "support": 4359}, "micro avg": {"f1-score": 0.9330121587520074, "precision": 0.9330121587520074, "recall": 0.9330121587520074, "support": 4359}, "weighted avg": {"f1-score": 0.9229643435491773, "precision": 0.9239869231913506, "recall": 0.9330121587520074, "support": 4359}, "\u2205": {"f1-score": 0.9570004463621485, "precision": 0.917546362339515, "recall": 1.0, "support": 3216}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 52}, "\u2423": {"f1-score": 0.8324946081955429, "precision": 0.9948453608247423, "recall": 0.715698393077874, "support": 809}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 544}, "macro avg": {"f1-score": 0.2925436518171512, "precision": 0.41605596045203674, "recall": 0.24993600076817904, "support": 6854}, "micro avg": {"f1-score": 0.7254080085614911, "precision": 0.9330121587520074, "recall": 0.593376130726583, "support": 6854}, "weighted avg": {"f1-score": 0.6412850240732989, "precision": 0.8224998217900574, "recall": 0.593376130726583, "support": 6854}, "\u2205": {"f1-score": 0.9420035149384887, "precision": 0.917546362339515, "recall": 0.9678001805597352, "support": 3323}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 452}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 183}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 175}, "\u2423": {"f1-score": 0.43913538111490336, "precision": 0.9948453608247423, "recall": 0.28175182481751826, "support": 2055}},
  "ppcr": 0.6359789903705865
}
```
</details>
