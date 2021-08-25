# Test report for javascript / file:///tmp/top-repos-quality-repos-xl1fos8m/workspace.git HEAD 8725ce0589c1b687d498be9f869abf71805d4c67

### Classification report

PPCR: 0.669

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.760| 1.000| 0.941| 0.864| 0.841| 1362| 1447| 0.941 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 243| 435| 0.559 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 67| 86| 0.779 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 62| 176| 0.352 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 58| 86| 0.674 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 186| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 264| 0.000 |
| `macro avg` | 0.109| 0.143| 0.134| 0.123| 0.120| 1792| 2680| 0.669 |
| `weighted avg` | 0.578| 0.760| 0.508| 0.656| 0.454| 1792| 2680| 0.669 |
| `micro avg` | 0.760| 0.760| 0.508| 0.760| 0.609| 1792| 2680| 0.669 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|85 |1362 |0 |0 |0 |0 |0 |0 |
|192 |243 |0 |0 |0 |0 |0 |0 |
|264 |0 |0 |0 |0 |0 |0 |0 |
|114 |62 |0 |0 |0 |0 |0 |0 |
|186 |0 |0 |0 |0 |0 |0 |0 |
|19 |67 |0 |0 |0 |0 |0 |0 |
|28 |58 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.12338074100914939, "precision": 0.10857780612244898, "recall": 0.14285714285714285, "support": 1792}, "micro avg": {"f1-score": 0.760044642857143, "precision": 0.7600446428571429, "recall": 0.7600446428571429, "support": 1792}, "weighted avg": {"f1-score": 0.6564240986502401, "precision": 0.5776678591358418, "recall": 0.7600446428571429, "support": 1792}, "\u2205": {"f1-score": 0.8636651870640457, "precision": 0.7600446428571429, "recall": 1.0, "support": 1362}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 67}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 243}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 186}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 264}, "macro avg": {"f1-score": 0.12014290124818065, "precision": 0.10857780612244898, "recall": 0.1344653963866127, "support": 2680}, "micro avg": {"f1-score": 0.60912343470483, "precision": 0.7600446428571429, "recall": 0.5082089552238805, "support": 2680}, "weighted avg": {"f1-score": 0.45407740550105297, "precision": 0.41036738739339024, "recall": 0.5082089552238805, "support": 2680}, "\u2205": {"f1-score": 0.8410003087372646, "precision": 0.7600446428571429, "recall": 0.9412577747062889, "support": 1447}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 176}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 435}},
  "ppcr": 0.6686567164179105
}
```
</details>
