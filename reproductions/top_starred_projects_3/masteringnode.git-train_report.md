# Train report for javascript / file:///tmp/top-repos-quality-repos-ufywvx10/masteringnode.git HEAD 6a8a2e78284f1ccdd18b468959e10bcf1b5807ef

### Classification report

PPCR: 0.201

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 1.000| 0.321| 0.997| 0.485| 357| 1112| 0.321 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 97| 0.021 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 344| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 234| 0.000 |
| `weighted avg` | 0.989| 0.994| 0.200| 0.992| 0.302| 359| 1787| 0.201 |
| `micro avg` | 0.994| 0.994| 0.200| 0.994| 0.333| 359| 1787| 0.201 |
| `macro avg` | 0.249| 0.250| 0.080| 0.249| 0.121| 359| 1787| 0.201 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|755 |357 |0 |0 |0 |
|344 |0 |0 |0 |0 |
|234 |0 |0 |0 |0 |
|95 |2 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/modules/compiler/extended.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24930167597765363, "precision": 0.24860724233983286, "recall": 0.25, "support": 359}, "micro avg": {"f1-score": 0.9944289693593314, "precision": 0.9944289693593314, "recall": 0.9944289693593314, "support": 359}, "weighted avg": {"f1-score": 0.9916512348080483, "precision": 0.9888889751010621, "recall": 0.9944289693593314, "support": 359}, "\u2205": {"f1-score": 0.9972067039106145, "precision": 0.9944289693593314, "recall": 1.0, "support": 357}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 234}, "macro avg": {"f1-score": 0.12134602311352823, "precision": 0.24860724233983286, "recall": 0.08026079136690648, "support": 1787}, "micro avg": {"f1-score": 0.3327120223671947, "precision": 0.9944289693593314, "recall": 0.19977616116396194, "support": 1787}, "weighted avg": {"f1-score": 0.30204091259595606, "precision": 0.6188052680064782, "recall": 0.19977616116396194, "support": 1787}, "\u2205": {"f1-score": 0.4853840924541129, "precision": 0.9944289693593314, "recall": 0.3210431654676259, "support": 1112}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 344}},
  "ppcr": 0.2008953553441522
}
```
</details>
