# Train report for javascript / file:///tmp/top-repos-quality-repos-zy_h3jma/sdow.git HEAD a2b07edf2c844b1b67c8080a6f286105155c45db

### Classification report

PPCR: 0.680

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.958| 0.999| 0.852| 0.978| 0.902| 5668| 6649| 0.852 |
| `␣` | 0.993| 0.872| 0.393| 0.928| 0.563| 1067| 2364| 0.451 |
| `'` | 1.000| 1.000| 0.869| 1.000| 0.930| 562| 647| 0.869 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 430| 0.109 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 33| 274| 0.120 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 236| 0.136 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 279| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 18| 0.000 |
| `weighted avg` | 0.952| 0.966| 0.657| 0.958| 0.728| 7409| 10897| 0.680 |
| `macro avg` | 0.369| 0.359| 0.264| 0.363| 0.299| 7409| 10897| 0.680 |
| `micro avg` | 0.966| 0.966| 0.657| 0.966| 0.782| 7409| 10897| 0.680 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|981 |5663 |5 |0 |0 |0 |0 |0 |0 |
|1297 |137 |930 |0 |0 |0 |0 |0 |0 |
|85 |0 |0 |562 |0 |0 |0 |0 |0 |
|383 |45 |2 |0 |0 |0 |0 |0 |0 |
|279 |0 |0 |0 |0 |0 |0 |0 |0 |
|241 |33 |0 |0 |0 |0 |0 |0 |0 |
|204 |32 |0 |0 |0 |0 |0 |0 |0 |
|18 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| website/src/components/charts/BarChart/index.js | 64 |
| website/src/actions.js | 44 |
| website/src/components/PageInput.js | 24 |
| website/src/registerServiceWorker.js | 21 |
| website/src/reducers/index.js | 15 |
| website/src/components/Home.js | 10 |
| website/src/components/blog/Blog/index.js | 9 |
| website/src/components/Loading.js | 8 |
| website/src/components/blog/posts/SearchResultsAnalysisPost/index.js | 7 |
| website/src/components/ResultsList.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 562}, "macro avg": {"f1-score": 0.3632972869255443, "precision": 0.36884197234577, "recall": 0.3588400598502982, "support": 7409}, "micro avg": {"f1-score": 0.9657173707652854, "precision": 0.9657173707652854, "recall": 0.9657173707652854, "support": 7409}, "weighted avg": {"f1-score": 0.9578840540956938, "precision": 0.9518346415658561, "recall": 0.9657173707652854, "support": 7409}, "\u2205": {"f1-score": 0.9782345828295042, "precision": 0.9582064297800338, "recall": 0.9991178546224417, "support": 5668}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 0.9281437125748504, "precision": 0.9925293489861259, "recall": 0.8716026241799437, "support": 1067}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u0027": {"f1-score": 0.9296939619520265, "precision": 1.0, "recall": 0.8686244204018547, "support": 647}, "macro avg": {"f1-score": 0.29937287150177155, "precision": 0.36884197234577, "recall": 0.2642165574053568, "support": 10897}, "micro avg": {"f1-score": 0.7817109144542773, "precision": 0.9657173707652854, "recall": 0.6566027346976232, "support": 10897}, "weighted avg": {"f1-score": 0.7277019780370738, "precision": 0.8593607353042715, "recall": 0.6566027346976232, "support": 10897}, "\u2205": {"f1-score": 0.9018233935822916, "precision": 0.9582064297800338, "recall": 0.8517070236125733, "support": 6649}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 430}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 279}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 274}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 236}, "\u2423": {"f1-score": 0.5634656164798546, "precision": 0.9925293489861259, "recall": 0.3934010152284264, "support": 2364}},
  "ppcr": 0.6799119023584472
}
```
</details>
