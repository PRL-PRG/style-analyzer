# Train report for javascript / file:///tmp/top-repos-quality-repos-bra8avhe/moook.git HEAD 8f36afb2074ada471327aef79b119ded832e9a38

### Classification report

PPCR: 0.477

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.995| 0.999| 0.753| 0.997| 0.857| 4834| 6410| 0.754 |
| `␣` | 0.988| 0.973| 0.156| 0.981| 0.269| 525| 3276| 0.160 |
| `⏎` | 0.950| 1.000| 0.287| 0.974| 0.441| 190| 661| 0.287 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 183| 0.055 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 109| 0.064 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 121| 0.033 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 80| 0.013 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 320| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 180| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 86| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 117| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 137| 0.000 |
| `macro avg` | 0.244| 0.248| 0.100| 0.246| 0.131| 5571| 11680| 0.477 |
| `micro avg` | 0.992| 0.992| 0.473| 0.992| 0.641| 5571| 11680| 0.477 |
| `weighted avg` | 0.989| 0.992| 0.473| 0.991| 0.571| 5571| 11680| 0.477 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1576 |4828 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2751 |14 |511 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|471 |0 |0 |190 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|320 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|173 |0 |0 |10 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|180 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|86 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|79 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|117 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|102 |7 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|137 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|117 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| analyzer_simulation.js | 13 |
| server.js | 6 |
| web-client/visuals.js | 6 |
| audio-client/visuals.js | 5 |
| audio-client/script.js | 4 |
| analyzer_static.js | 4 |
| audio-client/audio_engine.js | 2 |
| web-client/script.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24598850508894554, "precision": 0.24441984808769388, "recall": 0.2476743437686756, "support": 5571}, "micro avg": {"f1-score": 0.9924609585352719, "precision": 0.9924609585352719, "recall": 0.9924609585352719, "support": 5571}, "weighted avg": {"f1-score": 0.9905016083736501, "precision": 0.9886042514476492, "recall": 0.9924609585352719, "support": 5571}, "\u2205": {"f1-score": 0.9966969446738232, "precision": 0.9946435929130614, "recall": 0.9987587918907737, "support": 4834}, "\u23ce": {"f1-score": 0.9743589743589743, "precision": 0.95, "recall": 1.0, "support": 190}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.980806142034549, "precision": 0.988394584139265, "recall": 0.9733333333333334, "support": 525}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 180}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 320}, "macro avg": {"f1-score": 0.13066960840821837, "precision": 0.24441984808769388, "recall": 0.09971869180700998, "support": 11680}, "micro avg": {"f1-score": 0.6410063184742913, "precision": 0.9924609585352719, "recall": 0.4733732876712329, "support": 11680}, "weighted avg": {"f1-score": 0.571007211152911, "precision": 0.8768489801552188, "recall": 0.4733732876712329, "support": 11680}, "\u2205": {"f1-score": 0.8572443181818181, "precision": 0.9946435929130614, "recall": 0.753198127925117, "support": 6410}, "\u23ce": {"f1-score": 0.4413472706155633, "precision": 0.95, "recall": 0.2874432677760968, "support": 661}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 137}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 183}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 117}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 109}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 80}, "\u2423": {"f1-score": 0.26944371210123913, "precision": 0.988394584139265, "recall": 0.15598290598290598, "support": 3276}},
  "ppcr": 0.4769691780821918
}
```
</details>
