# Train report for javascript / file:///tmp/top-repos-quality-repos-xao_ipwo/localtunnel.git HEAD 485b81619de7ea1196ea91a1d2c5f1993151be5f

### Classification report

PPCR: 0.502

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 1.000| 0.783| 0.981| 0.864| 1251| 1597| 0.783 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 158| 316| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 704| 0.068 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 166| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 120| 0.000 |
| `weighted avg` | 0.935| 0.967| 0.485| 0.951| 0.548| 1457| 2903| 0.502 |
| `macro avg` | 0.280| 0.286| 0.183| 0.283| 0.219| 1457| 2903| 0.502 |
| `micro avg` | 0.967| 0.967| 0.485| 0.967| 0.646| 1457| 2903| 0.502 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|346 |1251 |0 |0 |0 |0 |
|656 |48 |0 |0 |0 |0 |
|158 |0 |0 |158 |0 |0 |
|166 |0 |0 |0 |0 |0 |
|120 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/Tunnel.js | 22 |
| lib/HeaderHostTransformer.js | 8 |
| lib/TunnelCluster.js | 8 |
| client.js | 5 |
| test/index.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 158}, "macro avg": {"f1-score": 0.2830252100840336, "precision": 0.28043549983503796, "recall": 0.2857142857142857, "support": 1457}, "micro avg": {"f1-score": 0.9670555936856554, "precision": 0.9670555936856554, "recall": 0.9670555936856554, "support": 1457}, "weighted avg": {"f1-score": 0.9508934555290887, "precision": 0.9353285326392774, "recall": 0.9670555936856554, "support": 1457}, "\u2205": {"f1-score": 0.9811764705882352, "precision": 0.9630484988452656, "recall": 1.0, "support": 1251}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 316}, "macro avg": {"f1-score": 0.21865956327282293, "precision": 0.28043549983503796, "recall": 0.18333482422399142, "support": 2903}, "micro avg": {"f1-score": 0.6463302752293577, "precision": 0.9670555936856554, "recall": 0.48535997244230106, "support": 2903}, "weighted avg": {"f1-score": 0.5478454212286902, "precision": 0.6386456950244193, "recall": 0.48535997244230106, "support": 2903}, "\u2205": {"f1-score": 0.8639502762430938, "precision": 0.9630484988452656, "recall": 0.7833437695679399, "support": 1597}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 120}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 704}},
  "ppcr": 0.5018945918015846
}
```
</details>
