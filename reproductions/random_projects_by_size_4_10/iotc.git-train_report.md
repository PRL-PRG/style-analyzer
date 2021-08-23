# Train report for javascript / file:///tmp/top-repos-quality-repos-t3cguefe/iotc.git HEAD 4b2b745cb343fe5969218c79f080d98b4bbc02f9

### Classification report

PPCR: 0.365

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 0.989| 0.489| 0.988| 0.654| 445| 899| 0.495 |
| `␣` | 0.958| 0.950| 0.215| 0.954| 0.351| 120| 530| 0.226 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 119| 0.000 |
| `macro avg` | 0.648| 0.646| 0.235| 0.647| 0.335| 565| 1548| 0.365 |
| `weighted avg` | 0.980| 0.981| 0.358| 0.981| 0.500| 565| 1548| 0.365 |
| `micro avg` | 0.981| 0.981| 0.358| 0.981| 0.524| 565| 1548| 0.365 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|454 |440 |5 |0 |
|410 |6 |114 |0 |
|119 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/objDetectOnMotion.js | 5 |
| static/objDetect.js | 4 |
| static/local.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.647209738795048, "precision": 0.6481767594930349, "recall": 0.6462546816479401, "support": 565}, "micro avg": {"f1-score": 0.9805309734513274, "precision": 0.9805309734513274, "recall": 0.9805309734513274, "support": 565}, "weighted avg": {"f1-score": 0.9805011686499202, "precision": 0.9804804178903992, "recall": 0.9805309734513274, "support": 565}, "\u2205": {"f1-score": 0.9876543209876543, "precision": 0.9865470852017937, "recall": 0.9887640449438202, "support": 445}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9539748953974896, "precision": 0.957983193277311, "recall": 0.95, "support": 120}},
  "cl_report_full": {"macro avg": {"f1-score": 0.3351949333929045, "precision": 0.6481767594930349, "recall": 0.23484234754199285, "support": 1548}, "micro avg": {"f1-score": 0.5243729294841458, "precision": 0.9805309734513274, "recall": 0.3578811369509044, "support": 1548}, "weighted avg": {"f1-score": 0.5002502928865623, "precision": 0.9009282442076145, "recall": 0.3578811369509044, "support": 1548}, "\u2205": {"f1-score": 0.654275092936803, "precision": 0.9865470852017937, "recall": 0.489432703003337, "support": 899}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 119}, "\u2423": {"f1-score": 0.35130970724191063, "precision": 0.957983193277311, "recall": 0.21509433962264152, "support": 530}},
  "ppcr": 0.3649870801033592
}
```
</details>
