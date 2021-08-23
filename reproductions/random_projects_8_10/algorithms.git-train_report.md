# Train report for javascript / file:///tmp/top-repos-quality-repos-ytvp6mi1/algorithms.git HEAD a6e5a59f0f51691108484cf61b1c1bf9a8ab7aaf

### Classification report

PPCR: 0.427

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.944| 1.000| 0.665| 0.971| 0.781| 793| 1192| 0.665 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 667| 0.070 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 109| 0.000 |
| `macro avg` | 0.315| 0.333| 0.222| 0.324| 0.260| 840| 1968| 0.427 |
| `micro avg` | 0.944| 0.944| 0.403| 0.944| 0.565| 840| 1968| 0.427 |
| `weighted avg` | 0.891| 0.944| 0.403| 0.917| 0.473| 840| 1968| 0.427 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|399 |793 |0 |0 |
|620 |47 |0 |0 |
|109 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/bfs.js | 9 |
| js/merge_sort.js | 8 |
| js/get_products_of_all_ints_except_at_index.js | 7 |
| js/heap_sort.js | 7 |
| js/dfs.js | 6 |
| js/quick_sort.js | 5 |
| js/get_max_profit.js | 3 |
| js/queue_with_stack.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.32373953868136357, "precision": 0.3146825396825397, "recall": 0.3333333333333333, "support": 840}, "micro avg": {"f1-score": 0.944047619047619, "precision": 0.944047619047619, "recall": 0.944047619047619, "support": 840}, "weighted avg": {"f1-score": 0.9168766220511475, "precision": 0.8912259070294783, "recall": 0.944047619047619, "support": 840}, "\u2205": {"f1-score": 0.9712186160440907, "precision": 0.944047619047619, "recall": 1.0, "support": 793}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}},
  "cl_report_full": {"macro avg": {"f1-score": 0.2601706036745407, "precision": 0.3146825396825397, "recall": 0.22175615212527966, "support": 1968}, "micro avg": {"f1-score": 0.5648148148148148, "precision": 0.944047619047619, "recall": 0.4029471544715447, "support": 1968}, "weighted avg": {"f1-score": 0.47274902375008004, "precision": 0.5718012001548586, "recall": 0.4029471544715448, "support": 1968}, "\u2205": {"f1-score": 0.7805118110236221, "precision": 0.944047619047619, "recall": 0.665268456375839, "support": 1192}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 109}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 667}},
  "ppcr": 0.4268292682926829
}
```
</details>
