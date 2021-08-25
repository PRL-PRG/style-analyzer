# Train report for javascript / file:///tmp/top-repos-quality-repos-7q7ag0s6/listenserverc-.git HEAD 1306eb5c17f7baa90a3ac6a59110c2ff90b62872

### Classification report

PPCR: 0.355

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.992| 0.998| 0.443| 0.995| 0.613| 2877| 6476| 0.444 |
| `␣` | 0.959| 0.978| 0.393| 0.968| 0.557| 980| 2439| 0.402 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 567| 0.058 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 211| 0.005 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 157| 0.006 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 508| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 203| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 148| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 144| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 96| 0.000 |
| `weighted avg` | 0.975| 0.984| 0.350| 0.979| 0.487| 3892| 10949| 0.355 |
| `macro avg` | 0.195| 0.198| 0.084| 0.196| 0.117| 3892| 10949| 0.355 |
| `micro avg` | 0.984| 0.984| 0.350| 0.984| 0.516| 3892| 10949| 0.355 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3599 |2871 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|1459 |22 |958 |0 |0 |0 |0 |0 |0 |0 |0 |
|534 |0 |33 |0 |0 |0 |0 |0 |0 |0 |0 |
|508 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|210 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|203 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|156 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|148 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|144 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|96 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| web/view/js/chartsLoader.js | 12 |
| app.js | 8 |
| api/models/serverModel.js | 8 |
| api/models/dataModel.js | 8 |
| api/utils/serverCmd.js | 4 |
| test.js | 4 |
| api/models/ipModel.js | 4 |
| api/utils/utils.js | 4 |
| testTCP.js | 3 |
| web/controllers/graphControllers.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19633130539643728, "precision": 0.19513543962213165, "recall": 0.19754655146730224, "support": 3892}, "micro avg": {"f1-score": 0.9838129496402878, "precision": 0.9838129496402878, "recall": 0.9838129496402878, "support": 3892}, "weighted avg": {"f1-score": 0.9794042258370699, "precision": 0.975051760735761, "recall": 0.9838129496402878, "support": 3892}, "\u2205": {"f1-score": 0.9951473136915079, "precision": 0.9923954372623575, "recall": 0.9979144942648592, "support": 2877}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9681657402728651, "precision": 0.958958958958959, "recall": 0.9775510204081632, "support": 980}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 508}, "macro avg": {"f1-score": 0.11701729944861047, "precision": 0.19513543962213165, "recall": 0.0836113143404442, "support": 10949}, "micro avg": {"f1-score": 0.516002964759787, "precision": 0.9838129496402878, "recall": 0.34971230249337837, "support": 10949}, "weighted avg": {"f1-score": 0.48663961633903424, "precision": 0.8005894376300966, "recall": 0.34971230249337837, "support": 10949}, "\u2205": {"f1-score": 0.6128722382324688, "precision": 0.9923954372623575, "recall": 0.4433292155651637, "support": 6476}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 567}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 157}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 148}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 211}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 144}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 203}, "\u2423": {"f1-score": 0.557300756253636, "precision": 0.958958958958959, "recall": 0.3927839278392784, "support": 2439}},
  "ppcr": 0.35546625262581055
}
```
</details>
