# Train report for javascript / file:///tmp/top-repos-quality-repos-nkl7zbzk/atom-script.git HEAD d36a9394c56eb0900a4e2d17589f69c1545d369b

### Classification report

PPCR: 0.713

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 0.998| 0.880| 0.994| 0.932| 7966| 9032| 0.882 |
| `␣` | 0.990| 0.846| 0.344| 0.912| 0.511| 1453| 3570| 0.407 |
| `⏎␣⁻␣⁻` | 0.825| 0.982| 0.929| 0.897| 0.874| 450| 476| 0.945 |
| `⏎␣⁺␣⁺` | 0.857| 0.984| 0.914| 0.916| 0.884| 443| 477| 0.929 |
| `⏎` | 0.964| 0.956| 0.344| 0.960| 0.508| 366| 1016| 0.360 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 32| 32| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 424| 0.021 |
| `macro avg` | 0.803| 0.824| 0.630| 0.811| 0.673| 10719| 15027| 0.713 |
| `weighted avg` | 0.975| 0.974| 0.695| 0.973| 0.774| 10719| 15027| 0.713 |
| `micro avg` | 0.974| 0.974| 0.695| 0.974| 0.811| 10719| 15027| 0.713 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1066 |7952 |6 |0 |0 |1 |7 |0 |
|2117 |67 |1229 |0 |1 |70 |86 |0 |
|0 |0 |0 |32 |0 |0 |0 |0 |
|650 |13 |2 |0 |350 |0 |1 |0 |
|34 |4 |3 |0 |0 |436 |0 |0 |
|26 |1 |0 |0 |5 |2 |442 |0 |
|415 |0 |2 |0 |7 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/script-options-view.js | 35 |
| lib/script-profile-run-view.js | 32 |
| lib/script.js | 26 |
| lib/script-view.js | 25 |
| lib/runner.js | 20 |
| spec/code-context-spec.js | 16 |
| spec/code-context-builder-spec.js | 15 |
| lib/code-context.js | 11 |
| lib/runtime.js | 11 |
| lib/command-context.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 32}, "macro avg": {"f1-score": 0.8112300883673625, "precision": 0.8034789502256997, "recall": 0.823826250363588, "support": 10719}, "micro avg": {"f1-score": 0.9740647448456012, "precision": 0.9740647448456012, "recall": 0.9740647448456012, "support": 10719}, "weighted avg": {"f1-score": 0.9734680094082361, "precision": 0.9753692170773891, "recall": 0.9740647448456012, "support": 10719}, "\u2205": {"f1-score": 0.9938136599387616, "precision": 0.9894239143959189, "recall": 0.9982425307557118, "support": 7966}, "\u23ce": {"f1-score": 0.9602194787379972, "precision": 0.9641873278236914, "recall": 0.9562841530054644, "support": 366}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9159663865546218, "precision": 0.8565815324165029, "recall": 0.9841986455981941, "support": 443}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.896551724137931, "precision": 0.8246268656716418, "recall": 0.9822222222222222, "support": 450}, "\u2423": {"f1-score": 0.9120593692022262, "precision": 0.9895330112721417, "recall": 0.8458362009635237, "support": 1453}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 32}, "macro avg": {"f1-score": 0.6725810383817425, "precision": 0.8034789502256997, "recall": 0.630255513889537, "support": 15027}, "micro avg": {"f1-score": 0.8110774489241047, "precision": 0.9740647448456012, "recall": 0.6948159978704997, "support": 15027}, "weighted avg": {"f1-score": 0.7735744590731581, "precision": 0.9504116423209378, "recall": 0.6948159978704997, "support": 15027}, "\u2205": {"f1-score": 0.9317476126310857, "precision": 0.9894239143959189, "recall": 0.8804251550044287, "support": 9032}, "\u23ce": {"f1-score": 0.5076142131979695, "precision": 0.9641873278236914, "recall": 0.34448818897637795, "support": 1016}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 424}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8843813387423934, "precision": 0.8565815324165029, "recall": 0.9140461215932913, "support": 477}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8735177865612648, "precision": 0.8246268656716418, "recall": 0.9285714285714286, "support": 476}, "\u2423": {"f1-score": 0.5108063175394846, "precision": 0.9895330112721417, "recall": 0.3442577030812325, "support": 3570}},
  "ppcr": 0.7133160311439409
}
```
</details>
