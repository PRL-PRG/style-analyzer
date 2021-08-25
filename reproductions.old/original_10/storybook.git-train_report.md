# Train report for javascript / file:///tmp/top-repos-quality-repos-jq84uay5/storybook.git HEAD d14c915adb7fb204973cfedcd595b7418cb1c2d8

### Classification report

PPCR: 0.959

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.988| 0.985| 0.976| 0.974| 55038| 55204| 0.997 |
| `␣` | 0.928| 0.942| 0.934| 0.935| 0.931| 29918| 30144| 0.993 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 12076| 12076| 1.000 |
| `⏎` | 0.830| 0.842| 0.584| 0.836| 0.686| 4765| 6865| 0.694 |
| `⏎␣⁻␣⁻` | 1.000| 0.849| 0.800| 0.918| 0.889| 3432| 3640| 0.943 |
| `⏎␣⁺␣⁺` | 0.876| 0.555| 0.501| 0.679| 0.637| 3384| 3752| 0.902 |
| `⏎⏎` | 0.942| 0.791| 0.319| 0.860| 0.476| 1122| 2786| 0.403 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 614| 614| 1.000 |
| `weighted avg` | 0.950| 0.951| 0.912| 0.949| 0.923| 110349| 115081| 0.959 |
| `macro avg` | 0.942| 0.871| 0.765| 0.900| 0.824| 110349| 115081| 0.959 |
| `micro avg` | 0.951| 0.951| 0.912| 0.951| 0.931| 110349| 115081| 0.959 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|166 |54403 |626 |0 |0 |9 |0 |0 |0 |
|226 |945 |28168 |0 |547 |258 |0 |0 |0 |
|0 |0 |0 |12076 |0 |0 |0 |0 |0 |
|2100 |322 |377 |0 |4011 |0 |0 |55 |0 |
|368 |419 |1086 |0 |1 |1878 |0 |0 |0 |
|208 |325 |90 |0 |104 |0 |2913 |0 |0 |
|1664 |44 |19 |0 |171 |0 |0 |888 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |614 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/api/src/tests/stories.test.js | 347 |
| addons/docs/src/mdx/mdx-compiler-plugin.js | 125 |
| lib/source-loader/src/abstract-syntax-tree/traverse-helpers.js | 115 |
| examples/official-storybook/stories/addon-actions.stories.js | 112 |
| lib/api/src/tests/refs.test.js | 111 |
| lib/codemod/src/transforms/storiesof-to-csf.js | 105 |
| lib/client-api/src/hooks.test.js | 88 |
| examples/cra-ts-kitchen-sink/src/stories/docgen-tests/types/prop-types.js | 85 |
| lib/codemod/src/transforms/mdx-to-csf.js | 80 |
| lib/codemod/src/transforms/__testfixtures__/update-addon-info/update-addon-info.input.js | 78 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 614}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 12076}, "macro avg": {"f1-score": 0.9004593166408423, "precision": 0.9422706593077341, "recall": 0.8708645961518734, "support": 110349}, "micro avg": {"f1-score": 0.9510824746939256, "precision": 0.9510824746939256, "recall": 0.9510824746939256, "support": 110349}, "weighted avg": {"f1-score": 0.9493158863536576, "precision": 0.950458963929356, "recall": 0.9510824746939256, "support": 110349}, "\u2205": {"f1-score": 0.9758735739398723, "precision": 0.9636012611144568, "recall": 0.98846251680657, "support": 55038}, "\u23ce": {"f1-score": 0.8357120533388895, "precision": 0.8297476210177906, "recall": 0.8417628541448059, "support": 4765}, "\u23ce\u23ce": {"f1-score": 0.8600484261501211, "precision": 0.9416755037115588, "recall": 0.7914438502673797, "support": 1122}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6793271839392296, "precision": 0.8755244755244755, "recall": 0.5549645390070922, "support": 3384}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9182033096926714, "precision": 1.0, "recall": 0.8487762237762237, "support": 3432}, "\u2423": {"f1-score": 0.9345099860659545, "precision": 0.9276164130935916, "recall": 0.9415067852129153, "support": 29918}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 614}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 12076}, "macro avg": {"f1-score": 0.8241752108526958, "precision": 0.9422706593077341, "recall": 0.7654688131578752, "support": 115081}, "micro avg": {"f1-score": 0.9311183072350618, "precision": 0.9510824746939256, "recall": 0.9119750436648969, "support": 115081}, "weighted avg": {"f1-score": 0.9228878013001409, "precision": 0.9479532362305773, "recall": 0.9119750436648969, "support": 115081}, "\u2205": {"f1-score": 0.9744228116995934, "precision": 0.9636012611144568, "recall": 0.9854901818708789, "support": 55204}, "\u23ce": {"f1-score": 0.6856996324472178, "precision": 0.8297476210177906, "recall": 0.5842680262199563, "support": 6865}, "\u23ce\u23ce": {"f1-score": 0.4762670957361222, "precision": 0.9416755037115588, "recall": 0.31873653984206746, "support": 2786}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6369340342547057, "precision": 0.8755244755244755, "recall": 0.5005330490405118, "support": 3752}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8890584465130474, "precision": 1.0, "recall": 0.8002747252747253, "support": 3640}, "\u2423": {"f1-score": 0.9310196661708808, "precision": 0.9276164130935916, "recall": 0.934447983014862, "support": 30144}},
  "ppcr": 0.9588811358955865
}
```
</details>
