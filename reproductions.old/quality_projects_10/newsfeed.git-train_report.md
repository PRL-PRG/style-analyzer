# Train report for javascript / file:///tmp/top-repos-quality-repos-ws6oywd0/newsfeed.git HEAD a69a02052c120132eb50c8ecb93ca15c6b2fc081

### Classification report

PPCR: 0.489

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 0.995| 0.587| 0.979| 0.729| 2487| 4216| 0.590 |
| `␣` | 0.984| 0.920| 0.318| 0.951| 0.481| 527| 1523| 0.346 |
| `"` | 1.000| 0.962| 0.485| 0.981| 0.653| 212| 421| 0.504 |
| `'` | 0.957| 1.000| 0.506| 0.978| 0.662| 180| 356| 0.506 |
| `⏎␣⁻␣⁻` | 0.893| 0.775| 0.626| 0.830| 0.736| 151| 187| 0.807 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 291| 0.065 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 221| 0.050 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 123| 0.000 |
| `macro avg` | 0.600| 0.582| 0.315| 0.590| 0.408| 3587| 7338| 0.489 |
| `micro avg` | 0.965| 0.965| 0.472| 0.965| 0.634| 3587| 7338| 0.489 |
| `weighted avg` | 0.957| 0.965| 0.472| 0.960| 0.607| 3587| 7338| 0.489 |

### Confusion matrix

|refusal|  ∅| ␣| '| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1729 |2475 |0 |0 |0 |0 |0 |12 |0 |
|996 |40 |485 |0 |0 |0 |0 |2 |0 |
|176 |0 |0 |180 |0 |0 |0 |0 |0 |
|209 |0 |0 |8 |204 |0 |0 |0 |0 |
|272 |14 |5 |0 |0 |0 |0 |0 |0 |
|210 |11 |0 |0 |0 |0 |0 |0 |0 |
|36 |31 |3 |0 |0 |0 |0 |117 |0 |
|123 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/serviceWorker.js | 13 |
| frontend/src/components/article/ArticleList.js | 13 |
| frontend/src/components/user/Profile.js | 12 |
| frontend/src/components/App.js | 11 |
| frontend/src/components/article/WhatsNews.js | 11 |
| frontend/src/components/article/Article.js | 9 |
| frontend/src/components/article/TopArticles.js | 9 |
| frontend/src/components/common/Header.js | 8 |
| frontend/src/components/activity/CommentList.js | 6 |
| frontend/src/components/activity/ProfileComments.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9807692307692307, "precision": 1.0, "recall": 0.9622641509433962, "support": 212}, "\u0027": {"f1-score": 0.9782608695652174, "precision": 0.9574468085106383, "recall": 1.0, "support": 180}, "macro avg": {"f1-score": 0.5898056766708255, "precision": 0.599626230297857, "recall": 0.581572137859017, "support": 3587}, "micro avg": {"f1-score": 0.9648731530526903, "precision": 0.9648731530526903, "recall": 0.9648731530526903, "support": 3587}, "weighted avg": {"f1-score": 0.9602373486492705, "precision": 0.9567292499491563, "recall": 0.9648731530526903, "support": 3587}, "\u2205": {"f1-score": 0.9786476868327403, "precision": 0.9626604434072346, "recall": 0.9951749095295537, "support": 2487}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8297872340425533, "precision": 0.8931297709923665, "recall": 0.7748344370860927, "support": 151}, "\u2423": {"f1-score": 0.9509803921568627, "precision": 0.9837728194726166, "recall": 0.920303605313093, "support": 527}},
  "cl_report_full": {"\"": {"f1-score": 0.6527999999999999, "precision": 1.0, "recall": 0.4845605700712589, "support": 421}, "\u0027": {"f1-score": 0.6617647058823529, "precision": 0.9574468085106383, "recall": 0.5056179775280899, "support": 356}, "macro avg": {"f1-score": 0.40761250630803886, "precision": 0.599626230297857, "recall": 0.31516834493122736, "support": 7338}, "micro avg": {"f1-score": 0.6335926773455377, "precision": 0.9648731530526903, "recall": 0.47165440174434453, "support": 7338}, "weighted avg": {"f1-score": 0.6072079465376752, "precision": 0.8838551055419809, "recall": 0.47165440174434453, "support": 7338}, "\u2205": {"f1-score": 0.7293354943273906, "precision": 0.9626604434072346, "recall": 0.5870493358633776, "support": 4216}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 291}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 123}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 221}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7358490566037736, "precision": 0.8931297709923665, "recall": 0.6256684491978609, "support": 187}, "\u2423": {"f1-score": 0.48115079365079366, "precision": 0.9837728194726166, "recall": 0.3184504267892318, "support": 1523}},
  "ppcr": 0.4888252929953666
}
```
</details>
