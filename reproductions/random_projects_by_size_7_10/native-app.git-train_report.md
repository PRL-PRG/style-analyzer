# Train report for javascript / file:///tmp/top-repos-quality-repos-okbqn8k2/native-app.git HEAD 5f7d469f44aa7b236a80d66641a505d5f95a0b6c

### Classification report

PPCR: 0.603

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.998| 0.872| 0.989| 0.923| 11726| 13418| 0.874 |
| `␣` | 0.936| 0.807| 0.131| 0.867| 0.230| 706| 4353| 0.162 |
| `⏎` | 0.989| 0.964| 0.451| 0.977| 0.620| 584| 1248| 0.468 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.993| 0.697| 0.192| 0.819| 0.322| 198| 717| 0.276 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.956| 0.878| 0.250| 0.915| 0.396| 196| 689| 0.284 |
| `'` | 1.000| 1.000| 0.114| 1.000| 0.205| 132| 1154| 0.114 |
| `"` | 1.000| 1.000| 0.144| 1.000| 0.252| 109| 757| 0.144 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 337| 0.045 |
| `weighted avg` | 0.978| 0.980| 0.590| 0.978| 0.666| 13666| 22673| 0.603 |
| `micro avg` | 0.980| 0.980| 0.590| 0.980| 0.737| 13666| 22673| 0.603 |
| `macro avg` | 0.857| 0.793| 0.269| 0.821| 0.369| 13666| 22673| 0.603 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1692 |11702 |24 |0 |0 |0 |0 |0 |0 |
|3647 |134 |570 |2 |0 |0 |0 |0 |0 |
|664 |8 |4 |563 |0 |0 |1 |8 |0 |
|1022 |0 |0 |0 |132 |0 |0 |0 |0 |
|648 |0 |0 |0 |0 |109 |0 |0 |0 |
|519 |50 |9 |1 |0 |0 |138 |0 |0 |
|493 |24 |0 |0 |0 |0 |0 |172 |0 |
|322 |10 |2 |3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| screens/FriendScreen.js | 56 |
| screens/SearchScreen.js | 35 |
| screens/ChangePreferenceScreen.js | 23 |
| index.android.js | 23 |
| screens/LoginScreen.js | 20 |
| screens/MessageInScreen.js | 18 |
| screens/PreferenceScreen.js | 13 |
| screens/MessageOutScreen.js | 12 |
| screens/RegisterScreen.js | 11 |
| screens/AdminOpenCasesScreen.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 109}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 132}, "macro avg": {"f1-score": 0.820852332186599, "precision": 0.8568537588990782, "recall": 0.7929850648259631, "support": 13666}, "micro avg": {"f1-score": 0.9795111956680814, "precision": 0.9795111956680814, "recall": 0.9795111956680814, "support": 13666}, "weighted avg": {"f1-score": 0.9781148000613754, "precision": 0.9781444263633235, "recall": 0.9795111956680814, "support": 13666}, "\u2205": {"f1-score": 0.9894309630506469, "precision": 0.9810529845741114, "recall": 0.9979532662459492, "support": 11726}, "\u23ce": {"f1-score": 0.9765828274067649, "precision": 0.9894551845342706, "recall": 0.964041095890411, "support": 584}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8189910979228487, "precision": 0.9928057553956835, "recall": 0.696969696969697, "support": 198}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9148936170212767, "precision": 0.9555555555555556, "recall": 0.8775510204081632, "support": 196}, "\u2423": {"f1-score": 0.8669201520912547, "precision": 0.9359605911330049, "recall": 0.8073654390934845, "support": 706}},
  "cl_report_full": {"\"": {"f1-score": 0.2517321016166282, "precision": 1.0, "recall": 0.14398943196829592, "support": 757}, "\u0027": {"f1-score": 0.2052877138413686, "precision": 1.0, "recall": 0.11438474870017332, "support": 1154}, "macro avg": {"f1-score": 0.36851704015785525, "precision": 0.8568537588990782, "recall": 0.2693322518442981, "support": 22673}, "micro avg": {"f1-score": 0.7367291339882771, "precision": 0.9795111956680814, "recall": 0.590393860538967, "support": 22673}, "weighted avg": {"f1-score": 0.6657603010543276, "precision": 0.959470073431511, "recall": 0.590393860538967, "support": 22673}, "\u2205": {"f1-score": 0.9233804150556301, "precision": 0.9810529845741114, "recall": 0.872112088239678, "support": 13418}, "\u23ce": {"f1-score": 0.6197028068244359, "precision": 0.9894551845342706, "recall": 0.4511217948717949, "support": 1248}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 337}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.3224299065420561, "precision": 0.9928057553956835, "recall": 0.19246861924686193, "support": 717}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.39585730724971235, "precision": 0.9555555555555556, "recall": 0.24963715529753266, "support": 689}, "\u2423": {"f1-score": 0.22974607013301085, "precision": 0.9359605911330049, "recall": 0.13094417643004824, "support": 4353}},
  "ppcr": 0.6027433511224805
}
```
</details>
