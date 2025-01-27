# Test report for javascript / file:///tmp/top-repos-quality-repos-9g95lo7u/advancedreact.git HEAD 636dd6fdc1019e76ca672f40c5b10ee4b257b8cf

### Classification report

PPCR: 0.934

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 0.989| 0.947| 0.984| 0.963| 6834| 7141| 0.957 |
| `␣` | 0.951| 0.967| 0.910| 0.959| 0.930| 2938| 3122| 0.941 |
| `'` | 0.984| 1.000| 1.000| 0.992| 0.992| 778| 778| 1.000 |
| `⏎` | 0.919| 0.932| 0.781| 0.926| 0.844| 607| 725| 0.837 |
| `⏎␣⁻␣⁻` | 0.964| 0.749| 0.679| 0.843| 0.797| 283| 312| 0.907 |
| `"` | 1.000| 0.990| 0.990| 0.995| 0.995| 202| 202| 1.000 |
| `⏎␣⁺␣⁺` | 0.825| 0.594| 0.270| 0.691| 0.407| 143| 315| 0.454 |
| `⏎⏎` | 0.917| 0.697| 0.559| 0.792| 0.695| 142| 177| 0.802 |
| `macro avg` | 0.942| 0.865| 0.767| 0.898| 0.828| 11927| 12772| 0.934 |
| `weighted avg` | 0.967| 0.968| 0.904| 0.967| 0.929| 11927| 12772| 0.934 |
| `micro avg` | 0.968| 0.968| 0.904| 0.968| 0.935| 11927| 12772| 0.934 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|307 |6760 |66 |0 |0 |0 |8 |0 |0 |
|184 |62 |2842 |8 |10 |16 |0 |0 |0 |
|118 |15 |14 |566 |1 |2 |0 |9 |0 |
|0 |0 |0 |0 |778 |0 |0 |0 |0 |
|172 |18 |40 |0 |0 |85 |0 |0 |0 |
|29 |42 |28 |1 |0 |0 |212 |0 |0 |
|35 |2 |0 |41 |0 |0 |0 |99 |0 |
|0 |0 |0 |0 |2 |0 |0 |0 |200 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9950248756218906, "precision": 1.0, "recall": 0.9900990099009901, "support": 202}, "\u0027": {"f1-score": 0.9917144678138942, "precision": 0.9835651074589128, "recall": 1.0, "support": 778}, "macro avg": {"f1-score": 0.8977075906899791, "precision": 0.9422869812207979, "recall": 0.8649694380875207, "support": 11927}, "micro avg": {"f1-score": 0.9677202984824348, "precision": 0.9677202984824348, "recall": 0.9677202984824348, "support": 11927}, "weighted avg": {"f1-score": 0.9666551344453512, "precision": 0.9671093453952436, "recall": 0.9677202984824348, "support": 11927}, "\u2205": {"f1-score": 0.9844899148037574, "precision": 0.9798521524858675, "recall": 0.9891717881182324, "support": 6834}, "\u23ce": {"f1-score": 0.9255928045789042, "precision": 0.9188311688311688, "recall": 0.9324546952224053, "support": 607}, "\u23ce\u23ce": {"f1-score": 0.7919999999999999, "precision": 0.9166666666666666, "recall": 0.6971830985915493, "support": 142}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6910569105691057, "precision": 0.8252427184466019, "recall": 0.5944055944055944, "support": 143}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8429423459244533, "precision": 0.9636363636363636, "recall": 0.7491166077738516, "support": 283}, "\u2423": {"f1-score": 0.9588394062078273, "precision": 0.9505016722408027, "recall": 0.9673247106875426, "support": 2938}},
  "cl_report_full": {"\"": {"f1-score": 0.9950248756218906, "precision": 1.0, "recall": 0.9900990099009901, "support": 202}, "\u0027": {"f1-score": 0.9917144678138942, "precision": 0.9835651074589128, "recall": 1.0, "support": 778}, "macro avg": {"f1-score": 0.8277812719840312, "precision": 0.9422869812207979, "recall": 0.767049897204841, "support": 12772}, "micro avg": {"f1-score": 0.9346127373577878, "precision": 0.9677202984824348, "recall": 0.9036955840901973, "support": 12772}, "weighted avg": {"f1-score": 0.9289219981472565, "precision": 0.9646737155033059, "recall": 0.9036955840901973, "support": 12772}, "\u2205": {"f1-score": 0.9629629629629631, "precision": 0.9798521524858675, "recall": 0.9466461279932783, "support": 7141}, "\u23ce": {"f1-score": 0.8441461595824011, "precision": 0.9188311688311688, "recall": 0.7806896551724138, "support": 725}, "\u23ce\u23ce": {"f1-score": 0.6947368421052631, "precision": 0.9166666666666666, "recall": 0.559322033898305, "support": 177}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.4066985645933014, "precision": 0.8252427184466019, "recall": 0.2698412698412698, "support": 315}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7969924812030076, "precision": 0.9636363636363636, "recall": 0.6794871794871795, "support": 312}, "\u2423": {"f1-score": 0.9299738219895287, "precision": 0.9505016722408027, "recall": 0.9103139013452914, "support": 3122}},
  "ppcr": 0.9338396492326966
}
```
</details>
