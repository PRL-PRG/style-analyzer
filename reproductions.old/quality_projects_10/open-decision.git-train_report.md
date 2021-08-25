# Train report for javascript / file:///tmp/top-repos-quality-repos-xlc6v3s5/open-decision.git HEAD d8ebd55ed79157c881455527501b1389d8f41f45

### Classification report

PPCR: 0.305

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.999| 0.359| 0.994| 0.526| 1583| 4410| 0.359 |
| `'` | 1.000| 1.000| 0.780| 1.000| 0.877| 416| 533| 0.780 |
| `⏎` | 0.969| 0.933| 0.369| 0.951| 0.535| 238| 601| 0.396 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 1818| 0.003 |
| `macro avg` | 0.740| 0.733| 0.377| 0.736| 0.485| 2242| 7362| 0.305 |
| `micro avg` | 0.990| 0.990| 0.301| 0.990| 0.462| 2242| 7362| 0.305 |
| `weighted avg` | 0.987| 0.990| 0.301| 0.989| 0.422| 2242| 7362| 0.305 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|2827 |1581 |0 |0 |2 |
|1813 |0 |0 |0 |5 |
|117 |0 |0 |416 |0 |
|363 |16 |0 |0 |222 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| opendecision/static/ckeditor/galleriffic/js/jquery.galleriffic.js | 16 |
| opendecision/static/tours/visualbuilder-tour.js | 4 |
| opendecision/static/ckeditor/ckeditor/plugins/abbr/dialogs/abbr.js | 2 |
| opendecision/static/ckeditor/ckeditor/plugins/abbr/plugin.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 416}, "macro avg": {"f1-score": 0.736272271827401, "precision": 0.7398533822970472, "recall": 0.7328774213412466, "support": 2242}, "micro avg": {"f1-score": 0.9897413024085637, "precision": 0.9897413024085637, "recall": 0.9897413024085637, "support": 2242}, "weighted avg": {"f1-score": 0.9885450469368938, "precision": 0.9874510052733303, "recall": 0.9897413024085637, "support": 2242}, "\u2205": {"f1-score": 0.9943396226415094, "precision": 0.9899812147777082, "recall": 0.9987365761212887, "support": 1583}, "\u23ce": {"f1-score": 0.9507494646680943, "precision": 0.9694323144104804, "recall": 0.9327731092436975, "support": 238}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8767123287671234, "precision": 1.0, "recall": 0.7804878048780488, "support": 533}, "macro avg": {"f1-score": 0.4845094927349022, "precision": 0.7398533822970472, "recall": 0.3770938914098978, "support": 7362}, "micro avg": {"f1-score": 0.46209912536443154, "precision": 0.9897413024085637, "recall": 0.30141265960336866, "support": 7362}, "weighted avg": {"f1-score": 0.42245968636849535, "precision": 0.7445593558992655, "recall": 0.30141265960336866, "support": 7362}, "\u2205": {"f1-score": 0.5263858831363408, "precision": 0.9899812147777082, "recall": 0.3585034013605442, "support": 4410}, "\u23ce": {"f1-score": 0.5349397590361447, "precision": 0.9694323144104804, "recall": 0.36938435940099834, "support": 601}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1818}},
  "ppcr": 0.30453681064928007
}
```
</details>
