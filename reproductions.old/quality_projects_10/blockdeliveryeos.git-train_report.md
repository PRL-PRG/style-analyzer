# Train report for javascript / file:///tmp/top-repos-quality-repos-t79wzl_9/blockdeliveryeos.git HEAD af8c3b962b37cc55a4f90c90145b3aeb2a1e16fc

### Classification report

PPCR: 0.458

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.960| 1.000| 0.869| 0.979| 0.912| 2583| 2972| 0.869 |
| `␣` | 0.994| 0.671| 0.086| 0.801| 0.159| 255| 1985| 0.128 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 362| 0.039 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 131| 0.061 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 131| 0.015 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 284| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 222| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 158| 0.000 |
| `macro avg` | 0.244| 0.209| 0.119| 0.223| 0.134| 2862| 6245| 0.458 |
| `micro avg` | 0.962| 0.962| 0.441| 0.962| 0.605| 2862| 6245| 0.458 |
| `weighted avg` | 0.955| 0.962| 0.441| 0.955| 0.484| 2862| 6245| 0.458 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|389 |2582 |1 |0 |0 |0 |0 |0 |0 |
|1730 |84 |171 |0 |0 |0 |0 |0 |0 |
|348 |14 |0 |0 |0 |0 |0 |0 |0 |
|284 |0 |0 |0 |0 |0 |0 |0 |0 |
|222 |0 |0 |0 |0 |0 |0 |0 |0 |
|129 |2 |0 |0 |0 |0 |0 |0 |0 |
|123 |8 |0 |0 |0 |0 |0 |0 |0 |
|158 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| eosio_docker/tests/test.js | 45 |
| frontend/src/services/KeyGenerator.js | 35 |
| frontend/src/reducers/OrderReducer.js | 16 |
| frontend/src/services/ApiServiceScatter.js | 7 |
| frontend/src/index.js | 2 |
| frontend/src/services/ApiService.js | 2 |
| frontend/src/reducers/ScatterReducer.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.22253317794553223, "precision": 0.2442546684533587, "recall": 0.20877513607068843, "support": 2862}, "micro avg": {"f1-score": 0.9619147449336128, "precision": 0.9619147449336128, "recall": 0.9619147449336128, "support": 2862}, "weighted avg": {"f1-score": 0.955221800424022, "precision": 0.9548614090290476, "recall": 0.9619147449336128, "support": 2862}, "\u2205": {"f1-score": 0.979328655414375, "precision": 0.9598513011152416, "recall": 0.9996128532713898, "support": 2583}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.8009367681498829, "precision": 0.9941860465116279, "recall": 0.6705882352941176, "support": 255}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 284}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 222}, "macro avg": {"f1-score": 0.13382484503723674, "precision": 0.2442546684533587, "recall": 0.11936516640618908, "support": 6245}, "micro avg": {"f1-score": 0.6045898759196222, "precision": 0.9619147449336128, "recall": 0.44083266613290634, "support": 6245}, "weighted avg": {"f1-score": 0.4844398983376386, "precision": 0.7728002192538157, "recall": 0.44083266613290634, "support": 6245}, "\u2205": {"f1-score": 0.9120452137054044, "precision": 0.9598513011152416, "recall": 0.8687752355316285, "support": 2972}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 362}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 158}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 131}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 131}, "\u2423": {"f1-score": 0.15855354659248957, "precision": 0.9941860465116279, "recall": 0.08614609571788413, "support": 1985}},
  "ppcr": 0.45828662930344277
}
```
</details>
