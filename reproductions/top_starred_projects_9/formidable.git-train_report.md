# Train report for javascript / file:///tmp/top-repos-quality-repos-chbgofv8/formidable.git HEAD 3d429e00a4e343cbce94440c9f2a9fcc1f03a8bd

### Classification report

PPCR: 0.766

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.978| 0.996| 0.964| 0.987| 0.971| 12026| 12434| 0.967 |
| `␣` | 0.978| 0.953| 0.651| 0.965| 0.782| 3512| 5138| 0.684 |
| `'` | 1.000| 1.000| 0.781| 1.000| 0.877| 1765| 2261| 0.781 |
| `⏎␣⁻␣⁻` | 1.000| 0.985| 0.654| 0.992| 0.791| 459| 691| 0.664 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 66| 1447| 0.046 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 66| 725| 0.091 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 680| 0.001 |
| `weighted avg` | 0.973| 0.980| 0.751| 0.977| 0.796| 17895| 23376| 0.766 |
| `micro avg` | 0.980| 0.980| 0.751| 0.980| 0.850| 17895| 23376| 0.766 |
| `macro avg` | 0.565| 0.562| 0.436| 0.564| 0.489| 17895| 23376| 0.766 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|408 |11983 |43 |0 |0 |0 |0 |0 |
|1626 |166 |3346 |0 |0 |0 |0 |0 |
|496 |0 |0 |1765 |0 |0 |0 |0 |
|1381 |52 |14 |0 |0 |0 |0 |0 |
|659 |53 |13 |0 |0 |0 |0 |0 |
|232 |2 |5 |0 |0 |0 |452 |0 |
|679 |1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/parsers/Multipart.js | 51 |
| test/unit/custom-plugins.test.js | 40 |
| src/Formidable.js | 39 |
| .eslintrc.js | 23 |
| test-legacy/simple/test-incoming-form.js | 22 |
| test-legacy/system/test-multi-video-upload.js | 17 |
| test/integration/test-fixtures.js | 17 |
| src/plugins/multipart.js | 15 |
| test/unit/formidable.test.js | 11 |
| test/fixture/js/special-chars-in-filename.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1765}, "macro avg": {"f1-score": 0.5635000641880249, "precision": 0.5651031446995011, "recall": 0.561986764900211, "support": 17895}, "micro avg": {"f1-score": 0.9804973456272702, "precision": 0.9804973456272702, "recall": 0.9804973456272702, "support": 17895}, "weighted avg": {"f1-score": 0.9767755938060305, "precision": 0.9732421836069547, "recall": 0.9804973456272702, "support": 17895}, "\u2205": {"f1-score": 0.9869455998023309, "precision": 0.9776454271028799, "recall": 0.9964244137701647, "support": 12026}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9923161361141604, "precision": 1.0, "recall": 0.9847494553376906, "support": 459}, "\u2423": {"f1-score": 0.9652387133996826, "precision": 0.9780765857936276, "recall": 0.9527334851936219, "support": 3512}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8768007948335818, "precision": 1.0, "recall": 0.7806280406899602, "support": 2261}, "macro avg": {"f1-score": 0.4886008638530154, "precision": 0.5651031446995011, "recall": 0.43567244892065593, "support": 23376}, "micro avg": {"f1-score": 0.8502822805359695, "precision": 0.9804973456272702, "recall": 0.7505989048596852, "support": 23376}, "weighted avg": {"f1-score": 0.7963335694048778, "precision": 0.8612851103013718, "recall": 0.7505989048596852, "support": 23376}, "\u2205": {"f1-score": 0.9706370742375764, "precision": 0.9776454271028799, "recall": 0.9637284864082355, "support": 12434}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1447}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 680}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 725}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7909011373578302, "precision": 1.0, "recall": 0.6541244573082489, "support": 691}, "\u2423": {"f1-score": 0.7818670405421193, "precision": 0.9780765857936276, "recall": 0.6512261580381471, "support": 5138}},
  "ppcr": 0.7655287474332649
}
```
</details>
