# Train report for javascript / file:///tmp/top-repos-quality-repos-gu2dgx8k/carbon.git HEAD 02ddcbdcb34e01efdf500cc4b55fc0d5dc6feefc

### Classification report

PPCR: 0.679

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.990| 0.835| 0.981| 0.899| 17524| 20773| 0.844 |
| `'` | 1.000| 1.000| 0.988| 1.000| 0.994| 5291| 5356| 0.988 |
| `␣` | 0.953| 0.941| 0.395| 0.947| 0.558| 4105| 9787| 0.419 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 1168| 1168| 1.000 |
| `⏎` | 0.979| 0.954| 0.253| 0.966| 0.402| 934| 3523| 0.265 |
| `⏎␣⁺␣⁺` | 0.952| 0.602| 0.175| 0.737| 0.296| 359| 1233| 0.291 |
| `⏎␣⁻␣⁻` | 0.911| 0.637| 0.173| 0.750| 0.291| 322| 1182| 0.272 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 733| 0.015 |
| `micro avg` | 0.975| 0.975| 0.662| 0.975| 0.789| 29714| 43755| 0.679 |
| `weighted avg` | 0.975| 0.975| 0.662| 0.974| 0.748| 29714| 43755| 0.679 |
| `macro avg` | 0.846| 0.765| 0.477| 0.798| 0.555| 29714| 43755| 0.679 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| "| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3249 |17350 |164 |0 |0 |4 |0 |6 |0 |
|5682 |217 |3863 |0 |4 |7 |0 |14 |0 |
|65 |0 |0 |5291 |0 |0 |0 |0 |0 |
|2589 |40 |3 |0 |891 |0 |0 |0 |0 |
|874 |128 |15 |0 |0 |216 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |1168 |0 |0 |
|860 |103 |8 |0 |6 |0 |0 |205 |0 |
|722 |2 |0 |0 |9 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| components/Carbon.js | 62 |
| lib/custom/modes/nim.js | 58 |
| components/Editor.js | 53 |
| components/Settings.js | 34 |
| components/ImagePicker.js | 26 |
| lib/routing.js | 23 |
| lib/custom/modes/apache.js | 22 |
| lib/api.js | 21 |
| lib/custom/modes/riscv.js | 20 |
| components/SelectionEditor.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1168}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5291}, "macro avg": {"f1-score": 0.7976739318962413, "precision": 0.8459285771962799, "recall": 0.7654246238966755, "support": 29714}, "micro avg": {"f1-score": 0.9754324560813085, "precision": 0.9754324560813085, "recall": 0.9754324560813085, "support": 29714}, "weighted avg": {"f1-score": 0.9742947592958621, "precision": 0.9747500126147363, "recall": 0.9754324560813085, "support": 29714}, "\u2205": {"f1-score": 0.9812238434566225, "precision": 0.9725336322869955, "recall": 0.9900707601004337, "support": 17524}, "\u23ce": {"f1-score": 0.9663774403470716, "precision": 0.9791208791208791, "recall": 0.9539614561027837, "support": 934}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7372013651877133, "precision": 0.9515418502202643, "recall": 0.6016713091922006, "support": 359}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7495429616087752, "precision": 0.9111111111111111, "recall": 0.6366459627329193, "support": 322}, "\u2423": {"f1-score": 0.9470458445697475, "precision": 0.9531211448309894, "recall": 0.941047503045067, "support": 4105}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1168}, "\u0027": {"f1-score": 0.9938949938949938, "precision": 1.0, "recall": 0.9878640776699029, "support": 5356}, "macro avg": {"f1-score": 0.5550085906972771, "precision": 0.8459285771962799, "recall": 0.47741461576680333, "support": 43755}, "micro avg": {"f1-score": 0.7890130531244469, "precision": 0.9754324560813085, "recall": 0.6624157239172666, "support": 43755}, "weighted avg": {"f1-score": 0.7484428043870209, "precision": 0.9542741419210702, "recall": 0.6624157239172666, "support": 43755}, "\u2205": {"f1-score": 0.8986610726957243, "precision": 0.9725336322869955, "recall": 0.8352187936263419, "support": 20773}, "\u23ce": {"f1-score": 0.40198511166253104, "precision": 0.9791208791208791, "recall": 0.2529094521714448, "support": 3523}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 733}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.29589041095890406, "precision": 0.9515418502202643, "recall": 0.17518248175182483, "support": 1233}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.2914001421464108, "precision": 0.9111111111111111, "recall": 0.17343485617597293, "support": 1182}, "\u2423": {"f1-score": 0.5582369942196532, "precision": 0.9531211448309894, "recall": 0.3947072647389394, "support": 9787}},
  "ppcr": 0.6790995314821163
}
```
</details>
