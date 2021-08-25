# Train report for javascript / file:///tmp/top-repos-quality-repos-ip41hahb/tournament-organiser.git HEAD 5affa81dfbe6697d79972f0013595f7efcfbe9ea

### Classification report

PPCR: 0.785

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 0.992| 0.968| 0.970| 0.959| 17939| 18377| 0.976 |
| `␣` | 0.940| 0.893| 0.589| 0.916| 0.724| 4012| 6084| 0.659 |
| `"` | 0.969| 1.000| 0.749| 0.984| 0.845| 3155| 4214| 0.749 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.929| 0.695| 0.336| 0.795| 0.493| 643| 1331| 0.483 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 279| 787| 0.355 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 151| 1967| 0.077 |
| `'` | 1.000| 0.057| 0.032| 0.107| 0.062| 106| 188| 0.564 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 221| 0.054 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 166| 0.036 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 181| 0.006 |
| `macro avg` | 0.479| 0.364| 0.267| 0.377| 0.308| 26304| 33516| 0.785 |
| `micro avg` | 0.950| 0.950| 0.746| 0.950| 0.835| 26304| 33516| 0.785 |
| `weighted avg` | 0.934| 0.950| 0.746| 0.939| 0.783| 26304| 33516| 0.785 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎⏎| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|438 |17798 |112 |0 |0 |29 |0 |0 |0 |0 |0 |
|2072 |427 |3582 |0 |0 |3 |0 |0 |0 |0 |0 |
|1059 |0 |0 |3155 |0 |0 |0 |0 |0 |0 |0 |
|1816 |105 |45 |0 |0 |1 |0 |0 |0 |0 |0 |
|688 |124 |72 |0 |0 |447 |0 |0 |0 |0 |0 |
|508 |278 |0 |0 |0 |1 |0 |0 |0 |0 |0 |
|82 |0 |0 |100 |0 |0 |0 |6 |0 |0 |0 |
|209 |12 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|180 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|160 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/dao/functionality/src/entry_rank_spec.js | 60 |
| web/src/models/menu.js | 55 |
| web/src/views/tournament-create.js | 53 |
| test/dao/functionality/src/entry_next_game_spec.js | 49 |
| web/src/controllers/tournament-routes.js | 49 |
| web/src/views/tournament-rankings.js | 45 |
| web/src/views/component-tournament-categories.js | 43 |
| web/src/views/entry-score.js | 42 |
| web/src/views/tournament-info.js | 41 |
| test/dao/functionality/src/tournament_score_categories_spec.js | 37 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.984399375975039, "precision": 0.9692780337941628, "recall": 1.0, "support": 3155}, "\u0027": {"f1-score": 0.10714285714285715, "precision": 1.0, "recall": 0.05660377358490566, "support": 106}, "macro avg": {"f1-score": 0.3772859726588463, "precision": 0.4787678791623394, "recall": 0.36367441882253715, "support": 26304}, "micro avg": {"f1-score": 0.9499695863746958, "precision": 0.9499695863746959, "recall": 0.9499695863746959, "support": 26304}, "weighted avg": {"f1-score": 0.9392754829368365, "precision": 0.9336911562190319, "recall": 0.9499695863746959, "support": 26304}, "\u2205": {"f1-score": 0.9701826110656855, "precision": 0.949176043944323, "recall": 0.9921400301020123, "support": 17939}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 151}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.7953736654804271, "precision": 0.9293139293139293, "recall": 0.6951788491446346, "support": 643}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 279}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.9157612169244536, "precision": 0.9399107845709788, "recall": 0.8928215353938186, "support": 4012}},
  "cl_report_full": {"\"": {"f1-score": 0.8448252778149684, "precision": 0.9692780337941628, "recall": 0.7486948267679164, "support": 4214}, "\u0027": {"f1-score": 0.06185567010309278, "precision": 1.0, "recall": 0.031914893617021274, "support": 188}, "macro avg": {"f1-score": 0.3082797793674892, "precision": 0.4787678791623394, "recall": 0.2673698058064833, "support": 33516}, "micro avg": {"f1-score": 0.8354396522902039, "precision": 0.9499695863746959, "recall": 0.7455543620957155, "support": 33516}, "weighted avg": {"f1-score": 0.7832662737645039, "precision": 0.8554385919328112, "recall": 0.7455543620957155, "support": 33516}, "\u2205": {"f1-score": 0.9587373410902823, "precision": 0.949176043944323, "recall": 0.9684932252271862, "support": 18377}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1967}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 221}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 181}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.49337748344370863, "precision": 0.9293139293139293, "recall": 0.33583771600300527, "support": 1331}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 787}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "\u2423": {"f1-score": 0.7240020212228397, "precision": 0.9399107845709788, "recall": 0.5887573964497042, "support": 6084}},
  "ppcr": 0.7848191908342285
}
```
</details>
