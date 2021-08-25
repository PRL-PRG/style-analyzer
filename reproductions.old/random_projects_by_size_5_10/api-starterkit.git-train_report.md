# Train report for javascript / file:///tmp/top-repos-quality-repos-brm_wqmi/api-starterkit.git HEAD 0547bc907530afc90f960c82c41caecc63d23966

### Classification report

PPCR: 0.261

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.996| 0.306| 0.989| 0.467| 473| 1538| 0.308 |
| `␣` | 0.989| 1.000| 0.280| 0.995| 0.436| 184| 658| 0.280 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 165| 0.048 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 183| 0.000 |
| `macro avg` | 0.493| 0.499| 0.146| 0.496| 0.226| 665| 2544| 0.261 |
| `weighted avg` | 0.973| 0.985| 0.257| 0.979| 0.395| 665| 2544| 0.261 |
| `micro avg` | 0.985| 0.985| 0.257| 0.985| 0.408| 665| 2544| 0.261 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|1065 |471 |2 |0 |0 |
|474 |0 |184 |0 |0 |
|183 |0 |0 |0 |0 |
|157 |8 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| express-mongoose/server/auth/auth.controller.js | 4 |
| express-mongoose/config/express.js | 2 |
| express-mongoose/config/config.js | 1 |
| express-mongoose/server/user/user.controller.js | 1 |
| express-mongoose/server/user/user.model.js | 1 |
| express-mongoose/config/winston.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4960225982284806, "precision": 0.49313646261252164, "recall": 0.4989429175475687, "support": 665}, "micro avg": {"f1-score": 0.9849624060150376, "precision": 0.9849624060150376, "recall": 0.9849624060150376, "support": 665}, "weighted avg": {"f1-score": 0.9790028842262367, "precision": 0.973115359615957, "recall": 0.9849624060150376, "support": 665}, "\u2205": {"f1-score": 0.9894957983193277, "precision": 0.9832985386221295, "recall": 0.9957716701902748, "support": 473}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.9945945945945946, "precision": 0.989247311827957, "recall": 1.0, "support": 184}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 183}, "macro avg": {"f1-score": 0.2257623000702559, "precision": 0.49313646261252164, "recall": 0.14646928273010779, "support": 2544}, "micro avg": {"f1-score": 0.40822686195076346, "precision": 0.9849624060150376, "recall": 0.2574685534591195, "support": 2544}, "weighted avg": {"f1-score": 0.3951230297043079, "precision": 0.8503293567545718, "recall": 0.2574685534591195, "support": 2544}, "\u2205": {"f1-score": 0.4670302429350521, "precision": 0.9832985386221295, "recall": 0.30624187256176855, "support": 1538}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 165}, "\u2423": {"f1-score": 0.43601895734597157, "precision": 0.989247311827957, "recall": 0.2796352583586626, "support": 658}},
  "ppcr": 0.2613993710691824
}
```
</details>
