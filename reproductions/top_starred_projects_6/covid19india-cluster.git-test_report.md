# Test report for javascript / file:///tmp/top-repos-quality-repos-j6ce3glj/covid19india-cluster.git HEAD dd60c7ee7054665404ee1a4ff7b7074b4776318e

### Classification report

PPCR: 0.846

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.874| 0.902| 0.817| 0.888| 0.845| 1055| 1165| 0.906 |
| `␣` | 0.725| 0.831| 0.718| 0.774| 0.722| 568| 657| 0.865 |
| `'` | 1.000| 0.951| 0.951| 0.975| 0.975| 102| 102| 1.000 |
| `⏎␣⁻␣⁻` | 0.915| 0.793| 0.774| 0.850| 0.839| 82| 84| 0.976 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 88| 0.636 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 108| 0.278 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 52| 0.288 |
| `micro avg` | 0.831| 0.831| 0.703| 0.831| 0.762| 1908| 2256| 0.846 |
| `weighted avg` | 0.792| 0.831| 0.703| 0.810| 0.722| 1908| 2256| 0.846 |
| `macro avg` | 0.502| 0.497| 0.466| 0.498| 0.483| 1908| 2256| 0.846 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|110 |952 |101 |0 |0 |0 |2 |0 |
|89 |93 |472 |0 |0 |0 |3 |0 |
|78 |8 |21 |0 |0 |0 |1 |0 |
|0 |4 |1 |0 |97 |0 |0 |0 |
|32 |7 |49 |0 |0 |0 |0 |0 |
|2 |15 |2 |0 |0 |0 |65 |0 |
|37 |10 |5 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9748743718592965, "precision": 1.0, "recall": 0.9509803921568627, "support": 102}, "macro avg": {"f1-score": 0.49814464659589985, "precision": 0.502103981537769, "recall": 0.4967169861036478, "support": 1908}, "micro avg": {"f1-score": 0.8312368972746331, "precision": 0.8312368972746331, "recall": 0.8312368972746331, "support": 1908}, "weighted avg": {"f1-score": 0.8102073143265949, "precision": 0.7920175858344719, "recall": 0.8312368972746331, "support": 1908}, "\u2205": {"f1-score": 0.8880597014925372, "precision": 0.8741965105601469, "recall": 0.9023696682464455, "support": 1055}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8496732026143791, "precision": 0.9154929577464789, "recall": 0.7926829268292683, "support": 82}, "\u2423": {"f1-score": 0.7744052502050862, "precision": 0.7250384024577573, "recall": 0.8309859154929577, "support": 568}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9748743718592965, "precision": 1.0, "recall": 0.9509803921568627, "support": 102}, "macro avg": {"f1-score": 0.48285958348562297, "precision": 0.502103981537769, "recall": 0.465767763589258, "support": 2256}, "micro avg": {"f1-score": 0.7617675312199808, "precision": 0.8312368972746331, "recall": 0.7030141843971631, "support": 2256}, "weighted avg": {"f1-score": 0.7216991645964276, "precision": 0.7418841195336976, "recall": 0.7030141843971631, "support": 2256}, "\u2205": {"f1-score": 0.84472049689441, "precision": 0.8741965105601469, "recall": 0.8171673819742489, "support": 1165}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 52}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 88}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8387096774193548, "precision": 0.9154929577464789, "recall": 0.7738095238095238, "support": 84}, "\u2423": {"f1-score": 0.7217125382262997, "precision": 0.7250384024577573, "recall": 0.7184170471841704, "support": 657}},
  "ppcr": 0.8457446808510638
}
```
</details>
