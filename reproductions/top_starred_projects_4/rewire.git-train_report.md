# Train report for javascript / file:///tmp/top-repos-quality-repos-l6x1g_xk/rewire.git HEAD 90e781f0c6d63ca1d4a28fd3f99aabbfe9a59018

### Classification report

PPCR: 0.752

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.984| 1.000| 0.965| 0.992| 0.974| 4847| 5020| 0.966 |
| `␣` | 0.999| 0.960| 0.479| 0.979| 0.647| 885| 1775| 0.499 |
| `"` | 1.000| 1.000| 0.467| 1.000| 0.636| 370| 793| 0.467 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.976| 0.808| 0.808| 0.884| 0.884| 297| 297| 1.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.977| 0.993| 0.888| 0.985| 0.930| 296| 331| 0.894 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 478| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 204| 0.000 |
| `macro avg` | 0.705| 0.680| 0.515| 0.691| 0.582| 6695| 8898| 0.752 |
| `weighted avg` | 0.986| 0.986| 0.742| 0.985| 0.800| 6695| 8898| 0.752 |
| `micro avg` | 0.986| 0.986| 0.742| 0.986| 0.847| 6695| 8898| 0.752 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|173 |4846 |1 |0 |0 |0 |0 |0 |
|890 |22 |850 |0 |0 |7 |6 |0 |
|423 |0 |0 |370 |0 |0 |0 |0 |
|478 |0 |0 |0 |0 |0 |0 |0 |
|35 |2 |0 |0 |0 |294 |0 |0 |
|0 |57 |0 |0 |0 |0 |240 |0 |
|204 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/__set__.test.js | 21 |
| test/__with__.test.js | 19 |
| testLib/sharedTestCases.js | 19 |
| test/__get__.test.js | 15 |
| lib/getImportGlobalsSrc.js | 6 |
| lib/__set__.js | 5 |
| lib/moduleEnv.js | 4 |
| lib/getDefinePropertySrc.js | 3 |
| test/getImportGlobalsSrc.test.js | 2 |
| testLib/moduleB.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 370}, "macro avg": {"f1-score": 0.6913965130577766, "precision": 0.7049626899509815, "recall": 0.6802242450773955, "support": 6695}, "micro avg": {"f1-score": 0.9858103061986557, "precision": 0.9858103061986557, "recall": 0.9858103061986557, "support": 6695}, "weighted avg": {"f1-score": 0.9853714973184577, "precision": 0.9858323563230345, "recall": 0.9858103061986557, "support": 6695}, "\u2205": {"f1-score": 0.991610394925312, "precision": 0.9835599756444083, "recall": 0.9997936868165875, "support": 4847}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9849246231155779, "precision": 0.9767441860465116, "recall": 0.9932432432432432, "support": 296}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8839779005524863, "precision": 0.975609756097561, "recall": 0.8080808080808081, "support": 297}, "\u2423": {"f1-score": 0.9792626728110598, "precision": 0.9988249118683902, "recall": 0.96045197740113, "support": 885}},
  "cl_report_full": {"\"": {"f1-score": 0.6362854686156492, "precision": 1.0, "recall": 0.4665825977301387, "support": 793}, "macro avg": {"f1-score": 0.5817685250632334, "precision": 0.7049626899509815, "recall": 0.515298973332072, "support": 8898}, "micro avg": {"f1-score": 0.8465337010196883, "precision": 0.9858103061986557, "recall": 0.7417397167902899, "support": 8898}, "weighted avg": {"f1-score": 0.7996702072118499, "precision": 0.912164949364317, "recall": 0.7417397167902899, "support": 8898}, "\u2205": {"f1-score": 0.9743641298884085, "precision": 0.9835599756444083, "recall": 0.9653386454183267, "support": 5020}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 478}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 204}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9303797468354431, "precision": 0.9767441860465116, "recall": 0.8882175226586103, "support": 331}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8839779005524863, "precision": 0.975609756097561, "recall": 0.8080808080808081, "support": 297}, "\u2423": {"f1-score": 0.6473724295506473, "precision": 0.9988249118683902, "recall": 0.4788732394366197, "support": 1775}},
  "ppcr": 0.7524162733198472
}
```
</details>
