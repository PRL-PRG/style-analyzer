# Train report for javascript / file:///tmp/top-repos-quality-repos-nstzg0kk/code-examples.git HEAD 0084688cf3513eeec12072e877036fc9ace15faa

### Classification report

PPCR: 0.274

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 1.000| 0.521| 0.979| 0.675| 708| 1360| 0.521 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 921| 0.033 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 251| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 165| 0.000 |
| `macro avg` | 0.240| 0.250| 0.130| 0.245| 0.169| 738| 2697| 0.274 |
| `weighted avg` | 0.920| 0.959| 0.263| 0.939| 0.340| 738| 2697| 0.274 |
| `micro avg` | 0.959| 0.959| 0.263| 0.959| 0.412| 738| 2697| 0.274 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|652 |708 |0 |0 |0 |
|891 |30 |0 |0 |0 |
|251 |0 |0 |0 |0 |
|165 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| adjListGraph.js | 12 |
| adjMatrixGraph.js | 9 |
| spellCheck.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24481327800829875, "precision": 0.23983739837398374, "recall": 0.25, "support": 738}, "micro avg": {"f1-score": 0.959349593495935, "precision": 0.959349593495935, "recall": 0.959349593495935, "support": 738}, "weighted avg": {"f1-score": 0.9394460749586748, "precision": 0.9203516425408157, "recall": 0.959349593495935, "support": 738}, "\u2205": {"f1-score": 0.979253112033195, "precision": 0.959349593495935, "recall": 1.0, "support": 708}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 251}, "macro avg": {"f1-score": 0.16873212583412775, "precision": 0.23983739837398374, "recall": 0.13014705882352942, "support": 2697}, "micro avg": {"f1-score": 0.4122270742358079, "precision": 0.959349593495935, "recall": 0.2625139043381535, "support": 2697}, "weighted avg": {"f1-score": 0.3403421448044698, "precision": 0.483765460568955, "recall": 0.2625139043381535, "support": 2697}, "\u2205": {"f1-score": 0.674928503336511, "precision": 0.959349593495935, "recall": 0.5205882352941177, "support": 1360}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 165}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 921}},
  "ppcr": 0.27363737486095663
}
```
</details>
