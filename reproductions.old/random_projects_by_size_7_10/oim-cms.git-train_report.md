# Train report for javascript / file:///tmp/top-repos-quality-repos-uciyj34o/oim-cms.git HEAD 7918ec7ed93a536669d7c17312aacbeec0cdfbe3

### Classification report

PPCR: 0.492

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.942| 0.995| 0.742| 0.968| 0.830| 1940| 2602| 0.746 |
| `␣` | 0.941| 0.720| 0.225| 0.816| 0.364| 354| 1131| 0.313 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 208| 0.120 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 128| 0.008 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 299| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 343| 0.000 |
| `weighted avg` | 0.931| 0.942| 0.464| 0.934| 0.546| 2320| 4711| 0.492 |
| `micro avg` | 0.942| 0.942| 0.464| 0.942| 0.622| 2320| 4711| 0.492 |
| `macro avg` | 0.314| 0.286| 0.161| 0.297| 0.199| 2320| 4711| 0.492 |

### Confusion matrix

|refusal|  ∅| ␣| '| "| ⏎| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|662 |1930 |10 |0 |0 |0 |0 |
|777 |99 |255 |0 |0 |0 |0 |
|299 |0 |0 |0 |0 |0 |0 |
|343 |0 |0 |0 |0 |0 |0 |
|183 |19 |6 |0 |0 |0 |0 |
|127 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| core/static/js/tinymce-textcolor-plugin.js | 48 |
| core/static/js/f6_oim_cms.js | 32 |
| core/static/js/jquery.glossarize.js | 27 |
| core/static/js/oim_cms.js | 20 |
| oim_cms/static/js/notification.js | 6 |
| oim_cms/static/js/validation.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.29727684465613774, "precision": 0.31381371646805784, "recall": 0.2858640573125983, "support": 2320}, "micro avg": {"f1-score": 0.9418103448275862, "precision": 0.9418103448275862, "recall": 0.9418103448275862, "support": 2320}, "weighted avg": {"f1-score": 0.9336752033609667, "precision": 0.931219843134464, "recall": 0.9418103448275862, "support": 2320}, "\u2205": {"f1-score": 0.9676610679368263, "precision": 0.9419228892142508, "recall": 0.9948453608247423, "support": 1940}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.816, "precision": 0.940959409594096, "recall": 0.7203389830508474, "support": 354}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 343}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 299}, "macro avg": {"f1-score": 0.19894918266980047, "precision": 0.31381371646805784, "recall": 0.1612002193782787, "support": 4711}, "micro avg": {"f1-score": 0.6215332100696913, "precision": 0.9418103448275862, "recall": 0.46380810868180855, "support": 4711}, "weighted avg": {"f1-score": 0.5457216689650345, "precision": 0.7461491084666532, "recall": 0.46380810868180855, "support": 4711}, "\u2205": {"f1-score": 0.8299290475166631, "precision": 0.9419228892142508, "recall": 0.7417371252882398, "support": 2602}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 208}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u2423": {"f1-score": 0.3637660485021398, "precision": 0.940959409594096, "recall": 0.22546419098143236, "support": 1131}},
  "ppcr": 0.4924644449161537
}
```
</details>
