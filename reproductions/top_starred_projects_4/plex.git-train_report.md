# Train report for javascript / file:///tmp/top-repos-quality-repos-8gryn42s/plex.git HEAD 7b6c58a8b522dcd35e724992f1b8ef3af4563ab4

### Classification report

PPCR: 0.932

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.995| 1.000| 0.998| 6312| 6342| 0.995 |
| `⏎` | 0.990| 0.988| 0.977| 0.989| 0.984| 3156| 3191| 0.989 |
| `∅` | 0.839| 1.000| 0.787| 0.912| 0.812| 1289| 1637| 0.787 |
| `␣` | 1.000| 0.609| 0.468| 0.757| 0.637| 570| 742| 0.768 |
| `⏎␣⁻␣⁻` | 0.855| 0.910| 0.888| 0.882| 0.871| 278| 285| 0.975 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 36| 297| 0.121 |
| `macro avg` | 0.781| 0.751| 0.686| 0.757| 0.717| 11641| 12494| 0.932 |
| `weighted avg` | 0.973| 0.972| 0.906| 0.969| 0.922| 11641| 12494| 0.932 |
| `micro avg` | 0.972| 0.972| 0.906| 0.972| 0.938| 11641| 12494| 0.932 |

### Confusion matrix

|refusal|  '| ⏎| ∅| ␣| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|30 |6312 |0 |0 |0 |0 |0 |
|35 |0 |3118 |25 |0 |0 |13 |
|348 |0 |0 |1289 |0 |0 |0 |
|172 |0 |30 |163 |347 |0 |30 |
|261 |0 |0 |36 |0 |0 |0 |
|7 |0 |1 |24 |0 |0 |253 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| scripts/generate-scss.js | 107 |
| scripts/build-zip.js | 67 |
| scripts/tools/index.js | 64 |
| scripts/compile-css.js | 34 |
| scripts/data/families.js | 23 |
| scripts/data/unicodes/index.js | 13 |
| scripts/data/unicodes/korean.js | 9 |
| scripts/data/weights.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6312}, "macro avg": {"f1-score": 0.7566080937651392, "precision": 0.7805886748437239, "recall": 0.7511338857671118, "support": 11641}, "micro avg": {"f1-score": 0.9723391461214672, "precision": 0.9723391461214672, "recall": 0.9723391461214672, "support": 11641}, "weighted avg": {"f1-score": 0.9694866988945453, "precision": 0.9729028063683217, "recall": 0.9723391461214672, "support": 11641}, "\u2205": {"f1-score": 0.9122434536447276, "precision": 0.8386467143786597, "recall": 1.0, "support": 1289}, "\u23ce": {"f1-score": 0.9890563045202221, "precision": 0.9901556049539536, "recall": 0.9879594423320659, "support": 3156}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8815331010452961, "precision": 0.8547297297297297, "recall": 0.9100719424460432, "support": 278}, "\u2423": {"f1-score": 0.7568157033805889, "precision": 1.0, "recall": 0.6087719298245614, "support": 570}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9976292081555239, "precision": 1.0, "recall": 0.9952696310312205, "support": 6342}, "macro avg": {"f1-score": 0.7169406459344762, "precision": 0.7805886748437239, "recall": 0.6858638465951827, "support": 12494}, "micro avg": {"f1-score": 0.9379738968303295, "precision": 0.9723391461214672, "recall": 0.9059548583319994, "support": 12494}, "weighted avg": {"f1-score": 0.9217467852387736, "precision": 0.9492595789834246, "recall": 0.9059548583319994, "support": 12494}, "\u2205": {"f1-score": 0.8122243226212981, "precision": 0.8386467143786597, "recall": 0.7874160048869884, "support": 1637}, "\u23ce": {"f1-score": 0.983596214511041, "precision": 0.9901556049539536, "recall": 0.9771231588843623, "support": 3191}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 297}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8709122203098106, "precision": 0.8547297297297297, "recall": 0.887719298245614, "support": 285}, "\u2423": {"f1-score": 0.6372819100091828, "precision": 1.0, "recall": 0.46765498652291104, "support": 742}},
  "ppcr": 0.9317272290699535
}
```
</details>
