# Train report for javascript / file:///tmp/top-repos-quality-repos-yflfzs2n/catboard.git HEAD 4ab36c6e4daa5b0693c8db6d143e8bf04816a232

### Classification report

PPCR: 0.566

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.944| 1.000| 0.871| 0.971| 0.906| 3702| 4249| 0.871 |
| `␣` | 1.000| 0.748| 0.261| 0.856| 0.413| 734| 2107| 0.348 |
| `'` | 1.000| 1.000| 0.273| 1.000| 0.429| 267| 977| 0.273 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 157| 0.178 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 349| 0.009 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 158| 0.013 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 254| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 116| 0.000 |
| `macro avg` | 0.368| 0.343| 0.176| 0.353| 0.219| 4736| 8367| 0.566 |
| `micro avg` | 0.954| 0.954| 0.540| 0.954| 0.690| 4736| 8367| 0.566 |
| `weighted avg` | 0.950| 0.954| 0.540| 0.948| 0.615| 4736| 8367| 0.566 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|547 |3702 |0 |0 |0 |0 |0 |0 |0 |
|1373 |185 |549 |0 |0 |0 |0 |0 |0 |
|710 |0 |0 |267 |0 |0 |0 |0 |0 |
|346 |3 |0 |0 |0 |0 |0 |0 |0 |
|254 |0 |0 |0 |0 |0 |0 |0 |0 |
|156 |2 |0 |0 |0 |0 |0 |0 |0 |
|129 |28 |0 |0 |0 |0 |0 |0 |0 |
|116 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/widgets/ping.test.js | 20 |
| test/widgets/number.test.js | 18 |
| test/widgets/sparkline.test.js | 16 |
| test/lib/logger.test.js | 15 |
| server.js | 14 |
| app.js | 14 |
| test/widgets/build-status.test.js | 13 |
| test/widgets/progress.test.js | 12 |
| lib/worker.js | 11 |
| feeder.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 267}, "macro avg": {"f1-score": 0.3534006607611024, "precision": 0.3680484693877551, "recall": 0.34349455040871935, "support": 4736}, "micro avg": {"f1-score": 0.9539695945945946, "precision": 0.9539695945945946, "recall": 0.9539695945945946, "support": 4736}, "weighted avg": {"f1-score": 0.9483276339693472, "precision": 0.9495615433673471, "recall": 0.9539695945945946, "support": 4736}, "\u2205": {"f1-score": 0.9713985830490686, "precision": 0.9443877551020409, "recall": 1.0, "support": 3702}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.8558067030397506, "precision": 1.0, "recall": 0.7479564032697548, "support": 734}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u0027": {"f1-score": 0.4292604501607717, "precision": 1.0, "recall": 0.27328556806550663, "support": 977}, "macro avg": {"f1-score": 0.21862716892932685, "precision": 0.3680484693877551, "recall": 0.17563867910211936, "support": 8367}, "micro avg": {"f1-score": 0.6896130657101427, "precision": 0.9539695945945946, "recall": 0.539978486912872, "support": 8367}, "weighted avg": {"f1-score": 0.6145002977352646, "precision": 0.8481777902986222, "recall": 0.539978486912872, "support": 8367}, "\u2205": {"f1-score": 0.9063532868160118, "precision": 0.9443877551020409, "recall": 0.8712638267827724, "support": 4249}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 349}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 254}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 157}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 158}, "\u2423": {"f1-score": 0.4134036144578313, "precision": 1.0, "recall": 0.26056003796867583, "support": 2107}},
  "ppcr": 0.5660332257678977
}
```
</details>
