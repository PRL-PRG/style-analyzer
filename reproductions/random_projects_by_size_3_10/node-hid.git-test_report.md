# Test report for javascript / file:///tmp/top-repos-quality-repos-52ykii9r/node-hid.git HEAD 2cdb2eb5060f1c861dd77d69575a505904cb400a

### Classification report

PPCR: 0.438

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.997| 0.694| 0.985| 0.810| 364| 523| 0.696 |
| `␣` | 0.974| 0.809| 0.158| 0.884| 0.271| 47| 241| 0.195 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 24| 0.042 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 38| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 55| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 23| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 20| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 16| 0.000 |
| `weighted avg` | 0.971| 0.973| 0.427| 0.971| 0.520| 412| 940| 0.438 |
| `micro avg` | 0.973| 0.973| 0.427| 0.973| 0.593| 412| 940| 0.438 |
| `macro avg` | 0.243| 0.226| 0.106| 0.234| 0.135| 412| 940| 0.438 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|159 |363 |1 |0 |0 |0 |0 |0 |0 |
|194 |9 |38 |0 |0 |0 |0 |0 |0 |
|55 |0 |0 |0 |0 |0 |0 |0 |0 |
|23 |1 |0 |0 |0 |0 |0 |0 |0 |
|23 |0 |0 |0 |0 |0 |0 |0 |0 |
|20 |0 |0 |0 |0 |0 |0 |0 |0 |
|16 |0 |0 |0 |0 |0 |0 |0 |0 |
|38 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.23359944463727872, "precision": 0.24344366536055545, "recall": 0.22572042319382746, "support": 412}, "micro avg": {"f1-score": 0.9733009708737864, "precision": 0.9733009708737864, "recall": 0.9733009708737864, "support": 412}, "weighted avg": {"f1-score": 0.9711214754855212, "precision": 0.9709615501411226, "recall": 0.9733009708737864, "support": 412}, "\u2205": {"f1-score": 0.9850746268656717, "precision": 0.9731903485254692, "recall": 0.9972527472527473, "support": 364}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8837209302325582, "precision": 0.9743589743589743, "recall": 0.8085106382978723, "support": 47}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "macro avg": {"f1-score": 0.13521205357142857, "precision": 0.24344366536055545, "recall": 0.10646862578643797, "support": 940}, "micro avg": {"f1-score": 0.5931952662721894, "precision": 0.9733009708737864, "recall": 0.42659574468085104, "support": 940}, "weighted avg": {"f1-score": 0.5204089095744682, "precision": 0.7912756011695034, "recall": 0.42659574468085104, "support": 940}, "\u2205": {"f1-score": 0.8102678571428572, "precision": 0.9731903485254692, "recall": 0.6940726577437859, "support": 523}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.2714285714285714, "precision": 0.9743589743589743, "recall": 0.15767634854771784, "support": 241}},
  "ppcr": 0.43829787234042555
}
```
</details>
