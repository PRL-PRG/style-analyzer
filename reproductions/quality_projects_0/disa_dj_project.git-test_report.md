# Test report for javascript / file:///tmp/top-repos-quality-repos-3b7jc6z7/disa_dj_project.git HEAD 37db0a60fe4fcdd32595ce32652cfd30acbdc61d

### Classification report

PPCR: 0.921

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.848| 0.947| 0.934| 0.894| 0.889| 1627| 1648| 0.987 |
| `␣` | 0.857| 0.731| 0.696| 0.789| 0.768| 1080| 1133| 0.953 |
| `⏎` | 0.895| 0.909| 0.634| 0.902| 0.742| 187| 268| 0.698 |
| `"` | 0.733| 0.159| 0.128| 0.262| 0.218| 69| 86| 0.802 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 68| 80| 0.850 |
| `'` | 0.431| 0.917| 0.579| 0.587| 0.494| 48| 76| 0.632 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 55| 0.545 |
| `⏎␣⁻␣⁻` | 0.452| 0.905| 0.792| 0.603| 0.576| 21| 24| 0.875 |
| `⏎⏎` | 0.417| 0.357| 0.125| 0.385| 0.192| 14| 40| 0.350 |
| `⏎␣⁺␣⁺` | 0.172| 0.769| 0.556| 0.282| 0.263| 13| 18| 0.722 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.437| 0.518| 0.404| 0.428| 0.377| 3157| 3428| 0.921 |
| `micro avg` | 0.820| 0.820| 0.755| 0.820| 0.786| 3157| 3428| 0.921 |
| `weighted avg` | 0.811| 0.820| 0.755| 0.806| 0.763| 3157| 3428| 0.921 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|21 |1540 |84 |0 |1 |1 |1 |0 |0 |0 |0 |
|53 |271 |789 |0 |8 |0 |11 |1 |0 |0 |0 |
|28 |0 |0 |44 |0 |0 |0 |0 |4 |0 |0 |
|81 |0 |11 |0 |170 |6 |0 |0 |0 |0 |0 |
|26 |1 |2 |0 |6 |5 |0 |0 |0 |0 |0 |
|5 |2 |0 |0 |1 |0 |10 |0 |0 |0 |0 |
|3 |2 |0 |0 |0 |0 |0 |19 |0 |0 |0 |
|17 |0 |0 |58 |0 |0 |0 |0 |11 |0 |0 |
|12 |1 |31 |0 |0 |0 |36 |0 |0 |0 |0 |
|25 |0 |4 |0 |4 |0 |0 |22 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.2619047619047619, "precision": 0.7333333333333333, "recall": 0.15942028985507245, "support": 69}, "\u0027": {"f1-score": 0.5866666666666666, "precision": 0.43137254901960786, "recall": 0.9166666666666666, "support": 48}, "macro avg": {"f1-score": 0.4275293601248247, "precision": 0.43683023355722705, "recall": 0.517581482114219, "support": 3157}, "micro avg": {"f1-score": 0.8197656002534052, "precision": 0.8197656002534052, "recall": 0.8197656002534052, "support": 3157}, "weighted avg": {"f1-score": 0.8056150461317232, "precision": 0.8110147096349444, "recall": 0.8197656002534052, "support": 3157}, "\u2205": {"f1-score": 0.8943089430894309, "precision": 0.8475509080902587, "recall": 0.9465273509526736, "support": 1627}, "\u23ce": {"f1-score": 0.9018567639257294, "precision": 0.8947368421052632, "recall": 0.9090909090909091, "support": 187}, "\u23ce\u23ce": {"f1-score": 0.3846153846153846, "precision": 0.4166666666666667, "recall": 0.35714285714285715, "support": 14}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.28169014084507044, "precision": 0.1724137931034483, "recall": 0.7692307692307693, "support": 13}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6031746031746031, "precision": 0.4523809523809524, "recall": 0.9047619047619048, "support": 21}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u2423": {"f1-score": 0.7886056971514243, "precision": 0.8566775244299675, "recall": 0.7305555555555555, "support": 1080}},
  "cl_report_full": {"\"": {"f1-score": 0.21782178217821782, "precision": 0.7333333333333333, "recall": 0.12790697674418605, "support": 86}, "\u0027": {"f1-score": 0.4943820224719101, "precision": 0.43137254901960786, "recall": 0.5789473684210527, "support": 76}, "macro avg": {"f1-score": 0.37663009039454975, "precision": 0.43683023355722705, "recall": 0.4040229303298355, "support": 3428}, "micro avg": {"f1-score": 0.7860288534548217, "precision": 0.8197656002534052, "recall": 0.7549591598599766, "support": 3428}, "weighted avg": {"f1-score": 0.7633690609222121, "precision": 0.7974466871610089, "recall": 0.7549591598599766, "support": 3428}, "\u2205": {"f1-score": 0.8888888888888888, "precision": 0.8475509080902587, "recall": 0.9344660194174758, "support": 1648}, "\u23ce": {"f1-score": 0.7423580786026202, "precision": 0.8947368421052632, "recall": 0.6343283582089553, "support": 268}, "\u23ce\u23ce": {"f1-score": 0.1923076923076923, "precision": 0.4166666666666667, "recall": 0.125, "support": 40}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.26315789473684215, "precision": 0.1724137931034483, "recall": 0.5555555555555556, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 80}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5757575757575758, "precision": 0.4523809523809524, "recall": 0.7916666666666666, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u2423": {"f1-score": 0.7682570593963, "precision": 0.8566775244299675, "recall": 0.6963812886142984, "support": 1133}},
  "ppcr": 0.9209451575262544
}
```
</details>