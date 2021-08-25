# Train report for javascript / file:///tmp/top-repos-quality-repos-jzt8fttm/realm-js.git HEAD ae82ff7263680cba4a015dbf16d2d5a95d17e3cb

### Classification report

PPCR: 0.719

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 0.993| 0.924| 0.992| 0.956| 52692| 56616| 0.931 |
| `␣` | 0.966| 0.992| 0.742| 0.979| 0.840| 16385| 21905| 0.748 |
| `⏎` | 0.979| 0.856| 0.262| 0.913| 0.413| 1946| 6367| 0.306 |
| `⏎⏎` | 0.978| 0.886| 0.165| 0.930| 0.283| 448| 2404| 0.186 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 163| 2493| 0.065 |
| `'` | 1.000| 1.000| 0.024| 1.000| 0.048| 156| 6383| 0.024 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 125| 2593| 0.048 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 118| 0.034 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 111| 0.018 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1085| 0.000 |
| `weighted avg` | 0.980| 0.984| 0.707| 0.982| 0.761| 71921| 100075| 0.719 |
| `micro avg` | 0.984| 0.984| 0.707| 0.984| 0.823| 71921| 100075| 0.719 |
| `macro avg` | 0.491| 0.473| 0.212| 0.481| 0.254| 71921| 100075| 0.719 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3924 |52320 |372 |0 |0 |0 |0 |0 |0 |0 |0 |
|5520 |120 |16262 |0 |0 |3 |0 |0 |0 |0 |0 |
|4421 |164 |110 |1666 |0 |6 |0 |0 |0 |0 |0 |
|6227 |0 |0 |0 |156 |0 |0 |0 |0 |0 |0 |
|1956 |13 |11 |27 |0 |397 |0 |0 |0 |0 |0 |
|2468 |59 |61 |5 |0 |0 |0 |0 |0 |0 |0 |
|2330 |143 |18 |2 |0 |0 |0 |0 |0 |0 |0 |
|1085 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|109 |0 |0 |2 |0 |0 |0 |0 |0 |0 |0 |
|114 |4 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| tests/js/list-tests.js | 85 |
| tests/js/open-behavior-tests.js | 79 |
| tests/js/async-tests.js | 74 |
| tests/js/realm-tests.js | 68 |
| examples/ReactExample/components/todo-listview.js | 65 |
| tests/js/session-tests.js | 61 |
| examples/ReactNativeBenchmarks/benchmarks.js | 52 |
| examples/ReactExample/components/todo-itemsview.js | 44 |
| scripts/download-realm.js | 44 |
| tests/js/schemas.js | 32 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 156}, "macro avg": {"f1-score": 0.4813905160287403, "precision": 0.491317970647294, "recall": 0.4727709060923111, "support": 71921}, "micro avg": {"f1-score": 0.9844273577953588, "precision": 0.9844273577953588, "recall": 0.9844273577953588, "support": 71921}, "weighted avg": {"f1-score": 0.982288854610116, "precision": 0.980484307966141, "recall": 0.9844273577953588, "support": 71921}, "\u2205": {"f1-score": 0.9917073401885987, "precision": 0.990477632849327, "recall": 0.9929401047597358, "support": 52692}, "\u23ce": {"f1-score": 0.913377192982456, "precision": 0.9788484136310224, "recall": 0.8561151079136691, "support": 1946}, "\u23ce\u23ce": {"f1-score": 0.9297423887587821, "precision": 0.9778325123152709, "recall": 0.8861607142857143, "support": 448}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 125}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u2423": {"f1-score": 0.9790782383575664, "precision": 0.9660211476773197, "recall": 0.9924931339639914, "support": 16385}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1085}, "\u0027": {"f1-score": 0.04771371769383698, "precision": 1.0, "recall": 0.024439918533604887, "support": 6383}, "macro avg": {"f1-score": 0.25389308501969554, "precision": 0.491317970647294, "recall": 0.2117751015326524, "support": 100075}, "micro avg": {"f1-score": 0.8232865880601875, "precision": 0.9844273577953588, "recall": 0.7074793904571571, "support": 100075}, "weighted avg": {"f1-score": 0.7608001498530379, "precision": 0.9213451122704355, "recall": 0.7074793904571571, "support": 100075}, "\u2205": {"f1-score": 0.9561490876195873, "precision": 0.990477632849327, "recall": 0.924120389995761, "support": 56616}, "\u23ce": {"f1-score": 0.41293840624612715, "precision": 0.9788484136310224, "recall": 0.26166169310507303, "support": 6367}, "\u23ce\u23ce": {"f1-score": 0.28256227758007113, "precision": 0.9778325123152709, "recall": 0.1651414309484193, "support": 2404}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 111}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2593}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 118}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2493}, "\u2423": {"f1-score": 0.8395673610573325, "precision": 0.9660211476773197, "recall": 0.7423875827436658, "support": 21905}},
  "ppcr": 0.7186709967524356
}
```
</details>
