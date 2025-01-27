# Test report for javascript / file:///tmp/top-repos-quality-repos-77014cct/eslint-plugin-jsx-a11y.git HEAD 125108849e4830a2aa98ae46039493900c45b0c7

### Classification report

PPCR: 0.979

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.939| 0.986| 0.974| 0.962| 0.956| 6185| 6260| 0.988 |
| `␣` | 0.940| 0.941| 0.938| 0.941| 0.939| 4740| 4757| 0.996 |
| `'` | 0.998| 0.997| 0.997| 0.998| 0.998| 2102| 2102| 1.000 |
| `⏎` | 0.957| 0.760| 0.668| 0.847| 0.787| 1132| 1287| 0.880 |
| `⏎␣⁺␣⁺` | 0.788| 0.777| 0.770| 0.782| 0.778| 391| 395| 0.990 |
| `⏎␣⁻␣⁻` | 0.979| 0.935| 0.848| 0.957| 0.909| 341| 376| 0.907 |
| `⏎⏎` | 0.866| 0.567| 0.455| 0.686| 0.597| 171| 213| 0.803 |
| `micro avg` | 0.945| 0.945| 0.925| 0.945| 0.935| 15062| 15390| 0.979 |
| `weighted avg` | 0.945| 0.945| 0.925| 0.944| 0.932| 15062| 15390| 0.979 |
| `macro avg` | 0.924| 0.852| 0.807| 0.882| 0.852| 15062| 15390| 0.979 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|75 |6099 |84 |0 |0 |2 |0 |0 |
|17 |182 |4460 |0 |14 |80 |3 |1 |
|0 |6 |0 |2096 |0 |0 |0 |0 |
|155 |86 |168 |0 |860 |0 |4 |14 |
|4 |60 |22 |4 |1 |304 |0 |0 |
|35 |14 |8 |0 |0 |0 |319 |0 |
|42 |49 |1 |0 |24 |0 |0 |97 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9976201808662543, "precision": 0.9980952380952381, "recall": 0.9971455756422455, "support": 2102}, "macro avg": {"f1-score": 0.8816523792338948, "precision": 0.9237280137579545, "recall": 0.8520164987634047, "support": 15062}, "micro avg": {"f1-score": 0.9450936130659939, "precision": 0.9450936130659939, "recall": 0.9450936130659939, "support": 15062}, "weighted avg": {"f1-score": 0.9436345080881082, "precision": 0.9450795289558558, "recall": 0.9450936130659939, "support": 15062}, "\u2205": {"f1-score": 0.961911521173409, "precision": 0.9388854679802956, "recall": 0.9860953920776071, "support": 6185}, "\u23ce": {"f1-score": 0.8468734613490891, "precision": 0.9566184649610678, "recall": 0.7597173144876325, "support": 1132}, "\u23ce\u23ce": {"f1-score": 0.6855123674911662, "precision": 0.8660714285714286, "recall": 0.5672514619883041, "support": 171}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7824967824967825, "precision": 0.7875647668393783, "recall": 0.7774936061381074, "support": 391}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9565217391304347, "precision": 0.9785276073619632, "recall": 0.9354838709677419, "support": 341}, "\u2423": {"f1-score": 0.9406306021301276, "precision": 0.9403331224963104, "recall": 0.9409282700421941, "support": 4740}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9976201808662543, "precision": 0.9980952380952381, "recall": 0.9971455756422455, "support": 2102}, "macro avg": {"f1-score": 0.8519846832495361, "precision": 0.9237280137579545, "recall": 0.8072338080289531, "support": 15390}, "micro avg": {"f1-score": 0.934913962958098, "precision": 0.9450936130659939, "recall": 0.9249512670565302, "support": 15390}, "weighted avg": {"f1-score": 0.9316923177978564, "precision": 0.9449798262318422, "recall": 0.9249512670565302, "support": 15390}, "\u2205": {"f1-score": 0.9562558795860772, "precision": 0.9388854679802956, "recall": 0.9742811501597444, "support": 6260}, "\u23ce": {"f1-score": 0.7868252516010978, "precision": 0.9566184649610678, "recall": 0.6682206682206682, "support": 1287}, "\u23ce\u23ce": {"f1-score": 0.596923076923077, "precision": 0.8660714285714286, "recall": 0.45539906103286387, "support": 213}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7784891165172856, "precision": 0.7875647668393783, "recall": 0.769620253164557, "support": 395}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9088319088319089, "precision": 0.9785276073619632, "recall": 0.848404255319149, "support": 376}, "\u2423": {"f1-score": 0.9389473684210526, "precision": 0.9403331224963104, "recall": 0.9375656926634434, "support": 4757}},
  "ppcr": 0.9786874593892138
}
```
</details>
