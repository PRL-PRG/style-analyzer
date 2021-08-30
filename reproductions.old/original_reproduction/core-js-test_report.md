# Test report for javascript / file:///tmp/top-repos-quality-repos-fp6ay4ue/core-js HEAD 4a85fe5f9678296bc9ffd5cfc44b32d34b18e52f

### Classification report

PPCR: 0.993

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.997| 0.995| 0.991| 0.990| 36740| 36803| 0.998 |
| `␣` | 0.986| 0.967| 0.956| 0.976| 0.971| 15499| 15670| 0.989 |
| `'` | 0.996| 0.992| 0.991| 0.994| 0.993| 6839| 6842| 1.000 |
| `⏎` | 0.942| 0.972| 0.941| 0.956| 0.941| 4423| 4568| 0.968 |
| `⏎␣⁺␣⁺` | 0.968| 0.942| 0.930| 0.955| 0.948| 868| 880| 0.986 |
| `⏎␣⁻␣⁻` | 0.980| 0.970| 0.965| 0.975| 0.973| 858| 862| 0.995 |
| `⏎⏎` | 0.862| 0.568| 0.516| 0.685| 0.645| 581| 640| 0.908 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 28| 1.000 |
| `weighted avg` | 0.981| 0.982| 0.975| 0.981| 0.978| 65836| 66293| 0.993 |
| `macro avg` | 0.840| 0.801| 0.787| 0.816| 0.808| 65836| 66293| 0.993 |
| `micro avg` | 0.982| 0.982| 0.975| 0.982| 0.979| 65836| 66293| 0.993 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|63 |36615 |124 |0 |1 |0 |0 |0 |0 |
|171 |451 |14981 |2 |15 |27 |14 |9 |0 |
|3 |46 |10 |6783 |0 |0 |0 |0 |0 |
|145 |54 |25 |0 |4297 |0 |3 |44 |0 |
|12 |10 |36 |0 |4 |818 |0 |0 |0 |
|4 |2 |23 |0 |1 |0 |832 |0 |0 |
|59 |6 |0 |0 |245 |0 |0 |330 |0 |
|0 |0 |0 |28 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u0027": {"f1-score": 0.9937005566949898, "precision": 0.9955966534566271, "recall": 0.9918116683725691, "support": 6839}, "macro avg": {"f1-score": 0.8164025187535088, "precision": 0.8396623652196054, "recall": 0.8008224915891367, "support": 65836}, "micro avg": {"f1-score": 0.9820766753751746, "precision": 0.9820766753751746, "recall": 0.9820766753751746, "support": 65836}, "weighted avg": {"f1-score": 0.9814024144550657, "precision": 0.9813813644087964, "recall": 0.9820766753751746, "support": 65836}, "\u2205": {"f1-score": 0.9906119798712191, "precision": 0.9846977194492255, "recall": 0.996597713663582, "support": 36740}, "\u23ce": {"f1-score": 0.9563765858001335, "precision": 0.9417050186280955, "recall": 0.9715125480443139, "support": 4423}, "\u23ce\u23ce": {"f1-score": 0.6846473029045642, "precision": 0.8616187989556136, "recall": 0.5679862306368331, "support": 581}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9550496205487449, "precision": 0.9680473372781065, "recall": 0.9423963133640553, "support": 868}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9748096074985355, "precision": 0.9799764428739693, "recall": 0.9696969696969697, "support": 858}, "\u2423": {"f1-score": 0.9760244967098833, "precision": 0.9856569511152049, "recall": 0.9665784889347699, "support": 15499}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u0027": {"f1-score": 0.9934822409373855, "precision": 0.9955966534566271, "recall": 0.9913767904121602, "support": 6842}, "macro avg": {"f1-score": 0.8076444514789083, "precision": 0.8396623652196054, "recall": 0.7866676336238437, "support": 66293}, "micro avg": {"f1-score": 0.9786799264355289, "precision": 0.9820766753751746, "recall": 0.9753065934563226, "support": 66293}, "weighted avg": {"f1-score": 0.977759489445069, "precision": 0.9812003198347061, "recall": 0.9753065934563226, "support": 66293}, "\u2205": {"f1-score": 0.9897684728398233, "precision": 0.9846977194492255, "recall": 0.9948917207836318, "support": 36803}, "\u23ce": {"f1-score": 0.9411893549446939, "precision": 0.9417050186280955, "recall": 0.9406742556917689, "support": 4568}, "\u23ce\u23ce": {"f1-score": 0.6451612903225806, "precision": 0.8616187989556136, "recall": 0.515625, "support": 640}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9484057971014492, "precision": 0.9680473372781065, "recall": 0.9295454545454546, "support": 880}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.972530683810637, "precision": 0.9799764428739693, "recall": 0.9651972157772621, "support": 862}, "\u2423": {"f1-score": 0.9706177718746963, "precision": 0.9856569511152049, "recall": 0.9560306317804722, "support": 15670}},
  "ppcr": 0.9931063611542696
}
```
</details>