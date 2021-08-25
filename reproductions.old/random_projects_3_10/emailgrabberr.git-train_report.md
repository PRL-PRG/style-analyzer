# Train report for javascript / file:///tmp/top-repos-quality-repos-sj6wdxje/emailgrabberr.git HEAD 14951eccfbfed778e94226fa1c8809cd65b30967

### Classification report

PPCR: 0.650

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.992| 0.934| 0.978| 0.949| 7379| 7839| 0.941 |
| `␣` | 0.972| 0.905| 0.445| 0.938| 0.611| 2143| 4356| 0.492 |
| `'` | 1.000| 1.000| 0.295| 1.000| 0.456| 808| 2736| 0.295 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.954| 0.932| 0.800| 0.943| 0.870| 398| 464| 0.858 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.962| 0.852| 0.681| 0.904| 0.797| 358| 448| 0.799 |
| `⏎` | 0.946| 0.890| 0.194| 0.917| 0.322| 237| 1087| 0.218 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 122| 0.041 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 371| 0.000 |
| `weighted avg` | 0.967| 0.967| 0.629| 0.967| 0.715| 11328| 17423| 0.650 |
| `micro avg` | 0.967| 0.967| 0.629| 0.967| 0.762| 11328| 17423| 0.650 |
| `macro avg` | 0.725| 0.697| 0.419| 0.710| 0.501| 11328| 17423| 0.650 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|460 |7323 |51 |0 |0 |5 |0 |0 |0 |
|2213 |180 |1940 |0 |0 |11 |12 |0 |0 |
|1928 |0 |0 |808 |0 |0 |0 |0 |0 |
|850 |24 |0 |0 |211 |2 |0 |0 |0 |
|66 |20 |4 |0 |3 |371 |0 |0 |0 |
|90 |47 |0 |0 |6 |0 |305 |0 |0 |
|371 |0 |0 |0 |0 |0 |0 |0 |0 |
|117 |2 |0 |0 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/static_root/admin/js/core.js | 78 |
| static/static_root/admin/js/SelectFilter2.js | 55 |
| static/static_root/admin/js/inlines.js | 49 |
| static/static_root/admin/js/SelectBox.js | 38 |
| static/static_root/admin/js/actions.js | 31 |
| static/static_root/admin/js/calendar.js | 28 |
| static/static_root/admin/js/admin/DateTimeShortcuts.js | 27 |
| static/static_root/admin/js/urlify.js | 26 |
| static/static_root/admin/js/timeparse.js | 13 |
| static/static_root/admin/js/popup_response.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 808}, "macro avg": {"f1-score": 0.7099496169890063, "precision": 0.7248190083667271, "recall": 0.6965119184398761, "support": 11328}, "micro avg": {"f1-score": 0.967337570621469, "precision": 0.967337570621469, "recall": 0.967337570621469, "support": 11328}, "weighted avg": {"f1-score": 0.9666719875788473, "precision": 0.9669841901923077, "recall": 0.967337570621469, "support": 11328}, "\u2205": {"f1-score": 0.9780300500834725, "precision": 0.9640600315955766, "recall": 0.9924108957853368, "support": 7379}, "\u23ce": {"f1-score": 0.9173913043478261, "precision": 0.9461883408071748, "recall": 0.890295358649789, "support": 237}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9428208386277, "precision": 0.9537275064267352, "recall": 0.9321608040201005, "support": 398}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9037037037037037, "precision": 0.9621451104100947, "recall": 0.8519553072625698, "support": 358}, "\u2423": {"f1-score": 0.9376510391493474, "precision": 0.9724310776942355, "recall": 0.9052729818012133, "support": 2143}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 371}, "\u0027": {"f1-score": 0.4559819413092551, "precision": 1.0, "recall": 0.2953216374269006, "support": 2736}, "macro avg": {"f1-score": 0.5006482291616856, "precision": 0.7248190083667271, "recall": 0.4186680506790179, "support": 17423}, "micro avg": {"f1-score": 0.7622691384647491, "precision": 0.967337570621469, "recall": 0.6289387591115193, "support": 17423}, "weighted avg": {"f1-score": 0.7150357513848741, "precision": 0.9430783252606865, "recall": 0.6289387591115193, "support": 17423}, "\u2205": {"f1-score": 0.9488824101068999, "precision": 0.9640600315955766, "recall": 0.9341752774588595, "support": 7839}, "\u23ce": {"f1-score": 0.32213740458015266, "precision": 0.9461883408071748, "recall": 0.19411223551057957, "support": 1087}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8698710433763189, "precision": 0.9537275064267352, "recall": 0.7995689655172413, "support": 464}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7973856209150326, "precision": 0.9621451104100947, "recall": 0.6808035714285714, "support": 448}, "\u2423": {"f1-score": 0.6109274130058259, "precision": 0.9724310776942355, "recall": 0.44536271808999084, "support": 4356}},
  "ppcr": 0.650175055960512
}
```
</details>
