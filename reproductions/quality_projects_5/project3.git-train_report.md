# Train report for javascript / file:///tmp/top-repos-quality-repos-vwwtzv2c/project3.git HEAD 5cdbd162477789cd187071a8c056d2f1531018d3

### Classification report

PPCR: 0.094

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.973| 1.000| 0.370| 0.986| 0.536| 1076| 2910| 0.370 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 6265| 0.002 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 272| 0.044 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 645| 0.008 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1021| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 245| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 240| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 132| 0.000 |
| `micro avg` | 0.973| 0.973| 0.092| 0.973| 0.168| 1106| 11730| 0.094 |
| `weighted avg` | 0.946| 0.973| 0.092| 0.959| 0.133| 1106| 11730| 0.094 |
| `macro avg` | 0.122| 0.125| 0.046| 0.123| 0.067| 1106| 11730| 0.094 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6252 |0 |13 |0 |0 |0 |0 |0 |0 |
|1834 |0 |1076 |0 |0 |0 |0 |0 |0 |
|1021 |0 |0 |0 |0 |0 |0 |0 |0 |
|640 |0 |5 |0 |0 |0 |0 |0 |0 |
|260 |0 |12 |0 |0 |0 |0 |0 |0 |
|245 |0 |0 |0 |0 |0 |0 |0 |0 |
|240 |0 |0 |0 |0 |0 |0 |0 |0 |
|132 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/Pages/ProfilePage.js | 4 |
| src/Pages/nutritionList.js | 4 |
| src/Components/Footer/index.js | 4 |
| src/Pages/workoutCard.js | 3 |
| src/Pages/LogInPage.js | 2 |
| src/Components/WorkoutUI/favorites.js | 2 |
| src/Components/WorkoutUI/SearchBox.js | 2 |
| src/Store/action/nutrition.js | 2 |
| src/ProtectedRoute.js | 1 |
| src/Auth.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1232813932172319, "precision": 0.12160940325497288, "recall": 0.125, "support": 1106}, "micro avg": {"f1-score": 0.972875226039783, "precision": 0.972875226039783, "recall": 0.972875226039783, "support": 1106}, "weighted avg": {"f1-score": 0.9594993063417109, "precision": 0.9464862054419589, "recall": 0.972875226039783, "support": 1106}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9862511457378552, "precision": 0.972875226039783, "recall": 1.0, "support": 1076}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1021}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 240}, "macro avg": {"f1-score": 0.06698207171314742, "precision": 0.12160940325497288, "recall": 0.046219931271477666, "support": 11730}, "micro avg": {"f1-score": 0.16765347460267999, "precision": 0.972875226039783, "recall": 0.0917306052855925, "support": 11730}, "weighted avg": {"f1-score": 0.13293628554834372, "precision": 0.2413526775597416, "recall": 0.0917306052855925, "support": 11730}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6265}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 645}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 272}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 245}, "\u2423": {"f1-score": 0.5358565737051794, "precision": 0.972875226039783, "recall": 0.3697594501718213, "support": 2910}},
  "ppcr": 0.09428815004262575
}
```
</details>
