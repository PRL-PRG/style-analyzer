# Train report for javascript / file:///tmp/top-repos-quality-repos-vmgouym_/apachecn-algo-zh.git HEAD 42b6ae71e44de3ec687be6f73068b0ec7e3007f8

### Classification report

PPCR: 0.909

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.996| 1.000| 0.998| 2533| 2544| 0.996 |
| `␣` | 0.999| 0.996| 0.911| 0.998| 0.953| 1268| 1387| 0.914 |
| `⏎` | 0.942| 1.000| 0.895| 0.970| 0.918| 196| 219| 0.895 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 257| 0.031 |
| `micro avg` | 0.997| 0.997| 0.906| 0.997| 0.949| 4005| 4407| 0.909 |
| `weighted avg` | 0.995| 0.997| 0.906| 0.996| 0.922| 4005| 4407| 0.909 |
| `macro avg` | 0.735| 0.749| 0.700| 0.742| 0.717| 4005| 4407| 0.909 |

### Confusion matrix

|refusal|  '| ␣| ⏎| ∅| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|11 |2533 |0 |0 |0 |
|119 |0 |1263 |5 |0 |
|23 |0 |0 |196 |0 |
|249 |0 |1 |7 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| asset/docsify-cnzz.js | 7 |
| asset/docsify-clicker.js | 3 |
| asset/docsify-apachecn-footer.js | 2 |
| asset/docsify-baidu-push.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2533}, "macro avg": {"f1-score": 0.7419818403641312, "precision": 0.7353791382667965, "recall": 0.7490141955835963, "support": 4005}, "micro avg": {"f1-score": 0.9967540574282148, "precision": 0.9967540574282148, "recall": 0.9967540574282148, "support": 4005}, "weighted avg": {"f1-score": 0.9957986213446415, "precision": 0.9949286250025832, "recall": 0.9967540574282148, "support": 4005}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce": {"f1-score": 0.9702970297029703, "precision": 0.9423076923076923, "recall": 1.0, "support": 196}, "\u2423": {"f1-score": 0.9976303317535546, "precision": 0.9992088607594937, "recall": 0.9960567823343849, "support": 1268}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9978333661611187, "precision": 1.0, "recall": 0.9956761006289309, "support": 2544}, "macro avg": {"f1-score": 0.7171785337349974, "precision": 0.7353791382667965, "recall": 0.7003129208553822, "support": 4407}, "micro avg": {"f1-score": 0.9491203043271518, "precision": 0.9967540574282148, "recall": 0.9058316314953483, "support": 4407}, "weighted avg": {"f1-score": 0.9215197219714574, "precision": 0.938567750054187, "recall": 0.9058316314953483, "support": 4407}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 257}, "\u23ce": {"f1-score": 0.9180327868852458, "precision": 0.9423076923076923, "recall": 0.8949771689497716, "support": 219}, "\u2423": {"f1-score": 0.9528479818936251, "precision": 0.9992088607594937, "recall": 0.9105984138428262, "support": 1387}},
  "ppcr": 0.908781484002723
}
```
</details>
