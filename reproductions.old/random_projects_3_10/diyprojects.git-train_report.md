# Train report for javascript / file:///tmp/top-repos-quality-repos-v0sn10is/diyprojects.git HEAD ec45e529788a55fbb1d69c2790c6ff3b2db4c267

### Classification report

PPCR: 0.678

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.996| 0.922| 0.991| 0.953| 18912| 20431| 0.926 |
| `␣` | 0.968| 0.962| 0.462| 0.965| 0.625| 4964| 10342| 0.480 |
| `⏎␣⁺␣⁺` | 0.930| 0.930| 0.805| 0.930| 0.863| 827| 955| 0.866 |
| `⏎` | 0.959| 0.838| 0.290| 0.895| 0.445| 786| 2274| 0.346 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 54| 500| 0.108 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 90| 0.089 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 833| 0.007 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1832| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 432| 0.000 |
| `weighted avg` | 0.977| 0.980| 0.664| 0.978| 0.737| 25557| 37689| 0.678 |
| `micro avg` | 0.980| 0.980| 0.664| 0.980| 0.792| 25557| 37689| 0.678 |
| `macro avg` | 0.427| 0.414| 0.275| 0.420| 0.321| 25557| 37689| 0.678 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1519 |18834 |56 |0 |0 |22 |0 |0 |0 |0 |
|5378 |163 |4775 |0 |0 |26 |0 |0 |0 |0 |
|1488 |67 |56 |659 |0 |4 |0 |0 |0 |0 |
|1832 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|128 |40 |14 |4 |0 |769 |0 |0 |0 |0 |
|827 |0 |0 |0 |0 |6 |0 |0 |0 |0 |
|446 |0 |30 |24 |0 |0 |0 |0 |0 |0 |
|432 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|82 |6 |2 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| docs/css/underscore-1.3.1.js | 78 |
| projects/UltimakerDIY/docSource/build/html/_static/underscore-1.3.1.js | 78 |
| docs/css/websupport.js | 61 |
| projects/UltimakerDIY/docSource/build/html/_static/websupport.js | 61 |
| docs/css/searchtools.js | 55 |
| projects/UltimakerDIY/docSource/build/html/_static/searchtools.js | 55 |
| docs/css/doctools.js | 44 |
| projects/UltimakerDIY/docSource/build/html/_static/doctools.js | 44 |
| projects/UltimakerDIY/docSource/build/html/_static/language_data.js | 20 |
| docs/css/language_data.js | 20 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4200297814688685, "precision": 0.42695979818659785, "recall": 0.4140100979699411, "support": 25557}, "micro avg": {"f1-score": 0.9796533239425598, "precision": 0.9796533239425598, "recall": 0.9796533239425598, "support": 25557}, "weighted avg": {"f1-score": 0.9781339564259571, "precision": 0.9769077675473357, "recall": 0.9796533239425598, "support": 25557}, "\u2205": {"f1-score": 0.9906896007574562, "precision": 0.9855572998430141, "recall": 0.9958756345177665, "support": 18912}, "\u23ce": {"f1-score": 0.8947725729803123, "precision": 0.9592430858806404, "recall": 0.8384223918575063, "support": 786}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9298669891172914, "precision": 0.9298669891172914, "recall": 0.9298669891172914, "support": 827}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.9649388703647569, "precision": 0.967970808838435, "recall": 0.9619258662369057, "support": 4964}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 432}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1832}, "macro avg": {"f1-score": 0.3206701240921499, "precision": 0.42695979818659785, "recall": 0.2753974796161388, "support": 37689}, "micro avg": {"f1-score": 0.7917338645922272, "precision": 0.9796533239425598, "recall": 0.6643052349491895, "support": 37689}, "weighted avg": {"f1-score": 0.7367008493346604, "precision": 0.8813186088779035, "recall": 0.6643052349491895, "support": 37689}, "\u2205": {"f1-score": 0.9526314458410258, "precision": 0.9855572998430141, "recall": 0.9218344672311684, "support": 20431}, "\u23ce": {"f1-score": 0.4451198919284025, "precision": 0.9592430858806404, "recall": 0.28979771328056286, "support": 2274}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 500}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8630751964085298, "precision": 0.9298669891172914, "recall": 0.8052356020942408, "support": 955}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 833}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 90}, "\u2423": {"f1-score": 0.6252045826513911, "precision": 0.967970808838435, "recall": 0.4617095339392767, "support": 10342}},
  "ppcr": 0.6781023640850116
}
```
</details>
