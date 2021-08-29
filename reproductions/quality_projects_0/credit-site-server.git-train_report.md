# Train report for javascript / file:///tmp/top-repos-quality-repos-il12m_si/credit-site-server.git HEAD 36e265a6d4834ae9a8f251c72da47d3bd7a8b58b

### Classification report

PPCR: 0.493

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 1.000| 0.635| 0.979| 0.764| 5191| 8175| 0.635 |
| `␣` | 1.000| 0.800| 0.235| 0.889| 0.381| 1271| 4327| 0.294 |
| `'` | 1.000| 1.000| 0.510| 1.000| 0.675| 567| 1112| 0.510 |
| `⏎` | 0.930| 1.000| 0.519| 0.964| 0.666| 532| 1025| 0.519 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.972| 0.994| 0.909| 0.983| 0.939| 522| 571| 0.914 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 488| 0.029 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 744| 0.008 |
| `macro avg` | 0.694| 0.685| 0.401| 0.688| 0.489| 8103| 16442| 0.493 |
| `micro avg` | 0.966| 0.966| 0.476| 0.966| 0.638| 8103| 16442| 0.493 |
| `weighted avg` | 0.965| 0.966| 0.476| 0.963| 0.600| 8103| 16442| 0.493 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2984 |5190 |0 |0 |0 |0 |1 |0 |
|3056 |214 |1017 |0 |26 |0 |14 |0 |
|545 |0 |0 |567 |0 |0 |0 |0 |
|493 |0 |0 |0 |532 |0 |0 |0 |
|738 |6 |0 |0 |0 |0 |0 |0 |
|49 |3 |0 |0 |0 |0 |519 |0 |
|474 |0 |0 |0 |14 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/controllers/auth.js | 24 |
| src/controllers/api/managers.js | 17 |
| src/controllers/api/clients.js | 14 |
| frontend/src/services/notification.js | 12 |
| frontend/src/shared/index.js | 11 |
| src/middlewares/auth.js | 10 |
| src/controllers/api/admins.js | 10 |
| src/business/api/users.js | 9 |
| src/controllers/api/loans.js | 8 |
| src/validator/index.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 567}, "macro avg": {"f1-score": 0.6877977958958476, "precision": 0.6943975606257629, "recall": 0.68488822698084, "support": 8103}, "micro avg": {"f1-score": 0.9656917191163766, "precision": 0.9656917191163766, "recall": 0.9656917191163766, "support": 8103}, "weighted avg": {"f1-score": 0.9631090842578119, "precision": 0.9647389907016904, "recall": 0.9656917191163766, "support": 8103}, "\u2205": {"f1-score": 0.978875895888344, "precision": 0.958802881950859, "recall": 0.9998073588903872, "support": 5191}, "\u23ce": {"f1-score": 0.963768115942029, "precision": 0.9300699300699301, "recall": 1.0, "support": 532}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9829545454545454, "precision": 0.9719101123595506, "recall": 0.9942528735632183, "support": 522}, "\u2423": {"f1-score": 0.888986013986014, "precision": 1.0, "recall": 0.8001573564122738, "support": 1271}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6754020250148899, "precision": 1.0, "recall": 0.5098920863309353, "support": 1112}, "macro avg": {"f1-score": 0.48936298033611564, "precision": 0.6943975606257629, "recall": 0.40110662603648756, "support": 16442}, "micro avg": {"f1-score": 0.637604400081483, "precision": 0.9656917191163766, "recall": 0.47591533876657344, "support": 16442}, "weighted avg": {"f1-score": 0.5998178664792805, "precision": 0.8992516672197576, "recall": 0.47591533876657344, "support": 16442}, "\u2205": {"f1-score": 0.7639093317633207, "precision": 0.958802881950859, "recall": 0.634862385321101, "support": 8175}, "\u23ce": {"f1-score": 0.6662492172824046, "precision": 0.9300699300699301, "recall": 0.5190243902439025, "support": 1025}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 488}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 744}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9393665158371041, "precision": 0.9719101123595506, "recall": 0.9089316987740805, "support": 571}, "\u2423": {"f1-score": 0.3806137724550898, "precision": 1.0, "recall": 0.23503582158539404, "support": 4327}},
  "ppcr": 0.49282325751125167
}
```
</details>
