# Train report for javascript / file:///tmp/top-repos-quality-repos-o5j5zc4k/arrow_war.git HEAD 4c07f1ed1d052b2c1646bf2b715504e4ae9e394b

### Classification report

PPCR: 0.675

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 0.994| 0.961| 0.982| 0.965| 7372| 7626| 0.967 |
| `␣` | 0.978| 0.942| 0.478| 0.960| 0.642| 2172| 4285| 0.507 |
| `'` | 1.000| 1.000| 0.351| 1.000| 0.520| 1043| 2972| 0.351 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.964| 0.930| 0.795| 0.947| 0.872| 372| 435| 0.855 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.983| 0.823| 0.667| 0.896| 0.794| 345| 426| 0.810 |
| `⏎` | 0.947| 0.817| 0.201| 0.878| 0.331| 263| 1072| 0.245 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 118| 0.034 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 211| 0.000 |
| `weighted avg` | 0.973| 0.974| 0.657| 0.973| 0.742| 11571| 17145| 0.675 |
| `micro avg` | 0.974| 0.974| 0.657| 0.974| 0.785| 11571| 17145| 0.675 |
| `macro avg` | 0.730| 0.688| 0.432| 0.708| 0.515| 11571| 17145| 0.675 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|254 |7330 |37 |0 |0 |5 |0 |0 |0 |
|2113 |115 |2047 |0 |0 |5 |5 |0 |0 |
|1929 |0 |0 |1043 |0 |0 |0 |0 |0 |
|809 |37 |8 |0 |215 |3 |0 |0 |0 |
|63 |22 |2 |0 |2 |346 |0 |0 |0 |
|81 |54 |0 |0 |7 |0 |284 |0 |0 |
|211 |0 |0 |0 |0 |0 |0 |0 |0 |
|114 |1 |0 |0 |3 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/admin/js/core.js | 62 |
| static/admin/js/admin/DateTimeShortcuts.js | 41 |
| static/admin/js/actions.js | 35 |
| static/admin/js/inlines.js | 34 |
| static/admin/js/SelectFilter2.js | 34 |
| static/admin/js/urlify.js | 25 |
| static/admin/js/SelectBox.js | 24 |
| static/admin/js/calendar.js | 15 |
| static/admin/js/admin/RelatedObjectLookups.js | 10 |
| static/admin/js/collapse.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1043}, "macro avg": {"f1-score": 0.7077036929556718, "precision": 0.7301104666122358, "recall": 0.688442318704441, "support": 11571}, "micro avg": {"f1-score": 0.973554576095411, "precision": 0.973554576095411, "recall": 0.973554576095411, "support": 11571}, "weighted avg": {"f1-score": 0.972920137686159, "precision": 0.9732582972168987, "recall": 0.973554576095411, "support": 11571}, "\u2205": {"f1-score": 0.9818498426093363, "precision": 0.9697049874322, "recall": 0.9943027672273467, "support": 7372}, "\u23ce": {"f1-score": 0.8775510204081634, "precision": 0.947136563876652, "recall": 0.8174904942965779, "support": 263}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9466484268125854, "precision": 0.9637883008356546, "recall": 0.9301075268817204, "support": 372}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8958990536277602, "precision": 0.9826989619377162, "recall": 0.8231884057971014, "support": 345}, "\u2423": {"f1-score": 0.9596812001875293, "precision": 0.9775549188156638, "recall": 0.9424493554327809, "support": 2172}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 211}, "\u0027": {"f1-score": 0.5195516811955169, "precision": 1.0, "recall": 0.3509421265141319, "support": 2972}, "macro avg": {"f1-score": 0.5154671820279868, "precision": 0.7301104666122358, "recall": 0.43155864549854994, "support": 17145}, "micro avg": {"f1-score": 0.78458002507313, "precision": 0.973554576095411, "recall": 0.6570428696412949, "support": 17145}, "weighted avg": {"f1-score": 0.7424279257986555, "precision": 0.9570720983614945, "recall": 0.6570428696412949, "support": 17145}, "\u2205": {"f1-score": 0.9654264076391176, "precision": 0.9697049874322, "recall": 0.961185418305796, "support": 7626}, "\u23ce": {"f1-score": 0.33102386451116245, "precision": 0.947136563876652, "recall": 0.20055970149253732, "support": 1072}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8715365239294709, "precision": 0.9637883008356546, "recall": 0.7954022988505747, "support": 435}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7944055944055944, "precision": 0.9826989619377162, "recall": 0.6666666666666666, "support": 426}, "\u2423": {"f1-score": 0.6417933845430318, "precision": 0.9775549188156638, "recall": 0.47771295215869314, "support": 4285}},
  "ppcr": 0.6748906386701662
}
```
</details>
