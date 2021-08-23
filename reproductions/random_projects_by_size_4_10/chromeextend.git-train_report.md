# Train report for javascript / file:///tmp/top-repos-quality-repos-9j4wroyv/chromeextend.git HEAD 54cb2b47c3647bb1b9fb3f9e6b38aa3a07357450

### Classification report

PPCR: 0.784

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.989| 0.969| 0.989| 0.979| 49413| 50441| 0.980 |
| `␣` | 0.955| 0.993| 0.903| 0.974| 0.928| 20823| 22901| 0.909 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.950| 0.878| 0.664| 0.912| 0.782| 2111| 2790| 0.757 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.973| 0.921| 0.636| 0.946| 0.769| 1796| 2602| 0.690 |
| `⏎` | 0.929| 0.611| 0.137| 0.737| 0.239| 1221| 5434| 0.225 |
| `'` | 1.000| 1.000| 0.052| 1.000| 0.099| 346| 6662| 0.052 |
| `"` | 1.000| 1.000| 0.077| 1.000| 0.142| 329| 4291| 0.077 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 423| 0.085 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 426| 0.077 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 1165| 0.010 |
| `macro avg` | 0.680| 0.639| 0.344| 0.656| 0.394| 76120| 97135| 0.784 |
| `weighted avg` | 0.977| 0.978| 0.767| 0.977| 0.797| 76120| 97135| 0.784 |
| `micro avg` | 0.978| 0.978| 0.767| 0.978| 0.860| 76120| 97135| 0.784 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1028 |48857 |524 |0 |0 |0 |23 |9 |0 |0 |0 |
|2078 |77 |20674 |0 |51 |0 |18 |3 |0 |0 |0 |
|6316 |0 |0 |346 |0 |0 |0 |0 |0 |0 |0 |
|4213 |146 |301 |0 |746 |0 |24 |4 |0 |0 |0 |
|3962 |0 |0 |0 |0 |329 |0 |0 |0 |0 |0 |
|679 |163 |95 |0 |0 |0 |1853 |0 |0 |0 |0 |
|806 |103 |39 |0 |0 |0 |0 |1654 |0 |0 |0 |
|1153 |0 |3 |0 |6 |0 |3 |0 |0 |0 |0 |
|393 |0 |3 |0 |0 |0 |30 |0 |0 |0 |0 |
|387 |6 |0 |0 |0 |0 |0 |30 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| idscassistantmap/mapSubmitRules.js | 181 |
| idscassistant/lib/diff.js | 116 |
| idscassistantmap/idscassistant/lib/diff.js | 116 |
| idscassistantmap/lib/diff.js | 116 |
| idscassistant/contentImageText.js | 76 |
| idscassistantmap/idscassistant/contentImageText.js | 76 |
| idscassistant/chromeExtend/content/publicboss/contentImageText.js | 76 |
| idscassistantmap/contentMap.js | 48 |
| idscassistant/contentVideo.js | 46 |
| idscassistantmap/idscassistant/contentVideo.js | 46 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 329}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 346}, "macro avg": {"f1-score": 0.6558860206969596, "precision": 0.6797101319717505, "recall": 0.6391285415411764, "support": 76120}, "micro avg": {"f1-score": 0.9781791907514451, "precision": 0.9781791907514451, "recall": 0.9781791907514451, "support": 76120}, "weighted avg": {"f1-score": 0.9769357864486893, "precision": 0.9770555995567739, "recall": 0.9781791907514451, "support": 76120}, "\u2205": {"f1-score": 0.9893585784437807, "precision": 0.9899700113470579, "recall": 0.9887479003501103, "support": 49413}, "\u23ce": {"f1-score": 0.7371541501976284, "precision": 0.9290161892901619, "recall": 0.6109746109746109, "support": 1221}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.912358444116199, "precision": 0.9497693490517684, "recall": 0.8777830412126955, "support": 2111}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9462242562929063, "precision": 0.9729411764705882, "recall": 0.920935412026726, "support": 1796}, "\u2423": {"f1-score": 0.9737647779190807, "precision": 0.9554045935579278, "recall": 0.9928444508476204, "support": 20823}},
  "cl_report_full": {"\"": {"f1-score": 0.14242424242424243, "precision": 1.0, "recall": 0.0766721044045677, "support": 4291}, "\u0027": {"f1-score": 0.09874429223744292, "precision": 1.0, "recall": 0.05193635544881417, "support": 6662}, "macro avg": {"f1-score": 0.39385233675026865, "precision": 0.6797101319717505, "recall": 0.34370671208619014, "support": 97135}, "micro avg": {"f1-score": 0.8595307494733196, "precision": 0.9781791907514451, "recall": 0.7665517063880167, "support": 97135}, "weighted avg": {"f1-score": 0.7968346532567816, "precision": 0.9574048626865774, "recall": 0.7665517063880167, "support": 97135}, "\u2205": {"f1-score": 0.9791668754321445, "precision": 0.9899700113470579, "recall": 0.9685969746832933, "support": 50441}, "\u23ce": {"f1-score": 0.2392175725509059, "precision": 0.9290161892901619, "recall": 0.13728376886271623, "support": 5434}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1165}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 426}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.78169162623919, "precision": 0.9497693490517684, "recall": 0.6641577060931899, "support": 2790}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 423}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7689446768944677, "precision": 0.9729411764705882, "recall": 0.6356648731744812, "support": 2602}, "\u2423": {"f1-score": 0.9283340817242928, "precision": 0.9554045935579278, "recall": 0.9027553381948387, "support": 22901}},
  "ppcr": 0.7836516188809389
}
```
</details>
