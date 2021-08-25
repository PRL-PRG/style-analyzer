# Train report for javascript / file:///tmp/top-repos-quality-repos-_e3vt3jc/dp-frontend.git HEAD e4ff586fe9626b3a27a1dbb8c5c60921a81537e0

### Classification report

PPCR: 0.385

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.995| 0.605| 0.984| 0.746| 1817| 2986| 0.609 |
| `␣` | 0.975| 0.981| 0.188| 0.978| 0.315| 360| 1879| 0.192 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 175| 0.160 |
| `'` | 1.000| 1.000| 0.686| 1.000| 0.814| 24| 35| 0.686 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 435| 0.018 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 201| 0.035 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 122| 0.000 |
| `macro avg` | 0.421| 0.425| 0.211| 0.423| 0.268| 2244| 5833| 0.385 |
| `micro avg` | 0.974| 0.974| 0.375| 0.974| 0.541| 2244| 5833| 0.385 |
| `weighted avg` | 0.955| 0.974| 0.375| 0.964| 0.489| 2244| 5833| 0.385 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1169 |1808 |9 |0 |0 |0 |0 |0 |
|1519 |7 |353 |0 |0 |0 |0 |0 |
|11 |0 |0 |24 |0 |0 |0 |0 |
|427 |8 |0 |0 |0 |0 |0 |0 |
|194 |7 |0 |0 |0 |0 |0 |0 |
|147 |28 |0 |0 |0 |0 |0 |0 |
|122 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/serviceWorker.js | 19 |
| src/Utils.js | 10 |
| src/store/features/trips.js | 7 |
| src/store/popularFeeds/index.js | 7 |
| src/routes.js | 6 |
| src/store/timeline/index.js | 4 |
| src/store/accordion/index.js | 3 |
| src/components/common/FontAwsome.js | 1 |
| src/App.js | 1 |
| src/store/specs/accordion.spec.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 24}, "macro avg": {"f1-score": 0.4231121304873354, "precision": 0.42117535213241997, "recall": 0.4250860479946886, "support": 2244}, "micro avg": {"f1-score": 0.9737076648841355, "precision": 0.9737076648841355, "recall": 0.9737076648841355, "support": 2244}, "weighted avg": {"f1-score": 0.9642830999603736, "precision": 0.9550592962024629, "recall": 0.9737076648841355, "support": 2244}, "\u2205": {"f1-score": 0.9839455782312926, "precision": 0.9730893433799784, "recall": 0.9950467804072647, "support": 1817}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u2423": {"f1-score": 0.9778393351800554, "precision": 0.9751381215469613, "recall": 0.9805555555555555, "support": 360}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8135593220338984, "precision": 1.0, "recall": 0.6857142857142857, "support": 35}, "macro avg": {"f1-score": 0.26786967932079947, "precision": 0.42117535213241997, "recall": 0.2112960670302469, "support": 5833}, "micro avg": {"f1-score": 0.5410424662622261, "precision": 0.9737076648841355, "recall": 0.3745928338762215, "support": 5833}, "weighted avg": {"f1-score": 0.4885053128560791, "precision": 0.8182632109925178, "recall": 0.3745928338762215, "support": 5833}, "\u2205": {"f1-score": 0.7464905037159372, "precision": 0.9730893433799784, "recall": 0.6054922973878097, "support": 2986}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 435}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 122}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 201}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 175}, "\u2423": {"f1-score": 0.3150379294957608, "precision": 0.9751381215469613, "recall": 0.18786588610963278, "support": 1879}},
  "ppcr": 0.384707697582719
}
```
</details>
