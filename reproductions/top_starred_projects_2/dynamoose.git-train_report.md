# Train report for javascript / file:///tmp/top-repos-quality-repos-hp7_ylut/dynamoose.git HEAD 1c933163051248846b1723678c82dffa74a9cb6a

### Classification report

PPCR: 0.953

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 0.998| 0.994| 0.989| 0.987| 42301| 42473| 0.996 |
| `"` | 1.000| 1.000| 0.986| 1.000| 0.993| 25035| 25384| 0.986 |
| `␣` | 0.984| 0.981| 0.915| 0.983| 0.948| 16877| 18101| 0.932 |
| `⏎⇥⁺` | 0.960| 0.930| 0.749| 0.945| 0.841| 2059| 2557| 0.805 |
| `⏎⇥⁻` | 1.000| 0.805| 0.626| 0.892| 0.770| 2038| 2623| 0.777 |
| `⏎` | 0.916| 0.860| 0.467| 0.887| 0.619| 1955| 3597| 0.544 |
| `⏎⏎` | 0.997| 0.818| 0.818| 0.899| 0.899| 863| 863| 1.000 |
| `micro avg` | 0.985| 0.985| 0.939| 0.985| 0.961| 91128| 95598| 0.953 |
| `macro avg` | 0.977| 0.913| 0.794| 0.942| 0.865| 91128| 95598| 0.953 |
| `weighted avg` | 0.985| 0.985| 0.939| 0.985| 0.957| 91128| 95598| 0.953 |

### Confusion matrix

|refusal|  ∅| "| ␣| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|172 |42225 |0 |3 |0 |73 |0 |0 |
|349 |0 |25035 |0 |0 |0 |0 |0 |
|1224 |309 |0 |16560 |3 |5 |0 |0 |
|1642 |19 |0 |252 |1681 |1 |0 |2 |
|498 |138 |0 |7 |0 |1914 |0 |0 |
|585 |397 |0 |0 |0 |0 |1641 |0 |
|0 |6 |0 |0 |151 |0 |0 |706 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/unit/Model.js | 412 |
| test/unit/Schema.js | 234 |
| test/unit/Condition.js | 106 |
| publish/index.js | 58 |
| test/unit/Query.js | 57 |
| docs/docusaurus.config.js | 39 |
| docs/src/plugins/remark-npm2yarn.js | 39 |
| test/unit/Logger.js | 39 |
| test/unit/utils/dynamoose/index_changes.js | 37 |
| test/unit/Populate.js | 34 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 25035}, "macro avg": {"f1-score": 0.9420604254722756, "precision": 0.9768389481802696, "recall": 0.9131602943203925, "support": 91128}, "micro avg": {"f1-score": 0.9850100956895795, "precision": 0.9850100956895795, "recall": 0.9850100956895795, "support": 91128}, "weighted avg": {"f1-score": 0.9846375585785159, "precision": 0.9850321476270065, "recall": 0.9850100956895795, "support": 91128}, "\u2205": {"f1-score": 0.9889337783242578, "precision": 0.9798347797837286, "recall": 0.9982033521666155, "support": 42301}, "\u23ce": {"f1-score": 0.8870712401055408, "precision": 0.9160762942779291, "recall": 0.859846547314578, "support": 1955}, "\u23ce\u21e5\u207a": {"f1-score": 0.9447186574531096, "precision": 0.9603612644254892, "recall": 0.9295774647887324, "support": 2059}, "\u23ce\u21e5\u207b": {"f1-score": 0.8920902419135635, "precision": 1.0, "recall": 0.8052011776251227, "support": 2038}, "\u23ce\u23ce": {"f1-score": 0.8987905792488861, "precision": 0.9971751412429378, "recall": 0.8180764774044033, "support": 863}, "\u2423": {"f1-score": 0.9828184812605715, "precision": 0.9844251575318036, "recall": 0.9812170409432956, "support": 16877}},
  "cl_report_full": {"\"": {"f1-score": 0.9930780063071462, "precision": 1.0, "recall": 0.9862511818468327, "support": 25384}, "macro avg": {"f1-score": 0.86530428626012, "precision": 0.9768389481802696, "recall": 0.7935488691734578, "support": 95598}, "micro avg": {"f1-score": 0.9614301168557138, "precision": 0.9850100956895795, "recall": 0.9389526977551832, "support": 95598}, "weighted avg": {"f1-score": 0.9567714069556644, "precision": 0.9838483515218291, "recall": 0.9389526977551832, "support": 95598}, "\u2205": {"f1-score": 0.986945902041675, "precision": 0.9798347797837286, "recall": 0.9941609963977115, "support": 42473}, "\u23ce": {"f1-score": 0.6189248895434463, "precision": 0.9160762942779291, "recall": 0.46733388935223796, "support": 3597}, "\u23ce\u21e5\u207a": {"f1-score": 0.8413186813186813, "precision": 0.9603612644254892, "recall": 0.7485334376222136, "support": 2557}, "\u23ce\u21e5\u207b": {"f1-score": 0.7696998123827392, "precision": 1.0, "recall": 0.6256195196340069, "support": 2623}, "\u23ce\u23ce": {"f1-score": 0.8987905792488861, "precision": 0.9971751412429378, "recall": 0.8180764774044033, "support": 863}, "\u2423": {"f1-score": 0.9483721329782665, "precision": 0.9844251575318036, "recall": 0.9148665819567979, "support": 18101}},
  "ppcr": 0.9532416996171468
}
```
</details>
