# Train report for javascript / file:///tmp/top-repos-quality-repos-98m30i6r/couas.git HEAD f60fb5f6c3eeea4f7934f72e619abb7fb44ece6f

### Classification report

PPCR: 0.615

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.990| 0.871| 0.990| 0.926| 4492| 5109| 0.879 |
| `␣` | 0.966| 0.967| 0.438| 0.966| 0.603| 1105| 2436| 0.454 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.931| 0.941| 0.897| 0.936| 0.914| 288| 302| 0.954 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 306| 0.023 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 152| 0.007 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 670| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 527| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 84| 0.000 |
| `macro avg` | 0.361| 0.362| 0.276| 0.362| 0.305| 5893| 9586| 0.615 |
| `micro avg` | 0.982| 0.982| 0.604| 0.982| 0.748| 5893| 9586| 0.615 |
| `weighted avg` | 0.981| 0.982| 0.604| 0.982| 0.676| 5893| 9586| 0.615 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|617 |4449 |31 |0 |0 |0 |12 |0 |0 |
|1331 |29 |1068 |0 |0 |0 |8 |0 |0 |
|670 |0 |0 |0 |0 |0 |0 |0 |0 |
|527 |0 |0 |0 |0 |0 |0 |0 |0 |
|299 |1 |6 |0 |0 |0 |0 |0 |0 |
|14 |17 |0 |0 |0 |0 |271 |0 |0 |
|151 |0 |1 |0 |0 |0 |0 |0 |0 |
|84 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Monitor/monitor-app/module/menu.js | 25 |
| Monitor/monitor/drone.js | 16 |
| Monitor/monitor/drone_cluster.js | 16 |
| Monitor/monitor-app/module/map.js | 16 |
| Monitor/monitor-app/module/window.js | 6 |
| Monitor/monitor-app/module/console.js | 6 |
| Monitor/user-module/mavc_example.js | 4 |
| Monitor/monitor/lib/transform.js | 4 |
| Monitor/user-module/mavc_delay_tester.js | 4 |
| Monitor/user-module/mavc_collision.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3615202580959091, "precision": 0.3608074617490189, "recall": 0.3622394357327948, "support": 5893}, "micro avg": {"f1-score": 0.9821822501272697, "precision": 0.9821822501272697, "recall": 0.9821822501272697, "support": 5893}, "weighted avg": {"f1-score": 0.9815260210887219, "precision": 0.9808726215103758, "recall": 0.9821822501272697, "support": 5893}, "\u2205": {"f1-score": 0.9899866488651534, "precision": 0.9895462633451957, "recall": 0.9904274265360641, "support": 4492}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9360967184801382, "precision": 0.9312714776632303, "recall": 0.9409722222222222, "support": 288}, "\u2423": {"f1-score": 0.966078697421981, "precision": 0.9656419529837251, "recall": 0.9665158371040724, "support": 1105}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 527}, "macro avg": {"f1-score": 0.30542978200139875, "precision": 0.3608074617490189, "recall": 0.2758238556739687, "support": 9586}, "micro avg": {"f1-score": 0.7478519284191485, "precision": 0.9821822501272697, "recall": 0.603797204256207, "support": 9586}, "weighted avg": {"f1-score": 0.6757765441144465, "precision": 0.8021218071305294, "recall": 0.603797204256207, "support": 9586}, "\u2205": {"f1-score": 0.9263925039042165, "precision": 0.9895462633451957, "recall": 0.8708162066940693, "support": 5109}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 670}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 306}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9139966273187183, "precision": 0.9312714776632303, "recall": 0.8973509933774835, "support": 302}, "\u2423": {"f1-score": 0.6030491247882553, "precision": 0.9656419529837251, "recall": 0.43842364532019706, "support": 2436}},
  "ppcr": 0.6147506780721886
}
```
</details>
