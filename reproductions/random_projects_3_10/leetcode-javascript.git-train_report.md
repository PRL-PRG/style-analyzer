# Train report for javascript / file:///tmp/top-repos-quality-repos-aue3mxdz/leetcode-javascript.git HEAD db6e2ad866e19671305fdad0f117523e91834fdd

### Classification report

PPCR: 0.866

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 0.991| 0.974| 0.982| 0.974| 6462| 6575| 0.983 |
| `␣` | 0.974| 0.982| 0.893| 0.978| 0.932| 4789| 5263| 0.910 |
| `'` | 1.000| 1.000| 0.645| 1.000| 0.784| 654| 1014| 0.645 |
| `⏎` | 0.919| 0.667| 0.258| 0.773| 0.402| 339| 877| 0.387 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 1.000| 1.000| 1.000| 1.000| 335| 335| 1.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.991| 0.982| 0.669| 0.987| 0.799| 224| 329| 0.681 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 54| 450| 0.120 |
| `weighted avg` | 0.971| 0.975| 0.845| 0.972| 0.879| 12857| 14843| 0.866 |
| `micro avg` | 0.975| 0.975| 0.845| 0.975| 0.905| 12857| 14843| 0.866 |
| `macro avg` | 0.837| 0.803| 0.634| 0.817| 0.699| 12857| 14843| 0.866 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|113 |6401 |61 |0 |0 |0 |0 |0 |
|474 |85 |4702 |0 |0 |0 |0 |2 |
|360 |0 |0 |654 |0 |0 |0 |0 |
|538 |86 |27 |0 |226 |0 |0 |0 |
|396 |2 |32 |0 |20 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |335 |0 |
|105 |0 |4 |0 |0 |0 |0 |220 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| tasks/problem_resource.js | 20 |
| testcase/001/situation_test.js | 20 |
| testcase/004/random_test.js | 18 |
| solution/005/solution.js | 13 |
| solution/002/solution.js | 12 |
| solution/004/solution.js | 11 |
| testcase/014/random_test.js | 11 |
| testcase/001/random_test.js | 11 |
| Gruntfile.js | 10 |
| solution/001/solution.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 654}, "macro avg": {"f1-score": 0.8170430697936849, "precision": 0.8368114616939578, "recall": 0.8030290128608462, "support": 12857}, "micro avg": {"f1-score": 0.9751886132068134, "precision": 0.9751886132068134, "recall": 0.9751886132068134, "support": 12857}, "weighted avg": {"f1-score": 0.9723750446275681, "precision": 0.9707022679154844, "recall": 0.9751886132068134, "support": 12857}, "\u2205": {"f1-score": 0.9820497084995399, "precision": 0.9736842105263158, "recall": 0.9905601980810894, "support": 6462}, "\u23ce": {"f1-score": 0.7726495726495726, "precision": 0.9186991869918699, "recall": 0.6666666666666666, "support": 339}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9865470852017937, "precision": 0.990990990990991, "recall": 0.9821428571428571, "support": 224}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 335}, "\u2423": {"f1-score": 0.9780551222048882, "precision": 0.9743058433485288, "recall": 0.9818333681353101, "support": 4789}},
  "cl_report_full": {"\u0027": {"f1-score": 0.7841726618705036, "precision": 1.0, "recall": 0.6449704142011834, "support": 1014}, "macro avg": {"f1-score": 0.6987040728789433, "precision": 0.8368114616939578, "recall": 0.6340432914956163, "support": 14843}, "micro avg": {"f1-score": 0.9052707581227437, "precision": 0.9751886132068134, "recall": 0.8447079431381797, "support": 14843}, "weighted avg": {"f1-score": 0.8794052168439108, "precision": 0.9439116459463543, "recall": 0.8447079431381797, "support": 14843}, "\u2205": {"f1-score": 0.9736101604684766, "precision": 0.9736842105263158, "recall": 0.9735361216730039, "support": 6575}, "\u23ce": {"f1-score": 0.402493321460374, "precision": 0.9186991869918699, "recall": 0.2576966932725199, "support": 877}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 450}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.7985480943738658, "precision": 0.990990990990991, "recall": 0.668693009118541, "support": 329}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 335}, "\u2423": {"f1-score": 0.9321042719793834, "precision": 0.9743058433485288, "recall": 0.8934068022040661, "support": 5263}},
  "ppcr": 0.8661995553459543
}
```
</details>
