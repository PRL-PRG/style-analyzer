# Train report for javascript / file:///tmp/top-repos-quality-repos-ma68wm4s/editly.git HEAD 6e69f4b33949502b9964c7f2826c07d2ffa20e1e

### Classification report

PPCR: 0.736

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.962| 0.981| 0.920| 0.972| 0.941| 6468| 6900| 0.937 |
| `␣` | 0.952| 0.958| 0.635| 0.955| 0.762| 3671| 5533| 0.663 |
| `'` | 1.000| 1.000| 0.869| 1.000| 0.930| 998| 1148| 0.869 |
| `⏎␣⁺␣⁺` | 0.953| 0.922| 0.668| 0.938| 0.785| 244| 337| 0.724 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 82| 915| 0.090 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 54| 333| 0.162 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 487| 0.021 |
| `macro avg` | 0.552| 0.552| 0.442| 0.552| 0.488| 11527| 15653| 0.736 |
| `weighted avg` | 0.950| 0.962| 0.708| 0.956| 0.769| 11527| 15653| 0.736 |
| `micro avg` | 0.962| 0.962| 0.708| 0.962| 0.816| 11527| 15653| 0.736 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|432 |6348 |120 |0 |0 |0 |0 |0 |
|1862 |144 |3516 |0 |0 |0 |11 |0 |
|150 |0 |0 |998 |0 |0 |0 |0 |
|833 |48 |34 |0 |0 |0 |0 |0 |
|477 |2 |8 |0 |0 |0 |0 |0 |
|93 |3 |16 |0 |0 |0 |225 |0 |
|279 |54 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| index.js | 88 |
| colors.js | 78 |
| audio.js | 68 |
| parseConfig.js | 55 |
| util.js | 25 |
| ffmpeg.js | 18 |
| sources/fabric/fabricFrameSources.js | 17 |
| sources/videoFrameSource.js | 17 |
| glTransitions.js | 13 |
| sources/fabric.js | 12 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 998}, "macro avg": {"f1-score": 0.5519848596941539, "precision": 0.5524525023525968, "recall": 0.5516222063254044, "support": 11527}, "micro avg": {"f1-score": 0.961828749891559, "precision": 0.961828749891559, "recall": 0.961828749891559, "support": 11527}, "weighted avg": {"f1-score": 0.9556805451521404, "precision": 0.9496588988464147, "recall": 0.961828749891559, "support": 11527}, "\u2205": {"f1-score": 0.9716078671462463, "precision": 0.9619639339293833, "recall": 0.9814471243042672, "support": 6468}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9375, "precision": 0.9533898305084746, "recall": 0.9221311475409836, "support": 244}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u2423": {"f1-score": 0.9547861507128309, "precision": 0.9518137520303195, "recall": 0.9577771724325796, "support": 3671}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9301025163094128, "precision": 1.0, "recall": 0.8693379790940766, "support": 1148}, "macro avg": {"f1-score": 0.48829544828668353, "precision": 0.5524525023525968, "recall": 0.44177910470173504, "support": 15653}, "micro avg": {"f1-score": 0.8158204562178072, "precision": 0.961828749891559, "recall": 0.7082987286782086, "support": 15653}, "weighted avg": {"f1-score": 0.7691002359063013, "precision": 0.8543556511197763, "recall": 0.7082987286782086, "support": 15653}, "\u2205": {"f1-score": 0.940514112156456, "precision": 0.9619639339293833, "recall": 0.92, "support": 6900}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 915}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 487}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7853403141361256, "precision": 0.9533898305084746, "recall": 0.6676557863501483, "support": 337}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 333}, "\u2423": {"f1-score": 0.7621111954047903, "precision": 0.9518137520303195, "recall": 0.6354599674679198, "support": 5533}},
  "ppcr": 0.7364083562256436
}
```
</details>
