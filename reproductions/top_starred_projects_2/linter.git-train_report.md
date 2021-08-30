# Train report for javascript / file:///tmp/top-repos-quality-repos-sbvnzbri/linter.git HEAD ba3fbaf9d7b741e3432a05bca2d6178169f36cd4

### Classification report

PPCR: 0.711

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.950| 0.981| 0.948| 0.966| 0.949| 14839| 15363| 0.966 |
| `'` | 1.000| 1.000| 0.910| 1.000| 0.953| 1545| 1697| 0.910 |
| `␣` | 0.703| 0.518| 0.125| 0.596| 0.212| 1261| 5231| 0.241 |
| `⏎␣⁻␣⁻` | 0.862| 0.902| 0.746| 0.882| 0.800| 796| 963| 0.827 |
| `⏎` | 0.976| 0.839| 0.115| 0.902| 0.206| 286| 2089| 0.137 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 153| 964| 0.159 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 258| 0.000 |
| `micro avg` | 0.939| 0.939| 0.667| 0.939| 0.780| 18880| 26565| 0.711 |
| `macro avg` | 0.642| 0.606| 0.406| 0.621| 0.446| 18880| 26565| 0.711 |
| `weighted avg` | 0.927| 0.939| 0.667| 0.931| 0.697| 18880| 26565| 0.711 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|524 |14563 |274 |0 |0 |0 |2 |0 |
|3970 |489 |653 |6 |0 |0 |113 |0 |
|1803 |44 |2 |240 |0 |0 |0 |0 |
|152 |0 |0 |0 |1545 |0 |0 |0 |
|811 |153 |0 |0 |0 |0 |0 |0 |
|167 |78 |0 |0 |0 |0 |718 |0 |
|258 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| spec/validate-spec.js | 197 |
| spec/linter-registry-spec.js | 149 |
| spec/message-registry-spec.js | 111 |
| lib/validate/index.js | 80 |
| spec/helpers-spec.js | 76 |
| lib/linter-registry.js | 65 |
| lib/helpers.js | 60 |
| spec/indie-delegate-spec.js | 55 |
| lib/message-registry.js | 39 |
| lib/main.js | 38 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1545}, "macro avg": {"f1-score": 0.6208070253465156, "precision": 0.6415163155889733, "recall": 0.6057734621540752, "support": 18880}, "micro avg": {"f1-score": 0.9385063559322034, "precision": 0.9385063559322034, "recall": 0.9385063559322034, "support": 18880}, "weighted avg": {"f1-score": 0.9313634891511988, "precision": 0.9266855148175731, "recall": 0.9385063559322034, "support": 18880}, "\u2205": {"f1-score": 0.96552409998011, "precision": 0.9501533241991257, "recall": 0.9814003639059236, "support": 14839}, "\u23ce": {"f1-score": 0.9022556390977443, "precision": 0.975609756097561, "recall": 0.8391608391608392, "support": 286}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 153}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8815224063842848, "precision": 0.8619447779111644, "recall": 0.9020100502512562, "support": 796}, "\u2423": {"f1-score": 0.5963470319634703, "precision": 0.7029063509149623, "recall": 0.5178429817605076, "support": 1261}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9531153608883405, "precision": 1.0, "recall": 0.9104301708898055, "support": 1697}, "macro avg": {"f1-score": 0.44561273432403453, "precision": 0.6415163155889733, "recall": 0.40623770717874563, "support": 26565}, "micro avg": {"f1-score": 0.7797997579491693, "precision": 0.9385063559322034, "recall": 0.6670054583098062, "support": 26565}, "weighted avg": {"f1-score": 0.6966294317358372, "precision": 0.8597481740230978, "recall": 0.6670054583098062, "support": 26565}, "\u2205": {"f1-score": 0.9490387748452265, "precision": 0.9501533241991257, "recall": 0.9479268372062748, "support": 15363}, "\u23ce": {"f1-score": 0.2055674518201285, "precision": 0.975609756097561, "recall": 0.11488750598372427, "support": 2089}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 258}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 964}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7995545657015591, "precision": 0.8619447779111644, "recall": 0.7455867082035307, "support": 963}, "\u2423": {"f1-score": 0.212012987012987, "precision": 0.7029063509149623, "recall": 0.12483272796788376, "support": 5231}},
  "ppcr": 0.7107095802747977
}
```
</details>
