# Train report for javascript / file:///tmp/top-repos-quality-repos-nyf5v0o_/eos_sales_contract.git HEAD 45fb7bdacbbeca6a425f5f2b9cbcbc557d85cd98

### Classification report

PPCR: 0.505

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 1.000| 0.772| 0.994| 0.867| 4027| 5215| 0.772 |
| `'` | 1.000| 1.000| 0.498| 1.000| 0.664| 599| 1204| 0.498 |
| `␣` | 0.987| 0.920| 0.191| 0.953| 0.320| 591| 2853| 0.207 |
| `⏎␣⁺␣⁺` | 0.978| 0.996| 0.740| 0.987| 0.843| 274| 369| 0.743 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 735| 0.010 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 369| 0.005 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 154| 0.013 |
| `macro avg` | 0.565| 0.560| 0.314| 0.562| 0.385| 5502| 10899| 0.505 |
| `weighted avg` | 0.987| 0.989| 0.499| 0.988| 0.601| 5502| 10899| 0.505 |
| `micro avg` | 0.989| 0.989| 0.499| 0.989| 0.664| 5502| 10899| 0.505 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1188 |4027 |0 |0 |0 |0 |0 |0 |
|2262 |43 |544 |0 |0 |0 |4 |0 |
|605 |0 |0 |599 |0 |0 |0 |0 |
|728 |0 |7 |0 |0 |0 |0 |0 |
|367 |2 |0 |0 |0 |0 |0 |0 |
|95 |1 |0 |0 |0 |0 |273 |0 |
|152 |0 |0 |0 |0 |0 |2 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/store/modules/eosStore.js | 37 |
| tests/services/testService.js | 8 |
| frontend/.eslintrc.js | 4 |
| .eslintrc.js | 3 |
| frontend/src/util/eosUtil.js | 2 |
| frontend/src/util/eos.js | 1 |
| abigeneration/convertService.js | 1 |
| tests/test/errorfulSaleTest.js | 1 |
| frontend/src/main.js | 1 |
| tests/test/successfulSaleTests.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 599}, "macro avg": {"f1-score": 0.562053899386878, "precision": 0.5649280804081632, "recall": 0.5595463054613078, "support": 5502}, "micro avg": {"f1-score": 0.9892766266812069, "precision": 0.9892766266812069, "recall": 0.9892766266812069, "support": 5502}, "weighted avg": {"f1-score": 0.9881345971595074, "precision": 0.9872989601408211, "recall": 0.9892766266812069, "support": 5502}, "\u2205": {"f1-score": 0.994320987654321, "precision": 0.9887061134299042, "recall": 1.0, "support": 4027}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9873417721518988, "precision": 0.978494623655914, "recall": 0.9963503649635036, "support": 274}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9527145359019263, "precision": 0.9872958257713249, "recall": 0.9204737732656514, "support": 591}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6644481419855796, "precision": 1.0, "recall": 0.49750830564784054, "support": 1204}, "macro avg": {"f1-score": 0.3848293003694601, "precision": 0.5649280804081632, "recall": 0.31431682493776847, "support": 10899}, "micro avg": {"f1-score": 0.6637400158526919, "precision": 0.9892766266812069, "recall": 0.4994036150105514, "support": 10899}, "weighted avg": {"f1-score": 0.600507990402117, "precision": 0.8751189915213847, "recall": 0.4994036150105514, "support": 10899}, "\u2205": {"f1-score": 0.8671403962101636, "precision": 0.9887061134299042, "recall": 0.772195589645254, "support": 5215}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 735}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8425925925925926, "precision": 0.978494623655914, "recall": 0.7398373983739838, "support": 369}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 369}, "\u2423": {"f1-score": 0.3196239717978848, "precision": 0.9872958257713249, "recall": 0.19067648089730108, "support": 2853}},
  "ppcr": 0.5048169556840078
}
```
</details>
