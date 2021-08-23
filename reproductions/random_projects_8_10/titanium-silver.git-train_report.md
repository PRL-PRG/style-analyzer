# Train report for javascript / file:///tmp/top-repos-quality-repos-4ilvzqwh/titanium-silver.git HEAD 138aaad297e822640162deb1284509c07b312877

### Classification report

PPCR: 0.698

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.962| 1.000| 0.888| 0.981| 0.924| 12927| 14550| 0.888 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 871| 1742| 0.500 |
| `␣` | 1.000| 0.533| 0.100| 0.695| 0.182| 503| 2680| 0.188 |
| `⏎` | 0.933| 0.577| 0.126| 0.713| 0.223| 194| 886| 0.219 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 82| 248| 0.331 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 64| 220| 0.291 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 228| 0.118 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 204| 0.069 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 228| 0.035 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 58| 0.000 |
| `macro avg` | 0.390| 0.311| 0.161| 0.339| 0.200| 14690| 21044| 0.698 |
| `micro avg` | 0.965| 0.965| 0.674| 0.965| 0.794| 14690| 21044| 0.698 |
| `weighted avg` | 0.953| 0.965| 0.674| 0.956| 0.727| 14690| 21044| 0.698 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⇥⁻| ⏎⏎| ⏎⇥⁺| '| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1623 |12927 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2177 |235 |268 |0 |0 |0 |0 |0 |0 |0 |0 |
|871 |0 |0 |871 |0 |0 |0 |0 |0 |0 |0 |
|692 |82 |0 |0 |112 |0 |0 |0 |0 |0 |0 |
|156 |64 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|190 |14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|166 |82 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|220 |0 |0 |0 |8 |0 |0 |0 |0 |0 |0 |
|201 |27 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|58 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/components/layout/teacher/SetTestBody.js | 49 |
| frontend/src/components/layout/student/StudentTestUI.js | 41 |
| frontend/src/components/layout/teacher/Profile.js | 36 |
| frontend/src/components/layout/student/Editor.js | 34 |
| frontend/src/components/layout/student/Profile.js | 26 |
| frontend/src/components/layout/student/QuestionKey.js | 26 |
| frontend/src/serviceWorker.js | 25 |
| frontend/src/components/layout/teacher/SetTest.js | 24 |
| frontend/src/components/layout/teacher/InactiveChallengeItem.js | 23 |
| frontend/src/components/layout/student/Challenge.js | 21 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 871}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3389455504404171, "precision": 0.38958082048991144, "recall": 0.31101227685433785, "support": 14690}, "micro avg": {"f1-score": 0.965146358066712, "precision": 0.965146358066712, "recall": 0.965146358066712, "support": 14690}, "weighted avg": {"f1-score": 0.9556773043828369, "precision": 0.9528236440706271, "recall": 0.965146358066712, "support": 14690}, "\u2205": {"f1-score": 0.9808786706123378, "precision": 0.9624748715657807, "recall": 1.0, "support": 12927}, "\u23ce": {"f1-score": 0.7133757961783439, "precision": 0.9333333333333333, "recall": 0.5773195876288659, "support": 194}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}, "\u2423": {"f1-score": 0.695201037613489, "precision": 1.0, "recall": 0.532803180914513, "support": 503}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 1742}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "macro avg": {"f1-score": 0.19951329964790535, "precision": 0.38958082048991144, "recall": 0.16148644434618697, "support": 21044}, "micro avg": {"f1-score": 0.7935299714557563, "precision": 0.965146358066712, "recall": 0.6737312298042197, "support": 21044}, "weighted avg": {"f1-score": 0.7265660283502025, "precision": 0.9148898837965901, "recall": 0.6737312298042197, "support": 21044}, "\u2205": {"f1-score": 0.9239841320896323, "precision": 0.9624748715657807, "recall": 0.8884536082474227, "support": 14550}, "\u23ce": {"f1-score": 0.22266401590457255, "precision": 0.9333333333333333, "recall": 0.12641083521444696, "support": 886}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 228}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 248}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 228}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 204}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 220}, "\u2423": {"f1-score": 0.18181818181818182, "precision": 1.0, "recall": 0.1, "support": 2680}},
  "ppcr": 0.6980612050940885
}
```
</details>
