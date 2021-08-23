# Train report for javascript / file:///tmp/top-repos-quality-repos-zg3sclt8/capstoneproj_courseme.git HEAD e6dceb2ac6887763a42295bdf6c3d7f0702769e0

### Classification report

PPCR: 0.288

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.999| 0.367| 0.992| 0.535| 1759| 4781| 0.368 |
| `'` | 1.000| 1.000| 0.502| 1.000| 0.668| 251| 500| 0.502 |
| `␣` | 0.992| 0.996| 0.164| 0.994| 0.281| 250| 1519| 0.165 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 509| 0.026 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 121| 0.074 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 124| 0.008 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 117| 0.009 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 125| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 121| 0.000 |
| `weighted avg` | 0.978| 0.988| 0.285| 0.983| 0.420| 2284| 7917| 0.288 |
| `micro avg` | 0.988| 0.988| 0.285| 0.988| 0.443| 2284| 7917| 0.288 |
| `macro avg` | 0.331| 0.333| 0.115| 0.332| 0.165| 2284| 7917| 0.288 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3022 |1757 |2 |0 |0 |0 |0 |0 |0 |0 |
|1269 |1 |249 |0 |0 |0 |0 |0 |0 |0 |
|249 |0 |0 |251 |0 |0 |0 |0 |0 |0 |
|496 |13 |0 |0 |0 |0 |0 |0 |0 |0 |
|125 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|121 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|112 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|123 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|116 |1 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| courseme-app/bin/rename.js | 9 |
| courseme-app/Screens/GroupScreen.js | 5 |
| courseme-app/Screens/AgendaScreen.js | 4 |
| courseme-app/Screens/LoginScreen.js | 3 |
| courseme-app/Screens/AssignmentEditScreen.js | 2 |
| Sam/ClassView.js | 1 |
| Dylan/components/Login.js | 1 |
| courseme-app/DatabaseDemo/ClassView.js | 1 |
| courseme-app/App.test.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 251}, "macro avg": {"f1-score": 0.33182077903874074, "precision": 0.33088918797934586, "recall": 0.33276255448171305, "support": 2284}, "micro avg": {"f1-score": 0.9881786339754814, "precision": 0.9881786339754816, "recall": 0.9881786339754816, "support": 2284}, "weighted avg": {"f1-score": 0.9829643962810933, "precision": 0.9778155163237717, "recall": 0.9881786339754816, "support": 2284}, "\u2205": {"f1-score": 0.9923750353007624, "precision": 0.9859708193041526, "recall": 0.9988629903354178, "support": 1759}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.9940119760479043, "precision": 0.9920318725099602, "recall": 0.996, "support": 250}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6684420772303595, "precision": 1.0, "recall": 0.502, "support": 500}, "macro avg": {"f1-score": 0.1650248757498044, "precision": 0.33088918797934586, "recall": 0.11482444151640096, "support": 7917}, "micro avg": {"f1-score": 0.4425056367022841, "precision": 0.9881786339754816, "recall": 0.2850827333585954, "support": 7917}, "weighted avg": {"f1-score": 0.4195366673194022, "precision": 0.8489103071157993, "recall": 0.2850827333585954, "support": 7917}, "\u2205": {"f1-score": 0.5354258723144903, "precision": 0.9859708193041526, "recall": 0.36749633967789164, "support": 4781}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 509}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 125}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 124}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 117}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u2423": {"f1-score": 0.28135593220338984, "precision": 0.9920318725099602, "recall": 0.16392363396971693, "support": 1519}},
  "ppcr": 0.288493116079323
}
```
</details>
