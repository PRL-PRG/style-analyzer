# Train report for javascript / file:///tmp/top-repos-quality-repos-0xia6ql_/learnvuejs.git HEAD 714a4570c202aa412e8a5dbe044aed6271cdb685

### Classification report

PPCR: 0.774

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.992| 0.935| 0.991| 0.962| 33935| 36011| 0.942 |
| `␣` | 0.963| 0.989| 0.884| 0.976| 0.922| 21804| 24406| 0.893 |
| `⏎␣⁻␣⁻` | 0.934| 0.968| 0.874| 0.951| 0.903| 2942| 3259| 0.903 |
| `⏎` | 0.963| 0.777| 0.232| 0.860| 0.374| 1736| 5806| 0.299 |
| `'` | 0.991| 0.996| 0.120| 0.994| 0.215| 462| 3821| 0.121 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 311| 3399| 0.091 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 146| 1348| 0.108 |
| `"` | 0.983| 0.967| 0.085| 0.975| 0.156| 122| 1390| 0.088 |
| `macro avg` | 0.728| 0.711| 0.391| 0.718| 0.441| 61458| 79440| 0.774 |
| `micro avg` | 0.977| 0.977| 0.756| 0.977| 0.852| 61458| 79440| 0.774 |
| `weighted avg` | 0.969| 0.977| 0.756| 0.973| 0.797| 61458| 79440| 0.774 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2076 |33677 |241 |2 |0 |0 |15 |0 |0 |
|2602 |50 |21569 |0 |0 |0 |185 |0 |0 |
|4070 |15 |372 |1349 |0 |0 |0 |0 |0 |
|3359 |0 |0 |0 |460 |0 |0 |2 |0 |
|3088 |214 |83 |14 |0 |0 |0 |0 |0 |
|317 |66 |20 |9 |0 |0 |2847 |0 |0 |
|1268 |0 |0 |0 |4 |0 |0 |118 |0 |
|1202 |1 |118 |27 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| js/vue.js | 1133 |
| Day8/03_vuex/src/store/index.js | 27 |
| Day6/03_webpack/webpack1.config.js | 16 |
| Day7/01_vueCli2Test/build/webpack.prod.conf.js | 13 |
| Day8/03_vuex/build/webpack.prod.conf.js | 13 |
| Day7/02_vuecli2test/build/webpack.prod.conf.js | 13 |
| Day8/01_tabbar/build/webpack.prod.conf.js | 13 |
| Day8/04_axios/build/webpack.prod.conf.js | 13 |
| Day7/04_learnvuerouter/build/webpack.prod.conf.js | 13 |
| Day7/04_learnvuerouter/src/router/index.js | 12 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9752066115702478, "precision": 0.9833333333333333, "recall": 0.9672131147540983, "support": 122}, "\u0027": {"f1-score": 0.9935205183585313, "precision": 0.9913793103448276, "recall": 0.9956709956709957, "support": 462}, "macro avg": {"f1-score": 0.7183072506629684, "precision": 0.728070152148524, "recall": 0.7111607844602833, "support": 61458}, "micro avg": {"f1-score": 0.9766019069933939, "precision": 0.9766019069933939, "recall": 0.9766019069933939, "support": 61458}, "weighted avg": {"f1-score": 0.9726685015717902, "precision": 0.9694528938196514, "recall": 0.9766019069933939, "support": 61458}, "\u2205": {"f1-score": 0.9911121575090497, "precision": 0.9898304088410781, "recall": 0.9923972299985266, "support": 33935}, "\u23ce": {"f1-score": 0.8600573796620976, "precision": 0.9628836545324768, "recall": 0.777073732718894, "support": 1736}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 311}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9507430288862915, "precision": 0.9343616672136528, "recall": 0.9677090414683889, "support": 2942}, "\u2423": {"f1-score": 0.9758183093175289, "precision": 0.9627728429228228, "recall": 0.9892221610713631, "support": 21804}},
  "cl_report_full": {"\"": {"f1-score": 0.1562913907284768, "precision": 0.9833333333333333, "recall": 0.08489208633093526, "support": 1390}, "\u0027": {"f1-score": 0.2147024504084014, "precision": 0.9913793103448276, "recall": 0.12038733315885894, "support": 3821}, "macro avg": {"f1-score": 0.4414511823982207, "precision": 0.728070152148524, "recall": 0.3912688355883466, "support": 79440}, "micro avg": {"f1-score": 0.8519638319919375, "precision": 0.9766019069933939, "recall": 0.7555387713997986, "support": 79440}, "weighted avg": {"f1-score": 0.7965606693491684, "precision": 0.9180853185671731, "recall": 0.7555387713997986, "support": 79440}, "\u2205": {"f1-score": 0.9617328726047348, "precision": 0.9898304088410781, "recall": 0.9351864708005887, "support": 36011}, "\u23ce": {"f1-score": 0.37435826280005546, "precision": 0.9628836545324768, "recall": 0.23234584912159834, "support": 5806}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1348}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3399}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9029495718363464, "precision": 0.9343616672136528, "recall": 0.8735808530223995, "support": 3259}, "\u2423": {"f1-score": 0.9215749108077507, "precision": 0.9627728429228228, "recall": 0.883758092272392, "support": 24406}},
  "ppcr": 0.7736404833836859
}
```
</details>
