# Train report for javascript / file:///tmp/top-repos-quality-repos-9b4rfuy9/cents_trip.git HEAD 47d202751de52444789f3635d404bea70f58f414

### Classification report

PPCR: 0.344

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.976| 1.000| 0.635| 0.988| 0.769| 2034| 3203| 0.635 |
| `'` | 1.000| 1.000| 0.496| 1.000| 0.663| 405| 816| 0.496 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 2275| 0.021 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 468| 0.009 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 178| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 150| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 154| 0.000 |
| `macro avg` | 0.282| 0.286| 0.162| 0.284| 0.205| 2490| 7244| 0.344 |
| `micro avg` | 0.980| 0.980| 0.337| 0.980| 0.501| 2490| 7244| 0.344 |
| `weighted avg` | 0.960| 0.980| 0.337| 0.969| 0.415| 2490| 7244| 0.344 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1169 |2034 |0 |0 |0 |0 |0 |0 |
|2228 |47 |0 |0 |0 |0 |0 |0 |
|411 |0 |0 |405 |0 |0 |0 |0 |
|464 |4 |0 |0 |0 |0 |0 |0 |
|178 |0 |0 |0 |0 |0 |0 |0 |
|150 |0 |0 |0 |0 |0 |0 |0 |
|154 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| frontend/src/main/polyfill/url.js | 15 |
| frontend/src/main/apis.js | 11 |
| frontend/src/main/epic.js | 8 |
| frontend/webpack_config/base.js | 3 |
| frontend/src/main/polyfill/arrayIncludes.js | 2 |
| frontend/src/main/store.js | 2 |
| frontend/src/main/util.js | 2 |
| frontend/yamlReducerLoader/actionPayloadType.js | 2 |
| frontend/src/main/config.js | 1 |
| frontend/src/main/reducer.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 405}, "macro avg": {"f1-score": 0.28394547913848717, "precision": 0.2822199383350462, "recall": 0.2857142857142857, "support": 2490}, "micro avg": {"f1-score": 0.9795180722891567, "precision": 0.9795180722891567, "recall": 0.9795180722891567, "support": 2490}, "weighted avg": {"f1-score": 0.9694039084232049, "precision": 0.9595371413712404, "recall": 0.9795180722891567, "support": 2490}, "\u2205": {"f1-score": 0.9876183539694101, "precision": 0.9755395683453237, "recall": 1.0, "support": 2034}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6633906633906634, "precision": 1.0, "recall": 0.4963235294117647, "support": 816}, "macro avg": {"f1-score": 0.20466851707396336, "precision": 0.2822199383350462, "recall": 0.16162188415797166, "support": 7244}, "micro avg": {"f1-score": 0.5011300595849599, "precision": 0.9795180722891567, "recall": 0.33669243511871894, "support": 7244}, "weighted avg": {"f1-score": 0.4148756636943427, "precision": 0.5439885750151949, "recall": 0.33669243511871894, "support": 7244}, "\u2205": {"f1-score": 0.76928895612708, "precision": 0.9755395683453237, "recall": 0.6350296596940368, "support": 3203}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 468}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 178}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 150}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2275}},
  "ppcr": 0.34373274434014356
}
```
</details>
