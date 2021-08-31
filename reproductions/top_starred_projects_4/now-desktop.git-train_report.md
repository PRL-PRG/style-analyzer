# Train report for javascript / file:///tmp/top-repos-quality-repos-tvmym2ff/now-desktop.git HEAD 73e576f1a969d6169ecd0dd1bda2014f3a409515

### Classification report

PPCR: 0.882

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.890| 0.997| 0.979| 0.941| 0.933| 18400| 18742| 0.982 |
| `␣` | 0.983| 0.925| 0.721| 0.953| 0.832| 8819| 11311| 0.780 |
| `'` | 0.998| 1.000| 0.921| 0.999| 0.958| 2453| 2664| 0.921 |
| `⏎␣⁻␣⁻` | 0.979| 0.681| 0.678| 0.803| 0.801| 1604| 1612| 0.995 |
| `⏎` | 0.940| 0.823| 0.712| 0.878| 0.810| 1199| 1387| 0.864 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 878| 878| 1.000 |
| `"` | 1.000| 0.989| 0.974| 0.995| 0.987| 374| 380| 0.984 |
| `⏎␣⁺␣⁺` | 0.976| 0.467| 0.099| 0.632| 0.180| 353| 1663| 0.212 |
| `macro avg` | 0.846| 0.735| 0.635| 0.775| 0.688| 34080| 38637| 0.882 |
| `weighted avg` | 0.907| 0.927| 0.817| 0.913| 0.842| 34080| 38637| 0.882 |
| `micro avg` | 0.927| 0.927| 0.817| 0.927| 0.869| 34080| 38637| 0.882 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|342 |18353 |35 |0 |0 |4 |8 |0 |0 |
|2492 |591 |8158 |0 |54 |0 |16 |0 |0 |
|211 |0 |0 |2453 |0 |0 |0 |0 |0 |
|188 |166 |46 |0 |987 |0 |0 |0 |0 |
|1310 |146 |42 |0 |0 |165 |0 |0 |0 |
|8 |505 |3 |0 |3 |0 |1093 |0 |0 |
|0 |860 |12 |0 |6 |0 |0 |0 |0 |
|6 |0 |0 |4 |0 |0 |0 |0 |370 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| main/utils/deploy/index.js | 231 |
| renderer/pages/feed.js | 206 |
| renderer/components/feed/switcher.js | 203 |
| main/utils/deploy/get-files.js | 142 |
| main/utils/binary.js | 131 |
| renderer/components/tutorial/login.js | 115 |
| main/utils/config.js | 86 |
| main/updates.js | 78 |
| main/utils/deploy/read-metadata.js | 61 |
| main/index.js | 54 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9946236559139785, "precision": 1.0, "recall": 0.9893048128342246, "support": 374}, "\u0027": {"f1-score": 0.9991853360488798, "precision": 0.9983719983719984, "recall": 1.0, "support": 2453}, "macro avg": {"f1-score": 0.7751357772931917, "precision": 0.8458247182913217, "recall": 0.7354785234297478, "support": 34080}, "micro avg": {"f1-score": 0.9266138497652583, "precision": 0.9266138497652582, "recall": 0.9266138497652582, "support": 34080}, "weighted avg": {"f1-score": 0.9126420258474442, "precision": 0.9070664464825057, "recall": 0.9266138497652582, "support": 34080}, "\u2205": {"f1-score": 0.9406729709643527, "precision": 0.8900150332185636, "recall": 0.997445652173913, "support": 18400}, "\u23ce": {"f1-score": 0.8777234326367275, "precision": 0.94, "recall": 0.823185988323603, "support": 1199}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 878}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.632183908045977, "precision": 0.9763313609467456, "recall": 0.46742209631728043, "support": 353}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8033811098860713, "precision": 0.9785138764547896, "recall": 0.6814214463840399, "support": 1604}, "\u2423": {"f1-score": 0.9533158048495471, "precision": 0.9833654773384763, "recall": 0.9250481914049212, "support": 8819}},
  "cl_report_full": {"\"": {"f1-score": 0.9866666666666666, "precision": 1.0, "recall": 0.9736842105263158, "support": 380}, "\u0027": {"f1-score": 0.9580160124975591, "precision": 0.9983719983719984, "recall": 0.9207957957957958, "support": 2664}, "macro avg": {"f1-score": 0.6875629820803222, "precision": 0.8458247182913217, "recall": 0.6354793823683144, "support": 38637}, "micro avg": {"f1-score": 0.8685451820069584, "precision": 0.9266138497652582, "recall": 0.817325361699925, "support": 38637}, "weighted avg": {"f1-score": 0.8419590513451428, "precision": 0.9148731809540177, "recall": 0.817325361699925, "support": 38637}, "\u2205": {"f1-score": 0.9325000635114193, "precision": 0.8900150332185636, "recall": 0.9792444776437946, "support": 18742}, "\u23ce": {"f1-score": 0.8100123102174804, "precision": 0.94, "recall": 0.7116077865897621, "support": 1387}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 878}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.18013100436681223, "precision": 0.9763313609467456, "recall": 0.09921828021647625, "support": 1663}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8010260168559912, "precision": 0.9785138764547896, "recall": 0.6780397022332506, "support": 1612}, "\u2423": {"f1-score": 0.8321517825266488, "precision": 0.9833654773384763, "recall": 0.7212448059411193, "support": 11311}},
  "ppcr": 0.8820560602531252
}
```
</details>
