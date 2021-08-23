# Train report for javascript / file:///tmp/top-repos-quality-repos-mewp_bos/tecnologie-per-iot.git HEAD 17a93fceceefa94f249d80f047eaeabed377410d

### Classification report

PPCR: 0.579

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 0.988| 0.808| 0.989| 0.890| 8035| 9823| 0.818 |
| `␣` | 0.939| 0.974| 0.375| 0.956| 0.535| 1828| 4753| 0.385 |
| `"` | 1.000| 1.000| 0.525| 1.000| 0.688| 717| 1366| 0.525 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 24| 1315| 0.018 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 293| 0.041 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 379| 0.008 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 410| 0.000 |
| `micro avg` | 0.982| 0.982| 0.569| 0.982| 0.720| 10619| 18339| 0.579 |
| `weighted avg` | 0.979| 0.982| 0.569| 0.981| 0.667| 10619| 18339| 0.579 |
| `macro avg` | 0.419| 0.423| 0.244| 0.421| 0.302| 10619| 18339| 0.579 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1788 |7935 |100 |0 |0 |0 |0 |0 |
|2925 |48 |1780 |0 |0 |0 |0 |0 |
|1291 |10 |14 |0 |0 |0 |0 |0 |
|649 |0 |0 |0 |717 |0 |0 |0 |
|376 |1 |2 |0 |0 |0 |0 |0 |
|281 |12 |0 |0 |0 |0 |0 |0 |
|410 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Software/Lab 1 SW/freeboard/plugins/mqtt/mqttws31.js | 132 |
| Software/Lab 1 SW/freeboard/plugins/freeboard/freeboard.widgets.js | 31 |
| Software/Lab 1 SW/freeboard/plugins/mqtt/paho.mqtt.plugin.js | 14 |
| Software/Lab 1 SW/freeboard/plugins/freeboard/freeboard.datasources.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 717}, "macro avg": {"f1-score": 0.42075730694583385, "precision": 0.4185643166660568, "recall": 0.42304232051358615, "support": 10619}, "micro avg": {"f1-score": 0.9823900555607873, "precision": 0.9823900555607873, "recall": 0.9823900555607873, "support": 10619}, "weighted avg": {"f1-score": 0.9806801715846872, "precision": 0.9790849567227919, "recall": 0.9823900555607873, "support": 10619}, "\u2205": {"f1-score": 0.9893398167196559, "precision": 0.9911316512615538, "recall": 0.9875544492843809, "support": 8035}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9559613319011814, "precision": 0.9388185654008439, "recall": 0.973741794310722, "support": 1828}},
  "cl_report_full": {"\"": {"f1-score": 0.6884301488238119, "precision": 1.0, "recall": 0.5248901903367497, "support": 1366}, "macro avg": {"f1-score": 0.3019959774840696, "precision": 0.4185643166660568, "recall": 0.2438840758528813, "support": 18339}, "micro avg": {"f1-score": 0.7204917466675874, "precision": 0.9823900555607873, "recall": 0.5688423578166748, "support": 18339}, "weighted avg": {"f1-score": 0.6668257821629252, "precision": 0.8486880883195623, "recall": 0.5688423578166748, "support": 18339}, "\u2205": {"f1-score": 0.8901228335857312, "precision": 0.9911316512615538, "recall": 0.8077980250432658, "support": 9823}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1315}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 379}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 293}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 410}, "\u2423": {"f1-score": 0.5354188599789441, "precision": 0.9388185654008439, "recall": 0.37450031559015357, "support": 4753}},
  "ppcr": 0.5790392060635804
}
```
</details>
