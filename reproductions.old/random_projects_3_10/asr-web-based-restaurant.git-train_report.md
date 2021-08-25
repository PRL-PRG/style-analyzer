# Train report for javascript / file:///tmp/top-repos-quality-repos-5e6n9nzx/asr-web-based-restaurant.git HEAD e50ebeb0e2cbc03f6571314682411f6097f8a71c

### Classification report

PPCR: 0.669

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.950| 0.999| 0.881| 0.974| 0.914| 6009| 6813| 0.882 |
| `"` | 1.000| 1.000| 0.913| 1.000| 0.954| 973| 1066| 0.913 |
| `␣` | 0.989| 0.764| 0.192| 0.862| 0.322| 563| 2238| 0.252 |
| `⏎` | 0.947| 0.871| 0.221| 0.907| 0.359| 225| 886| 0.254 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 130| 406| 0.320 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 30| 374| 0.080 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 84| 0.095 |
| `weighted avg` | 0.938| 0.958| 0.641| 0.947| 0.698| 7938| 11867| 0.669 |
| `micro avg` | 0.958| 0.958| 0.641| 0.958| 0.768| 7938| 11867| 0.669 |
| `macro avg` | 0.555| 0.519| 0.315| 0.535| 0.364| 7938| 11867| 0.669 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|804 |6005 |4 |0 |0 |0 |0 |0 |
|1675 |123 |430 |0 |10 |0 |0 |0 |
|93 |0 |0 |973 |0 |0 |0 |0 |
|661 |29 |0 |0 |196 |0 |0 |0 |
|276 |128 |1 |0 |1 |0 |0 |0 |
|344 |30 |0 |0 |0 |0 |0 |0 |
|76 |8 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| asr-web/src/components/Search.js | 107 |
| asr-web/src/card/OrderCard.js | 36 |
| asr-web/src/card/MenuCard.js | 32 |
| asr-web/src/components/Billing.js | 31 |
| asr-web/src/card/TableCard.js | 27 |
| asr-web/src/serviceWorker.js | 26 |
| asr-web/src/components/Menu.js | 25 |
| asr-web/src/components/ReserveTable.js | 13 |
| asr-web/src/components/MenuBar.js | 12 |
| asr-web/src/card/ThankYouCard.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 973}, "macro avg": {"f1-score": 0.5347171319128208, "precision": 0.5550104382676077, "recall": 0.5191729978124808, "support": 7938}, "micro avg": {"f1-score": 0.9579239103048627, "precision": 0.9579239103048627, "recall": 0.9579239103048627, "support": 7938}, "weighted avg": {"f1-score": 0.9466384962737424, "precision": 0.9384434473217925, "recall": 0.9579239103048627, "support": 7938}, "\u2205": {"f1-score": 0.9738890690885502, "precision": 0.9497074173651747, "recall": 0.99933433183558, "support": 6009}, "\u23ce": {"f1-score": 0.9074074074074072, "precision": 0.9468599033816425, "recall": 0.8711111111111111, "support": 225}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "\u2423": {"f1-score": 0.8617234468937875, "precision": 0.9885057471264368, "recall": 0.7637655417406749, "support": 563}},
  "cl_report_full": {"\"": {"f1-score": 0.9543894065718489, "precision": 1.0, "recall": 0.9127579737335835, "support": 1066}, "macro avg": {"f1-score": 0.3641503680985819, "precision": 0.5550104382676077, "recall": 0.3153594243844987, "support": 11867}, "micro avg": {"f1-score": 0.7678868972481697, "precision": 0.9579239103048627, "recall": 0.6407685177382658, "support": 11867}, "weighted avg": {"f1-score": 0.6980857190345122, "precision": 0.8921842395697342, "recall": 0.6407685177382658, "support": 11867}, "\u2205": {"f1-score": 0.9142813641900122, "precision": 0.9497074173651747, "recall": 0.8814031997651548, "support": 6813}, "\u23ce": {"f1-score": 0.3586459286367795, "precision": 0.9468599033816425, "recall": 0.22121896162528218, "support": 886}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 374}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 406}, "\u2423": {"f1-score": 0.3217358772914328, "precision": 0.9885057471264368, "recall": 0.19213583556747096, "support": 2238}},
  "ppcr": 0.6689137945563327
}
```
</details>
