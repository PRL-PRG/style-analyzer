# Train report for javascript / file:///tmp/top-repos-quality-repos-kh8l8bue/autoprefixer.git HEAD b5b5f5d01c03923d2750f827421b0f4db4b5e1e1

### Classification report

PPCR: 0.900

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.953| 0.986| 0.974| 0.969| 0.963| 20279| 20526| 0.988 |
| `␣` | 0.956| 0.935| 0.885| 0.946| 0.919| 12740| 13460| 0.947 |
| `'` | 1.000| 1.000| 0.973| 1.000| 0.986| 5682| 5842| 0.973 |
| `⏎␣⁺␣⁺` | 0.925| 0.884| 0.857| 0.904| 0.890| 1843| 1900| 0.970 |
| `⏎` | 0.777| 0.658| 0.256| 0.713| 0.385| 903| 2323| 0.389 |
| `⏎⏎` | 0.917| 0.967| 0.545| 0.942| 0.683| 676| 1201| 0.563 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 230| 1831| 0.126 |
| `weighted avg` | 0.950| 0.955| 0.859| 0.952| 0.877| 42353| 47083| 0.900 |
| `micro avg` | 0.955| 0.955| 0.859| 0.955| 0.905| 42353| 47083| 0.900 |
| `macro avg` | 0.790| 0.776| 0.641| 0.782| 0.690| 42353| 47083| 0.900 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|247 |19990 |288 |0 |0 |0 |0 |1 |
|720 |559 |11913 |0 |136 |132 |0 |0 |
|160 |0 |0 |5682 |0 |0 |0 |0 |
|1420 |91 |163 |0 |594 |0 |0 |55 |
|57 |137 |76 |0 |0 |1629 |0 |1 |
|1601 |197 |5 |0 |26 |0 |0 |2 |
|525 |0 |14 |0 |8 |0 |0 |654 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| data/prefixes.js | 251 |
| lib/hacks/grid-utils.js | 232 |
| lib/processor.js | 214 |
| test/autoprefixer.test.js | 164 |
| lib/hacks/gradient.js | 87 |
| lib/supports.js | 57 |
| lib/prefixes.js | 56 |
| lib/transition.js | 54 |
| lib/hacks/grid-rows-columns.js | 32 |
| lib/info.js | 30 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5682}, "macro avg": {"f1-score": 0.7818561490381226, "precision": 0.7898630831535224, "recall": 0.7757118638342989, "support": 42353}, "micro avg": {"f1-score": 0.9553514509007626, "precision": 0.9553514509007627, "recall": 0.9553514509007627, "support": 42353}, "weighted avg": {"f1-score": 0.9521696291370543, "precision": 0.9495968997780183, "recall": 0.9553514509007627, "support": 42353}, "\u2205": {"f1-score": 0.9691416381838898, "precision": 0.9530847716220082, "recall": 0.9857488041816658, "support": 20279}, "\u23ce": {"f1-score": 0.7126574685062987, "precision": 0.7774869109947644, "recall": 0.6578073089700996, "support": 903}, "\u23ce\u23ce": {"f1-score": 0.9416846652267818, "precision": 0.9172510518934082, "recall": 0.9674556213017751, "support": 676}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9039955604883463, "precision": 0.9250425894378195, "recall": 0.8838849701573521, "support": 1843}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 230}, "\u2423": {"f1-score": 0.9455137108615421, "precision": 0.9561762581266554, "recall": 0.9350863422291994, "support": 12740}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9861159319680667, "precision": 1.0, "recall": 0.9726121191372817, "support": 5842}, "macro avg": {"f1-score": 0.6895548386416624, "precision": 0.7898630831535224, "recall": 0.6413120322089819, "support": 47083}, "micro avg": {"f1-score": 0.9048257972181225, "precision": 0.9553514509007627, "recall": 0.8593759955822696, "support": 47083}, "weighted avg": {"f1-score": 0.877466506668751, "precision": 0.912016056415156, "recall": 0.8593759955822696, "support": 47083}, "\u2205": {"f1-score": 0.9633734939759037, "precision": 0.9530847716220082, "recall": 0.9738867777452986, "support": 20526}, "\u23ce": {"f1-score": 0.3848396501457726, "precision": 0.7774869109947644, "recall": 0.25570383125269047, "support": 2323}, "\u23ce\u23ce": {"f1-score": 0.6833855799373041, "precision": 0.9172510518934082, "recall": 0.5445462114904246, "support": 1201}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8899207866703087, "precision": 0.9250425894378195, "recall": 0.8573684210526316, "support": 1900}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1831}, "\u2423": {"f1-score": 0.9192484277942822, "precision": 0.9561762581266554, "recall": 0.8850668647845468, "support": 13460}},
  "ppcr": 0.8995391117813224
}
```
</details>
