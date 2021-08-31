# Train report for javascript / file:///tmp/top-repos-quality-repos-jlznd02u/nylas-mail.git HEAD e16cb1982e944ae7edb69b1abef1436dd16b442d

### Classification report

PPCR: 0.872

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 0.995| 0.963| 0.992| 0.976| 184436| 190402| 0.969 |
| `␣` | 0.980| 0.976| 0.910| 0.978| 0.944| 76823| 82418| 0.932 |
| `⏎␣⁻␣⁻` | 0.945| 0.968| 0.911| 0.956| 0.927| 10877| 11560| 0.941 |
| `⏎` | 0.955| 0.892| 0.416| 0.922| 0.579| 9961| 21363| 0.466 |
| `⏎␣⁺␣⁺` | 0.944| 0.951| 0.790| 0.948| 0.860| 9686| 11661| 0.831 |
| `'` | 0.969| 1.000| 0.461| 0.984| 0.624| 8770| 19036| 0.461 |
| `⏎⏎` | 0.941| 0.901| 0.446| 0.921| 0.605| 3349| 6772| 0.495 |
| `"` | 1.000| 0.485| 0.046| 0.654| 0.087| 548| 5820| 0.094 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 121| 124| 0.976 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 60| 121| 0.496 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 83| 0.590 |
| `weighted avg` | 0.981| 0.982| 0.856| 0.981| 0.897| 304680| 349360| 0.872 |
| `micro avg` | 0.982| 0.982| 0.856| 0.982| 0.915| 304680| 349360| 0.872 |
| `macro avg` | 0.702| 0.652| 0.449| 0.669| 0.509| 304680| 349360| 0.872 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎⏎⏎| ⏎⏎␣⁺␣⁺| ⏎⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5966 |183446 |684 |4 |0 |42 |260 |0 |0 |0 |0 |0 |
|5595 |846 |75015 |215 |0 |380 |330 |37 |0 |0 |0 |0 |
|11402 |521 |446 |8881 |0 |4 |1 |108 |0 |0 |0 |0 |
|10266 |0 |0 |0 |8770 |0 |0 |0 |0 |0 |0 |0 |
|1975 |288 |175 |7 |0 |9216 |0 |0 |0 |0 |0 |0 |
|683 |296 |44 |4 |0 |6 |10527 |0 |0 |0 |0 |0 |
|3423 |7 |137 |186 |0 |0 |1 |3018 |0 |0 |0 |0 |
|5272 |0 |0 |0 |282 |0 |0 |0 |266 |0 |0 |0 |
|61 |1 |13 |3 |0 |0 |0 |43 |0 |0 |0 |0 |
|3 |1 |0 |0 |0 |119 |0 |1 |0 |0 |0 |0 |
|34 |27 |0 |1 |0 |0 |21 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| packages/client-app/spec/n1-spec-runner/jasmine.js | 150 |
| packages/client-app/internal_packages/thread-search/spec/search-query-parser-spec.es6 | 94 |
| packages/client-app/src/flux/stores/database-store.es6 | 70 |
| packages/client-app/internal_packages/thread-search/lib/search-query-parser.es6 | 67 |
| packages/client-app/src/browser/application.es6 | 60 |
| packages/client-app/src/flux/attributes/matcher.es6 | 59 |
| packages/client-app/src/flux/stores/file-download-store.es6 | 59 |
| packages/client-app/src/flux/stores/thread-list-actions-store.es6 | 59 |
| packages/client-private-plugins/packages/composer-mail-merge/spec/selection-state-reducers-spec.es6 | 58 |
| packages/client-app/src/flux/stores/database-transaction.es6 | 57 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.6535626535626535, "precision": 1.0, "recall": 0.4854014598540146, "support": 548}, "\u0027": {"f1-score": 0.9841768600605992, "precision": 0.9688466637207247, "recall": 1.0, "support": 8770}, "macro avg": {"f1-score": 0.6686099314260432, "precision": 0.7020916920742881, "recall": 0.651685379152064, "support": 304680}, "micro avg": {"f1-score": 0.9818137061835368, "precision": 0.9818137061835368, "recall": 0.9818137061835368, "support": 304680}, "weighted avg": {"f1-score": 0.9812085178601495, "precision": 0.9810400574912679, "recall": 0.9818137061835368, "support": 304680}, "\u2205": {"f1-score": 0.9919512043453222, "precision": 0.9892845394293357, "recall": 0.9946322843696458, "support": 184436}, "\u23ce": {"f1-score": 0.9221264666182121, "precision": 0.9548435652080421, "recall": 0.891577150888465, "support": 9961}, "\u23ce\u23ce": {"f1-score": 0.9206833435021354, "precision": 0.941066417212348, "recall": 0.9011645267243953, "support": 3349}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 60}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9475145221816687, "precision": 0.9435855431555237, "recall": 0.9514763576295685, "support": 9686}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9562610709905981, "precision": 0.9449730700179533, "recall": 0.9678220097453342, "support": 10877}, "\u2423": {"f1-score": 0.9784331244252854, "precision": 0.9804088140732415, "recall": 0.9764653814612811, "support": 76823}},
  "cl_report_full": {"\"": {"f1-score": 0.08741373644429838, "precision": 1.0, "recall": 0.04570446735395189, "support": 5820}, "\u0027": {"f1-score": 0.6244659641127883, "precision": 0.9688466637207247, "recall": 0.460706030678714, "support": 19036}, "macro avg": {"f1-score": 0.5094419529144851, "precision": 0.7020916920742881, "recall": 0.4493089896009996, "support": 349360}, "micro avg": {"f1-score": 0.9147422176013699, "precision": 0.9818137061835368, "recall": 0.8562485688115411, "support": 349360}, "weighted avg": {"f1-score": 0.8967600687087541, "precision": 0.9792944761115928, "recall": 0.8562485688115411, "support": 349360}, "\u2205": {"f1-score": 0.9762049835699177, "precision": 0.9892845394293357, "recall": 0.9634667703070345, "support": 190402}, "\u23ce": {"f1-score": 0.5792460213931646, "precision": 0.9548435652080421, "recall": 0.41571876609090486, "support": 21363}, "\u23ce\u23ce": {"f1-score": 0.6048702274777031, "precision": 0.941066417212348, "recall": 0.4456585942114589, "support": 6772}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 124}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8601829382116857, "precision": 0.9435855431555237, "recall": 0.7903267301260612, "support": 11661}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9274889867841409, "precision": 0.9449730700179533, "recall": 0.9106401384083045, "support": 11560}, "\u2423": {"f1-score": 0.9439886240656381, "precision": 0.9804088140732415, "recall": 0.9101773884345653, "support": 82418}},
  "ppcr": 0.8721089993130295
}
```
</details>
