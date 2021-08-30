# Train report for javascript / file:///tmp/top-repos-quality-repos-1c2kl6io/pylearning.git HEAD b9d35011309557eccaf694637a46d02bfedb8092

### Classification report

PPCR: 0.660

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.873| 0.967| 0.890| 0.917| 0.881| 25524| 27745| 0.920 |
| `'` | 1.000| 1.000| 0.909| 1.000| 0.953| 11896| 13081| 0.909 |
| `␣` | 0.945| 0.666| 0.263| 0.781| 0.411| 10740| 27228| 0.394 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 0.970| 0.890| 0.985| 0.942| 2397| 2613| 0.917 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.971| 0.983| 0.866| 0.977| 0.916| 2377| 2698| 0.881 |
| `⏎⏎` | 0.423| 1.000| 0.328| 0.595| 0.370| 462| 1408| 0.328 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 184| 6385| 0.029 |
| `micro avg` | 0.912| 0.912| 0.602| 0.912| 0.725| 53580| 81158| 0.660 |
| `macro avg` | 0.745| 0.798| 0.592| 0.751| 0.639| 53580| 81158| 0.660 |
| `weighted avg` | 0.919| 0.912| 0.602| 0.908| 0.660| 53580| 81158| 0.660 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2221 |24687 |322 |0 |0 |12 |0 |503 |
|16488 |3539 |7150 |0 |0 |6 |0 |45 |
|1185 |0 |0 |11896 |0 |0 |0 |0 |
|6201 |0 |62 |0 |0 |40 |0 |82 |
|321 |4 |36 |0 |0 |2337 |0 |0 |
|216 |60 |0 |0 |0 |11 |2326 |0 |
|946 |0 |0 |0 |0 |0 |0 |462 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Projects/Microblog/app/static/js/moment-with-locales.js | 4384 |
| Projects/Microblog/app/static/js/moment.js | 338 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 11896}, "macro avg": {"f1-score": 0.7507514733315118, "precision": 0.744508137343544, "recall": 0.7980706584415661, "support": 53580}, "micro avg": {"f1-score": 0.9118701007838745, "precision": 0.9118701007838745, "recall": 0.9118701007838745, "support": 53580}, "weighted avg": {"f1-score": 0.9081836753090522, "precision": 0.918527269386281, "recall": 0.9118701007838745, "support": 53580}, "\u2205": {"f1-score": 0.9174935890288772, "precision": 0.8726405090137858, "recall": 0.9672073342736248, "support": 25524}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 184}, "\u23ce\u23ce": {"f1-score": 0.5945945945945945, "precision": 0.4230769230769231, "recall": 1.0, "support": 462}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.97721095546728, "precision": 0.9713216957605985, "recall": 0.983172065628944, "support": 2377}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9849671818759262, "precision": 1.0, "recall": 0.9703796412181894, "support": 2397}, "\u2423": {"f1-score": 0.7809939923539049, "precision": 0.9445178335535006, "recall": 0.6657355679702048, "support": 10740}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9525563518436962, "precision": 1.0, "recall": 0.9094105955202202, "support": 13081}, "macro avg": {"f1-score": 0.6388386854532332, "precision": 0.744508137343544, "recall": 0.5923252299141871, "support": 81158}, "micro avg": {"f1-score": 0.7252297050572222, "precision": 0.9118701007838745, "recall": 0.6020108923334729, "support": 81158}, "weighted avg": {"f1-score": 0.6598080362178707, "precision": 0.8482104134827941, "recall": 0.6020108923334729, "support": 81158}, "\u2205": {"f1-score": 0.8811278665120015, "precision": 0.8726405090137858, "recall": 0.889781942692377, "support": 27745}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6385}, "\u23ce\u23ce": {"f1-score": 0.3696, "precision": 0.4230769230769231, "recall": 0.328125, "support": 1408}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9157523510971786, "precision": 0.9713216957605985, "recall": 0.8661971830985915, "support": 2698}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9418910710670175, "precision": 1.0, "recall": 0.8901645618063528, "support": 2613}, "\u2423": {"f1-score": 0.4109431576527387, "precision": 0.9445178335535006, "recall": 0.2625973262817688, "support": 27228}},
  "ppcr": 0.6601936962468272
}
```
</details>