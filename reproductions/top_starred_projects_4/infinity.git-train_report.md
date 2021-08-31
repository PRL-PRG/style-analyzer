# Train report for javascript / file:///tmp/top-repos-quality-repos-ry4jnrsx/infinity.git HEAD 2f38017af57363ef3b7f6d584264074b337eba22

### Classification report

PPCR: 0.564

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 0.997| 0.938| 0.985| 0.956| 3794| 4031| 0.941 |
| `␣` | 0.919| 0.749| 0.102| 0.825| 0.183| 243| 1793| 0.136 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 181| 0.099 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 187| 0.080 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 846| 0.007 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 202| 0.030 |
| `macro avg` | 0.315| 0.291| 0.173| 0.302| 0.190| 4082| 7240| 0.564 |
| `weighted avg` | 0.960| 0.971| 0.548| 0.965| 0.577| 4082| 7240| 0.564 |
| `micro avg` | 0.971| 0.971| 0.548| 0.971| 0.700| 4082| 7240| 0.564 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|237 |3782 |12 |0 |0 |0 |0 |
|1550 |61 |182 |0 |0 |0 |0 |
|840 |6 |0 |0 |0 |0 |0 |
|196 |6 |0 |0 |0 |0 |0 |
|172 |11 |4 |0 |0 |0 |0 |
|163 |18 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| infinity.js | 63 |
| build/infinity.js | 55 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.30175820147166854, "precision": 0.3154883888663497, "recall": 0.2909680507739822, "support": 4082}, "micro avg": {"f1-score": 0.9710926016658501, "precision": 0.9710926016658501, "recall": 0.9710926016658501, "support": 4082}, "weighted avg": {"f1-score": 0.9647818646048347, "precision": 0.9597567807699854, "recall": 0.9710926016658501, "support": 4082}, "\u2205": {"f1-score": 0.9851523834331858, "precision": 0.9737384140061792, "recall": 0.9968371112282551, "support": 3794}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u2423": {"f1-score": 0.8253968253968255, "precision": 0.9191919191919192, "recall": 0.7489711934156379, "support": 243}},
  "cl_report_full": {"macro avg": {"f1-score": 0.18974608733615864, "precision": 0.3154883888663497, "recall": 0.1732890972450034, "support": 7240}, "micro avg": {"f1-score": 0.700229641406112, "precision": 0.9710926016658501, "recall": 0.5475138121546961, "support": 7240}, "weighted avg": {"f1-score": 0.5773538205633816, "precision": 0.7697860024820469, "recall": 0.5475138121546961, "support": 7240}, "\u2205": {"f1-score": 0.9556538218572331, "precision": 0.9737384140061792, "recall": 0.9382287273629373, "support": 4031}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 846}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 202}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 187}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 181}, "\u2423": {"f1-score": 0.18282270215971871, "precision": 0.9191919191919192, "recall": 0.1015058561070831, "support": 1793}},
  "ppcr": 0.5638121546961326
}
```
</details>
