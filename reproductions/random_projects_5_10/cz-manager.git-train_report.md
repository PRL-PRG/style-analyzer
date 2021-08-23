# Train report for javascript / file:///tmp/top-repos-quality-repos-x01dm51g/cz-manager.git HEAD 706de5e0fc8dc8341b1e96d5b252cb4cd5d4db54

### Classification report

PPCR: 0.616

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.994| 1.000| 0.963| 0.997| 0.978| 10273| 10669| 0.963 |
| `␣` | 0.991| 0.987| 0.306| 0.989| 0.467| 1598| 5162| 0.310 |
| `⏎⏎` | 0.977| 1.000| 0.160| 0.989| 0.275| 86| 537| 0.160 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 1075| 0.012 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 163| 0.080 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 471| 0.025 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 87| 0.080 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 476| 0.011 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 503| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 351| 0.000 |
| `weighted avg` | 0.990| 0.994| 0.612| 0.992| 0.667| 12007| 19494| 0.616 |
| `macro avg` | 0.296| 0.299| 0.143| 0.297| 0.172| 12007| 19494| 0.616 |
| `micro avg` | 0.994| 0.994| 0.612| 0.994| 0.758| 12007| 19494| 0.616 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|396 |10269 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|3564 |20 |1578 |0 |0 |0 |0 |0 |0 |0 |0 |
|1062 |8 |3 |0 |0 |2 |0 |0 |0 |0 |0 |
|503 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|451 |0 |0 |0 |0 |86 |0 |0 |0 |0 |0 |
|471 |2 |3 |0 |0 |0 |0 |0 |0 |0 |0 |
|459 |12 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|351 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|150 |9 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|80 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| odm2admin/static/leaflet-marker-cluster/leaflet.markercluster-src.js | 30 |
| odm2admin/static/js/Map.js | 19 |
| odm2admin/static/leaflet-grouped-layer-control/leaflet.groupedlayercontrol.js | 9 |
| odm2admin/static/js/webmap.js | 6 |
| odm2admin/static/leaflet-legend/leaflet-legend.js | 4 |
| odm2admin/static/leaflet-awesome-marker/js/leaflet.awesome-markers.js | 3 |
| odm2admin/static/js/functions.js | 2 |
| odm2admin/static/js/App.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2974837731178541, "precision": 0.29628624119214, "recall": 0.2987094985250594, "support": 12007}, "micro avg": {"f1-score": 0.9938369284583992, "precision": 0.9938369284583992, "recall": 0.9938369284583992, "support": 12007}, "weighted avg": {"f1-score": 0.9917598718636766, "precision": 0.9896973409997061, "recall": 0.9938369284583992, "support": 12007}, "\u2205": {"f1-score": 0.9969902912621359, "precision": 0.9943836544979181, "recall": 0.9996106298062883, "support": 10273}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.9885057471264368, "precision": 0.9772727272727273, "recall": 1.0, "support": 86}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.9893416927899685, "precision": 0.9912060301507538, "recall": 0.9874843554443054, "support": 1598}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 351}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 503}, "macro avg": {"f1-score": 0.1720664970893112, "precision": 0.29628624119214, "recall": 0.14283526439956976, "support": 19494}, "micro avg": {"f1-score": 0.7576267420081901, "precision": 0.9938369284583992, "recall": 0.6121370678157382, "support": 19494}, "weighted avg": {"f1-score": 0.666674086930383, "precision": 0.8336144553207107, "recall": 0.6121370678157382, "support": 19494}, "\u2205": {"f1-score": 0.9781863212040389, "precision": 0.9943836544979181, "recall": 0.9625082013309588, "support": 10669}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1075}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 476}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 471}, "\u23ce\u23ce": {"f1-score": 0.27520000000000006, "precision": 0.9772727272727273, "recall": 0.1601489757914339, "support": 537}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 87}, "\u2423": {"f1-score": 0.4672786496890732, "precision": 0.9912060301507538, "recall": 0.3056954668733049, "support": 5162}},
  "ppcr": 0.6159331076228584
}
```
</details>
