# Train report for javascript / file:///tmp/top-repos-quality-repos-ck818s4y/resumake.io.git HEAD 9f52bcc874b54e1449b9225ab9a117678238d01f

### Classification report

PPCR: 0.691

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.954| 0.985| 0.897| 0.969| 0.925| 14805| 16251| 0.911 |
| `␣` | 0.949| 0.923| 0.466| 0.936| 0.625| 5043| 10004| 0.504 |
| `'` | 1.000| 1.000| 0.851| 1.000| 0.920| 2918| 3428| 0.851 |
| `⏎` | 0.956| 0.913| 0.524| 0.934| 0.677| 1128| 1966| 0.574 |
| `⏎⏎` | 0.926| 0.980| 0.564| 0.952| 0.701| 601| 1045| 0.575 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 190| 1246| 0.152 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 104| 1382| 0.075 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 84| 0.202 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 77| 0.104 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 322| 0.000 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 85| 0.000 |
| `weighted avg` | 0.946| 0.958| 0.662| 0.952| 0.738| 24814| 35890| 0.691 |
| `micro avg` | 0.958| 0.958| 0.662| 0.958| 0.783| 24814| 35890| 0.691 |
| `macro avg` | 0.435| 0.436| 0.300| 0.436| 0.350| 24814| 35890| 0.691 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1446 |14579 |197 |0 |0 |0 |0 |29 |0 |0 |0 |0 |
|4961 |344 |4657 |0 |39 |0 |0 |3 |0 |0 |0 |0 |
|510 |0 |0 |2918 |0 |0 |0 |0 |0 |0 |0 |0 |
|838 |91 |5 |0 |1030 |0 |0 |2 |0 |0 |0 |0 |
|1278 |90 |14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1056 |141 |32 |0 |4 |0 |0 |13 |0 |0 |0 |0 |
|444 |8 |0 |0 |4 |0 |0 |589 |0 |0 |0 |0 |
|322 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|85 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|69 |8 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|67 |17 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/client/src/features/form/tests/reducer.js | 106 |
| app/client/src/features/form/reducer.js | 56 |
| app/client/src/features/form/components/Form.js | 33 |
| app/server/src/generator/templates/template2/index.js | 28 |
| app/server/src/generator/templates/template1/index.js | 27 |
| app/client/src/features/preview/components/Toolbar.js | 26 |
| app/client/src/features/form/actions.js | 26 |
| app/server/src/generator/templates/template4/index.js | 26 |
| app/server/src/generator/templates/template7/index.js | 26 |
| app/client/src/features/form/components/fragments/Job.js | 25 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2918}, "macro avg": {"f1-score": 0.4356422415974945, "precision": 0.43510437952077635, "recall": 0.43648609009831657, "support": 24814}, "micro avg": {"f1-score": 0.95804787619892, "precision": 0.95804787619892, "recall": 0.95804787619892, "support": 24814}, "weighted avg": {"f1-score": 0.9517019548374123, "precision": 0.9457977031692475, "recall": 0.95804787619892, "support": 24814}, "\u2205": {"f1-score": 0.9692517368613501, "precision": 0.9542479382118079, "recall": 0.9847348868625464, "support": 14805}, "\u23ce": {"f1-score": 0.9342403628117913, "precision": 0.9563602599814299, "recall": 0.9131205673758865, "support": 1128}, "\u23ce\u23ce": {"f1-score": 0.952303961196443, "precision": 0.9261006289308176, "recall": 0.9800332778702163, "support": 601}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 190}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.9362685967028548, "precision": 0.9494393476044852, "recall": 0.9234582589728336, "support": 5043}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 322}, "\u0027": {"f1-score": 0.9196344153797668, "precision": 1.0, "recall": 0.8512252042007001, "support": 3428}, "macro avg": {"f1-score": 0.34971763635786074, "precision": 0.43510437952077635, "recall": 0.300126890456713, "support": 35890}, "micro avg": {"f1-score": 0.78324327886136, "precision": 0.95804787619892, "recall": 0.662385065477849, "support": 35890}, "weighted avg": {"f1-score": 0.7382104269600643, "precision": 0.8715980469677224, "recall": 0.662385065477849, "support": 35890}, "\u2205": {"f1-score": 0.9247993910368233, "precision": 0.9542479382118079, "recall": 0.8971140237523845, "support": 16251}, "\u23ce": {"f1-score": 0.6769635228393033, "precision": 0.9563602599814299, "recall": 0.5239064089521872, "support": 1966}, "\u23ce\u23ce": {"f1-score": 0.7007733491969067, "precision": 0.9261006289308176, "recall": 0.5636363636363636, "support": 1045}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 85}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1382}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 77}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1246}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u2423": {"f1-score": 0.6247233214836677, "precision": 0.9494393476044852, "recall": 0.46551379448220714, "support": 10004}},
  "ppcr": 0.6913903594315965
}
```
</details>
