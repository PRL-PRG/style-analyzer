# Test report for javascript / file:///tmp/top-repos-quality-repos-acyjc8oy/create-snowpack-app.git HEAD 45660ff36399f50de2d60d4af5743c3c0b65bdad

### Classification report

PPCR: 0.917

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.871| 0.976| 0.959| 0.920| 0.913| 800| 814| 0.983 |
| `␣` | 0.838| 0.881| 0.836| 0.859| 0.837| 411| 433| 0.949 |
| `⏎` | 0.696| 0.586| 0.582| 0.637| 0.634| 133| 134| 0.993 |
| `"` | 0.953| 0.762| 0.424| 0.847| 0.587| 80| 144| 0.556 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 69| 83| 0.831 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 82| 0.683 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 19| 1.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 6| 0.833 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.769| 0.815| 0.748| 0.789| 0.743| 1573| 1715| 0.917 |
| `micro avg` | 0.815| 0.815| 0.748| 0.815| 0.780| 1573| 1715| 0.917 |
| `macro avg` | 0.336| 0.321| 0.280| 0.326| 0.297| 1573| 1715| 0.917 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|14 |781 |19 |0 |0 |0 |0 |0 |0 |0 |0 |
|22 |40 |362 |8 |1 |0 |0 |0 |0 |0 |0 |
|1 |24 |31 |78 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|64 |2 |12 |5 |0 |0 |61 |0 |0 |0 |0 |
|1 |0 |0 |2 |0 |0 |3 |0 |0 |0 |0 |
|26 |17 |7 |0 |32 |0 |0 |0 |0 |0 |0 |
|14 |25 |0 |9 |0 |35 |0 |0 |0 |0 |0 |
|0 |8 |1 |10 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8472222222222222, "precision": 0.953125, "recall": 0.7625, "support": 80}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "macro avg": {"f1-score": 0.3263242250417329, "precision": 0.33581965789846224, "recall": 0.32059947542213196, "support": 1573}, "micro avg": {"f1-score": 0.8150031786395423, "precision": 0.8150031786395423, "recall": 0.8150031786395423, "support": 1573}, "weighted avg": {"f1-score": 0.7894494455089808, "precision": 0.7691174910694522, "recall": 0.8150031786395423, "support": 1573}, "\u2205": {"f1-score": 0.920447849145551, "precision": 0.8706800445930881, "recall": 0.97625, "support": 800}, "\u23ce": {"f1-score": 0.636734693877551, "precision": 0.6964285714285714, "recall": 0.5864661654135338, "support": 133}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 69}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8588374851720048, "precision": 0.8379629629629629, "recall": 0.8807785888077859, "support": 411}},
  "cl_report_full": {"\"": {"f1-score": 0.5865384615384616, "precision": 0.953125, "recall": 0.4236111111111111, "support": 144}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "macro avg": {"f1-score": 0.29705954457994155, "precision": 0.33581965789846224, "recall": 0.28011878364352427, "support": 1715}, "micro avg": {"f1-score": 0.7798053527980534, "precision": 0.8150031786395423, "recall": 0.7475218658892129, "support": 1715}, "weighted avg": {"f1-score": 0.7434225153160184, "precision": 0.7592670249756066, "recall": 0.7475218658892129, "support": 1715}, "\u2205": {"f1-score": 0.91291642314436, "precision": 0.8706800445930881, "recall": 0.9594594594594594, "support": 814}, "\u23ce": {"f1-score": 0.6341463414634148, "precision": 0.6964285714285714, "recall": 0.582089552238806, "support": 134}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8369942196531792, "precision": 0.8379629629629629, "recall": 0.836027713625866, "support": 433}},
  "ppcr": 0.917201166180758
}
```
</details>
