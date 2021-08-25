# Train report for javascript / file:///tmp/top-repos-quality-repos-9642l4ut/blog.git HEAD 3004d96740f06a27bebf8b9c492db7babea45aa3

### Classification report

PPCR: 0.643

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.999| 0.994| 0.897| 0.996| 0.945| 3429| 3803| 0.902 |
| `␣` | 0.967| 0.991| 0.227| 0.979| 0.368| 530| 2314| 0.229 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.994| 1.000| 0.968| 0.997| 0.981| 329| 340| 0.968 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.991| 0.991| 0.913| 0.991| 0.950| 317| 344| 0.922 |
| `'` | 1.000| 1.000| 0.712| 1.000| 0.832| 314| 441| 0.712 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 415| 0.002 |
| `macro avg` | 0.825| 0.829| 0.619| 0.827| 0.679| 4920| 7657| 0.643 |
| `micro avg` | 0.994| 0.994| 0.639| 0.994| 0.778| 4920| 7657| 0.643 |
| `weighted avg` | 0.994| 0.994| 0.639| 0.994| 0.714| 4920| 7657| 0.643 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|374 |3410 |15 |0 |0 |2 |2 |
|1784 |5 |525 |0 |0 |0 |0 |
|127 |0 |0 |314 |0 |0 |0 |
|414 |0 |0 |0 |0 |1 |0 |
|27 |0 |3 |0 |0 |314 |0 |
|11 |0 |0 |0 |0 |0 |329 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| entry/static/entry/js/multi_slider.js | 23 |
| main/static/main/js/slideshow.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 314}, "macro avg": {"f1-score": 0.8270940041707963, "precision": 0.8249801135693265, "recall": 0.8292602235489103, "support": 4920}, "micro avg": {"f1-score": 0.9943089430894309, "precision": 0.9943089430894309, "recall": 0.9943089430894309, "support": 4920}, "weighted avg": {"f1-score": 0.9942312626501376, "precision": 0.9941915703282229, "recall": 0.9943089430894309, "support": 4920}, "\u2205": {"f1-score": 0.9964932787843367, "precision": 0.9985358711566618, "recall": 0.994459025955089, "support": 3429}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9905362776025236, "precision": 0.9905362776025236, "recall": 0.9905362776025236, "support": 317}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.996969696969697, "precision": 0.9939577039274925, "recall": 1.0, "support": 329}, "\u2423": {"f1-score": 0.9785647716682199, "precision": 0.9668508287292817, "recall": 0.9905660377358491, "support": 530}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8317880794701986, "precision": 1.0, "recall": 0.7120181405895691, "support": 441}, "macro avg": {"f1-score": 0.6791446836403199, "precision": 0.8249801135693265, "recall": 0.6193327149930751, "support": 7657}, "micro avg": {"f1-score": 0.7779279637433409, "precision": 0.9943089430894309, "recall": 0.6388925166514301, "support": 7657}, "weighted avg": {"f1-score": 0.7144831862048006, "precision": 0.934362653065033, "recall": 0.6388925166514301, "support": 7657}, "\u2205": {"f1-score": 0.944860072042117, "precision": 0.9985358711566618, "recall": 0.8966605311596109, "support": 3803}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 415}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9500756429652042, "precision": 0.9905362776025236, "recall": 0.9127906976744186, "support": 344}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9806259314456037, "precision": 0.9939577039274925, "recall": 0.9676470588235294, "support": 340}, "\u2423": {"f1-score": 0.36751837591879594, "precision": 0.9668508287292817, "recall": 0.22687986171132238, "support": 2314}},
  "ppcr": 0.6425493012929345
}
```
</details>
