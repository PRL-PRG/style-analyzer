# Train report for javascript / file:///tmp/top-repos-quality-repos-hyvkzv60/tv-news-viewer.git HEAD b36622a1520d183edeff87ebc42788373f85ebac

### Classification report

PPCR: 0.712

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.993| 0.897| 0.987| 0.937| 10270| 11371| 0.903 |
| `␣` | 0.979| 0.957| 0.294| 0.967| 0.452| 1475| 4803| 0.307 |
| `'` | 1.000| 1.000| 0.953| 1.000| 0.976| 1418| 1488| 0.953 |
| `⏎␣⁺␣⁺` | 0.925| 0.888| 0.819| 0.906| 0.869| 735| 797| 0.922 |
| `⏎␣⁻␣⁻` | 0.983| 0.896| 0.781| 0.938| 0.871| 665| 763| 0.872 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 985| 0.005 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 262| 0.000 |
| `macro avg` | 0.695| 0.676| 0.535| 0.686| 0.586| 14568| 20469| 0.712 |
| `micro avg` | 0.980| 0.980| 0.698| 0.980| 0.815| 14568| 20469| 0.712 |
| `weighted avg` | 0.980| 0.980| 0.698| 0.980| 0.764| 14568| 20469| 0.712 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1101 |10200 |24 |0 |0 |42 |4 |0 |
|3328 |48 |1411 |0 |0 |10 |6 |0 |
|70 |0 |0 |1418 |0 |0 |0 |0 |
|980 |0 |5 |0 |0 |0 |0 |0 |
|62 |80 |2 |0 |0 |653 |0 |0 |
|98 |68 |0 |0 |0 |1 |596 |0 |
|262 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/js/chart.js | 75 |
| static/js/editor.js | 48 |
| static/js/home.js | 36 |
| static/js/videos.js | 28 |
| static/js/syntax-highlight.js | 24 |
| static/js/query.js | 23 |
| static/js/embed.js | 20 |
| static/js/example-queries.js | 18 |
| static/js/table.js | 7 |
| static/js/instructions.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1418}, "macro avg": {"f1-score": 0.6855314552347148, "precision": 0.6954394576561767, "recall": 0.6763528823290942, "support": 14568}, "micro avg": {"f1-score": 0.9800933552992861, "precision": 0.9800933552992861, "recall": 0.9800933552992861, "support": 14568}, "weighted avg": {"f1-score": 0.9797214101269196, "precision": 0.9796482321883706, "recall": 0.9800933552992861, "support": 14568}, "\u2205": {"f1-score": 0.9871286170521629, "precision": 0.9811465948441709, "recall": 0.9931840311587147, "support": 10270}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9063150589868147, "precision": 0.9249291784702549, "recall": 0.8884353741496599, "support": 735}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9378442171518488, "precision": 0.9834983498349835, "recall": 0.8962406015037594, "support": 665}, "\u2423": {"f1-score": 0.9674322934521769, "precision": 0.978502080443828, "recall": 0.9566101694915254, "support": 1475}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9759119064005506, "precision": 1.0, "recall": 0.9529569892473119, "support": 1488}, "macro avg": {"f1-score": 0.5863756112208217, "precision": 0.6954394576561767, "recall": 0.534885719173269, "support": 20469}, "micro avg": {"f1-score": 0.8150241173616461, "precision": 0.9800933552992861, "recall": 0.6975426254335825, "support": 20469}, "weighted avg": {"f1-score": 0.7639022395956205, "precision": 0.920022532537479, "recall": 0.6975426254335825, "support": 20469}, "\u2205": {"f1-score": 0.9371985115082464, "precision": 0.9811465948441709, "recall": 0.8970187318617536, "support": 11371}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 985}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 262}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8689288090485695, "precision": 0.9249291784702549, "recall": 0.8193224592220828, "support": 797}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.870708546384222, "precision": 0.9834983498349835, "recall": 0.781127129750983, "support": 763}, "\u2423": {"f1-score": 0.45188150520416337, "precision": 0.978502080443828, "recall": 0.2937747241307516, "support": 4803}},
  "ppcr": 0.7117103913234647
}
```
</details>
