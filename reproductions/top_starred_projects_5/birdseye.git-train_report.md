# Train report for javascript / file:///tmp/top-repos-quality-repos-nbgsvwuk/birdseye.git HEAD 0f0e0b40e16584d232587e0cffec41660a158aa0

### Classification report

PPCR: 0.816

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.987| 0.959| 0.986| 0.972| 6134| 6313| 0.972 |
| `␣` | 0.941| 0.967| 0.579| 0.954| 0.717| 2476| 4134| 0.599 |
| `⏎` | 0.924| 0.896| 0.813| 0.910| 0.865| 714| 787| 0.907 |
| `⏎␣⁻␣⁻` | 0.996| 0.985| 0.977| 0.990| 0.987| 263| 265| 0.992 |
| `⏎␣⁺␣⁺` | 0.974| 1.000| 0.974| 0.987| 0.974| 262| 269| 0.974 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 215| 0.260 |
| `'` | 1.000| 1.000| 0.012| 1.000| 0.024| 2| 164| 0.012 |
| `weighted avg` | 0.965| 0.970| 0.791| 0.967| 0.849| 9907| 12147| 0.816 |
| `macro avg` | 0.831| 0.834| 0.616| 0.832| 0.648| 9907| 12147| 0.816 |
| `micro avg` | 0.970| 0.970| 0.791| 0.970| 0.871| 9907| 12147| 0.816 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|179 |6052 |77 |0 |5 |0 |0 |0 |
|1658 |77 |2395 |1 |2 |1 |0 |0 |
|73 |4 |70 |640 |0 |0 |0 |0 |
|7 |0 |0 |0 |262 |0 |0 |0 |
|2 |4 |0 |0 |0 |259 |0 |0 |
|162 |0 |0 |0 |0 |0 |2 |0 |
|159 |0 |4 |52 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| birdseye/static/js/libs/underscore.js | 171 |
| birdseye/static/js/call.js | 124 |
| gulp/gulpfile.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2}, "macro avg": {"f1-score": 0.8324554383332183, "precision": 0.8314990471099281, "recall": 0.8335810358361512, "support": 9907}, "micro avg": {"f1-score": 0.97002119713334, "precision": 0.97002119713334, "recall": 0.97002119713334, "support": 9907}, "weighted avg": {"f1-score": 0.9672677235801552, "precision": 0.9646469888646387, "recall": 0.97002119713334, "support": 9907}, "\u2205": {"f1-score": 0.9863906772064217, "precision": 0.9861495844875346, "recall": 0.9866318878382785, "support": 6134}, "\u23ce": {"f1-score": 0.9097370291400142, "precision": 0.9235209235209235, "recall": 0.896358543417367, "support": 714}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9868173258003766, "precision": 0.9739776951672863, "recall": 1.0, "support": 262}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9904397705544934, "precision": 0.9961538461538462, "recall": 0.9847908745247148, "support": 263}, "\u2423": {"f1-score": 0.9538032656312226, "precision": 0.9406912804399057, "recall": 0.9672859450726979, "support": 2476}},
  "cl_report_full": {"\u0027": {"f1-score": 0.024096385542168676, "precision": 1.0, "recall": 0.012195121951219513, "support": 164}, "macro avg": {"f1-score": 0.6484114736922607, "precision": 0.8314990471099281, "recall": 0.6163921184097261, "support": 12147}, "micro avg": {"f1-score": 0.8714972340618482, "precision": 0.97002119713334, "recall": 0.7911418457232239, "support": 12147}, "weighted avg": {"f1-score": 0.8487669218288854, "precision": 0.9493020347616788, "recall": 0.7911418457232239, "support": 12147}, "\u2205": {"f1-score": 0.9722088353413655, "precision": 0.9861495844875346, "recall": 0.9586567400601933, "support": 6313}, "\u23ce": {"f1-score": 0.8648648648648649, "precision": 0.9235209235209235, "recall": 0.8132147395171537, "support": 787}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 215}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9739776951672863, "precision": 0.9739776951672863, "recall": 0.9739776951672863, "support": 269}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9866666666666667, "precision": 0.9961538461538462, "recall": 0.9773584905660377, "support": 265}, "\u2423": {"f1-score": 0.7170658682634731, "precision": 0.9406912804399057, "recall": 0.5793420416061925, "support": 4134}},
  "ppcr": 0.815592327323619
}
```
</details>