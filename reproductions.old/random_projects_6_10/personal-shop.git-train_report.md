# Train report for javascript / file:///tmp/top-repos-quality-repos-tsc38vid/personal-shop.git HEAD 1c6ee0d8f873ca15edf37c40658ec31a045cf2ee

### Classification report

PPCR: 0.147

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 1.000| 1.000| 0.870| 1.000| 0.931| 369| 424| 0.870 |
| `␣` | 0.903| 1.000| 0.141| 0.949| 0.243| 140| 995| 0.141 |
| `⏎` | 0.985| 1.000| 0.471| 0.992| 0.637| 130| 276| 0.471 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 2755| 0.006 |
| `micro avg` | 0.974| 0.974| 0.144| 0.974| 0.250| 656| 4450| 0.147 |
| `weighted avg` | 0.950| 0.974| 0.144| 0.962| 0.183| 656| 4450| 0.147 |
| `macro avg` | 0.722| 0.750| 0.371| 0.735| 0.453| 656| 4450| 0.147 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|2738 |0 |15 |0 |2 |
|855 |0 |140 |0 |0 |
|55 |0 |0 |369 |0 |
|146 |0 |0 |0 |130 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/serviceWorker.js | 5 |
| client/src/components/AddItemPanel.js | 3 |
| client/src/components/ShopItem.js | 2 |
| client/src/components/ItemsTable.js | 2 |
| client/src/components/NotFound.js | 2 |
| client/src/Routes.js | 1 |
| client/src/components/Navigation.js | 1 |
| client/src/App.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 369}, "macro avg": {"f1-score": 0.7353797386466554, "precision": 0.7220185728250245, "recall": 0.75, "support": 656}, "micro avg": {"f1-score": 0.9740853658536586, "precision": 0.9740853658536586, "recall": 0.9740853658536586, "support": 656}, "weighted avg": {"f1-score": 0.9617210206097403, "precision": 0.950429749898672, "recall": 0.9740853658536586, "support": 656}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce": {"f1-score": 0.9923664122137404, "precision": 0.9848484848484849, "recall": 1.0, "support": 130}, "\u2423": {"f1-score": 0.9491525423728813, "precision": 0.9032258064516129, "recall": 1.0, "support": 140}},
  "cl_report_full": {"\"": {"f1-score": 0.9306431273644388, "precision": 1.0, "recall": 0.8702830188679245, "support": 424}, "macro avg": {"f1-score": 0.4528440725486971, "precision": 0.7220185728250245, "recall": 0.3705002573023718, "support": 4450}, "micro avg": {"f1-score": 0.25029377203290243, "precision": 0.9740853658536586, "recall": 0.14359550561797751, "support": 4450}, "weighted avg": {"f1-score": 0.18263728281099234, "precision": 0.3583208672443902, "recall": 0.14359550561797751, "support": 4450}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2755}, "\u23ce": {"f1-score": 0.6372549019607844, "precision": 0.9848484848484849, "recall": 0.47101449275362317, "support": 276}, "\u2423": {"f1-score": 0.2434782608695652, "precision": 0.9032258064516129, "recall": 0.1407035175879397, "support": 995}},
  "ppcr": 0.14741573033707867
}
```
</details>
