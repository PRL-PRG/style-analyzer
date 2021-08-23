# Train report for javascript / file:///tmp/top-repos-quality-repos-72xspyhr/tu-blog.git HEAD 4f91de47b0969a24be623dc796cab62beb428d3a

### Classification report

PPCR: 0.741

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.991| 0.871| 0.983| 0.920| 15815| 17996| 0.879 |
| `␣` | 0.969| 0.969| 0.819| 0.969| 0.888| 6883| 8141| 0.845 |
| `⏎` | 0.953| 0.959| 0.550| 0.956| 0.697| 1382| 2411| 0.573 |
| `'` | 0.994| 1.000| 0.426| 0.997| 0.596| 952| 2235| 0.426 |
| `"` | 1.000| 0.974| 0.315| 0.987| 0.479| 228| 705| 0.323 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 120| 1167| 0.103 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 118| 1226| 0.096 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 576| 0.056 |
| `macro avg` | 0.611| 0.612| 0.373| 0.611| 0.448| 25530| 34457| 0.741 |
| `micro avg` | 0.973| 0.973| 0.721| 0.973| 0.828| 25530| 34457| 0.741 |
| `weighted avg` | 0.963| 0.973| 0.721| 0.968| 0.788| 25530| 34457| 0.741 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2181 |15675 |127 |13 |0 |0 |0 |0 |0 |
|1258 |192 |6670 |21 |0 |0 |0 |0 |0 |
|1029 |27 |29 |1326 |0 |0 |0 |0 |0 |
|1283 |0 |0 |0 |952 |0 |0 |0 |0 |
|1108 |86 |32 |0 |0 |0 |0 |0 |0 |
|1047 |98 |22 |0 |0 |0 |0 |0 |0 |
|477 |0 |0 |0 |6 |0 |0 |222 |0 |
|544 |0 |0 |32 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| admin/src/pages/article/selectTags.js | 65 |
| admin/src/pages/article/selectCategories.js | 63 |
| server/app/utils/userAgent.js | 46 |
| admin/src/App.js | 40 |
| admin/src/pages/laboratory/index.js | 32 |
| admin/src/pages/tag/index.js | 32 |
| admin/src/pages/category/index.js | 31 |
| admin/src/pages/read/index.js | 30 |
| admin/src/common/topSnackbar/index.js | 24 |
| admin/src/pages/login/index.js | 18 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9866666666666666, "precision": 1.0, "recall": 0.9736842105263158, "support": 228}, "\u0027": {"f1-score": 0.9968586387434555, "precision": 0.9937369519832986, "recall": 1.0, "support": 952}, "macro avg": {"f1-score": 0.6114731543789856, "precision": 0.6113418245544648, "recall": 0.6116706328215885, "support": 25530}, "micro avg": {"f1-score": 0.973168820994908, "precision": 0.973168820994908, "recall": 0.973168820994908, "support": 25530}, "weighted avg": {"f1-score": 0.9679742302447591, "precision": 0.9628676976927313, "recall": 0.973168820994908, "support": 25530}, "\u2205": {"f1-score": 0.9829743203837833, "precision": 0.9749346933698221, "recall": 0.9911476446411634, "support": 15815}, "\u23ce": {"f1-score": 0.9560201874549387, "precision": 0.9525862068965517, "recall": 0.959479015918958, "support": 1382}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 120}, "\u2423": {"f1-score": 0.9692654217830414, "precision": 0.9694767441860465, "recall": 0.9690541914862705, "support": 6883}},
  "cl_report_full": {"\"": {"f1-score": 0.4789644012944984, "precision": 1.0, "recall": 0.3148936170212766, "support": 705}, "\u0027": {"f1-score": 0.5963044159098027, "precision": 0.9937369519832986, "recall": 0.42595078299776284, "support": 2235}, "macro avg": {"f1-score": 0.4475949218027246, "precision": 0.6113418245544648, "recall": 0.37264502796484456, "support": 34457}, "micro avg": {"f1-score": 0.8283461416640272, "precision": 0.973168820994908, "recall": 0.7210436195838291, "support": 34457}, "weighted avg": {"f1-score": 0.7876188272531655, "precision": 0.8898082348669989, "recall": 0.7210436195838291, "support": 34457}, "\u2205": {"f1-score": 0.9200563479485825, "precision": 0.9749346933698221, "recall": 0.8710268948655256, "support": 17996}, "\u23ce": {"f1-score": 0.6973442019458322, "precision": 0.9525862068965517, "recall": 0.5499792617171299, "support": 2411}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 576}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1226}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1167}, "\u2423": {"f1-score": 0.8880900073230811, "precision": 0.9694767441860465, "recall": 0.8193096671170618, "support": 8141}},
  "ppcr": 0.7409234698319644
}
```
</details>
