# Train report for javascript / file:///tmp/top-repos-quality-repos-3zqha9m_/practice.git HEAD 477e21eb8fc6b6b77c74507f1b5a1034e67c50de

### Classification report

PPCR: 0.515

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.953| 0.997| 0.849| 0.974| 0.898| 10235| 12012| 0.852 |
| `␣` | 0.961| 0.854| 0.198| 0.904| 0.328| 1338| 5775| 0.232 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 96| 258| 0.372 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 55| 1223| 0.045 |
| `⏎␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 50| 276| 0.181 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 336| 0.116 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 237| 0.165 |
| `⏎␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 286| 0.122 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 206| 0.053 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 963| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 948| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 589| 0.000 |
| `macro avg` | 0.159| 0.154| 0.087| 0.157| 0.102| 11898| 23109| 0.515 |
| `micro avg` | 0.954| 0.954| 0.491| 0.954| 0.648| 11898| 23109| 0.515 |
| `weighted avg` | 0.928| 0.954| 0.491| 0.940| 0.549| 11898| 23109| 0.515 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1777 |10203 |32 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4437 |195 |1143 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1168 |55 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|963 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|948 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|589 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|297 |38 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|251 |27 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|226 |50 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|162 |96 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|198 |39 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|195 |5 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| React/ajax-async/src/serviceWorker.js | 30 |
| React/router-exercise/src/serviceWorker.js | 30 |
| pallete/src/serviceWorker.js | 30 |
| React/Context/context/src/serviceWorker.js | 30 |
| React/hooks/src/serviceWorker.js | 30 |
| React/pokedex-project/src/serviceWorker.js | 28 |
| React/State/src/serviceWorker.js | 28 |
| React/router-exercise/src/App.js | 12 |
| Js/NEEWW/promise resolve, reject with value.js | 12 |
| React/State/src/CoinContainer.js | 10 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.15655259385480905, "precision": 0.15944526671333456, "recall": 0.15426113025514754, "support": 11898}, "micro avg": {"f1-score": 0.9536056480080686, "precision": 0.9536056480080686, "recall": 0.9536056480080686, "support": 11898}, "weighted avg": {"f1-score": 0.9398621820309806, "precision": 0.9276737077978493, "recall": 0.9536056480080686, "support": 11898}, "\u2205": {"f1-score": 0.9743589743589743, "precision": 0.9528389988793425, "recall": 0.9968734733756717, "support": 10235}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u2423": {"f1-score": 0.9042721518987342, "precision": 0.9605042016806723, "recall": 0.8542600896860987, "support": 1338}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 948}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 963}, "macro avg": {"f1-score": 0.10219699162310547, "precision": 0.15944526671333456, "recall": 0.08727688977688978, "support": 23109}, "micro avg": {"f1-score": 0.6482132144999572, "precision": 0.9536056480080686, "recall": 0.49097754121770737, "support": 23109}, "weighted avg": {"f1-score": 0.5488780065770156, "precision": 0.7353158431452917, "recall": 0.49097754121770737, "support": 23109}, "\u2205": {"f1-score": 0.8981514084507043, "precision": 0.9528389988793425, "recall": 0.8494005994005994, "support": 12012}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1223}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 589}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 336}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 286}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 206}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 258}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 276}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 237}, "\u2423": {"f1-score": 0.32821249102656136, "precision": 0.9605042016806723, "recall": 0.19792207792207792, "support": 5775}},
  "ppcr": 0.5148643385693885
}
```
</details>
