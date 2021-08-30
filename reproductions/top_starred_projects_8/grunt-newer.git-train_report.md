# Train report for javascript / file:///tmp/top-repos-quality-repos-lmoqc1uq/grunt-newer.git HEAD 54484d394f22e2841b94b7fb76eee42a0b4d0f63

### Classification report

PPCR: 0.718

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.946| 0.999| 0.939| 0.972| 0.942| 4140| 4405| 0.940 |
| `␣` | 1.000| 0.844| 0.485| 0.915| 0.653| 1018| 1770| 0.575 |
| `'` | 1.000| 1.000| 0.504| 1.000| 0.670| 406| 806| 0.504 |
| `⏎␣⁻␣⁻` | 0.988| 0.935| 0.878| 0.961| 0.930| 339| 361| 0.939 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 480| 0.069 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 146| 0.116 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 330| 0.018 |
| `weighted avg` | 0.952| 0.960| 0.689| 0.954| 0.745| 5959| 8298| 0.718 |
| `macro avg` | 0.562| 0.540| 0.401| 0.550| 0.456| 5959| 8298| 0.718 |
| `micro avg` | 0.960| 0.960| 0.689| 0.960| 0.802| 5959| 8298| 0.718 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|265 |4136 |0 |0 |0 |4 |0 |0 |
|752 |159 |859 |0 |0 |0 |0 |0 |
|400 |0 |0 |406 |0 |0 |0 |0 |
|447 |33 |0 |0 |0 |0 |0 |0 |
|22 |22 |0 |0 |0 |317 |0 |0 |
|324 |6 |0 |0 |0 |0 |0 |0 |
|129 |17 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| tasks/newer.js | 68 |
| lib/util.js | 49 |
| test/lib/util.spec.js | 24 |
| test/helper.js | 22 |
| test/integration/tasks/index.js | 19 |
| gruntfile.js | 7 |
| test/integration/fixtures/newer-override/gruntfile.js | 6 |
| test/integration/newer-modify-one.spec.js | 5 |
| test/integration/newer-clean-dest.spec.js | 5 |
| test/integration/newer-override.spec.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 406}, "macro avg": {"f1-score": 0.5496552533549821, "precision": 0.5619061052615725, "recall": 0.5397069223078319, "support": 5959}, "micro avg": {"f1-score": 0.9595569726464172, "precision": 0.9595569726464172, "recall": 0.9595569726464172, "support": 5959}, "weighted avg": {"f1-score": 0.9542220358210738, "precision": 0.9522408821046615, "recall": 0.9595569726464172, "support": 5959}, "\u2205": {"f1-score": 0.9716903559262304, "precision": 0.9458037960210381, "recall": 0.9990338164251208, "support": 4140}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9606060606060606, "precision": 0.9875389408099688, "recall": 0.9351032448377581, "support": 339}, "\u2423": {"f1-score": 0.9152903569525839, "precision": 1.0, "recall": 0.843811394891945, "support": 1018}},
  "cl_report_full": {"\u0027": {"f1-score": 0.66996699669967, "precision": 1.0, "recall": 0.5037220843672456, "support": 806}, "macro avg": {"f1-score": 0.456488866507859, "precision": 0.5619061052615725, "recall": 0.4008688847096885, "support": 8298}, "micro avg": {"f1-score": 0.8021322858946482, "precision": 0.9595569726464172, "recall": 0.6890817064352857, "support": 8298}, "weighted avg": {"f1-score": 0.7451583268332937, "precision": 0.8554793057489843, "recall": 0.6890817064352857, "support": 8298}, "\u2205": {"f1-score": 0.9423558897243108, "precision": 0.9458037960210381, "recall": 0.9389330306469921, "support": 4405}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 480}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 146}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 330}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9296187683284458, "precision": 0.9875389408099688, "recall": 0.8781163434903048, "support": 361}, "\u2423": {"f1-score": 0.6534804108025866, "precision": 1.0, "recall": 0.4853107344632768, "support": 1770}},
  "ppcr": 0.7181248493612918
}
```
</details>
