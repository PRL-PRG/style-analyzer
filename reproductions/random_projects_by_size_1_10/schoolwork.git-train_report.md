# Train report for javascript / file:///tmp/top-repos-quality-repos-u09lph72/schoolwork.git HEAD 1afb6b507d106a90c2493e4b4bb632a160dfe3e5

### Classification report

PPCR: 0.702

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.990| 0.796| 0.991| 0.884| 81821| 101660| 0.805 |
| `␣` | 0.977| 0.992| 0.780| 0.984| 0.868| 61823| 78575| 0.787 |
| `⏎` | 0.951| 0.913| 0.334| 0.931| 0.495| 4529| 12363| 0.366 |
| `⏎␣⁺␣⁺` | 0.958| 0.983| 0.766| 0.971| 0.851| 3514| 4513| 0.779 |
| `⏎⇥⁺` | 0.949| 0.858| 0.613| 0.901| 0.745| 2118| 2964| 0.715 |
| `⏎⏎` | 0.960| 0.880| 0.224| 0.918| 0.364| 1579| 6198| 0.255 |
| `⏎␣⁻␣⁻` | 0.997| 0.971| 0.327| 0.983| 0.493| 1470| 4358| 0.337 |
| `⏎⇥⁻` | 0.980| 0.962| 0.242| 0.971| 0.388| 678| 2692| 0.252 |
| `"` | 0.967| 1.000| 0.122| 0.983| 0.217| 528| 4324| 0.122 |
| `'` | 1.000| 0.897| 0.022| 0.945| 0.044| 174| 6974| 0.025 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 90| 188| 0.479 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 42| 378| 0.111 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 406| 0.089 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 116| 0.017 |
| `weighted avg` | 0.983| 0.984| 0.690| 0.983| 0.784| 158404| 225709| 0.702 |
| `macro avg` | 0.695| 0.675| 0.302| 0.684| 0.382| 158404| 225709| 0.702 |
| `micro avg` | 0.984| 0.984| 0.690| 0.984| 0.811| 158404| 225709| 0.702 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎⇥⁻| ⏎⏎⇥⁺| ⏎⏎⏎| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|19839 |80963 |803 |1 |0 |1 |0 |48 |5 |0 |0 |0 |0 |0 |0 |
|16752 |448 |61316 |0 |0 |14 |0 |42 |0 |0 |3 |0 |0 |0 |0 |
|7834 |54 |305 |4135 |0 |21 |0 |0 |0 |14 |0 |0 |0 |0 |0 |
|6800 |0 |0 |0 |156 |0 |18 |0 |0 |0 |0 |0 |0 |0 |0 |
|4619 |17 |6 |116 |0 |1390 |0 |0 |0 |50 |0 |0 |0 |0 |0 |
|3796 |0 |0 |0 |0 |0 |528 |0 |0 |0 |0 |0 |0 |0 |0 |
|999 |10 |48 |0 |0 |0 |0 |3456 |0 |0 |0 |0 |0 |0 |0 |
|2888 |33 |2 |8 |0 |0 |0 |0 |1427 |0 |0 |0 |0 |0 |0 |
|846 |4 |250 |2 |0 |0 |0 |44 |0 |1818 |0 |0 |0 |0 |0 |
|2014 |4 |18 |0 |0 |0 |0 |0 |0 |4 |652 |0 |0 |0 |0 |
|370 |0 |0 |0 |0 |20 |0 |0 |0 |6 |10 |0 |0 |0 |0 |
|336 |2 |0 |0 |0 |0 |0 |18 |0 |22 |0 |0 |0 |0 |0 |
|98 |0 |0 |88 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|114 |0 |0 |0 |0 |0 |0 |0 |0 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Web App Development/Homework2/Homework2/scripts/jquery-3.3.1.slim.js | 688 |
| Web App Development/Homework3/PriceQuote/scripts/jquery-3.3.1.slim.js | 688 |
| Web App Development/Homework3/PriceQuote/scripts/bootstrap.bundle.js | 207 |
| Web App Development/Homework2/Homework2/scripts/bootstrap.bundle.js | 207 |
| Web App Development/Homework 1 - Resume/.cache/static-entry.js | 139 |
| Web App Development/Homework3/PriceQuote/scripts/popper.js | 89 |
| Web App Development/Homework2/Homework2/scripts/popper.js | 89 |
| Web App Development/Homework 1 - Resume/.cache/loader.js | 53 |
| Web App Development/Homework3/PriceQuote/scripts/umd/popper.js | 46 |
| Web App Development/Homework2/Homework2/scripts/umd/popper.js | 46 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9832402234636871, "precision": 0.967032967032967, "recall": 1.0, "support": 528}, "\u0027": {"f1-score": 0.9454545454545454, "precision": 1.0, "recall": 0.896551724137931, "support": 174}, "macro avg": {"f1-score": 0.684318603905829, "precision": 0.6950998766800479, "recall": 0.6746732432715226, "support": 158404}, "micro avg": {"f1-score": 0.9838198530340143, "precision": 0.9838198530340143, "recall": 0.9838198530340143, "support": 158404}, "weighted avg": {"f1-score": 0.9831888192604782, "precision": 0.9827393487970213, "recall": 0.9838198530340143, "support": 158404}, "\u2205": {"f1-score": 0.9912461127843483, "precision": 0.9929846078371252, "recall": 0.9895136945282996, "support": 81821}, "\u23ce": {"f1-score": 0.9314111949543867, "precision": 0.9505747126436782, "recall": 0.9130050783837492, "support": 4529}, "\u23ce\u21e5\u207a": {"f1-score": 0.901338621715419, "precision": 0.9488517745302714, "recall": 0.8583569405099151, "support": 2118}, "\u23ce\u21e5\u207b": {"f1-score": 0.9709605361131795, "precision": 0.9804511278195489, "recall": 0.9616519174041298, "support": 678}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.9184010571522959, "precision": 0.9599447513812155, "recall": 0.8803039898670044, "support": 1579}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 90}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.970513900589722, "precision": 0.9578713968957872, "recall": 0.983494593056346, "support": 3514}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.983459682977257, "precision": 0.9965083798882681, "recall": 0.9707482993197278, "support": 1470}, "\u2423": {"f1-score": 0.9844345794767643, "precision": 0.9771785554918085, "recall": 0.9917991685942125, "support": 61823}},
  "cl_report_full": {"\"": {"f1-score": 0.21683778234086246, "precision": 0.967032967032967, "recall": 0.12210915818686402, "support": 4324}, "\u0027": {"f1-score": 0.04375876577840112, "precision": 1.0, "recall": 0.022368798394034987, "support": 6974}, "macro avg": {"f1-score": 0.38201583300579156, "precision": 0.6950998766800479, "recall": 0.30205429402464157, "support": 225709}, "micro avg": {"f1-score": 0.8114330939072617, "precision": 0.9838198530340143, "recall": 0.690450978915329, "support": 225709}, "weighted avg": {"f1-score": 0.7837406739528656, "precision": 0.9778218491551415, "recall": 0.690450978915329, "support": 225709}, "\u2205": {"f1-score": 0.8838996697508119, "precision": 0.9929846078371252, "recall": 0.7964096006295495, "support": 101660}, "\u23ce": {"f1-score": 0.4948243882008018, "precision": 0.9505747126436782, "recall": 0.3344657445603818, "support": 12363}, "\u23ce\u21e5\u207a": {"f1-score": 0.7450819672131147, "precision": 0.9488517745302714, "recall": 0.6133603238866396, "support": 2964}, "\u23ce\u21e5\u207b": {"f1-score": 0.38844206136431336, "precision": 0.9804511278195489, "recall": 0.24219910846953938, "support": 2692}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 116}, "\u23ce\u23ce": {"f1-score": 0.36358880460371434, "precision": 0.9599447513812155, "recall": 0.22426589222329785, "support": 6198}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 378}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 406}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 188}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8511267085334319, "precision": 0.9578713968957872, "recall": 0.7657877243518724, "support": 4513}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.49291882556131256, "precision": 0.9965083798882681, "recall": 0.32744378155117027, "support": 4358}, "\u2423": {"f1-score": 0.8677426887343179, "precision": 0.9771785554918085, "recall": 0.7803499840916323, "support": 78575}},
  "ppcr": 0.7018063081224054
}
```
</details>
