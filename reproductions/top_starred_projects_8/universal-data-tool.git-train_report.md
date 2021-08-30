# Train report for javascript / file:///tmp/top-repos-quality-repos-rnviw1oa/universal-data-tool.git HEAD 1e6d01c99ba437a4242f336ba0504ea352f73ed0

### Classification report

PPCR: 0.821

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.944| 0.992| 0.917| 0.967| 0.930| 40005| 43262| 0.925 |
| `␣` | 0.966| 0.936| 0.727| 0.950| 0.830| 18454| 23742| 0.777 |
| `"` | 1.000| 1.000| 0.894| 1.000| 0.944| 9480| 10602| 0.894 |
| `⏎` | 0.933| 0.858| 0.463| 0.894| 0.619| 3669| 6799| 0.540 |
| `⏎␣⁺␣⁺` | 0.893| 0.703| 0.438| 0.787| 0.588| 2185| 3501| 0.624 |
| `⏎␣⁻␣⁻` | 0.938| 0.743| 0.471| 0.829| 0.627| 2076| 3273| 0.634 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 115| 1228| 0.094 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 104| 173| 0.601 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 97| 0.216 |
| `weighted avg` | 0.951| 0.954| 0.784| 0.952| 0.845| 76109| 92677| 0.821 |
| `macro avg` | 0.630| 0.581| 0.435| 0.603| 0.504| 76109| 92677| 0.821 |
| `micro avg` | 0.954| 0.954| 0.784| 0.954| 0.861| 76109| 92677| 0.821 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3257 |39670 |293 |0 |9 |0 |33 |0 |0 |0 |
|5288 |879 |17268 |0 |129 |171 |7 |0 |0 |0 |
|1122 |0 |0 |9480 |0 |0 |0 |0 |0 |0 |
|3130 |448 |72 |0 |3149 |0 |0 |0 |0 |0 |
|1316 |430 |220 |0 |0 |1535 |0 |0 |0 |0 |
|1197 |483 |25 |0 |25 |0 |1543 |0 |0 |0 |
|1113 |48 |5 |0 |62 |0 |0 |0 |0 |0 |
|69 |41 |0 |0 |1 |0 |62 |0 |0 |0 |
|76 |8 |1 |0 |0 |12 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/ConfigureDataEntry/index.js | 74 |
| src/components/StartingPage/index.js | 70 |
| src/utils/dataset-helper/index.js | 68 |
| src/components/SampleContainer/index.js | 51 |
| src/components/ImportPage/index.js | 51 |
| src/components/ImageClassification/index.js | 50 |
| src/utils/ria-format.js | 49 |
| src/components/ImageSegmentation/index.js | 49 |
| src/components/LoginDrawer/CompleteSignUp.js | 46 |
| src/components/StartingPage/templates.js | 46 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 9480}, "macro avg": {"f1-score": 0.6030938983787821, "precision": 0.630492663202201, "recall": 0.5812670633981475, "support": 76109}, "micro avg": {"f1-score": 0.9544863288178796, "precision": 0.9544863288178796, "recall": 0.9544863288178796, "support": 76109}, "weighted avg": {"f1-score": 0.9518099212136997, "precision": 0.9512750009034574, "recall": 0.9544863288178796, "support": 76109}, "\u2205": {"f1-score": 0.967419402038726, "precision": 0.9443664151212893, "recall": 0.991626046744157, "support": 40005}, "\u23ce": {"f1-score": 0.8940942646223735, "precision": 0.933037037037037, "recall": 0.8582720087217225, "support": 3669}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7865744299256981, "precision": 0.8934807916181606, "recall": 0.7025171624713958, "support": 2185}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8293469497446923, "precision": 0.9379939209726443, "recall": 0.7432562620423893, "support": 2076}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "\u2423": {"f1-score": 0.9504100390775496, "precision": 0.9655558040706776, "recall": 0.9357320906036631, "support": 18454}},
  "cl_report_full": {"\"": {"f1-score": 0.9441290708096803, "precision": 1.0, "recall": 0.8941709111488398, "support": 10602}, "macro avg": {"f1-score": 0.5043361092906865, "precision": 0.630492663202201, "recall": 0.4346106712935914, "support": 92677}, "micro avg": {"f1-score": 0.8607941416942164, "precision": 0.9544863288178796, "recall": 0.7838514410263604, "support": 92677}, "weighted avg": {"f1-score": 0.8446929201039608, "precision": 0.9379161487838051, "recall": 0.7838514410263604, "support": 92677}, "\u2205": {"f1-score": 0.9304671099696256, "precision": 0.9443664151212893, "recall": 0.9169710138227544, "support": 43262}, "\u23ce": {"f1-score": 0.619028897188913, "precision": 0.933037037037037, "recall": 0.4631563465215473, "support": 6799}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1228}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.588235294117647, "precision": 0.8934807916181606, "recall": 0.4384461582405027, "support": 3501}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6274908499389995, "precision": 0.9379939209726443, "recall": 0.4714329361442102, "support": 3273}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 173}, "\u2423": {"f1-score": 0.8296737615913132, "precision": 0.9655558040706776, "recall": 0.7273186757644681, "support": 23742}},
  "ppcr": 0.8212285680373771
}
```
</details>
