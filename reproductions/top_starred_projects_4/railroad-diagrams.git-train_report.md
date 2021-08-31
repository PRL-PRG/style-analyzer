# Train report for javascript / file:///tmp/top-repos-quality-repos-7uf1o08w/railroad-diagrams.git HEAD d0399118a2fc3cd753ad88af2f457c44511678ad

### Classification report

PPCR: 0.589

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 0.987| 0.748| 0.990| 0.854| 11270| 14859| 0.758 |
| `␣` | 0.946| 0.984| 0.458| 0.964| 0.617| 2824| 6070| 0.465 |
| `⏎⇥⁻` | 0.941| 0.908| 0.578| 0.924| 0.716| 282| 443| 0.637 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 492| 0.028 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 1287| 0.008 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 834| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 482| 0.000 |
| `macro avg` | 0.412| 0.411| 0.255| 0.411| 0.312| 14400| 24467| 0.589 |
| `weighted avg` | 0.982| 0.983| 0.578| 0.982| 0.684| 14400| 24467| 0.589 |
| `micro avg` | 0.983| 0.983| 0.578| 0.983| 0.728| 14400| 24467| 0.589 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|3589 |11119 |151 |0 |0 |0 |0 |0 |
|3246 |30 |2778 |0 |0 |0 |0 |16 |
|1277 |10 |0 |0 |0 |0 |0 |0 |
|834 |0 |0 |0 |0 |0 |0 |0 |
|482 |0 |0 |0 |0 |0 |0 |0 |
|478 |6 |8 |0 |0 |0 |0 |0 |
|161 |26 |0 |0 |0 |0 |0 |256 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| railroad-diagrams.js | 125 |
| railroad.js | 122 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4112393293499529, "precision": 0.4115151221181862, "recall": 0.4111591519655657, "support": 14400}, "micro avg": {"f1-score": 0.9828472222222222, "precision": 0.9828472222222222, "recall": 0.9828472222222222, "support": 14400}, "weighted avg": {"f1-score": 0.9820999494187014, "precision": 0.981529233745708, "recall": 0.9828472222222222, "support": 14400}, "\u2205": {"f1-score": 0.9900716798005431, "precision": 0.9935662586006613, "recall": 0.9866015971606034, "support": 11270}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u21e5\u207b": {"f1-score": 0.924187725631769, "precision": 0.9411764705882353, "recall": 0.9078014184397163, "support": 282}, "\u2423": {"f1-score": 0.9644159000173582, "precision": 0.9458631256384066, "recall": 0.9837110481586402, "support": 2824}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 482}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 834}, "macro avg": {"f1-score": 0.31237192875697545, "precision": 0.4115151221181862, "recall": 0.25483420329281553, "support": 24467}, "micro avg": {"f1-score": 0.7282784881776314, "precision": 0.9828472222222222, "recall": 0.5784526096374709, "support": 24467}, "weighted avg": {"f1-score": 0.6844382133997808, "precision": 0.8550999871517938, "recall": 0.5784526096374709, "support": 24467}, "\u2205": {"f1-score": 0.8536660268714013, "precision": 0.9935662586006613, "recall": 0.748300693182583, "support": 14859}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1287}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 492}, "\u23ce\u21e5\u207b": {"f1-score": 0.716083916083916, "precision": 0.9411764705882353, "recall": 0.5778781038374717, "support": 443}, "\u2423": {"f1-score": 0.6168535583435106, "precision": 0.9458631256384066, "recall": 0.457660626029654, "support": 6070}},
  "ppcr": 0.5885478399476847
}
```
</details>
