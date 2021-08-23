# Train report for javascript / file:///tmp/top-repos-quality-repos-9_onuwcr/sixcfj.git HEAD 38e495aabf5986ba91fc34fc3708a69a22cf92b7

### Classification report

PPCR: 0.619

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.924| 0.997| 0.856| 0.959| 0.889| 7369| 8584| 0.858 |
| `␣` | 0.972| 0.760| 0.312| 0.853| 0.473| 1189| 2895| 0.411 |
| `⏎` | 0.954| 0.869| 0.328| 0.909| 0.488| 473| 1254| 0.377 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 148| 417| 0.355 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 109| 444| 0.245 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 159| 0.164 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 977| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 308| 0.000 |
| `micro avg` | 0.930| 0.930| 0.576| 0.930| 0.711| 9314| 15038| 0.619 |
| `macro avg` | 0.356| 0.328| 0.187| 0.340| 0.231| 9314| 15038| 0.619 |
| `weighted avg` | 0.904| 0.930| 0.576| 0.914| 0.639| 9314| 15038| 0.619 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1215 |7348 |21 |0 |0 |0 |0 |0 |0 |
|1706 |285 |904 |0 |0 |0 |0 |0 |0 |
|781 |62 |0 |411 |0 |0 |0 |0 |0 |
|977 |0 |0 |0 |0 |0 |0 |0 |0 |
|335 |99 |5 |5 |0 |0 |0 |0 |0 |
|269 |140 |0 |8 |0 |0 |0 |0 |0 |
|308 |0 |0 |0 |0 |0 |0 |0 |0 |
|133 |19 |0 |7 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| sixcfj/ConfigurationScreen.js | 185 |
| sixcfj/MainScreen.js | 139 |
| slides/src/registerServiceWorker.js | 58 |
| sixcfj/Controller.js | 51 |
| slides/config/webpack.config.prod.js | 44 |
| slides/src/presentation.js | 36 |
| sixcfj/App.js | 24 |
| slides/config/webpack.config.dev.js | 23 |
| slides/scripts/build.js | 22 |
| slides/config/env.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.34020855090394897, "precision": 0.3561959219888441, "recall": 0.3282968469063807, "support": 9314}, "micro avg": {"f1-score": 0.9301052179514709, "precision": 0.9301052179514709, "recall": 0.9301052179514709, "support": 9314}, "weighted avg": {"f1-score": 0.913948764983586, "precision": 0.9035039925525795, "recall": 0.9301052179514709, "support": 9314}, "\u2205": {"f1-score": 0.9591437149197233, "precision": 0.9239280774550485, "recall": 0.9971502239109784, "support": 7369}, "\u23ce": {"f1-score": 0.9092920353982301, "precision": 0.9535962877030162, "recall": 0.86892177589852, "support": 473}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 109}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 148}, "\u2423": {"f1-score": 0.8532326569136385, "precision": 0.9720430107526882, "recall": 0.7603027754415476, "support": 1189}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 308}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 977}, "macro avg": {"f1-score": 0.23114843115793934, "precision": 0.3561959219888441, "recall": 0.18700311266982322, "support": 15038}, "micro avg": {"f1-score": 0.7114816031537451, "precision": 0.9301052179514709, "recall": 0.576073946003458, "support": 15038}, "weighted avg": {"f1-score": 0.6389498652308352, "precision": 0.7940466071141609, "recall": 0.576073946003458, "support": 15038}, "\u2205": {"f1-score": 0.8886738828082482, "precision": 0.9239280774550485, "recall": 0.8560111835973905, "support": 8584}, "\u23ce": {"f1-score": 0.4878338278931751, "precision": 0.9535962877030162, "recall": 0.3277511961722488, "support": 1254}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 159}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 444}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 417}, "\u2423": {"f1-score": 0.4726797385620915, "precision": 0.9720430107526882, "recall": 0.31226252158894646, "support": 2895}},
  "ppcr": 0.6193642771645166
}
```
</details>
