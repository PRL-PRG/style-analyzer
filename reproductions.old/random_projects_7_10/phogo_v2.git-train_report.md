# Train report for javascript / file:///tmp/top-repos-quality-repos-2_e7t08_/phogo_v2.git HEAD 513f38b722e565568f7dd8b14c5c8db132e37b3a

### Classification report

PPCR: 0.695

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.996| 0.945| 0.980| 0.955| 3261| 3438| 0.949 |
| `␣` | 0.972| 0.901| 0.587| 0.935| 0.732| 1059| 1626| 0.651 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 0.924| 0.897| 0.961| 0.946| 198| 204| 0.971 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 537| 0.028 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 351| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 198| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 165| 0.000 |
| `macro avg` | 0.420| 0.403| 0.347| 0.411| 0.376| 4533| 6519| 0.695 |
| `micro avg` | 0.968| 0.968| 0.673| 0.968| 0.794| 4533| 6519| 0.695 |
| `weighted avg` | 0.965| 0.968| 0.673| 0.966| 0.716| 4533| 6519| 0.695 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|177 |3249 |12 |0 |0 |0 |0 |0 |
|567 |105 |954 |0 |0 |0 |0 |0 |
|522 |0 |15 |0 |0 |0 |0 |0 |
|351 |0 |0 |0 |0 |0 |0 |0 |
|6 |15 |0 |0 |0 |183 |0 |0 |
|198 |0 |0 |0 |0 |0 |0 |0 |
|165 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| ArduinoSketchbook/PhogoServer/bdata/js/phogo.js | 25 |
| ArduinoSketchbook/PhogoAsyncServer/data/js/phogo.js | 25 |
| ArduinoSketchbook/PhogoServer/data/js/phogo.js | 25 |
| ArduinoSketchbook/PhogoServer/bdata/js/p2_term.js | 22 |
| ArduinoSketchbook/PhogoServer/data/js/p2_term.js | 22 |
| ArduinoSketchbook/PhogoAsyncServer/data/js/p2_term.js | 22 |
| ArduinoSketchbook/PhogoServer/bdata/js/p2_main.js | 2 |
| ArduinoSketchbook/PhogoAsyncServer/data/js/p2_main.js | 2 |
| ArduinoSketchbook/PhogoServer/data/js/p2_main.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.4108592195206368, "precision": 0.41955116945926296, "recall": 0.40305891854192527, "support": 4533}, "micro avg": {"f1-score": 0.9675711449371277, "precision": 0.9675711449371277, "recall": 0.9675711449371277, "support": 4533}, "weighted avg": {"f1-score": 0.9655319453165359, "precision": 0.964637116663082, "recall": 0.9675711449371277, "support": 4533}, "\u2205": {"f1-score": 0.9800904977375566, "precision": 0.9643811219946572, "recall": 0.9963201471941122, "support": 3261}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9606299212598425, "precision": 1.0, "recall": 0.9242424242424242, "support": 198}, "\u2423": {"f1-score": 0.9352941176470588, "precision": 0.9724770642201835, "recall": 0.9008498583569405, "support": 1059}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 165}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 351}, "macro avg": {"f1-score": 0.37603110091901637, "precision": 0.41955116945926296, "recall": 0.3469715526712221, "support": 6519}, "micro avg": {"f1-score": 0.7937024972855592, "precision": 0.9675711449371277, "recall": 0.6728025770823746, "support": 6519}, "weighted avg": {"f1-score": 0.7155842987556336, "precision": 0.782449762822465, "recall": 0.6728025770823746, "support": 6519}, "\u2205": {"f1-score": 0.9546055531070956, "precision": 0.9643811219946572, "recall": 0.9450261780104712, "support": 3438}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 537}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 198}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9457364341085273, "precision": 1.0, "recall": 0.8970588235294118, "support": 204}, "\u2423": {"f1-score": 0.7318757192174914, "precision": 0.9724770642201835, "recall": 0.5867158671586716, "support": 1626}},
  "ppcr": 0.6953520478601013
}
```
</details>
