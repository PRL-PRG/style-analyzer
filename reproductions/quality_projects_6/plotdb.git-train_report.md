# Train report for javascript / file:///tmp/top-repos-quality-repos-da_p2t1f/plotdb.git HEAD ad11ce4247c7af1fdf64999ce328f62e61cbca11

### Classification report

PPCR: 0.881

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.986| 0.973| 0.990| 0.983| 498243| 505026| 0.987 |
| `␣` | 0.966| 0.991| 0.916| 0.979| 0.940| 303965| 328956| 0.924 |
| `'` | 0.887| 0.996| 0.737| 0.938| 0.805| 29943| 40500| 0.739 |
| `⏎` | 0.928| 0.898| 0.491| 0.913| 0.642| 28091| 51384| 0.547 |
| `⏎␣⁻␣⁻` | 0.939| 0.961| 0.559| 0.950| 0.701| 13173| 22671| 0.581 |
| `⏎␣⁺␣⁺` | 0.967| 0.939| 0.365| 0.953| 0.530| 8955| 23020| 0.389 |
| `⏎⏎` | 0.971| 0.852| 0.444| 0.908| 0.609| 8954| 17197| 0.521 |
| `"` | 0.977| 0.551| 0.195| 0.705| 0.325| 8490| 24045| 0.353 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 808| 4805| 0.168 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 685| 2950| 0.232 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 212| 469| 0.452 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 139| 760| 0.183 |
| `␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 98| 163| 0.601 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 70| 1332| 0.053 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 51| 255| 0.200 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 216| 0.218 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 91| 0.516 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 493| 0.077 |
| `macro avg` | 0.424| 0.399| 0.260| 0.407| 0.307| 902009| 1024333| 0.881 |
| `weighted avg` | 0.975| 0.977| 0.860| 0.975| 0.896| 902009| 1024333| 0.881 |
| `micro avg` | 0.977| 0.977| 0.860| 0.977| 0.915| 902009| 1024333| 0.881 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁺␣⁺| ⏎⏎␣⁻␣⁻| ␣␣␣| ␣␣| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6783 |491182 |7000 |13 |0 |37 |6 |0 |5 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|24991 |1856 |301333 |722 |0 |30 |24 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|23293 |355 |2285 |25228 |0 |1 |3 |0 |219 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|10557 |0 |0 |0 |29831 |0 |0 |112 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|14065 |279 |200 |65 |0 |8410 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|9498 |437 |42 |30 |0 |0 |12664 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|15555 |0 |0 |0 |3808 |0 |0 |4682 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8243 |29 |400 |897 |0 |0 |1 |0 |7627 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3997 |258 |374 |48 |0 |128 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2265 |14 |62 |88 |0 |0 |521 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1262 |1 |4 |56 |0 |0 |9 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|621 |19 |33 |45 |0 |42 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|455 |25 |11 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|257 |0 |5 |0 |0 |0 |207 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|204 |0 |0 |4 |0 |47 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|169 |0 |0 |0 |0 |0 |47 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|65 |0 |98 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|44 |1 |45 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/js/data/sample.js | 2591 |
| static/assets/medium-editor/5.15.1/js/medium-editor.js | 1634 |
| static/assets/canvg/1.4.0/canvg.js | 1073 |
| static/assets/select2/4.0.3/js/select2.full.js | 696 |
| static/assets/select2/4.0.1/js/select2.full.js | 679 |
| static/lib/d3-scale-chromatic/0.0/index.js | 566 |
| static/assets/select2/4.0.3/js/select2.js | 556 |
| static/lib/d3-geo-projection/1.0/index.js | 544 |
| static/assets/select2/4.0.1/js/select2.js | 539 |
| static/assets/medium-insert/2.2.4/js/medium-editor-insert-plugin.js | 499 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.7049081601927131, "precision": 0.9766374634960368, "recall": 0.5514723203769141, "support": 8490}, "\u0027": {"f1-score": 0.9383473310056306, "precision": 0.8867980617735367, "recall": 0.9962595598303443, "support": 29943}, "macro avg": {"f1-score": 0.40749201379281974, "precision": 0.42380230701256516, "recall": 0.3986267226326226, "support": 902009}, "micro avg": {"f1-score": 0.9766609867529038, "precision": 0.9766609867529038, "recall": 0.9766609867529038, "support": 902009}, "weighted avg": {"f1-score": 0.9749403830990557, "precision": 0.9747702468538726, "recall": 0.9766609867529038, "support": 902009}, "\u2205": {"f1-score": 0.989588989210224, "precision": 0.9933785817140454, "recall": 0.9858282002958396, "support": 498243}, "\u23ce": {"f1-score": 0.9125700850063304, "precision": 0.9275341005184015, "recall": 0.8980812359830551, "support": 28091}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 139}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u23ce": {"f1-score": 0.9077060398690865, "precision": 0.9714686027257674, "recall": 0.8517980790708063, "support": 8954}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 70}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9529745042492918, "precision": 0.9672225416906268, "recall": 0.9391401451702959, "support": 8955}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 808}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9501800720288114, "precision": 0.9392568419491211, "recall": 0.9613603583086616, "support": 13173}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 685}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 212}, "\u2423": {"f1-score": 0.9785810667086677, "precision": 0.966145332358637, "recall": 0.9913411083512904, "support": 303965}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}},
  "cl_report_full": {"\"": {"f1-score": 0.32469919206629916, "precision": 0.9766374634960368, "recall": 0.19471823663963403, "support": 24045}, "\u0027": {"f1-score": 0.804731652706403, "precision": 0.8867980617735367, "recall": 0.736567901234568, "support": 40500}, "macro avg": {"f1-score": 0.3074830071910148, "precision": 0.42380230701256516, "recall": 0.25990628303362734, "support": 1024333}, "micro avg": {"f1-score": 0.9146423636093693, "precision": 0.9766609867529038, "recall": 0.8600298926228092, "support": 1024333}, "weighted avg": {"f1-score": 0.8958885768044391, "precision": 0.9633839751597723, "recall": 0.8600298926228092, "support": 1024333}, "\u2205": {"f1-score": 0.9828731282804494, "precision": 0.9933785817140454, "recall": 0.9725875499479235, "support": 505026}, "\u23ce": {"f1-score": 0.6420727129277325, "precision": 0.9275341005184015, "recall": 0.49096995173594893, "support": 51384}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 760}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 493}, "\u23ce\u23ce": {"f1-score": 0.6089907377834558, "precision": 0.9714686027257674, "recall": 0.44350758853288363, "support": 17197}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 255}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 216}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1332}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5303484155762258, "precision": 0.9672225416906268, "recall": 0.36533449174630755, "support": 23020}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4805}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7005587210267191, "precision": 0.9392568419491211, "recall": 0.558599091350183, "support": 22671}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2950}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 469}, "\u2423": {"f1-score": 0.9404195690709809, "precision": 0.966145332358637, "recall": 0.9160282834178431, "support": 328956}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 91}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}},
  "ppcr": 0.8805818029878956
}
```
</details>
