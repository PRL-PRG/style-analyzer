# Train report for javascript / file:///tmp/top-repos-quality-repos-conxs3vr/kingsland-blockchain-advanced-project.git HEAD 30c7ea23a73509b0abee2d47920032f112a205c3

### Classification report

PPCR: 0.849

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.970| 0.951| 0.982| 0.972| 42774| 43629| 0.980 |
| `␣` | 0.938| 0.993| 0.911| 0.964| 0.924| 20338| 22159| 0.918 |
| `⏎␣⁺␣⁺` | 0.955| 0.965| 0.963| 0.960| 0.959| 2310| 2315| 0.998 |
| `⏎␣⁻␣⁻` | 0.985| 0.928| 0.921| 0.956| 0.952| 2233| 2249| 0.993 |
| `⏎` | 0.936| 0.881| 0.207| 0.908| 0.339| 885| 3764| 0.235 |
| `'` | 1.000| 1.000| 0.078| 1.000| 0.145| 305| 3898| 0.078 |
| `⏎⏎` | 0.953| 0.916| 0.109| 0.934| 0.196| 285| 2390| 0.119 |
| `"` | 1.000| 1.000| 0.009| 1.000| 0.018| 10| 1073| 0.009 |
| `micro avg` | 0.974| 0.974| 0.827| 0.974| 0.894| 69140| 81477| 0.849 |
| `macro avg` | 0.970| 0.957| 0.519| 0.963| 0.563| 69140| 81477| 0.849 |
| `weighted avg` | 0.975| 0.974| 0.827| 0.974| 0.854| 69140| 81477| 0.849 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|855 |41500 |1199 |0 |1 |0 |58 |16 |0 |
|1821 |58 |20194 |0 |29 |0 |44 |13 |0 |
|3593 |0 |0 |305 |0 |0 |0 |0 |0 |
|2879 |25 |64 |0 |780 |13 |1 |2 |0 |
|2105 |1 |0 |0 |22 |261 |1 |0 |0 |
|5 |11 |70 |0 |0 |0 |2229 |0 |0 |
|16 |150 |10 |0 |1 |0 |0 |2072 |0 |
|1063 |0 |0 |0 |0 |0 |0 |0 |10 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Wallet/static/bootstrap4/js/bootstrap.bundle.js | 378 |
| Faucet/public/static/bootstrap4/js/bootstrap.bundle.js | 378 |
| block-explorer/src/containers/BlockPage.js | 155 |
| block-explorer/src/components/HomePage.js | 100 |
| block-explorer/src/containers/AddressPage.js | 97 |
| Wallet/static/js/wallet.js | 91 |
| block-explorer/src/common/Header.js | 74 |
| block-explorer/src/containers/TransactionPage.js | 63 |
| Faucet/public/static/js/wallet.js | 57 |
| block-explorer/src/routes.js | 52 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 10}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 305}, "macro avg": {"f1-score": 0.9630293578805613, "precision": 0.9701729852391605, "recall": 0.9566394208287188, "support": 69140}, "micro avg": {"f1-score": 0.9741249638414811, "precision": 0.9741249638414811, "recall": 0.9741249638414811, "support": 69140}, "weighted avg": {"f1-score": 0.9742242777342821, "precision": 0.9750507144536474, "recall": 0.9741249638414811, "support": 69140}, "\u2205": {"f1-score": 0.9820277097457376, "precision": 0.9941310336567254, "recall": 0.9702155515032497, "support": 42774}, "\u23ce": {"f1-score": 0.9080325960419092, "precision": 0.936374549819928, "recall": 0.8813559322033898, "support": 885}, "\u23ce\u23ce": {"f1-score": 0.9338103756708408, "precision": 0.9525547445255474, "recall": 0.9157894736842105, "support": 285}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9601550721516261, "precision": 0.9554222031718816, "recall": 0.964935064935065, "support": 2310}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9557195571955719, "precision": 0.9852591535901094, "recall": 0.9278996865203761, "support": 2233}, "\u2423": {"f1-score": 0.964489552238806, "precision": 0.9376421971490922, "recall": 0.9929196577834596, "support": 20338}},
  "cl_report_full": {"\"": {"f1-score": 0.018467220683287162, "precision": 1.0, "recall": 0.009319664492078284, "support": 1073}, "\u0027": {"f1-score": 0.14513442778967403, "precision": 1.0, "recall": 0.07824525397639816, "support": 3898}, "macro avg": {"f1-score": 0.5633394375379228, "precision": 0.9701729852391605, "recall": 0.5188338145288564, "support": 81477}, "micro avg": {"f1-score": 0.894334636860381, "precision": 0.9741249638414811, "recall": 0.8266259189709979, "support": 81477}, "weighted avg": {"f1-score": 0.8541100015128208, "precision": 0.9738935674575812, "recall": 0.8266259189709979, "support": 81477}, "\u2205": {"f1-score": 0.9721929393023637, "precision": 0.9941310336567254, "recall": 0.951202182034885, "support": 43629}, "\u23ce": {"f1-score": 0.3393517511420492, "precision": 0.936374549819928, "recall": 0.20722635494155153, "support": 3764}, "\u23ce\u23ce": {"f1-score": 0.19594594594594594, "precision": 0.9525547445255474, "recall": 0.10920502092050209, "support": 2390}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9591222030981068, "precision": 0.9554222031718816, "recall": 0.9628509719222462, "support": 2315}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9522058823529412, "precision": 0.9852591535901094, "recall": 0.9212983548243664, "support": 2249}, "\u2423": {"f1-score": 0.9242951299890151, "precision": 0.9376421971490922, "recall": 0.9113227131188231, "support": 22159}},
  "ppcr": 0.8485830357033273
}
```
</details>
