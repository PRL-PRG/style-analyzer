# Train report for javascript / file:///tmp/top-repos-quality-repos-4w_qnvtl/zeronet.git HEAD 454c0b2e7e000fda7000cba49027541fbf327b96

### Classification report

PPCR: 0.769

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 0.988| 0.956| 0.990| 0.973| 17107| 17689| 0.967 |
| `␣` | 0.970| 0.989| 0.884| 0.980| 0.925| 10944| 12252| 0.893 |
| `"` | 1.000| 1.000| 0.158| 1.000| 0.273| 173| 1095| 0.158 |
| `⏎` | 1.000| 0.675| 0.052| 0.806| 0.099| 169| 2179| 0.078 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 77| 533| 0.144 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 489| 0.041 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 871| 0.011 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 814| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 870| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 249| 0.000 |
| `weighted avg` | 0.979| 0.983| 0.756| 0.981| 0.784| 28500| 37041| 0.769 |
| `macro avg` | 0.396| 0.365| 0.205| 0.377| 0.227| 28500| 37041| 0.769 |
| `micro avg` | 0.983| 0.983| 0.756| 0.983| 0.855| 28500| 37041| 0.769 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|582 |16902 |205 |0 |0 |0 |0 |0 |0 |0 |0 |
|1308 |119 |10825 |0 |0 |0 |0 |0 |0 |0 |0 |
|2010 |20 |35 |114 |0 |0 |0 |0 |0 |0 |0 |
|922 |0 |0 |0 |173 |0 |0 |0 |0 |0 |0 |
|456 |0 |77 |0 |0 |0 |0 |0 |0 |0 |0 |
|469 |14 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|814 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|870 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|249 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|861 |0 |10 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/Ui/media/lib/jquery.easing.js | 129 |
| plugins/UiConfig/media/js/all.js | 98 |
| plugins/Sidebar/media_globe/globe.js | 97 |
| plugins/UiPluginManager/media/js/all.js | 79 |
| plugins/Sidebar/media_globe/Detector.js | 30 |
| plugins/UiConfig/media/js/lib/maquette.js | 16 |
| plugins/UiPluginManager/media/js/lib/maquette.js | 16 |
| plugins/ContentFilter/media/js/ZeroFrame.js | 10 |
| plugins/Sidebar/media/Scrollable.js | 7 |
| src/Ui/media/lib/jquery.cssanim.js | 4 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 173}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3774723592171919, "precision": 0.3961184965671778, "recall": 0.3651699276397299, "support": 28500}, "micro avg": {"f1-score": 0.9829473684210527, "precision": 0.9829473684210527, "recall": 0.9829473684210527, "support": 28500}, "weighted avg": {"f1-score": 0.9809498676775218, "precision": 0.9794007065846618, "recall": 0.9829473684210527, "support": 28500}, "\u2205": {"f1-score": 0.9895205198758856, "precision": 0.9910290237467019, "recall": 0.9880166013912434, "support": 17107}, "\u23ce": {"f1-score": 0.8056537102473499, "precision": 1.0, "recall": 0.6745562130177515, "support": 169}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9795493620486834, "precision": 0.9701559419250761, "recall": 0.9891264619883041, "support": 10944}},
  "cl_report_full": {"\"": {"f1-score": 0.27287066246056785, "precision": 1.0, "recall": 0.15799086757990868, "support": 1095}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 814}, "macro avg": {"f1-score": 0.22700671421587462, "precision": 0.3961184965671778, "recall": 0.20493467376047306, "support": 37041}, "micro avg": {"f1-score": 0.8548542133931433, "precision": 0.9829473684210527, "recall": 0.7562970762128453, "support": 37041}, "weighted avg": {"f1-score": 0.7844485099261758, "precision": 0.8825534678200221, "recall": 0.7562970762128453, "support": 37041}, "\u2205": {"f1-score": 0.9729449689154963, "precision": 0.9910290237467019, "recall": 0.9555090734354684, "support": 17689}, "\u23ce": {"f1-score": 0.09943305713039687, "precision": 1.0, "recall": 0.05231757687012391, "support": 2179}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 249}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 533}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 871}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 489}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 870}, "\u2423": {"f1-score": 0.9248184536522853, "precision": 0.9701559419250761, "recall": 0.8835292197192295, "support": 12252}},
  "ppcr": 0.7694176723090629
}
```
</details>
