# Train report for javascript / file:///tmp/top-repos-quality-repos-syesqkv9/interactive-tabletop-projected-old.git HEAD 9456d214c4116020ae10bd093fee88e3d5a897a4

### Classification report

PPCR: 0.790

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.976| 0.837| 0.980| 0.905| 6159| 7181| 0.858 |
| `␣` | 0.927| 0.982| 0.803| 0.954| 0.861| 3158| 3862| 0.818 |
| `⏎␣⁺␣⁺` | 0.946| 0.813| 0.799| 0.875| 0.866| 343| 349| 0.983 |
| `⏎␣⁻␣⁻` | 0.992| 0.839| 0.695| 0.909| 0.817| 280| 338| 0.828 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 647| 0.057 |
| `"` | 1.000| 1.000| 0.022| 1.000| 0.043| 2| 90| 0.022 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 164| 0.012 |
| `macro avg` | 0.693| 0.659| 0.451| 0.674| 0.499| 9981| 12631| 0.790 |
| `weighted avg` | 0.962| 0.965| 0.762| 0.963| 0.824| 9981| 12631| 0.790 |
| `micro avg` | 0.965| 0.965| 0.762| 0.965| 0.852| 9981| 12631| 0.790 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1022 |6010 |145 |0 |4 |0 |0 |0 |
|704 |46 |3102 |0 |8 |2 |0 |0 |
|610 |0 |35 |0 |2 |0 |0 |0 |
|6 |0 |64 |0 |279 |0 |0 |0 |
|58 |45 |0 |0 |0 |235 |0 |0 |
|88 |0 |0 |0 |0 |0 |2 |0 |
|162 |0 |0 |0 |2 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Content/MAZE/src/game/mazeModel.js | 111 |
| Content/MAZE/src/game/game.js | 88 |
| Client/src/library/clientLibrary.js | 67 |
| Content/MAZE/lib/kiwipreloader.js | 56 |
| Client/gruntfile.js | 12 |
| Content/MAZE/src/util/util.js | 8 |
| Content/MAZE/src/main.js | 4 |
| Content/MAZE/src/game/player.js | 4 |
| Content/MAZE/gruntfile.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2}, "macro avg": {"f1-score": 0.673999677408971, "precision": 0.6927836303834061, "recall": 0.6586816873944503, "support": 9981}, "micro avg": {"f1-score": 0.9646328023244164, "precision": 0.9646328023244164, "recall": 0.9646328023244164, "support": 9981}, "weighted avg": {"f1-score": 0.9625592769940693, "precision": 0.9617150727156714, "recall": 0.9646328023244164, "support": 9981}, "\u2205": {"f1-score": 0.9804241435562805, "precision": 0.9850844123914112, "recall": 0.9758077610001623, "support": 6159}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8746081504702196, "precision": 0.9457627118644067, "recall": 0.8134110787172012, "support": 343}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9090909090909091, "precision": 0.9915611814345991, "recall": 0.8392857142857143, "support": 280}, "\u2423": {"f1-score": 0.9538745387453875, "precision": 0.9270771069934249, "recall": 0.9822672577580748, "support": 3158}},
  "cl_report_full": {"\"": {"f1-score": 0.04347826086956522, "precision": 1.0, "recall": 0.022222222222222223, "support": 90}, "macro avg": {"f1-score": 0.4990033862199032, "precision": 0.6927836303834061, "recall": 0.4510081413876482, "support": 12631}, "micro avg": {"f1-score": 0.8515832301432867, "precision": 0.9646328023244164, "recall": 0.76225160319848, "support": 12631}, "weighted avg": {"f1-score": 0.8237938753428847, "precision": 0.9032920448386433, "recall": 0.76225160319848, "support": 12631}, "\u2205": {"f1-score": 0.9049841891281434, "precision": 0.9850844123914112, "recall": 0.8369307895836234, "support": 7181}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 647}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 164}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8664596273291926, "precision": 0.9457627118644067, "recall": 0.7994269340974212, "support": 349}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8173913043478261, "precision": 0.9915611814345991, "recall": 0.6952662721893491, "support": 338}, "\u2423": {"f1-score": 0.8607103218645948, "precision": 0.9270771069934249, "recall": 0.8032107716209218, "support": 3862}},
  "ppcr": 0.7901987174412161
}
```
</details>
