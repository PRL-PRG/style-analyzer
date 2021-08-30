# Train report for javascript / file:///tmp/top-repos-quality-repos-f_a16m61/node-ar-drone.git HEAD 11667d4640a55111a6ad0206336685e69c7323fb

### Classification report

PPCR: 0.746

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.996| 0.979| 0.985| 0.977| 21045| 21414| 0.983 |
| `'` | 1.000| 1.000| 0.954| 1.000| 0.976| 1699| 1781| 0.954 |
| `␣` | 0.898| 0.636| 0.103| 0.745| 0.185| 1043| 6418| 0.163 |
| `⏎` | 0.937| 0.868| 0.330| 0.901| 0.488| 873| 2300| 0.380 |
| `⏎␣⁻␣⁻` | 0.877| 0.959| 0.946| 0.916| 0.911| 627| 635| 0.987 |
| `⏎⏎` | 1.000| 0.906| 0.196| 0.951| 0.328| 171| 791| 0.216 |
| `␣␣␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 47| 83| 0.566 |
| `␣␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 43| 128| 0.336 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 23| 88| 0.261 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 665| 0.029 |
| `weighted avg` | 0.965| 0.971| 0.724| 0.967| 0.752| 25590| 34303| 0.746 |
| `macro avg` | 0.569| 0.536| 0.351| 0.550| 0.386| 25590| 34303| 0.746 |
| `micro avg` | 0.971| 0.971| 0.724| 0.971| 0.829| 25590| 34303| 0.746 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ␣␣␣| ␣␣| ␣␣␣␣␣| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|369 |20961 |35 |0 |0 |0 |0 |49 |0 |0 |0 |
|5375 |307 |663 |38 |0 |0 |0 |35 |0 |0 |0 |
|1427 |76 |39 |758 |0 |0 |0 |0 |0 |0 |0 |
|82 |0 |0 |0 |1699 |0 |0 |0 |0 |0 |0 |
|620 |3 |0 |13 |0 |155 |0 |0 |0 |0 |0 |
|646 |18 |1 |0 |0 |0 |0 |0 |0 |0 |0 |
|8 |26 |0 |0 |0 |0 |0 |601 |0 |0 |0 |
|85 |43 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|65 |23 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|36 |47 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/unit/navdata/test-parseNavdata.js | 184 |
| lib/navdata/parseNavdata.js | 140 |
| lib/control/AtCommandCreator.js | 48 |
| test/unit/control/test-AtCommandCreator.js | 33 |
| lib/Client.js | 31 |
| lib/constants.js | 31 |
| examples/simple_flight.js | 29 |
| test/unit/control/test-UdpControl.js | 25 |
| lib/navdata/NavdataReader.js | 22 |
| test/unit/test-Client.js | 14 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1699}, "macro avg": {"f1-score": 0.5498176344644474, "precision": 0.5687454339342007, "recall": 0.5364910676276912, "support": 25590}, "micro avg": {"f1-score": 0.9705744431418523, "precision": 0.9705744431418523, "recall": 0.9705744431418523, "support": 25590}, "weighted avg": {"f1-score": 0.9665612608905609, "precision": 0.9647781213538942, "recall": 0.9705744431418523, "support": 25590}, "\u2205": {"f1-score": 0.9852640485087781, "precision": 0.9747488839285714, "recall": 0.996008553100499, "support": 21045}, "\u23ce": {"f1-score": 0.9013079667063021, "precision": 0.9369592088998764, "recall": 0.868270332187858, "support": 873}, "\u23ce\u23ce": {"f1-score": 0.950920245398773, "precision": 1.0, "recall": 0.9064327485380117, "support": 171}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9161585365853658, "precision": 0.8773722627737226, "recall": 0.9585326953748007, "support": 627}, "\u2423": {"f1-score": 0.7445255474452556, "precision": 0.8983739837398373, "recall": 0.6356663470757431, "support": 1043}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9764367816091954, "precision": 1.0, "recall": 0.9539584503088153, "support": 1781}, "macro avg": {"f1-score": 0.38644470031992517, "precision": 0.5687454339342007, "recall": 0.3508083673344259, "support": 34303}, "micro avg": {"f1-score": 0.8293790593224584, "precision": 0.9705744431418523, "recall": 0.7240474594058829, "support": 34303}, "weighted avg": {"f1-score": 0.7522458017110135, "precision": 0.9306233972369686, "recall": 0.7240474594058829, "support": 34303}, "\u2205": {"f1-score": 0.9767929540053125, "precision": 0.9747488839285714, "recall": 0.9788456150182124, "support": 21414}, "\u23ce": {"f1-score": 0.48761659697651977, "precision": 0.9369592088998764, "recall": 0.32956521739130434, "support": 2300}, "\u23ce\u23ce": {"f1-score": 0.3276955602536998, "precision": 1.0, "recall": 0.1959544879898862, "support": 791}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 665}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9106060606060608, "precision": 0.8773722627737226, "recall": 0.9464566929133859, "support": 635}, "\u2423": {"f1-score": 0.1852990497484628, "precision": 0.8983739837398373, "recall": 0.10330320972265503, "support": 6418}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 88}, "\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 128}, "\u2423\u2423\u2423\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}},
  "ppcr": 0.7459988922251698
}
```
</details>
