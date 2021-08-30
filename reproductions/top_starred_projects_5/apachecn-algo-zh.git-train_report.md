# Train report for javascript / file:///tmp/top-repos-quality-repos-m3paek18/apachecn-algo-zh.git HEAD 71862d95dd7e1e34c8e193b3ebf1063172a2cbf0

### Classification report

PPCR: 0.797

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.989| 1.000| 0.994| 2527| 2555| 0.989 |
| `␣` | 0.953| 1.000| 0.851| 0.976| 0.899| 1290| 1516| 0.851 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 59| 560| 0.105 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 236| 0.017 |
| `weighted avg` | 0.968| 0.984| 0.784| 0.976| 0.802| 3880| 4867| 0.797 |
| `macro avg` | 0.488| 0.500| 0.460| 0.494| 0.473| 3880| 4867| 0.797 |
| `micro avg` | 0.984| 0.984| 0.784| 0.984| 0.873| 3880| 4867| 0.797 |

### Confusion matrix

|refusal|  '| ␣| ∅| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|28 |2527 |0 |0 |0 |
|226 |0 |1290 |0 |0 |
|501 |0 |59 |0 |0 |
|232 |0 |4 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| asset/dark-mode.js | 17 |
| asset/docsify-quick-page.js | 13 |
| asset/docsify-apachecn-footer.js | 12 |
| asset/docsify-baidu-push.js | 8 |
| asset/docsify-baidu-stat.js | 6 |
| asset/docsify-cnzz.js | 5 |
| asset/docsify-clicker.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2527}, "macro avg": {"f1-score": 0.4940408626560726, "precision": 0.48835920177383596, "recall": 0.5, "support": 3880}, "micro avg": {"f1-score": 0.9837628865979381, "precision": 0.9837628865979381, "recall": 0.9837628865979381, "support": 3880}, "weighted avg": {"f1-score": 0.9758378482745709, "precision": 0.9682818250394313, "recall": 0.9837628865979381, "support": 3880}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9761634506242906, "precision": 0.9534368070953437, "recall": 1.0, "support": 1290}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9944903581267218, "precision": 1.0, "recall": 0.989041095890411, "support": 2555}, "macro avg": {"f1-score": 0.473439598942625, "precision": 0.48835920177383596, "recall": 0.4599911446850038, "support": 4867}, "micro avg": {"f1-score": 0.8727563736138104, "precision": 0.9837628865979381, "recall": 0.7842613519621944, "support": 4867}, "weighted avg": {"f1-score": 0.8021806472327393, "precision": 0.8219457981418823, "recall": 0.7842613519621944, "support": 4867}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 560}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 236}, "\u2423": {"f1-score": 0.8992680376437783, "precision": 0.9534368070953437, "recall": 0.8509234828496042, "support": 1516}},
  "ppcr": 0.7972056708444627
}
```
</details>
