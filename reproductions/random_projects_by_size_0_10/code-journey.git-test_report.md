# Test report for javascript / file:///tmp/top-repos-quality-repos-8qoq8a52/code-journey.git HEAD 55fb871653646cd6ded47ab7d52cf37546739203

### Classification report

PPCR: 0.686

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.921| 0.978| 0.883| 0.949| 0.902| 1697| 1879| 0.903 |
| `␣` | 0.894| 0.923| 0.636| 0.908| 0.744| 569| 825| 0.690 |
| `'` | 0.844| 0.839| 0.458| 0.842| 0.594| 168| 308| 0.545 |
| `⏎` | 0.925| 0.544| 0.135| 0.685| 0.236| 68| 274| 0.248 |
| `"` | 0.300| 0.064| 0.021| 0.105| 0.039| 47| 144| 0.326 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 67| 0.269 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 98| 0.143 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 77| 0.117 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 64| 0.141 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 64| 0.109 |
| `micro avg` | 0.908| 0.908| 0.623| 0.908| 0.739| 2606| 3800| 0.686 |
| `weighted avg` | 0.879| 0.908| 0.623| 0.890| 0.674| 2606| 3800| 0.686 |
| `macro avg` | 0.388| 0.335| 0.213| 0.349| 0.251| 2606| 3800| 0.686 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|182 |1660 |37 |0 |0 |0 |0 |0 |0 |0 |0 |
|256 |44 |525 |0 |0 |0 |0 |0 |0 |0 |0 |
|206 |28 |3 |37 |0 |0 |0 |0 |0 |0 |0 |
|140 |19 |1 |0 |141 |7 |0 |0 |0 |0 |0 |
|97 |12 |3 |3 |26 |3 |0 |0 |0 |0 |0 |
|68 |7 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|84 |0 |14 |0 |0 |0 |0 |0 |0 |0 |0 |
|49 |18 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|57 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|55 |7 |2 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.10526315789473684, "precision": 0.3, "recall": 0.06382978723404255, "support": 47}, "\u0027": {"f1-score": 0.8417910447761194, "precision": 0.844311377245509, "recall": 0.8392857142857143, "support": 168}, "macro avg": {"f1-score": 0.3489386412562062, "precision": 0.38848882395998496, "recall": 0.3348101319743864, "support": 2606}, "micro avg": {"f1-score": 0.9079048349961627, "precision": 0.9079048349961627, "recall": 0.9079048349961627, "support": 2606}, "weighted avg": {"f1-score": 0.8902424725302024, "precision": 0.8791326338166238, "recall": 0.9079048349961627, "support": 2606}, "\u2205": {"f1-score": 0.9488425264361245, "precision": 0.9211986681465039, "recall": 0.9781968179139658, "support": 1697}, "\u23ce": {"f1-score": 0.6851851851851851, "precision": 0.925, "recall": 0.5441176470588235, "support": 68}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.9083044982698962, "precision": 0.8943781942078365, "recall": 0.9226713532513181, "support": 569}},
  "cl_report_full": {"\"": {"f1-score": 0.03896103896103896, "precision": 0.3, "recall": 0.020833333333333332, "support": 144}, "\u0027": {"f1-score": 0.5936842105263158, "precision": 0.844311377245509, "recall": 0.4577922077922078, "support": 308}, "macro avg": {"f1-score": 0.25138689253084323, "precision": 0.38848882395998496, "recall": 0.21334743167346998, "support": 3800}, "micro avg": {"f1-score": 0.738682485170153, "precision": 0.9079048349961627, "recall": 0.6226315789473684, "support": 3800}, "weighted avg": {"f1-score": 0.6740142463127903, "precision": 0.7961821610158848, "recall": 0.6226315789473684, "support": 3800}, "\u2205": {"f1-score": 0.9019288236892148, "precision": 0.9211986681465039, "recall": 0.883448642895157, "support": 1879}, "\u23ce": {"f1-score": 0.2356687898089172, "precision": 0.925, "recall": 0.13503649635036497, "support": 274}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 67}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}, "\u2423": {"f1-score": 0.7436260623229461, "precision": 0.8943781942078365, "recall": 0.6363636363636364, "support": 825}},
  "ppcr": 0.6857894736842105
}
```
</details>