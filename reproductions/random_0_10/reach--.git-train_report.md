# Train report for javascript / file:///tmp/top-repos-quality-repos-ihfhmd3h/reach--.git HEAD 9fdd933ae18aabcb7d8b61e398829b5f63e657a0

### Classification report

PPCR: 0.757

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 1.000| 0.885| 0.980| 0.921| 9173| 10364| 0.885 |
| `␣` | 0.994| 0.851| 0.481| 0.917| 0.649| 1479| 2614| 0.566 |
| `"` | 0.979| 1.000| 0.891| 0.989| 0.933| 1107| 1242| 0.891 |
| `⏎` | 0.958| 0.881| 0.307| 0.918| 0.465| 235| 674| 0.349 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 59| 191| 0.309 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 51| 162| 0.315 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 118| 0.254 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 703| 0.034 |
| `macro avg` | 0.487| 0.466| 0.321| 0.476| 0.371| 12158| 16068| 0.757 |
| `weighted avg` | 0.954| 0.966| 0.731| 0.959| 0.791| 12158| 16068| 0.757 |
| `micro avg` | 0.966| 0.966| 0.731| 0.966| 0.832| 12158| 16068| 0.757 |

### Confusion matrix

|refusal|  ∅| ␣| "| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1191 |9173 |0 |0 |0 |0 |0 |0 |0 |
|1135 |216 |1258 |0 |0 |5 |0 |0 |0 |
|135 |0 |0 |1107 |0 |0 |0 |0 |0 |
|679 |0 |0 |24 |0 |0 |0 |0 |0 |
|439 |21 |7 |0 |0 |207 |0 |0 |0 |
|132 |56 |0 |0 |0 |3 |0 |0 |0 |
|111 |50 |0 |0 |0 |1 |0 |0 |0 |
|88 |30 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/components/Main.js | 158 |
| frontend/src/components/EditProfile.js | 75 |
| frontend/src/serviceWorker.js | 60 |
| frontend/src/components/Register.js | 43 |
| frontend/src/components/Landing.js | 34 |
| frontend/src/components/PasswordReset.js | 25 |
| backend/static/js/rango-ajax.js | 6 |
| frontend/src/components/MatchAnimation.js | 5 |
| frontend/src/App.test.js | 4 |
| frontend/src/components/ReachPercentagesTable.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9892761394101875, "precision": 0.9787798408488063, "recall": 1.0, "support": 1107}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "macro avg": {"f1-score": 0.47552744637251126, "precision": 0.4865632024581983, "recall": 0.4664282220591832, "support": 12158}, "micro avg": {"f1-score": 0.9660305971376871, "precision": 0.9660305971376871, "recall": 0.9660305971376871, "support": 12158}, "weighted avg": {"f1-score": 0.958806949126815, "precision": 0.9536196754107389, "recall": 0.9660305971376871, "support": 12158}, "\u2205": {"f1-score": 0.9800737218868529, "precision": 0.9609260423213911, "recall": 1.0, "support": 9173}, "\u23ce": {"f1-score": 0.917960088691796, "precision": 0.9583333333333334, "recall": 0.8808510638297873, "support": 235}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u2423": {"f1-score": 0.9169096209912536, "precision": 0.9944664031620554, "recall": 0.8505747126436781, "support": 1479}},
  "cl_report_full": {"\"": {"f1-score": 0.9329962073324904, "precision": 0.9787798408488063, "recall": 0.8913043478260869, "support": 1242}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 703}, "macro avg": {"f1-score": 0.37102900431266495, "precision": 0.4865632024581983, "recall": 0.32059547137938926, "support": 16068}, "micro avg": {"f1-score": 0.832211436264437, "precision": 0.9660305971376871, "recall": 0.7309559372666169, "support": 16068}, "weighted avg": {"f1-score": 0.7914906162836499, "precision": 0.897444231359559, "recall": 0.7309559372666169, "support": 16068}, "\u2205": {"f1-score": 0.921446509291813, "precision": 0.9609260423213911, "recall": 0.8850829795445774, "support": 10364}, "\u23ce": {"f1-score": 0.46516853932584273, "precision": 0.9583333333333334, "recall": 0.30712166172106825, "support": 674}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 191}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 162}, "\u2423": {"f1-score": 0.648620778551173, "precision": 0.9944664031620554, "recall": 0.4812547819433818, "support": 2614}},
  "ppcr": 0.7566591984067712
}
```
</details>
