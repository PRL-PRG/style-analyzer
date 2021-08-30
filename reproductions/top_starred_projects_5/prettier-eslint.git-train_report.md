# Train report for javascript / file:///tmp/top-repos-quality-repos-613ywc0_/prettier-eslint.git HEAD c62769ef430749fb9650c7e1d05f5cf7442a48a2

### Classification report

PPCR: 0.405

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.943| 1.000| 0.584| 0.971| 0.721| 1958| 3355| 0.584 |
| `'` | 1.000| 1.000| 0.818| 1.000| 0.900| 710| 868| 0.818 |
| `␣` | 1.000| 0.595| 0.076| 0.746| 0.141| 269| 2116| 0.127 |
| `⏎` | 0.851| 1.000| 0.337| 0.920| 0.483| 200| 594| 0.337 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 380| 0.100 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 390| 0.018 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 155| 0.000 |
| `weighted avg` | 0.941| 0.952| 0.385| 0.941| 0.482| 3182| 7858| 0.405 |
| `macro avg` | 0.542| 0.514| 0.259| 0.519| 0.321| 3182| 7858| 0.405 |
| `micro avg` | 0.952| 0.952| 0.385| 0.952| 0.549| 3182| 7858| 0.405 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1397 |1958 |0 |0 |0 |0 |0 |0 |
|1847 |79 |160 |0 |30 |0 |0 |0 |
|158 |0 |0 |710 |0 |0 |0 |0 |
|394 |0 |0 |0 |200 |0 |0 |0 |
|383 |7 |0 |0 |0 |0 |0 |0 |
|342 |33 |0 |0 |5 |0 |0 |0 |
|155 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/utils.js | 62 |
| src/__tests__/utils.js | 44 |
| src/index.js | 23 |
| src/__tests__/index.js | 15 |
| src/__mocks__/loglevel-colored-level-prefix.js | 3 |
| src/__mocks__/eslint.js | 3 |
| package-scripts.js | 3 |
| src/__mocks__/prettier.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 710}, "macro avg": {"f1-score": 0.5194241471898182, "precision": 0.5419670936424847, "recall": 0.5135422198619225, "support": 3182}, "micro avg": {"f1-score": 0.9516027655562539, "precision": 0.9516027655562539, "recall": 0.9516027655562539, "support": 3182}, "weighted avg": {"f1-score": 0.9411739463813886, "precision": 0.9412416004707536, "recall": 0.9516027655562539, "support": 3182}, "\u2205": {"f1-score": 0.9705080545229244, "precision": 0.9427058257101589, "recall": 1.0, "support": 1958}, "\u23ce": {"f1-score": 0.9195402298850576, "precision": 0.851063829787234, "recall": 1.0, "support": 200}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u2423": {"f1-score": 0.7459207459207459, "precision": 1.0, "recall": 0.5947955390334573, "support": 269}},
  "cl_report_full": {"\u0027": {"f1-score": 0.899873257287706, "precision": 1.0, "recall": 0.8179723502304147, "support": 868}, "macro avg": {"f1-score": 0.32055613591234877, "precision": 0.5419670936424847, "recall": 0.2591276587196399, "support": 7858}, "micro avg": {"f1-score": 0.5485507246376812, "precision": 0.9516027655562539, "recall": 0.3853397811147875, "support": 7858}, "weighted avg": {"f1-score": 0.48153069873287213, "precision": 0.8465652787161109, "recall": 0.3853397811147875, "support": 7858}, "\u2205": {"f1-score": 0.7209131075110456, "precision": 0.9427058257101589, "recall": 0.5836065573770491, "support": 3355}, "\u23ce": {"f1-score": 0.48250904704463204, "precision": 0.851063829787234, "recall": 0.3367003367003367, "support": 594}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 390}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 380}, "\u2423": {"f1-score": 0.14059753954305798, "precision": 1.0, "recall": 0.07561436672967864, "support": 2116}},
  "ppcr": 0.40493764316620007
}
```
</details>
