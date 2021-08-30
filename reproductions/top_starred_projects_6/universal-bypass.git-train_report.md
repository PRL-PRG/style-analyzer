# Train report for javascript / file:///tmp/top-repos-quality-repos-wzyxmepu/universal-bypass.git HEAD 404d1a46fd22020635a977fdf8767ce707b10a76

### Classification report

PPCR: 0.882

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.997| 0.955| 0.990| 0.969| 17785| 18568| 0.958 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 3292| 3292| 1.000 |
| `⏎` | 0.948| 0.974| 0.532| 0.960| 0.682| 1022| 1870| 0.547 |
| `⏎⇥⁺` | 0.989| 0.994| 0.941| 0.992| 0.965| 834| 881| 0.947 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 270| 829| 0.326 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 881| 0.002 |
| `micro avg` | 0.984| 0.984| 0.868| 0.984| 0.922| 23205| 26321| 0.882 |
| `weighted avg` | 0.973| 0.984| 0.868| 0.978| 0.889| 23205| 26321| 0.882 |
| `macro avg` | 0.653| 0.661| 0.571| 0.657| 0.602| 23205| 26321| 0.882 |

### Confusion matrix

|refusal|  ∅| "| ⏎| ␣| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|783 |17724 |0 |52 |0 |9 |0 |
|0 |0 |3292 |0 |0 |0 |0 |
|848 |27 |0 |995 |0 |0 |0 |
|559 |270 |0 |0 |0 |0 |0 |
|47 |2 |0 |3 |0 |829 |0 |
|879 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| background.js | 235 |
| html/options.js | 53 |
| injection_script.js | 50 |
| content_script.js | 13 |
| html/base.js | 5 |
| html/crowd-bypassed.js | 4 |
| html/i18n.js | 2 |
| html/before-navigate.js | 2 |
| html/popup.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3292}, "macro avg": {"f1-score": 0.656990432759477, "precision": 0.6533633602818208, "recall": 0.6606926921415938, "support": 23205}, "micro avg": {"f1-score": 0.9842706313294548, "precision": 0.9842706313294548, "recall": 0.9842706313294548, "support": 23205}, "weighted avg": {"f1-score": 0.9784866998250671, "precision": 0.9727867870322847, "recall": 0.9842706313294548, "support": 23205}, "\u2205": {"f1-score": 0.9898910918737782, "precision": 0.9833009708737864, "recall": 0.9965701433792522, "support": 17785}, "\u23ce": {"f1-score": 0.9604247104247103, "precision": 0.9476190476190476, "recall": 0.9735812133072407, "support": 1022}, "\u23ce\u21e5\u207a": {"f1-score": 0.9916267942583733, "precision": 0.9892601431980907, "recall": 0.9940047961630696, "support": 834}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 270}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3292}, "macro avg": {"f1-score": 0.6024551607215755, "precision": 0.6533633602818208, "recall": 0.5712678632489009, "support": 26321}, "micro avg": {"f1-score": 0.9223438194079877, "precision": 0.9842706313294548, "recall": 0.8677481858591999, "support": 26321}, "weighted avg": {"f1-score": 0.8891440139936739, "precision": 0.9191716968348317, "recall": 0.8677481858591999, "support": 26321}, "\u2205": {"f1-score": 0.9687098625420162, "precision": 0.9833009708737864, "recall": 0.9545454545454546, "support": 18568}, "\u23ce": {"f1-score": 0.6815068493150686, "precision": 0.9476190476190476, "recall": 0.5320855614973262, "support": 1870}, "\u23ce\u21e5\u207a": {"f1-score": 0.9645142524723677, "precision": 0.9892601431980907, "recall": 0.9409761634506243, "support": 881}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 881}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 829}},
  "ppcr": 0.8816154401428518
}
```
</details>
