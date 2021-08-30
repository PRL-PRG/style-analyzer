# Train report for javascript / file:///tmp/top-repos-quality-repos-ntyhw93u/microstates.git HEAD df4ae5c5984e296e0481b823419df7ee578b64d4

### Classification report

PPCR: 0.795

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.946| 0.991| 0.961| 0.968| 0.953| 13991| 14426| 0.970 |
| `␣` | 0.945| 0.864| 0.634| 0.903| 0.759| 4892| 6668| 0.734 |
| `⏎␣⁻␣⁻` | 0.988| 0.869| 0.780| 0.924| 0.872| 929| 1034| 0.898 |
| `⏎␣⁺␣⁺` | 0.916| 0.908| 0.778| 0.912| 0.841| 916| 1070| 0.856 |
| `⏎⏎` | 0.927| 0.836| 0.553| 0.879| 0.693| 397| 600| 0.662 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 96| 1010| 0.095 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1287| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 584| 0.000 |
| `weighted avg` | 0.941| 0.945| 0.752| 0.942| 0.788| 21221| 26679| 0.795 |
| `micro avg` | 0.945| 0.945| 0.752| 0.945| 0.838| 21221| 26679| 0.795 |
| `macro avg` | 0.590| 0.559| 0.463| 0.573| 0.515| 21221| 26679| 0.795 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|435 |13866 |121 |0 |4 |0 |0 |0 |0 |
|1776 |583 |4227 |0 |72 |10 |0 |0 |0 |
|1287 |0 |0 |0 |0 |0 |0 |0 |0 |
|154 |56 |28 |0 |832 |0 |0 |0 |0 |
|105 |100 |22 |0 |0 |807 |0 |0 |0 |
|914 |38 |32 |0 |0 |0 |0 |0 |26 |
|584 |0 |0 |0 |0 |0 |0 |0 |0 |
|203 |20 |45 |0 |0 |0 |0 |0 |332 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| tests/type-shifting.test.js | 145 |
| README/redux-todomvc.js | 73 |
| tests/identity.test.js | 65 |
| tests/parameterized.test.js | 59 |
| tests/pathmap.test.js | 58 |
| tests/types/object.test.js | 57 |
| tests/types/array.test.js | 54 |
| rollup.config.js | 50 |
| src/dsl.js | 48 |
| README/microstates-todomvc.js | 44 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5733128345603645, "precision": 0.5902075583672268, "recall": 0.5585468052384183, "support": 21221}, "micro avg": {"f1-score": 0.9454785354130343, "precision": 0.9454785354130343, "recall": 0.9454785354130343, "support": 21221}, "weighted avg": {"f1-score": 0.9424416440181361, "precision": 0.9413573171387967, "recall": 0.9454785354130343, "support": 21221}, "\u2205": {"f1-score": 0.9678229915544078, "precision": 0.945645502284662, "recall": 0.9910656850832679, "support": 13991}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u23ce": {"f1-score": 0.8794701986754967, "precision": 0.9273743016759777, "recall": 0.836272040302267, "support": 397}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.912280701754386, "precision": 0.9162995594713657, "recall": 0.9082969432314411, "support": 916}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9243986254295532, "precision": 0.9877600979192166, "recall": 0.868675995694295, "support": 929}, "\u2423": {"f1-score": 0.9025301590690723, "precision": 0.9445810055865922, "recall": 0.8640637775960752, "support": 4892}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 584}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1287}, "macro avg": {"f1-score": 0.5147947731984837, "precision": 0.5902075583672268, "recall": 0.46330900742408065, "support": 26679}, "micro avg": {"f1-score": 0.8377453027139875, "precision": 0.9454785354130343, "recall": 0.7520521758686608, "support": 26679}, "weighted avg": {"f1-score": 0.7882431572599413, "precision": 0.8433058664904363, "recall": 0.7520521758686608, "support": 26679}, "\u2205": {"f1-score": 0.9533500635979235, "precision": 0.945645502284662, "recall": 0.9611812006100097, "support": 14426}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1010}, "\u23ce\u23ce": {"f1-score": 0.6931106471816283, "precision": 0.9273743016759777, "recall": 0.5533333333333333, "support": 600}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8412537917087968, "precision": 0.9162995594713657, "recall": 0.7775700934579439, "support": 1070}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8719611021069692, "precision": 0.9877600979192166, "recall": 0.7804642166344294, "support": 1034}, "\u2423": {"f1-score": 0.7586825809925514, "precision": 0.9445810055865922, "recall": 0.6339232153569286, "support": 6668}},
  "ppcr": 0.7954196184264778
}
```
</details>
