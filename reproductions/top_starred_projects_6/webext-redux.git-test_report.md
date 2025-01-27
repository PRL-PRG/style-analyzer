# Test report for javascript / file:///tmp/top-repos-quality-repos-lsin22na/webext-redux.git HEAD 2a77acb1bd9e8059df10fe58e30293ffa46d770d

### Classification report

PPCR: 0.826

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.960| 0.992| 0.987| 0.976| 0.973| 1561| 1569| 0.995 |
| `␣` | 0.909| 0.963| 0.820| 0.935| 0.862| 736| 865| 0.851 |
| `⏎␣⁻␣⁻` | 1.000| 0.643| 0.488| 0.783| 0.656| 98| 129| 0.760 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 81| 156| 0.519 |
| `"` | 0.467| 1.000| 0.667| 0.637| 0.549| 64| 96| 0.667 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 130| 0.285 |
| `⏎` | 0.833| 0.161| 0.030| 0.270| 0.058| 31| 166| 0.187 |
| `⏎⏎` | 0.724| 1.000| 0.292| 0.840| 0.416| 21| 72| 0.292 |
| `micro avg` | 0.917| 0.917| 0.757| 0.917| 0.830| 2629| 3183| 0.826 |
| `weighted avg` | 0.889| 0.917| 0.757| 0.896| 0.770| 2629| 3183| 0.826 |
| `macro avg` | 0.612| 0.595| 0.410| 0.555| 0.439| 2629| 3183| 0.826 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8 |1549 |11 |0 |1 |0 |0 |0 |0 |
|129 |26 |709 |0 |0 |0 |0 |0 |1 |
|32 |0 |0 |64 |0 |0 |0 |0 |0 |
|135 |5 |14 |0 |5 |0 |0 |0 |7 |
|93 |2 |35 |0 |0 |0 |0 |0 |0 |
|31 |24 |11 |0 |0 |0 |63 |0 |0 |
|75 |8 |0 |73 |0 |0 |0 |0 |0 |
|51 |0 |0 |0 |0 |0 |0 |0 |21 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.6368159203980099, "precision": 0.46715328467153283, "recall": 1.0, "support": 64}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 81}, "macro avg": {"f1-score": 0.5550998897930277, "precision": 0.6116657866739563, "recall": 0.5949719128680504, "support": 2629}, "micro avg": {"f1-score": 0.9170787371624193, "precision": 0.9170787371624192, "recall": 0.9170787371624192, "support": 2629}, "weighted avg": {"f1-score": 0.8957912096658459, "precision": 0.8885803031130213, "recall": 0.9170787371624192, "support": 2629}, "\u2205": {"f1-score": 0.975748031496063, "precision": 0.959727385377943, "recall": 0.9923126201153107, "support": 1561}, "\u23ce": {"f1-score": 0.27027027027027023, "precision": 0.8333333333333334, "recall": 0.16129032258064516, "support": 31}, "\u23ce\u23ce": {"f1-score": 0.8400000000000001, "precision": 0.7241379310344828, "recall": 1.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.782608695652174, "precision": 1.0, "recall": 0.6428571428571429, "support": 98}, "\u2423": {"f1-score": 0.9353562005277044, "precision": 0.908974358974359, "recall": 0.9633152173913043, "support": 736}},
  "cl_report_full": {"\"": {"f1-score": 0.5493562231759656, "precision": 0.46715328467153283, "recall": 0.6666666666666666, "support": 96}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "macro avg": {"f1-score": 0.4393611317870049, "precision": 0.6116657866739563, "recall": 0.4104665143601304, "support": 3183}, "micro avg": {"f1-score": 0.8296627666896077, "precision": 0.9170787371624192, "recall": 0.7574615142946906, "support": 3183}, "weighted avg": {"f1-score": 0.7696268648727681, "precision": 0.8345564146613561, "recall": 0.7574615142946906, "support": 3183}, "\u2205": {"f1-score": 0.9732956330505812, "precision": 0.959727385377943, "recall": 0.9872530274059911, "support": 1569}, "\u23ce": {"f1-score": 0.05813953488372092, "precision": 0.8333333333333334, "recall": 0.030120481927710843, "support": 166}, "\u23ce\u23ce": {"f1-score": 0.4158415841584159, "precision": 0.7241379310344828, "recall": 0.2916666666666667, "support": 72}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.65625, "precision": 1.0, "recall": 0.4883720930232558, "support": 129}, "\u2423": {"f1-score": 0.8620060790273557, "precision": 0.908974358974359, "recall": 0.8196531791907514, "support": 865}},
  "ppcr": 0.8259503612943764
}
```
</details>
