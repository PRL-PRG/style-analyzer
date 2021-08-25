# Train report for javascript / file:///tmp/top-repos-quality-repos-o2d7vayp/piecewise.git HEAD 0b21a7cccf6a2c2b2d6f9e95a281a39fede932ac

### Classification report

PPCR: 0.760

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.945| 0.995| 0.877| 0.970| 0.910| 9284| 10536| 0.881 |
| `␣` | 0.970| 0.889| 0.630| 0.928| 0.764| 3445| 4863| 0.708 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 1762| 1762| 1.000 |
| `⏎` | 0.908| 0.851| 0.288| 0.879| 0.438| 524| 1546| 0.339 |
| `⏎␣⁺␣⁺` | 0.960| 0.779| 0.424| 0.860| 0.588| 466| 857| 0.544 |
| `⏎␣⁻␣⁻` | 0.972| 0.693| 0.192| 0.809| 0.321| 202| 728| 0.277 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 314| 0.061 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 73| 0.055 |
| `macro avg` | 0.720| 0.651| 0.426| 0.681| 0.503| 15706| 20679| 0.760 |
| `micro avg` | 0.956| 0.956| 0.726| 0.956| 0.825| 15706| 20679| 0.760 |
| `weighted avg` | 0.955| 0.956| 0.726| 0.954| 0.797| 15706| 20679| 0.760 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1252 |9238 |46 |0 |0 |0 |0 |0 |0 |
|1418 |329 |3064 |0 |37 |15 |0 |0 |0 |
|0 |0 |0 |1762 |0 |0 |0 |0 |0 |
|1022 |48 |30 |0 |446 |0 |0 |0 |0 |
|391 |87 |16 |0 |0 |363 |0 |0 |0 |
|526 |54 |1 |0 |7 |0 |140 |0 |0 |
|295 |17 |1 |0 |1 |0 |0 |0 |0 |
|69 |0 |0 |0 |0 |0 |4 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/frontend/assets/js/ndt-browser-client.js | 123 |
| src/backend/config.js | 49 |
| src/backend/models/submissions.js | 41 |
| src/backend/controllers/auth.js | 40 |
| src/backend/__tests__/submissions.test.js | 34 |
| src/backend/controllers/submissions.js | 33 |
| src/backend/models/forms.js | 32 |
| src/backend/__tests__/forms.test.js | 31 |
| src/backend/middleware/cloudflare.js | 28 |
| src/backend/middleware/ssr.js | 27 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1762}, "macro avg": {"f1-score": 0.6807289223894235, "precision": 0.719547706902091, "recall": 0.6509543094986499, "support": 15706}, "micro avg": {"f1-score": 0.9558767350057303, "precision": 0.9558767350057303, "recall": 0.9558767350057303, "support": 15706}, "weighted avg": {"f1-score": 0.9540906833526042, "precision": 0.9550553162379769, "recall": 0.9558767350057303, "support": 15706}, "\u2205": {"f1-score": 0.9695125150863199, "precision": 0.9452573416555817, "recall": 0.9950452391210685, "support": 9284}, "\u23ce": {"f1-score": 0.8788177339901478, "precision": 0.9083503054989817, "recall": 0.851145038167939, "support": 524}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8601895734597156, "precision": 0.9603174603174603, "recall": 0.778969957081545, "support": 466}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8092485549132948, "precision": 0.9722222222222222, "recall": 0.693069306930693, "support": 202}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9280630016659095, "precision": 0.9702343255224826, "recall": 0.8894049346879536, "support": 3445}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1762}, "macro avg": {"f1-score": 0.5025741252507205, "precision": 0.719547706902091, "recall": 0.4264039739438158, "support": 20679}, "micro avg": {"f1-score": 0.8252301772708535, "precision": 0.9558767350057303, "recall": 0.72600222447894, "support": 20679}, "weighted avg": {"f1-score": 0.796794673143932, "precision": 0.9369195942874564, "recall": 0.72600222447894, "support": 20679}, "\u2205": {"f1-score": 0.9097444482741641, "precision": 0.9452573416555817, "recall": 0.8768033409263477, "support": 10536}, "\u23ce": {"f1-score": 0.43789887088856155, "precision": 0.9083503054989817, "recall": 0.2884864165588616, "support": 1546}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 314}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5878542510121457, "precision": 0.9603174603174603, "recall": 0.4235705950991832, "support": 857}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.3211009174311926, "precision": 0.9722222222222222, "recall": 0.19230769230769232, "support": 728}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 73}, "\u2423": {"f1-score": 0.7639945143997008, "precision": 0.9702343255224826, "recall": 0.6300637466584413, "support": 4863}},
  "ppcr": 0.7595144832922288
}
```
</details>
