# Train report for javascript / file:///tmp/top-repos-quality-repos-4ze3pxpw/vanta.git HEAD 2f5cfabfd4388a362b337a9b74783cb96de92b4f

### Classification report

PPCR: 0.580

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 0.990| 0.800| 0.982| 0.879| 9905| 12264| 0.808 |
| `␣` | 0.964| 0.913| 0.382| 0.938| 0.547| 2884| 6887| 0.419 |
| `⏎␣⁻␣⁻` | 0.971| 0.976| 0.762| 0.973| 0.854| 374| 479| 0.781 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 1632| 0.003 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 492| 0.008 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 494| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 338| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 129| 0.000 |
| `weighted avg` | 0.971| 0.972| 0.564| 0.972| 0.658| 13172| 22715| 0.580 |
| `macro avg` | 0.364| 0.360| 0.243| 0.362| 0.285| 13172| 22715| 0.580 |
| `micro avg` | 0.972| 0.972| 0.564| 0.972| 0.714| 13172| 22715| 0.580 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2359 |9809 |94 |0 |0 |0 |2 |0 |0 |
|4003 |245 |2632 |0 |0 |0 |7 |0 |0 |
|1627 |2 |1 |0 |0 |0 |2 |0 |0 |
|494 |0 |0 |0 |0 |0 |0 |0 |0 |
|488 |1 |3 |0 |0 |0 |0 |0 |0 |
|105 |9 |0 |0 |0 |0 |365 |0 |0 |
|338 |0 |0 |0 |0 |0 |0 |0 |0 |
|129 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/vanta.birds.js | 178 |
| src/vanta.globe.js | 50 |
| src/vanta.net.js | 32 |
| src/vanta.waves.js | 19 |
| src/vanta.rings.js | 18 |
| src/_base.js | 12 |
| src/helpers.js | 10 |
| src/gallery.js | 10 |
| src/vanta.trunk.js | 10 |
| src/vanta.halo.js | 8 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3616641955024311, "precision": 0.3636644691002287, "recall": 0.3598581391738205, "support": 13172}, "micro avg": {"f1-score": 0.9722137868205284, "precision": 0.9722137868205284, "recall": 0.9722137868205284, "support": 13172}, "weighted avg": {"f1-score": 0.9716177540313864, "precision": 0.9714273364520598, "recall": 0.9722137868205284, "support": 13172}, "\u2205": {"f1-score": 0.9823243703369886, "precision": 0.9744685078482018, "recall": 0.9903079252902575, "support": 9905}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9733333333333333, "precision": 0.9707446808510638, "recall": 0.9759358288770054, "support": 374}, "\u2423": {"f1-score": 0.9376558603491272, "precision": 0.9641025641025641, "recall": 0.912621359223301, "support": 2884}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 129}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 494}, "macro avg": {"f1-score": 0.2849642812001466, "precision": 0.3636644691002287, "recall": 0.24299926162860452, "support": 22715}, "micro avg": {"f1-score": 0.7136846211720121, "precision": 0.9722137868205284, "recall": 0.5637684349548756, "support": 22715}, "weighted avg": {"f1-score": 0.6582959416741385, "precision": 0.8389012917170312, "recall": 0.5637684349548756, "support": 22715}, "\u2205": {"f1-score": 0.8785490371697269, "precision": 0.9744685078482018, "recall": 0.7998206131767776, "support": 12264}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1632}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 338}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 492}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8538011695906432, "precision": 0.9707446808510638, "recall": 0.7620041753653445, "support": 479}, "\u2423": {"f1-score": 0.5473640428408026, "precision": 0.9641025641025641, "recall": 0.3821693044867141, "support": 6887}},
  "ppcr": 0.5798811358133392
}
```
</details>
