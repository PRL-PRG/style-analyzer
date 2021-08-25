# Train report for javascript / file:///tmp/top-repos-quality-repos-dxxuftfx/py-microservice-template.git HEAD 9a899bbaad60e3025b37ea499d86ba334eb8c01e

### Classification report

PPCR: 0.490

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 0.999| 0.936| 0.987| 0.955| 4843| 5172| 0.936 |
| `␣` | 0.975| 0.846| 0.210| 0.906| 0.346| 680| 2733| 0.249 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.967| 0.819| 0.556| 0.887| 0.706| 177| 261| 0.678 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.944| 0.883| 0.436| 0.913| 0.596| 154| 312| 0.494 |
| `⏎` | 0.949| 0.991| 0.115| 0.969| 0.206| 112| 963| 0.116 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 155| 0.013 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2062| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 522| 0.000 |
| `weighted avg` | 0.973| 0.973| 0.477| 0.972| 0.530| 5968| 12180| 0.490 |
| `micro avg` | 0.973| 0.973| 0.477| 0.973| 0.640| 5968| 12180| 0.490 |
| `macro avg` | 0.601| 0.567| 0.282| 0.583| 0.351| 5968| 12180| 0.490 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|329 |4840 |0 |0 |0 |2 |1 |0 |0 |
|2053 |95 |575 |0 |0 |6 |4 |0 |0 |
|2062 |0 |0 |0 |0 |0 |0 |0 |0 |
|851 |1 |0 |0 |111 |0 |0 |0 |0 |
|158 |1 |15 |0 |2 |136 |0 |0 |0 |
|84 |30 |0 |0 |2 |0 |145 |0 |0 |
|522 |0 |0 |0 |0 |0 |0 |0 |0 |
|153 |0 |0 |0 |2 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| apidoc/main.js | 103 |
| apidoc/utils/handlebars_helper.js | 15 |
| apidoc/utils/send_sample_request.js | 12 |
| apidoc/utils/send_sample_request_utils.js | 9 |
| apidoc/locales/locale.js | 7 |
| apidoc/locales/pl.js | 1 |
| apidoc/locales/de.js | 1 |
| apidoc/locales/ro.js | 1 |
| apidoc/locales/ca.js | 1 |
| apidoc/locales/cs.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5826617715372081, "precision": 0.6011045721550732, "recall": 0.5672957669720984, "support": 5968}, "micro avg": {"f1-score": 0.9730227882037533, "precision": 0.9730227882037533, "recall": 0.9730227882037533, "support": 5968}, "weighted avg": {"f1-score": 0.9719641918171826, "precision": 0.9726345919195247, "recall": 0.9730227882037533, "support": 5968}, "\u2205": {"f1-score": 0.9867482161060143, "precision": 0.9744312462250856, "recall": 0.9993805492463349, "support": 4843}, "\u23ce": {"f1-score": 0.9694323144104804, "precision": 0.9487179487179487, "recall": 0.9910714285714286, "support": 112}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.912751677852349, "precision": 0.9444444444444444, "recall": 0.8831168831168831, "support": 154}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8868501529051989, "precision": 0.9666666666666667, "recall": 0.8192090395480226, "support": 177}, "\u2423": {"f1-score": 0.905511811023622, "precision": 0.9745762711864406, "recall": 0.8455882352941176, "support": 680}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 522}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2062}, "macro avg": {"f1-score": 0.3510556224603123, "precision": 0.6011045721550732, "recall": 0.2816146872637312, "support": 12180}, "micro avg": {"f1-score": 0.6399603262067446, "precision": 0.9730227882037533, "recall": 0.47676518883415436, "support": 12180}, "weighted avg": {"f1-score": 0.5297120385372236, "precision": 0.7523692451486647, "recall": 0.47676518883415436, "support": 12180}, "\u2205": {"f1-score": 0.9547292632409508, "precision": 0.9744312462250856, "recall": 0.9358081979891725, "support": 5172}, "\u23ce": {"f1-score": 0.20555555555555557, "precision": 0.9487179487179487, "recall": 0.11526479750778816, "support": 963}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.5964912280701755, "precision": 0.9444444444444444, "recall": 0.4358974358974359, "support": 312}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7055961070559612, "precision": 0.9666666666666667, "recall": 0.5555555555555556, "support": 261}, "\u2423": {"f1-score": 0.34607282575985554, "precision": 0.9745762711864406, "recall": 0.21039151115989754, "support": 2733}},
  "ppcr": 0.48998357963875205
}
```
</details>
