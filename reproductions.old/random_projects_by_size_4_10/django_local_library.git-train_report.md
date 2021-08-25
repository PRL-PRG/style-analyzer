# Train report for javascript / file:///tmp/top-repos-quality-repos-86wcyod8/django_local_library.git HEAD 2f8e44a0395657712ec3fb621cf20ddf3a55be19

### Classification report

PPCR: 0.715

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.962| 0.987| 0.943| 0.975| 0.953| 7500| 7852| 0.955 |
| `␣` | 0.954| 0.921| 0.604| 0.937| 0.740| 2899| 4421| 0.656 |
| `'` | 1.000| 1.000| 0.375| 1.000| 0.545| 1028| 2744| 0.375 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.951| 0.930| 0.830| 0.940| 0.886| 415| 465| 0.892 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.962| 0.842| 0.677| 0.898| 0.795| 361| 449| 0.804 |
| `⏎` | 0.945| 0.805| 0.191| 0.870| 0.317| 257| 1085| 0.237 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 122| 0.041 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 292| 0.000 |
| `macro avg` | 0.722| 0.686| 0.452| 0.702| 0.529| 12465| 17430| 0.715 |
| `weighted avg` | 0.962| 0.963| 0.689| 0.962| 0.766| 12465| 17430| 0.715 |
| `micro avg` | 0.963| 0.963| 0.689| 0.963| 0.803| 12465| 17430| 0.715 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|352 |7406 |89 |0 |0 |5 |0 |0 |0 |
|1522 |204 |2670 |0 |0 |13 |12 |0 |0 |
|1716 |0 |0 |1028 |0 |0 |0 |0 |0 |
|828 |25 |23 |0 |207 |2 |0 |0 |0 |
|50 |20 |6 |0 |3 |386 |0 |0 |0 |
|88 |39 |12 |0 |6 |0 |304 |0 |0 |
|292 |0 |0 |0 |0 |0 |0 |0 |0 |
|117 |2 |0 |0 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| staticfiles/admin/js/core.js | 88 |
| staticfiles/admin/js/SelectFilter2.js | 74 |
| staticfiles/admin/js/inlines.js | 73 |
| staticfiles/admin/js/actions.js | 44 |
| staticfiles/admin/js/admin/DateTimeShortcuts.js | 44 |
| staticfiles/admin/js/calendar.js | 36 |
| staticfiles/admin/js/urlify.js | 26 |
| staticfiles/admin/js/SelectBox.js | 25 |
| staticfiles/admin/js/timeparse.js | 17 |
| staticfiles/admin/js/admin/RelatedObjectLookups.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1028}, "macro avg": {"f1-score": 0.7024851292891138, "precision": 0.7217324035066781, "recall": 0.685768390805824, "support": 12465}, "micro avg": {"f1-score": 0.9627757721620537, "precision": 0.9627757721620538, "recall": 0.9627757721620538, "support": 12465}, "weighted avg": {"f1-score": 0.9621196704692104, "precision": 0.9622587103104866, "recall": 0.9627757721620538, "support": 12465}, "\u2205": {"f1-score": 0.9747301921558306, "precision": 0.9623180873180873, "recall": 0.9874666666666667, "support": 7500}, "\u23ce": {"f1-score": 0.8697478991596639, "precision": 0.9452054794520548, "recall": 0.8054474708171206, "support": 257}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9403166869671132, "precision": 0.9507389162561576, "recall": 0.9301204819277108, "support": 415}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8980797636632201, "precision": 0.9620253164556962, "recall": 0.8421052631578947, "support": 361}, "\u2423": {"f1-score": 0.9370064923670821, "precision": 0.9535714285714286, "recall": 0.921007243877199, "support": 2899}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 292}, "\u0027": {"f1-score": 0.545068928950159, "precision": 1.0, "recall": 0.3746355685131195, "support": 2744}, "macro avg": {"f1-score": 0.5294793576868821, "precision": 0.7217324035066781, "recall": 0.4524651981530539, "support": 17430}, "micro avg": {"f1-score": 0.8028767352400066, "precision": 0.9627757721620538, "recall": 0.6885255306942054, "support": 17430}, "weighted avg": {"f1-score": 0.7664268273327873, "precision": 0.941792989999375, "recall": 0.6885255306942054, "support": 17430}, "\u2205": {"f1-score": 0.9526627218934912, "precision": 0.9623180873180873, "recall": 0.9431991849210393, "support": 7852}, "\u23ce": {"f1-score": 0.3174846625766871, "precision": 0.9452054794520548, "recall": 0.19078341013824884, "support": 1085}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.886337543053961, "precision": 0.9507389162561576, "recall": 0.8301075268817204, "support": 465}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7947712418300653, "precision": 0.9620253164556962, "recall": 0.6770601336302895, "support": 449}, "\u2423": {"f1-score": 0.7395097631906937, "precision": 0.9535714285714286, "recall": 0.6039357611400136, "support": 4421}},
  "ppcr": 0.7151462994836488
}
```
</details>
