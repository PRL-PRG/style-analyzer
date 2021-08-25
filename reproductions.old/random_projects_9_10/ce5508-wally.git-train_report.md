# Train report for javascript / file:///tmp/top-repos-quality-repos-y77l8pv8/ce5508-wally.git HEAD 4754ebe118e0967bde3475738afd51977940470a

### Classification report

PPCR: 0.360

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 0.985| 0.535| 0.974| 0.688| 1146| 2109| 0.543 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 222| 444| 0.500 |
| `␣` | 0.777| 0.885| 0.105| 0.828| 0.186| 122| 1025| 0.119 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 192| 0.104 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 187| 0.102 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 297| 0.013 |
| `micro avg` | 0.952| 0.952| 0.343| 0.952| 0.504| 1533| 4254| 0.360 |
| `macro avg` | 0.457| 0.478| 0.190| 0.467| 0.257| 1533| 4254| 0.360 |
| `weighted avg` | 0.927| 0.952| 0.343| 0.939| 0.455| 1533| 4254| 0.360 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|963 |1129 |17 |0 |0 |0 |0 |
|903 |14 |108 |0 |0 |0 |0 |
|222 |0 |0 |222 |0 |0 |0 |
|293 |4 |0 |0 |0 |0 |0 |
|172 |6 |14 |0 |0 |0 |0 |
|168 |19 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| code_app/Wally/src/components/mqtt.js | 21 |
| code_app/Wally/src/screens/tabs-control-page/tab-controller/controller.js | 13 |
| code_app/Wally/src/screens/log-in/index.js | 11 |
| code_app/Wally/src/screens/tabs-control-page/tab-dashboard/dashboard.js | 11 |
| code_app/Wally/src/screens/sign-up/index.js | 8 |
| code_app/Wally/src/screens/home-page/index.js | 5 |
| code_server/server_hapijs/src/index.js | 4 |
| code_app/Wally/src/screens/settings-page/index.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 222}, "macro avg": {"f1-score": 0.466950303967947, "precision": 0.4567148329118275, "recall": 0.478401949284277, "support": 1533}, "micro avg": {"f1-score": 0.9517286366601435, "precision": 0.9517286366601435, "recall": 0.9517286366601435, "support": 1533}, "weighted avg": {"f1-score": 0.938879330868581, "precision": 0.9267744891201244, "recall": 0.9517286366601435, "support": 1533}, "\u2205": {"f1-score": 0.9741156169111302, "precision": 0.9633105802047781, "recall": 0.9851657940663177, "support": 1146}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u2423": {"f1-score": 0.8275862068965517, "precision": 0.7769784172661871, "recall": 0.8852459016393442, "support": 122}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 444}, "macro avg": {"f1-score": 0.2567397487634906, "precision": 0.4567148329118275, "recall": 0.19011510869020495, "support": 4254}, "micro avg": {"f1-score": 0.5042336270952134, "precision": 0.9517286366601435, "recall": 0.342971321109544, "support": 4254}, "weighted avg": {"f1-score": 0.45548428342217967, "precision": 0.7691642903972071, "recall": 0.342971321109544, "support": 4254}, "\u2205": {"f1-score": 0.6882048156049985, "precision": 0.9633105802047781, "recall": 0.5353247984826932, "support": 2109}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 297}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 192}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 187}, "\u2423": {"f1-score": 0.18556701030927836, "precision": 0.7769784172661871, "recall": 0.10536585365853658, "support": 1025}},
  "ppcr": 0.3603667136812412
}
```
</details>
