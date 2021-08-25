# Train report for javascript / file:///tmp/top-repos-quality-repos-qi5qoyr3/yeelight-adapter.git HEAD 2cc604d8f474859ba78ab854fdd69a13ccb81d76

### Classification report

PPCR: 0.432

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.927| 1.000| 0.726| 0.962| 0.814| 985| 1356| 0.726 |
| `'` | 1.000| 1.000| 0.497| 1.000| 0.664| 311| 626| 0.497 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 61| 575| 0.106 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 156| 0.083 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 170| 0.024 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 298| 0.000 |
| `macro avg` | 0.321| 0.333| 0.204| 0.327| 0.246| 1374| 3181| 0.432 |
| `micro avg` | 0.943| 0.943| 0.407| 0.943| 0.569| 1374| 3181| 0.432 |
| `weighted avg` | 0.891| 0.943| 0.407| 0.916| 0.478| 1374| 3181| 0.432 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|371 |985 |0 |0 |0 |0 |0 |
|315 |0 |311 |0 |0 |0 |0 |
|514 |61 |0 |0 |0 |0 |0 |
|298 |0 |0 |0 |0 |0 |0 |
|166 |4 |0 |0 |0 |0 |0 |
|143 |13 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/yeelight-device.js | 32 |
| lib/yeelight-property.js | 22 |
| .eslintrc.js | 12 |
| lib/yeelight-adapter.js | 10 |
| index.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 311}, "macro avg": {"f1-score": 0.3269856770833333, "precision": 0.3211037942928818, "recall": 0.3333333333333333, "support": 1374}, "micro avg": {"f1-score": 0.9432314410480349, "precision": 0.9432314410480349, "recall": 0.9432314410480349, "support": 1374}, "weighted avg": {"f1-score": 0.9159282034661572, "precision": 0.8906284019439091, "recall": 0.9432314410480349, "support": 1374}, "\u2205": {"f1-score": 0.9619140625, "precision": 0.9266227657572906, "recall": 1.0, "support": 985}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}},
  "cl_report_full": {"\u0027": {"f1-score": 0.663820704375667, "precision": 1.0, "recall": 0.4968051118210863, "support": 626}, "macro avg": {"f1-score": 0.24636780238974357, "precision": 0.3211037942928818, "recall": 0.20386771529368153, "support": 3181}, "micro avg": {"f1-score": 0.5690450054884743, "precision": 0.9432314410480349, "recall": 0.40741905061301475, "support": 3181}, "weighted avg": {"f1-score": 0.4777929349414388, "precision": 0.5917951808761037, "recall": 0.40741905061301475, "support": 3181}, "\u2205": {"f1-score": 0.8143861099627945, "precision": 0.9266227657572906, "recall": 0.726401179941003, "support": 1356}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 298}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 170}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 575}},
  "ppcr": 0.4319396416221314
}
```
</details>
