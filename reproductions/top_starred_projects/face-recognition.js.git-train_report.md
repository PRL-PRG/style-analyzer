# Train report for javascript / file:///tmp/top-repos-quality-repos-lhes_b1s/face-recognition.js.git HEAD 409ffd82793784bbb633621aa463df2a62936407

### Classification report

PPCR: 0.577

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.970| 0.999| 0.718| 0.984| 0.825| 4141| 5758| 0.719 |
| `␣` | 0.975| 0.923| 0.354| 0.948| 0.519| 875| 2284| 0.383 |
| `'` | 1.000| 1.000| 0.789| 1.000| 0.882| 480| 608| 0.789 |
| `⏎␣⁻␣⁻` | 0.944| 0.746| 0.608| 0.833| 0.740| 181| 222| 0.815 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 280| 0.121 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 296| 0.014 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 457| 0.007 |
| `weighted avg` | 0.965| 0.972| 0.561| 0.968| 0.670| 5718| 9905| 0.577 |
| `micro avg` | 0.972| 0.972| 0.561| 0.972| 0.712| 5718| 9905| 0.577 |
| `macro avg` | 0.555| 0.524| 0.353| 0.538| 0.424| 5718| 9905| 0.577 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1617 |4136 |5 |0 |0 |0 |0 |0 |
|1409 |59 |808 |0 |0 |0 |0 |8 |
|128 |0 |0 |480 |0 |0 |0 |0 |
|454 |3 |0 |0 |0 |0 |0 |0 |
|292 |4 |0 |0 |0 |0 |0 |0 |
|246 |34 |0 |0 |0 |0 |0 |0 |
|41 |30 |16 |0 |0 |0 |0 |135 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/src/FaceRecognizer/commons.js | 19 |
| examples/opencv4nodejs/faceRecognition.js | 17 |
| lib/src/FaceRecognizer/AsyncFaceRecognizer.js | 12 |
| lib/src/withCv.js | 12 |
| examples/asyncFaceRecognition.js | 10 |
| lib/index.js | 10 |
| tests/FaceRecognizer/FaceRecognizerTest.js | 8 |
| tests/FaceRecognizer/AsyncFaceRecognizerTest.js | 8 |
| lib/src/FaceRecognizer/FaceRecognizer.js | 7 |
| examples/faceRecognition2.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 480}, "macro avg": {"f1-score": 0.5379474419970519, "precision": 0.5554643867999902, "recall": 0.524011069600397, "support": 5718}, "micro avg": {"f1-score": 0.972193074501574, "precision": 0.972193074501574, "recall": 0.972193074501574, "support": 5718}, "weighted avg": {"f1-score": 0.968021890142569, "precision": 0.9651133360373005, "recall": 0.972193074501574, "support": 5718}, "\u2205": {"f1-score": 0.9839419531342928, "precision": 0.9695264885138303, "recall": 0.9987925621830476, "support": 4141}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8333333333333334, "precision": 0.9440559440559441, "recall": 0.7458563535911602, "support": 181}, "\u2423": {"f1-score": 0.9483568075117371, "precision": 0.9746682750301568, "recall": 0.9234285714285714, "support": 875}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8823529411764706, "precision": 1.0, "recall": 0.7894736842105263, "support": 608}, "macro avg": {"f1-score": 0.4237731196109115, "precision": 0.5554643867999902, "recall": 0.3528074404734372, "support": 9905}, "micro avg": {"f1-score": 0.7116430903155605, "precision": 0.972193074501574, "recall": 0.5612317011610298, "support": 9905}, "weighted avg": {"f1-score": 0.6701623915753712, "precision": 0.870899170177883, "recall": 0.5612317011610298, "support": 9905}, "\u2205": {"f1-score": 0.825219473264166, "precision": 0.9695264885138303, "recall": 0.7183049670024314, "support": 5758}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 457}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 296}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 280}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7397260273972602, "precision": 0.9440559440559441, "recall": 0.6081081081081081, "support": 222}, "\u2423": {"f1-score": 0.5191133954384838, "precision": 0.9746682750301568, "recall": 0.35376532399299476, "support": 2284}},
  "ppcr": 0.5772841998990409
}
```
</details>
