# Test report for javascript / file:///tmp/top-repos-quality-repos-559ffeqn/friendslauncher.git HEAD ffdda5d176760eb7d54e67ce35454bfd88aa6f5d

### Classification report

PPCR: 0.929

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.983| 0.963| 0.983| 0.973| 3958| 4040| 0.980 |
| `␣` | 0.918| 0.959| 0.815| 0.938| 0.863| 1395| 1642| 0.850 |
| `'` | 0.997| 0.970| 0.955| 0.983| 0.975| 634| 644| 0.984 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.888| 0.873| 0.868| 0.881| 0.878| 300| 302| 0.993 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.919| 0.881| 0.881| 0.900| 0.900| 295| 295| 1.000 |
| `⏎` | 0.877| 0.883| 0.608| 0.880| 0.718| 274| 398| 0.688 |
| `⏎⏎` | 0.842| 0.623| 0.333| 0.716| 0.478| 77| 144| 0.535 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 16| 1.000 |
| `micro avg` | 0.958| 0.958| 0.890| 0.958| 0.923| 6949| 7481| 0.929 |
| `macro avg` | 0.803| 0.772| 0.678| 0.785| 0.723| 6949| 7481| 0.929 |
| `weighted avg` | 0.956| 0.958| 0.890| 0.957| 0.917| 6949| 7481| 0.929 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|82 |3892 |28 |0 |5 |13 |18 |2 |0 |
|247 |42 |1338 |0 |4 |3 |5 |3 |0 |
|10 |17 |2 |615 |0 |0 |0 |0 |0 |
|124 |3 |26 |0 |242 |1 |0 |2 |0 |
|2 |3 |30 |2 |1 |262 |0 |2 |0 |
|0 |6 |28 |0 |1 |0 |260 |0 |0 |
|67 |0 |6 |0 |23 |0 |0 |48 |0 |
|0 |0 |0 |0 |0 |16 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9832134292565947, "precision": 0.9967585089141004, "recall": 0.9700315457413249, "support": 634}, "macro avg": {"f1-score": 0.7850777292407883, "precision": 0.8027898284408475, "recall": 0.7717217262505657, "support": 6949}, "micro avg": {"f1-score": 0.9579795654050942, "precision": 0.9579795654050942, "recall": 0.9579795654050942, "support": 6949}, "weighted avg": {"f1-score": 0.956574844720953, "precision": 0.9557883945117255, "recall": 0.9579795654050942, "support": 6949}, "\u2205": {"f1-score": 0.9827042040146445, "precision": 0.9820842795861721, "recall": 0.9833249115715007, "support": 3958}, "\u23ce": {"f1-score": 0.8800000000000001, "precision": 0.8768115942028986, "recall": 0.8832116788321168, "support": 274}, "\u23ce\u23ce": {"f1-score": 0.7164179104477613, "precision": 0.8421052631578947, "recall": 0.6233766233766234, "support": 77}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8806722689075629, "precision": 0.888135593220339, "recall": 0.8733333333333333, "support": 300}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8996539792387543, "precision": 0.9187279151943463, "recall": 0.8813559322033898, "support": 295}, "\u2423": {"f1-score": 0.9379600420609885, "precision": 0.9176954732510288, "recall": 0.9591397849462365, "support": 1395}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9754163362410785, "precision": 0.9967585089141004, "recall": 0.9549689440993789, "support": 644}, "macro avg": {"f1-score": 0.7230457696578848, "precision": 0.8027898284408475, "recall": 0.6779342928834193, "support": 7481}, "micro avg": {"f1-score": 0.9226611226611227, "precision": 0.9579795654050942, "recall": 0.889854297553803, "support": 7481}, "weighted avg": {"f1-score": 0.9170006117755608, "precision": 0.952528377605851, "recall": 0.889854297553803, "support": 7481}, "\u2205": {"f1-score": 0.9726352617768337, "precision": 0.9820842795861721, "recall": 0.9633663366336633, "support": 4040}, "\u23ce": {"f1-score": 0.7181008902077152, "precision": 0.8768115942028986, "recall": 0.6080402010050251, "support": 398}, "\u23ce\u23ce": {"f1-score": 0.4776119402985075, "precision": 0.8421052631578947, "recall": 0.3333333333333333, "support": 144}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8777219430485762, "precision": 0.888135593220339, "recall": 0.8675496688741722, "support": 302}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8996539792387543, "precision": 0.9187279151943463, "recall": 0.8813559322033898, "support": 295}, "\u2423": {"f1-score": 0.8632258064516128, "precision": 0.9176954732510288, "recall": 0.8148599269183922, "support": 1642}},
  "ppcr": 0.9288865124983291
}
```
</details>